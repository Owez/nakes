//! Contains [Package] and implementations

use crate::Result;
use sqlx::{FromRow, SqlitePool};

/// Type alias for a package's identifier
pub type PackageId = i64;

/// Representation of a single package from pypi
pub struct Package {
    /// Identifier
    pub id: PackageId,
    /// Name
    pub name: String,
    /// Version linked to name
    pub version: String,
    /// Hash of version
    pub hash: String,
    /// All packages this one depends upon
    pub depends_on: Vec<PackageId>,
}

#[derive(Debug, FromRow)]
struct SqlPackage {
    id: PackageId,
    name: String,
    version: String,
    hash: String,
}

impl SqlPackage {
    /// Loads from id
    pub fn load_id(pool: &SqlitePool, id: PackageId) -> Result<Option<Self>> {
        todo!("load from id")
    }

    /// Loads from name and version
    pub fn load_namver(pool: &SqlitePool, name: String, version: String) -> Result<Option<Self>> {
        todo!("load from name and version")
    }

    /// Loads from hash
    pub fn load_hash(pool: &SqlitePool, id: String) -> Result<Option<Self>> {
        todo!("load from hash")
    }

    /// Converts to a full package by fetching all this depends on
    async fn to_pkg(self, pool: &SqlitePool) -> Result<Package> {
        Ok(Package {
            id: self.id,
            name: self.name,
            version: self.version,
            hash: self.hash,
            depends_on: load_depends_on(pool, self.id).await?,
        })
    }
}

/// Loads all ids for a package from the `depends` table
async fn load_depends_on(pool: &SqlitePool, id: PackageId) -> Result<Vec<PackageId>> {
    todo!("load ids for package from depends table")
}
