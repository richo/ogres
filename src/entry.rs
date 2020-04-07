use crate::{
    edit,
    today, tomorrow, yesterday,
    get_notes_dir,
    NoteKind,
    ToNoteFile,
};
use std::path::PathBuf;
use failure::Error;

#[derive(Fail, Debug)]
pub enum RuntimeError {
    #[fail(display = "{} is unset", _0)]
    UnknownDay(String),
    #[fail(display = "Invalid unicode in binary name")]
    InvalidBinary,
}

pub fn main() -> Result<(), Error> {
    let argv0 = std::env::args().next()
        .ok_or(RuntimeError::InvalidBinary)?;
    let as_path: PathBuf = argv0.clone().into();
    let file_name = as_path.file_name()
        .ok_or(RuntimeError::InvalidBinary)?
        .to_str()
        .ok_or(RuntimeError::InvalidBinary)?;

    let day = match file_name {
        "today" => today(),
        "yesterday" => yesterday(),
        "tomorrow" => tomorrow(),
        _ => Err(RuntimeError::UnknownDay(argv0))?,
    };

    let notes = get_notes_dir()?;
    let note = day
        .to_note_file(NoteKind::Markdown, notes);
    Err(edit(note))
}
