use crate::scanner::ScanResult;
use colored::Colorize;

pub fn print_scan_result(result: &ScanResult) {
    println!("{}\n", format!("Target: {}", result.target).cyan());
    println!("{:<6} : {:<8}", "PORT".cyan(), "STATUS".cyan());
    for port in &result.ports {
        println!(
            "{:<6} : {:<8}",
            format!("{}", port.number).blue(),
            port.status
        )
    }
}
