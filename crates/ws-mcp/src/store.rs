use std::sync::Arc;

use ws_auth::{File, Keyring, Store};

/// Path of a session file to use instead of the system keychain. Containers have no keychain.
pub const SESSION_FILE: &str = "WS_SESSION_FILE";

/// Where the session is kept: the file at `$WS_SESSION_FILE` when set, else the system keychain.
pub fn session() -> Arc<dyn Store> {
    match std::env::var_os(SESSION_FILE).filter(|path| !path.is_empty()) {
        Some(path) => Arc::new(File::new(path)),
        None => Arc::new(Keyring::default()),
    }
}

/// Where `session()` keeps it, for messages.
pub fn describe() -> String {
    match std::env::var(SESSION_FILE) {
        Ok(path) if !path.is_empty() => path,
        _ => "the system keychain".to_owned(),
    }
}
