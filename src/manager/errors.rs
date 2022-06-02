#[derive(Debug, thiserror::Error)]
pub enum ManagerError {
    #[error("Entry `{name}` already exists")]
    EntryExists { name: String },
    #[error("Entry `{name}` does not exist")]
    EntryDoesntExist { name: String },
    #[error("Note ID out of bounds")]
    NoteIdOOB,
}
