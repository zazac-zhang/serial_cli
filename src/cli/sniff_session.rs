//! Sniff session manager
//!
//! Tracks active sniffing sessions across CLI invocations using file-based state.
//! When `sniff start` runs, it spawns a background child process and records
//! the session metadata (PID, port, output path) in a session file.
//! Subsequent commands (`stop`, `stats`, `save`) read the session file and
//! interact with the running process.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::{Result, SerialError};

/// Session file directory name (under the user's config/cache dir)
const SESSION_DIR_NAME: &str = "serial_cli";
const SESSION_FILE_NAME: &str = "sniff_session.json";

/// Active sniff session metadata persisted to disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SniffSessionMeta {
    /// Process ID of the background sniffing process
    pub pid: u32,
    /// Port being sniffed
    pub port: String,
    /// Output file path (where captured packets are saved)
    pub output_path: Option<PathBuf>,
    /// Start timestamp (UNIX epoch seconds)
    pub started_at: u64,
    /// Sniffer config: max_packets
    pub max_packets: usize,
    /// Sniffer config: hex_display
    pub hex_display: bool,
}

/// Get the directory where session files are stored
fn session_dir() -> Result<PathBuf> {
    let cache = directories::BaseDirs::new()
        .ok_or_else(|| {
            SerialError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine user home directory",
            ))
        })?
        .cache_dir()
        .to_path_buf();
    let dir = cache.join(SESSION_DIR_NAME);
    fs::create_dir_all(&dir).map_err(SerialError::Io)?;
    Ok(dir)
}

/// Get the session file path
fn session_file() -> Result<PathBuf> {
    Ok(session_dir()?.join(SESSION_FILE_NAME))
}

/// Save session metadata to disk
pub fn save_session(meta: &SniffSessionMeta) -> Result<()> {
    let path = session_file()?;
    let json = serde_json::to_string_pretty(meta).map_err(|e| {
        SerialError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
    })?;
    fs::write(&path, json).map_err(SerialError::Io)?;
    Ok(())
}

/// Load session metadata from disk
pub fn load_session() -> Result<Option<SniffSessionMeta>> {
    let path = session_file()?;
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&path).map_err(SerialError::Io)?;
    let meta: SniffSessionMeta = serde_json::from_str(&content).map_err(|e| {
        SerialError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
    })?;
    Ok(Some(meta))
}

/// Remove the session file (clears active session)
pub fn clear_session() -> Result<()> {
    let path = session_file()?;
    if path.exists() {
        fs::remove_file(&path).map_err(SerialError::Io)?;
    }
    Ok(())
}

/// Check if a process with the given PID is still running
#[cfg(unix)]
pub fn is_process_running(pid: u32) -> bool {
    // SAFETY: kill syscall with sig=0 is the standard POSIX way to check process existence
    unsafe {
        libc::kill(pid as libc::pid_t, 0) == 0
    }
}

#[cfg(windows)]
pub fn is_process_running(pid: u32) -> bool {
    use windows::Win32::Foundation::{CloseHandle, HANDLE};
    use windows::Win32::System::Threading::{
        OpenProcess, PROCESS_QUERY_INFORMATION, SYNCHRONIZE,
    };
    let rights = PROCESS_QUERY_INFORMATION | SYNCHRONIZE;
    unsafe {
        let handle = OpenProcess(rights, false, pid);
        if handle.is_ok() {
            let h = handle.unwrap();
            if h.0.is_null() {
                false
            } else {
                CloseHandle(h).ok();
                true
            }
        } else {
            false
        }
    }
}

/// Send SIGTERM to a process
#[cfg(unix)]
pub fn stop_process(pid: u32) -> Result<()> {
    // SAFETY: kill with SIGTERM is the standard way to terminate a process
    let ret = unsafe { libc::kill(pid as libc::pid_t, libc::SIGTERM) };
    if ret != 0 {
        return Err(SerialError::Io(std::io::Error::other(format!(
            "Failed to send SIGTERM to process {}",
            pid
        ))));
    }
    Ok(())
}

#[cfg(windows)]
pub fn stop_process(pid: u32) -> Result<()> {
    use windows::Win32::Foundation::{CloseHandle, HANDLE, WIN32_ERROR};
    use windows::Win32::System::Threading::{
        OpenProcess, PROCESS_TERMINATE, TerminateProcess,
    };
    let rights = PROCESS_TERMINATE;
    let handle = unsafe { OpenProcess(rights, false, pid) }.map_err(|e| {
        SerialError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to open process {}: {:?}", pid, e),
        ))
    })?;
    unsafe {
        let result = TerminateProcess(HANDLE(handle.0), 1);
        CloseHandle(HANDLE(handle.0)).ok();
        if result.is_ok() {
            Ok(())
        } else {
            Err(SerialError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to terminate process {}", pid),
            )))
        }
    }
}

/// Spawn a background sniffing process
///
/// Re-executes the current binary with `--sniff-daemon` arguments,
/// detaches it from the controlling terminal, and writes the session file.
pub fn spawn_sniff_daemon(
    port: &str,
    output: Option<&Path>,
    max_packets: usize,
    hex_display: bool,
) -> Result<SniffSessionMeta> {
    let current_exe = std::env::current_exe().map_err(SerialError::Io)?;

    let mut args = vec![
        "__sniff_daemon__".to_string(),
        "--port".to_string(),
        port.to_string(),
        "--max-packets".to_string(),
        max_packets.to_string(),
        "--hex".to_string(),
        hex_display.to_string(),
    ];

    if let Some(out) = output {
        args.push("--output".to_string());
        args.push(out.to_string_lossy().to_string());
    }

    let mut child = Command::new(&current_exe)
        .args(&args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(SerialError::Io)?;

    // Give the daemon a moment to start up and open the port
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Check if the child is still alive
    if let Ok(Some(status)) = child.try_wait() {
        // Process exited quickly — likely failed to start
        let stderr = read_pipe(&mut child);
        return Err(SerialError::Io(std::io::Error::other(format!(
            "Sniff daemon exited with status: {:?}\nstderr: {}",
            status, stderr
        ))));
    }

    let pid = child.id();

    let output_path = output.map(|p| p.to_path_buf());

    let meta = SniffSessionMeta {
        pid,
        port: port.to_string(),
        output_path,
        started_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        max_packets,
        hex_display,
    };

    save_session(&meta)?;

    // Detach: let the child run independently
    // Explicitly close stdio pipes so the daemon doesn't block on writes,
    // then forget the Child so it outlives the parent.
    let _ = child.stdin.take();
    let _ = child.stdout.take();
    let _ = child.stderr.take();
    std::mem::forget(child);

    Ok(meta)
}

#[cfg(unix)]
fn read_pipe(child: &mut std::process::Child) -> String {
    let mut buf = Vec::new();
    if let Some(ref mut stderr_pipe) = child.stderr {
        let _ = std::io::Read::read_to_end(stderr_pipe, &mut buf);
    }
    String::from_utf8_lossy(&buf).to_string()
}

#[cfg(windows)]
fn read_pipe(child: &mut std::process::Child) -> String {
    let mut buf = Vec::new();
    if let Some(ref mut stderr_pipe) = child.stderr {
        let _ = std::io::Read::read_to_end(stderr_pipe, &mut buf);
    }
    String::from_utf8_lossy(&buf).to_string()
}

/// Stop the active sniff session
pub fn stop_active_session() -> Result<()> {
    if let Some(meta) = load_session()? {
        if !is_process_running(meta.pid) {
            // Process already dead, clean up
            clear_session()?;
            return Err(SerialError::Io(std::io::Error::other(
                "Sniff process is no longer running (session cleaned up)",
            )));
        }

        println!(
            "Stopping sniff session on port {} (PID {})...",
            meta.port, meta.pid
        );

        stop_process(meta.pid)?;

        // Wait briefly for the process to exit
        std::thread::sleep(std::time::Duration::from_millis(200));

        if is_process_running(meta.pid) {
            println!("Process did not exit gracefully, sending SIGKILL...");
            #[cfg(unix)]
            {
                // SAFETY: SIGKILL to force-terminate a non-responsive process
                unsafe { libc::kill(meta.pid as libc::pid_t, libc::SIGKILL) };
            }
            std::thread::sleep(std::time::Duration::from_millis(200));
        }

        clear_session()?;
        println!("Sniff session stopped successfully");

        Ok(())
    } else {
        Err(SerialError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No active sniff session",
        )))
    }
}

/// Get stats for the active sniff session
pub fn get_session_stats() -> Result<SniffSessionMeta> {
    if let Some(meta) = load_session()? {
        if !is_process_running(meta.pid) {
            clear_session()?;
            return Err(SerialError::Io(std::io::Error::other(
                "Sniff process is no longer running (session cleaned up)",
            )));
        }
        Ok(meta)
    } else {
        Err(SerialError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No active sniff session",
        )))
    }
}

/// Read captured packets from the session's output file
pub fn read_captured_packets(output_path: &Path) -> Result<String> {
    let content = fs::read_to_string(output_path).map_err(SerialError::Io)?;
    Ok(content)
}

/// The actual sniff daemon logic — runs in a spawned child process
pub async fn run_sniff_daemon(
    port: &str,
    output: Option<&std::path::Path>,
    max_packets: usize,
    hex_display: bool,
) -> Result<()> {
    use crate::serial_core::{SerialSniffer, SnifferConfig};

    tracing::info!("[sniff-daemon] Starting on port: {}", port);

    let mut sniffer_config = SnifferConfig {
        max_packets,
        hex_display,
        ..SnifferConfig::default()
    };

    if output.is_some() {
        sniffer_config.save_to_file = true;
        if let Some(out_path) = output {
            if let Some(parent) = out_path.parent() {
                sniffer_config.output_dir = parent.to_path_buf();
            }
        }
    }

    let sniffer = SerialSniffer::new(sniffer_config);

    match sniffer.start_sniffing(port).await {
        Ok(session) => {
            tracing::info!("[sniff-daemon] Sniffing started on port: {}", port);

            // Run until SIGTERM or Ctrl+C
            loop {
                if !session.is_running().await {
                    break;
                }

                // Check for SIGTERM via ctrl_c (daemon has no TTY but signal still works)
                if tokio::signal::ctrl_c().await.is_ok() {
                    break;
                }
            }

            tracing::info!("[sniff-daemon] Stopping sniff...");
            session.stop().await?;

            let packet_count = sniffer.packet_count().await;
            tracing::info!("[sniff-daemon] Captured {} packets", packet_count);

            // Save to file if configured
            if let Some(out_path) = output {
                tracing::info!("[sniff-daemon] Saving to: {}", out_path.display());
                if let Err(e) = sniffer.save_to_file(&out_path.to_path_buf()).await {
                    tracing::warn!("[sniff-daemon] Failed to save: {}", e);
                } else {
                    tracing::info!("[sniff-daemon] Saved successfully");
                }
            }
        }
        Err(e) => {
            tracing::error!("[sniff-daemon] Failed to start sniffing: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
