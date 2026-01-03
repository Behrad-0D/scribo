use crate::Note;
use crate::cprintln;


pub fn run(yes: bool) {
            if yes {
                Note::save_to_json(&vec![]);
                cprintln!("<green>all note cleared!</green>")
            } else {
                cprintln!("<yellow>use --yes tag to confirm clear notes</yellow>")
            }
}