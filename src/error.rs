#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Entry `{name}` already exists")]
    EntryExists { name: String },
    #[error("Entry `{name}` does not exist")]
    EntryDoesntExist { name: String },
    #[error("Note ID was {id} but there are only {len} notes")]
    NoteIdOOB { id: usize, len: usize },
    #[error(transparent)]
    IOErr(#[from] std::io::Error),
    #[error(transparent)]
    JSONErr(#[from] serde_json::Error),
    #[error(transparent)]
    ClipboardErr(#[from] arboard::Error),
    #[error(transparent)]
    ConfigErr(#[from] confy::ConfyError),
}

pub type Result<T> = std::result::Result<T, Error>;
