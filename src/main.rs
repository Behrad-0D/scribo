mod note;
use clap::{Parser, Subcommand};
use comfy_table::Table;
use note::Note;
use color_print::cprintln;
#[derive(Parser)]
#[command(name = "notes")]
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
            let notes = match Note::load_from_json() {
                Some(notes) => notes,
                _ => panic!("not good"),
            };
            if notes.is_empty() {
                cprintln!("<red>No notes found.</red>\n<yellow>Add one with:</yellow><green> notes add 'your first note'</green>")
            } else {
                let mut table = Table::new();
                table.set_header(vec!["id", "content", "tag", "date","time"]);
                for note in &notes {
                    table.add_row(vec![
                        note.id.to_string(),
                        note.content.to_string(),
                        note.tag.to_string(),
                        note.date.to_string(),
                        note.time.to_string(),
                    ]);
                }

                println!("{table}");
            }
        }
        Commands::Add { content, tag } => {
            let mut notes = match Note::load_from_json() {
                Some(notes) => notes,
                _ => panic!("not good"),
            };
            notes.push(Note::new(content, tag));
            if Note::save_to_json(&notes) {
                cprintln!("<green>your note added!</green>");
            }
        }
        Commands::Delete { id } => {
            let mut notes = match Note::load_from_json() {
                Some(notes) => notes,
                _ => panic!("not good"),
            };
            notes.retain(|n| n.id != id);
            Note::save_to_json(&notes);
            println!("note {id} deleted!");
        }
        Commands::Clear { yes } => {
            if yes {
                Note::save_to_json(&vec![]);
                println!("all note cleared!")
            } else {
                cprintln!("<yellow>use --yes tag to confrim clear notes</yellow>")
            }
        }
    }
}
