//! Benchmark command handler

use crate::benchmark::{BenchmarkCategory, BenchmarkRunner, BenchmarkReport};
use crate::cli::types::BenchmarkCommand;
use crate::error::{Result, SerialError};
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
    let json = serde_json::to_string_pretty(report)
        .map_err(|e| SerialError::Config(format!("Failed to serialize benchmark results: {}", e)))?;
    std::fs::write(path, json)
        .map_err(|e| SerialError::Config(format!("Failed to write benchmark results: {}", e)))?;

    println!("\nResults saved to: {}", path.display());
    Ok(())
}

fn load_benchmark_results(path: &PathBuf) -> Result<BenchmarkReport> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| SerialError::Config(format!("Failed to read benchmark results: {}", e)))?;
    serde_json::from_str(&content)
        .map_err(|e| SerialError::Config(format!("Failed to parse benchmark results: {}", e)))
}
