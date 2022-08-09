#[derive(Debug, thiserror::Error)]
pub enum ManagerError {
    #[error("Entry `{name}` already exists")]
    EntryExists { name: String },
    #[error("Entry `{name}` does not exist")]
    EntryDoesntExist { name: String },
    #[error("Note ID was {id} but there are only {len} notes")]
    NoteIdOOB { id: usize, len: usize },
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    ClipboardError(#[from] arboard::Error),
    #[error(transparent)]
    ConfigError(#[from] confy::ConfyError),
}

pub type Result<T> = std::result::Result<T, ManagerError>;
