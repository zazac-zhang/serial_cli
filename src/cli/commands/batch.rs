//! Batch command handler

use crate::error::{Result, SerialError};
use crate::cli::types::BatchCommand;
use crate::cli::batch::{BatchConfig, BatchRunner};

pub async fn handle_batch_command(cmd: BatchCommand) -> Result<()> {
    match cmd {
        BatchCommand::Run {
            script,
            concurrent,
            continue_on_error,
            timeout,
        } => {
            println!("Running batch script: {}", script.display());
            println!("Max concurrent tasks: {}", concurrent);
            println!();

            // Check if script exists
            if !script.exists() {
                println!("\u{2717} Batch script not found: {}", script.display());
                return Err(SerialError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Batch script not found",
                )));
            }

            // Create batch configuration
            let config = BatchConfig {
                max_concurrent: concurrent,
                timeout_secs: timeout,
                continue_on_error,
                show_progress: true,
                verbose: false,
            };

            // Create batch runner
            let runner = BatchRunner::new(config)?;

            // Check if it's a single script or a batch file
            if script.extension().is_some_and(|e| e == "lua") {
                // Single Lua script
                println!("Executing single script...");

                match runner.run_script(&script).await {
                    Ok(_) => {
                        println!("\u{2713} Script executed successfully");
                    }
                    Err(e) => {
                        println!("\u{2717} Script execution failed: {}", e);
                        return Err(e);
                    }
                }
            } else {
                // Assume it's a batch file containing list of scripts
                println!("Executing batch script file...");

                let content =
                    std::fs::read_to_string(&script).map_err(SerialError::Io)?;

                let script_paths: Vec<&std::path::Path> = content
                    .lines()
                    .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
                    .map(|line| std::path::Path::new(line.trim()))
                    .collect();

                if script_paths.is_empty() {
                    println!("\u{26A0} No scripts found in batch file");
                    return Ok(());
                }

                println!("Found {} scripts to execute", script_paths.len());

                // Run scripts in sequence
                match runner.run_scripts(script_paths).await {
                    Ok(result) => {
                        println!();
                        println!("Batch execution completed:");
                        println!("  Total scripts: {}", result.results.len());

                        let successful = result.results.iter().filter(|r| r.success).count();
                        let failed = result.results.len() - successful;

                        println!("  Successful: {}", successful);
                        println!("  Failed: {}", failed);

                        if failed > 0 {
                            println!();
                            println!("Failed scripts:");
                            for result in result.results.iter().filter(|r| !r.success) {
                                println!("  - {}", result.script);
                            }
                        }
                    }
                    Err(e) => {
                        println!("\u{2717} Batch execution failed: {}", e);
                        return Err(e);
                    }
                }
            }
        }
        BatchCommand::List => {
            println!("Batch scripts:");
            println!("Looking for batch scripts in current directory...");

            // List common batch script locations
            let batch_files = vec!["batch.txt", "scripts.txt", "batch.lua", "scripts.batch"];

            let mut found = false;
            for batch_file in batch_files {
                if std::path::Path::new(batch_file).exists() {
                    println!("  \u{2713} {}", batch_file);
                    found = true;
                }
            }

            if !found {
                println!("  No batch scripts found");
                println!();
                println!("Create a batch script file with one Lua script per line:");
                println!("  # Comments start with #");
                println!("  script1.lua");
                println!("  script2.lua");
                println!("  script3.lua");
            }
        }
    }
    Ok(())
}
