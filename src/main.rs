mod note;
use note::Note;
use std::env;
fn main() {
    let mut notes = Vec::new();
    let my_note1 = Note::new("hello world !".to_string(), "new".to_string());
    let my_note2 = Note::new("hello world !".to_string(), "new".to_string());
    let my_note3 = Note::new("hello world !".to_string(), "new".to_string());
    let my_note4 = Note::new("hello world !".to_string(), "new".to_string());

    notes.push(my_note1);
    notes.push(my_note2);
    notes.push(my_note3);
    notes.push(my_note4);

    for note in &notes {
        println!(
            "note {} content : {}, and it tag is : {} ",
            note.id, note.content, note.tag
        );
    }
    let data = Note::load_from_json();
    match data {
        Some(notes) => {
            for note in notes {
                println! {"{:?}", note}
            }
        }
        None => {
            println!("not found !")
        }
    }
    Note::save_to_json(&notes);
}
