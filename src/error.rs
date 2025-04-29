pub enum DBError {
    DbNotFound,
    DirError,
    ReadError,
    CreateError,
    WriteError,
    DbConflict,
    Misc(String),
    KeyNotFound,
    DupKey
}