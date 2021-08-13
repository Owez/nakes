//! Contains [Package] and implementations

use crate::{Error, Result};
use async_trait::async_trait;
use futures::{future::join3, TryFutureExt};
use serde_json::Value;
use sqlx::{FromRow, SqlitePool};

const PYPI_PREFIX: &str = "https://pypi.org/pypi/";
const PYPI_SUFFIX: &str = "/json";

/// Formats urls to the pypi api, integrated `format` macro
macro_rules! pypi {
    ($p:expr) => {
        format!("{}{}{}", PYPI_PREFIX, $p, PYPI_SUFFIX)
    };
    ($($arg:tt)*) => ({
        format!("{}{}{}", PYPI_PREFIX, format_args_nl!($($arg)*), PYPI_SUFFIX)
    })
}

/// Type alias for a package's identifier
pub type PackageId = i64;

/// Loading methods allowing loading of a package from the database/lockfile by some characteristic
#[async_trait]
pub trait PackageLoad: Sized {
    /// Loads package from database/lockfile using it's id
    async fn load_id(pool: &SqlitePool, id: PackageId) -> Result<Option<Self>>;
    /// Loads package from database/lockfile using it's name and version
    async fn load_namver(pool: &SqlitePool, name: String, version: String) -> Result<Option<Self>>;
    /// Loads package from database/lockfile using it's hash
    async fn load_hash(pool: &SqlitePool, hash: String) -> Result<Option<Self>>;
}

/// Representation of a single package from the api
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

impl Package {
    /// Fetches package from api using it's name and version; saves this and others to database
    pub async fn fetch_namver(name: String, version: String) -> Result<Option<Self>> {
        fn ue(get: Option<&Value>) -> Result<&Value> {
            get.ok_or(Error::InvalidPackageSchema)
        }

        let json: Value = reqwest::get(pypi!(name))
            .and_then(|resp| resp.json())
            .await?;
        let _release = ue(ue(json.get("releases"))?.get(version))?;

        todo!()
    }

    /// Saves new package to database from values; ensure it doesn't already exist
    pub async fn save_new(
        pool: &SqlitePool,
        name: String,
        version: String,
        hash: String,
    ) -> Result<PackageId> {
        let query_result = sqlx::query!(
            "INSERT INTO package (name, version, hash) VALUES (?, ?, ?)",
            name,
            version,
            hash
        )
        .execute(pool)
        .await?;
        Ok(query_result.last_insert_rowid())
    }

    /// Saves [stored](Package::depends_on) dependencies to database
    pub async fn save_dependencies(&self, pool: &SqlitePool) -> Result<()> {
        // I'm very sorry for this hideous code, the borrow checker was shouting with sqlx and this is the best perf I could get - Owez
        for chunk in self.depends_on.chunks_exact(3) {
            let (res1, res2, res3) = join3(
                sqlx::query!(
                    "INSERT INTO depends (id, target_id) VALUES (?, ?)",
                    self.id,
                    chunk[0]
                )
                .execute(pool),
                sqlx::query!(
                    "INSERT INTO depends (id, target_id) VALUES (?, ?)",
                    self.id,
                    chunk[1]
                )
                .execute(pool),
                sqlx::query!(
                    "INSERT INTO depends (id, target_id) VALUES (?, ?)",
                    self.id,
                    chunk[2]
                )
                .execute(pool),
            )
            .await;

            res1?;
            res2?;
            res3?;
        }

        Ok(())
    }
}

#[async_trait]
impl PackageLoad for Package {
    async fn load_id(pool: &SqlitePool, id: PackageId) -> Result<Option<Self>> {
        Ok(match SqlPackage::load_id(pool, id).await? {
            Some(sqlpkg) => Some(sqlpkg.to_pkg(pool).await?),
            None => None,
        })
    }

    async fn load_namver(pool: &SqlitePool, name: String, version: String) -> Result<Option<Self>> {
        Ok(match SqlPackage::load_namver(pool, name, version).await? {
            Some(sqlpkg) => Some(sqlpkg.to_pkg(pool).await?),
            None => None,
        })
    }

    async fn load_hash(pool: &SqlitePool, hash: String) -> Result<Option<Self>> {
        Ok(match SqlPackage::load_hash(pool, hash).await? {
            Some(sqlpkg) => Some(sqlpkg.to_pkg(pool).await?),
            None => None,
        })
    }
}

#[derive(FromRow)]
struct SqlPackage {
    id: PackageId,
    name: String,
    version: String,
    hash: String,
}

impl SqlPackage {
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

#[async_trait]
impl PackageLoad for SqlPackage {
    async fn load_id(pool: &SqlitePool, id: PackageId) -> Result<Option<Self>> {
        let query = sqlx::query_as!(Self, "SELECT * FROM package WHERE id=?", id);
        Ok(query.fetch_optional(pool).await?)
    }

    async fn load_namver(pool: &SqlitePool, name: String, version: String) -> Result<Option<Self>> {
        let query = sqlx::query_as!(
            Self,
            "SELECT * FROM package WHERE name=? AND version=?",
            name,
            version
        );
        Ok(query.fetch_optional(pool).await?)
    }

    async fn load_hash(pool: &SqlitePool, hash: String) -> Result<Option<Self>> {
        // let query = sqlx::query_as!(Self, "SELECT * FROM package WHERE hash=?", hash);
        // Ok(query.fetch_optional(pool).await?)
        todo!("fix load hash for sqlpackage")
    }
}

/// Loads all ids for a package from the `depends` table
async fn load_depends_on(pool: &SqlitePool, id: PackageId) -> Result<Vec<PackageId>> {
    let query = sqlx::query!("SELECT target_id FROM depends WHERE id=?", id);
    Ok(query
        .fetch_all(pool)
        .await?
        .iter()
        .map(|record| record.target_id)
        .collect())
}
