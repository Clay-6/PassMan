pub const ENTRY_EXISTS: &str = "Entry already exists";
pub const ENTRY_DOESNT_EXIST: &str = "Entry does not exist";
pub const ENTRY_ID_OOB: &str = "Entry id out of bounds";

#[derive(Debug, thiserror::Error)]
pub enum ManagerError {
    #[error("Entry already exists")]
    EntryExists,
    #[error("Entry does not exist")]
    EntryDoesntExist,
    #[error("Note ID out of bounds")]
    NoteIdOOB,
}
