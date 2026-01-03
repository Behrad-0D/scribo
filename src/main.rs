mod note;
mod commands;
use clap::{Parser, Subcommand};
use comfy_table::Table;
use note::Note;
use commands::{add,list,delete,clear};
use color_print::cprintln;
#[derive(Parser)]

#[command(name = "scribo")]
#[command(about = "Terminal note manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        content: String,
        #[arg(short, long, default_value = "general")]
        tag: String,
    },
    List,
    Delete {
        id: u32,
    },
    Clear {
        #[arg(short, long)]
        yes: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::List => {
            list::run();
        }
        Commands::Add { content, tag } => {
            add::run(content, tag);
        }
        Commands::Delete { id } => {
            delete::run(id);
        }
        Commands::Clear { yes } => {
            clear::run(yes);
        }
    }
}
