use crate::scanner::{PortStatus, ScanResult};
use colored::Colorize;

pub fn print_scan_result(result: &ScanResult, verbose: bool) {
    println!("{}\n", format!("Target: {}", result.target).cyan());
    println!("{:<6} : {:<8}", "PORT".cyan(), "STATUS".cyan());
    for port in &result.ports {
        if verbose {
            println!(
                "{:<6} : {:<8}",
                format!("{}", port.number).blue(),
                port.status
            )
        } else if port.status == PortStatus::Open {
            println!(
                "{:<6} : {:<8}",
                format!("{}", port.number).blue(),
                port.status
            )
        }
    }
}
