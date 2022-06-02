#[derive(Debug, thiserror::Error)]
pub enum ManagerError {
    #[error("Entry already exists")]
    EntryExists,
    #[error("Entry does not exist")]
    EntryDoesntExist,
    #[error("Note ID out of bounds")]
    NoteIdOOB,
}
