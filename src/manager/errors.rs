#[derive(Debug, thiserror::Error)]
pub enum ManagerError {
    #[error("Entry `{name}` already exists")]
    EntryExists { name: String },
    #[error("Entry does not exist")]
    EntryDoesntExist,
    #[error("Note ID out of bounds")]
    NoteIdOOB,
}
