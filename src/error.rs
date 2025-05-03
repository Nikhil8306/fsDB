pub enum DBError {
    DbNotFound,
    DirError,
    CreateError(String),
    ReadError,
    WriteError,
    DbConflict,
    Misc(String),
    KeyNotFound,
    DupKey
}