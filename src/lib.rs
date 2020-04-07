use std::env;
use std::ffi::{OsStr, OsString};
use std::path::{Path};
use std::process::Command;
use std::os::unix::process::CommandExt;

pub mod entry;

#[macro_use]
extern crate failure;
use failure::Error;
use chrono::prelude::*;

#[derive(Fail, Debug)]
pub enum VarError {
    #[fail(display = "{} is unset", _0)]
    MissingEnvVar(&'static str),
    #[fail(display = "Invalid Unicode sequence")]
    InvalidUnicode,
}

pub fn today() -> Date<Local> {
    Local::now().date()
}

pub fn yesterday() -> Date<Local> {
    let mut day = today().pred();
    while day.weekday() == Weekday::Sat|| day.weekday() == Weekday::Sun {
        day = day.pred()
    }
    day
}

pub fn tomorrow() -> Date<Local> {
    let mut day = today().succ();
    while day.weekday() == Weekday::Sat|| day.weekday() == Weekday::Sun {
        day = day.succ()
    }
    day
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

pub trait ToNoteFile {
    fn to_note_file<P: AsRef<Path>>(&self, kind: NoteKind, dir: P) -> OsString;
}

impl ToNoteFile for Date<Local> {
    fn to_note_file<P: AsRef<Path>>(&self, kind: NoteKind, dir: P) -> OsString {
        let mut out = dir.as_ref().to_path_buf();
        let date = self.format("%Y-%m-%d");
        let filename = format!("{}.{}", date, kind.to_extension());
        out.push(filename);

        out.into()
    }
}
pub fn get_notes_dir() -> Result<String, VarError> {
    get_env_var("OGRES_NOTES_DIR")
}

fn get_editor() -> Result<String, VarError> {
    get_env_var("EDITOR")
}

fn get_env_var(var: &'static str) -> Result<String, VarError> {
    env::var(var)
        .map_err(|e| {
            match e {
                env::VarError::NotPresent => VarError::MissingEnvVar(var),
                env::VarError::NotUnicode(_) => VarError::InvalidUnicode,
            }
        })
}


#[must_use]
pub fn edit<P: AsRef<OsStr>>(path: P) -> Error {
    let editor = match get_editor() {
        Ok(editor) => editor,
        Err(e) => {
            return e.into()
        },
    };

    Command::new(editor)
        .args(&[path])
        .exec()
        .into()
}
