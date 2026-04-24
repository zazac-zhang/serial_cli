//! Benchmark command handler

use crate::benchmark::{BenchmarkCategory, BenchmarkRunner, BenchmarkReport};
use crate::cli::types::BenchmarkCommand;
use crate::error::Result;
use std::path::PathBuf;

pub fn handle_benchmark_command(cmd: BenchmarkCommand) -> Result<()> {
    match cmd {
        BenchmarkCommand::Run {
            category,
            iterations,
            output,
        } => run_benchmarks(&category, iterations, output),
        BenchmarkCommand::Compare { baseline, current } => {
            compare_benchmarks(&baseline, &current)
        }
        BenchmarkCommand::List => list_benchmarks(),
    }
}

fn run_benchmarks(category: &str, iterations: u64, output: Option<PathBuf>) -> Result<()> {
    let runner = BenchmarkRunner::new().with_iterations(iterations);
    let mut results = Vec::new();

    match category {
        "all" => {
            results.extend(runner.run_all()?);
        }
        "serial-io" => {
            results.extend(runner.run_serial_io_benchmarks()?);
        }
        "virtual-port" => {
            results.extend(runner.run_virtual_port_benchmarks()?);
        }
        "protocol" => {
            results.extend(runner.run_protocol_benchmarks()?);
        }
        "startup" => {
            results.extend(runner.run_startup_benchmarks()?);
        }
        "memory" => {
            results.extend(runner.run_memory_benchmarks()?);
        }
        "concurrency" => {
            results.extend(runner.run_concurrency_benchmarks()?);
        }
        _ => {
            println!("Unknown category: {}", category);
            println!("Available categories: all, serial-io, virtual-port, protocol, startup, memory, concurrency");
            return Ok(());
        }
    }

    // Generate report
    let report = BenchmarkReport::new(results);
    report.print_summary();

    // Save results if requested
    if let Some(output_path) = output {
        save_benchmark_results(&report, &output_path)?;
    }

    Ok(())
}

fn compare_benchmarks(baseline: &PathBuf, current: &PathBuf) -> Result<()> {
    use crate::benchmark::reporter::compare_benchmarks;

    println!("Comparing benchmark results...");
    println!("Baseline: {}", baseline.display());
    println!("Current: {}", current.display());
    println!();

    // Load baseline and current reports
    let baseline_report = load_benchmark_results(baseline)?;
    let current_report = load_benchmark_results(current)?;

    // Compare
    let comparisons = compare_benchmarks(&baseline_report, &current_report);

    if comparisons.is_empty() {
        println!("No matching benchmarks found to compare.");
        return Ok(());
    }

    println!("=== Benchmark Comparison ===\n");

    let mut regression_count = 0;
    let mut improvement_count = 0;

    for comparison in &comparisons {
        comparison.print();
        println!();

        if comparison.is_regression {
            regression_count += 1;
        } else if comparison.is_improvement {
            improvement_count += 1;
        }
    }

    println!("Summary:");
    println!("  Regressions: {}", regression_count);
    println!("  Improvements: {}", improvement_count);
    println!("  Unchanged: {}", comparisons.len() - regression_count - improvement_count);

    Ok(())
}

fn list_benchmarks() -> Result<()> {
    println!("Available benchmark categories:");
    println!();

    for category in BenchmarkCategory::all() {
        println!("  {}", category.name());
    }

    println!();
    println!("Usage:");
    println!("  serial-cli benchmark run <category>");
    println!("  serial-cli benchmark run serial-io --iterations 1000");
    println!("  serial-cli benchmark run all --output results.json");

    Ok(())
}

fn save_benchmark_results(report: &BenchmarkReport, path: &PathBuf) -> Result<()> {
    use std::io::Write;

    // For now, save a simple text format
    // TODO: Implement JSON serialization
    let mut file = std::fs::File::create(path)?;
    writeln!(file, "# Benchmark Report")?;
    writeln!(file, "# Timestamp: {}", report.timestamp.format("%Y-%m-%d %H:%M:%S UTC"))?;
    writeln!(file, "# Total benchmarks: {}", report.results.len())?;
    writeln!(file)?;

    for result in &report.results {
        writeln!(file, "[{}]", result.name)?;
        writeln!(file, "  category: {}", result.category.name())?;
        writeln!(file, "  iterations: {}", result.iterations)?;
        writeln!(file, "  elapsed_ns: {}", result.elapsed_ns)?;
        if let Some(bytes) = result.bytes_processed {
            writeln!(file, "  bytes_processed: {}", bytes)?;
        }
        writeln!(file)?;
    }

    println!("\nResults saved to: {}", path.display());
    Ok(())
}

fn load_benchmark_results(path: &PathBuf) -> Result<BenchmarkReport> {
    use crate::benchmark::BenchmarkResult;
    use std::io::BufRead;

    // For now, this is a simple parser
    // TODO: Implement JSON deserialization
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);

    let mut results = Vec::new();
    let mut current_name = String::new();
    let mut current_category = BenchmarkCategory::SerialIo;
    let mut iterations = 0u64;
    let mut elapsed_ns = 0u64;
    let mut bytes_processed = None;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("[") && line.ends_with("]") {
            // Save previous result
            if !current_name.is_empty() {
                results.push(BenchmarkResult {
                    name: current_name.clone(),
                    category: current_category,
                    iterations,
                    elapsed_ns,
                    bytes_processed,
                });
            }

            // Start new result
            current_name = line[1..line.len() - 1].to_string();
            iterations = 0;
            elapsed_ns = 0;
            bytes_processed = None;
        } else if line.starts_with("  category: ") {
            let cat = line.trim_start_matches("  category: ");
            current_category = match cat {
                "serial-io" => BenchmarkCategory::SerialIo,
                "virtual-port" => BenchmarkCategory::VirtualPort,
                "protocol" => BenchmarkCategory::Protocol,
                "startup" => BenchmarkCategory::Startup,
                "memory" => BenchmarkCategory::Memory,
                "concurrency" => BenchmarkCategory::Concurrency,
                _ => BenchmarkCategory::SerialIo,
            };
        } else if line.starts_with("  iterations: ") {
            iterations = line
                .trim_start_matches("  iterations: ")
                .parse()
                .unwrap_or(0);
        } else if line.starts_with("  elapsed_ns: ") {
            elapsed_ns = line
                .trim_start_matches("  elapsed_ns: ")
                .parse()
                .unwrap_or(0);
        } else if line.starts_with("  bytes_processed: ") {
            bytes_processed = Some(
                line.trim_start_matches("  bytes_processed: ")
                    .parse()
                    .unwrap_or(0),
            );
        }
    }

    // Save last result
    if !current_name.is_empty() {
        results.push(BenchmarkResult {
            name: current_name,
            category: current_category,
            iterations,
            elapsed_ns,
            bytes_processed,
        });
    }

    Ok(BenchmarkReport::new(results))
}
