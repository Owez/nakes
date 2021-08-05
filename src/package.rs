//! Contains [Package] and implementations

use sqlx::{FromRow, SqlitePool};

use crate::Result;

/// Representation of a single package which may be saved to a lockfile
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Package {
    /// In-house identifier; auto-incrementing
    pub id: i32,
    /// Package name
    pub name: String,
    /// Version of this package tied to it's name; extended semvar
    pub version: String,
    /// Packages that this package depends on itself
    pub depends_on: Vec<Package>,
}

impl Package {
    /// Tries to get an existing package via it's id
    pub fn from_id(pool: &SqlitePool, id: i32) -> Result<Option<Self>> {
        todo!("get pkg from id")
        // sqlx::query_as!(BasePackage, "SELECT * FROM package WHERE id=?", id)
    }

    /// Tries to get an existing package via it's name and version
    pub fn from_namever(
        pool: &SqlitePool,
        name: String,
        version: String,
    ) -> Result<Option<Self>> {
        todo!("get pkg from name and version")
    }
}

/// Package pulled directly from the `packages` model in a lockfile
#[derive(FromRow)]
struct BasePackage {
    id: i32,
    name: String,
    version: String,
}
