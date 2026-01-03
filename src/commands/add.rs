use crate::note::Note;
use crate::cprintln;
pub fn run(content: String, tag: String) {
    let mut notes = match Note::load_from_json() {
        Some(notes) => notes,
        _ => panic!("not good"),
    };
    notes.push(Note::new(content, tag));
    if Note::save_to_json(&notes) {
        cprintln!("<green>your note added!</green>");
    }
}
