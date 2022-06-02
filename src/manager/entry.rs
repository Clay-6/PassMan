use core::fmt;

use serde::{Deserialize, Serialize};

/// Struct to serialise & deserialise JSON to & from
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entry {
    pub(crate) name: String,
    pub(crate) username: String,
    pub(crate) password: Vec<u8>,
    pub(crate) location: String,
    pub(crate) notes: Vec<String>,
}

impl Entry {
    pub fn new(name: String, location: String, username: String, password: String) -> Self {
        let password = Self::hide_password(password);

        Self {
            name,
            username,
            password,
            location,
            notes: Vec::new(),
        }
    }

    fn hide_password(password: String) -> Vec<u8> {
        Vec::from(password.as_bytes())
    }

    pub fn show_password(&self) -> String {
        String::from_utf8(self.password.clone()).unwrap()
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} [for {}]", self.name, self.location)?;
        writeln!(f, "   Username: {}", self.username)?;
        writeln!(f, "   Password: {}", self.show_password())?;
        write!(f, "   Contains {} notes", self.notes.len())
    }
}
