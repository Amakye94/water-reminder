use clap::{Parser, Subcommand};
use chrono::Local;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};

const DAILY_GOAL: i32 = 2000; // 2000 ml = 2 liters
const FILE_NAME: &str = "water_log.txt";

/// Simple CLI to help track daily water intake
#[derive(Parser)]
#[command(name = "water-reminder", version, about = "Track your water intake")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Log water intake in milliliters
    Log { amount: i32 },

    /// Show daily progress
    Status,

    /// Reset today’s log
    Reset,
}

fn read_log() -> i32 {
    if let Ok(mut file) = OpenOptions::new().read(true).open(FILE_NAME) {
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content.trim().parse::<i32>().unwrap_or(0)
    } else {
        0
    }
}

fn write_log(amount: i32) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_NAME)
        .unwrap();
    file.write_all(amount.to_string().as_bytes()).unwrap();
}

fn main() {
    let cli = Cli::parse();
    let today = Local::now().format("%Y-%m-%d").to_string();

    match cli.command {
        Command::Log { amount } => {
            let current = read_log();
            let new_total = current + amount;
            write_log(new_total);

            println!("✔ Logged {amount} ml. Total today: {new_total} ml.");
        }

        Command::Status => {
            let total = read_log();
            println!("📊 Date: {}", today);
            println!("💧 Total: {} ml", total);
            println!("🎯 Goal: {} ml", DAILY_GOAL);

            if total >= DAILY_GOAL {
                println!("🎉 You’ve reached your daily goal!");
            } else {
                println!(
                    "➡ You need {} ml more.",
                    DAILY_GOAL - total
                );
            }
        }

        Command::Reset => {
            write_log(0);
            println!("🔄 Log reset.");
        }
    };
}
