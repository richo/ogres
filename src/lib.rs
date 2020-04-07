use std::env;
use std::path::{Path, PathBuf};

use chrono::prelude::*;

pub fn today() -> Date<Local> {
    Local::now().date()
}

pub fn yesterday() -> Date<Local> {
    today().pred()
}

pub fn tomorrow() -> Date<Local> {
    today().succ()
}

pub enum NoteKind {
    Markdown,
}

impl NoteKind {
    fn to_extension(&self) -> &'static str {
        match *self {
            NoteKind::Markdown => "md"
        }
    }
}

trait ToNoteFile {
    fn to_note_file<P: AsRef<Path>>(&self, kind: NoteKind, dir: P) -> PathBuf;
}

impl ToNoteFile for Date<Local> {
    fn to_note_file<P: AsRef<Path>>(&self, kind: NoteKind, dir: P) -> PathBuf {
        let mut out = dir.as_ref().to_path_buf();
        let date = self.format("%Y-%m-%d.{}");
        let filename = format!("{}.{}", date, kind.to_extension());
        out.push(filename);

        out
    }
}

pub fn editor() -> Result<String, env::VarError> {
    env::var("EDITOR")
}
