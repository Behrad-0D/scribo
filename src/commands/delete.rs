use crate::cprintln;
use crate::Note;
pub fn run(id : u32) {
    let mut notes = match Note::load_from_json() {
        Some(notes) => notes,
        _ => panic!("not good"),
    };
    notes.retain(|n| n.id != id);
    Note::save_to_json(&notes);
    cprintln!("<green>note {id} deleted!</green>");
}
