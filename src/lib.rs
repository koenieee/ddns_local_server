pub mod application;
pub mod cli;
pub mod config;
pub mod core;
pub mod domain;
pub mod infrastructure;
pub mod interface;

pub use interface::CliInterface;

/// Main entry point for the DDNS updater
pub fn run() {
    if let Err(e) = interface::CliInterface::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
