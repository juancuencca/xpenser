use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "xpenser", about = "A CLI tool to track expenses.")]
pub struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    Add {
        #[arg(short, long)]
        description: String,
        #[arg(short, long)]
        amount: f64,
    },
    Update {
        #[arg(short, long)]
        id: u32,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        amount: Option<f64>,
    },
    Delete {
        #[arg(short, long)]
        id: u32,
    },
    List,
    Summary {
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=12))]
        month: Option<u8>,
    },
}