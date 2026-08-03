use std::time::Duration;

use crate::scanner::{Port, PortStatus, ScanResult};
use colored::Colorize;

pub fn print_scan_result(result: &ScanResult, verbose: bool, time_passed: Duration) {
    println!("{}\n", format!("Target: {}", result.target).cyan());
    println!(
        "{:<6} : {:<8} : {:<15}",
        "PORT".cyan(),
        "STATUS".cyan(),
        "BANNER".cyan()
    );
    let mut closed = 0;
    let mut opened = 0;
    let mut timedout = 0;
    for port in &result.ports {
        match port.status {
            PortStatus::Open => {
                opened += 1;
                print_port(port);
            }
            PortStatus::Closed => {
                closed += 1;
                if verbose {
                    print_port(port);
                }
            }
            PortStatus::Timeout => {
                timedout += 1;
                if verbose {
                    print_port(port);
                }
            }
        }
    }
    println!(
        "\n{}",
        format!(
            "Scanned {} ports\nOpen: {}\nClosed: {}\nTimeout: {}\nTime elapsed: {:.2}",
            result.ports.len(),
            opened,
            closed,
            timedout,
            time_passed.as_secs_f64()
        )
        .magenta(),
    )
}

fn print_port(port: &Port) {
    let status = match port.status {
        PortStatus::Open => format!("{:<8}", port.status).green(),
        PortStatus::Closed => format!("{:<8}", port.status).red(),
        PortStatus::Timeout => format!("{:<8}", port.status).yellow(),
    };

    println!(
        "{:<6} : {} : {}",
        format!("{}", port.number).blue(),
        status,
        match &port.banner {
            Some(banner) => banner,
            None => "",
        }
    )
}
