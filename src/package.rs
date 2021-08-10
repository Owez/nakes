//! Contains [Package], [RawPackage] and implementations

use crate::validation::*;
use crate::Result;
use sqlx::{FromRow, SqlitePool};

const PYPI_URL_PREFIX: &str = "https://pypi.org/pypi/";
const PYPI_URL_SUFFIX: &str = "/json";

/// Representation of a single package which may be saved to a lockfiles
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Package {
    /// In-house identifier; auto-incrementing
    pub id: i64,
    /// Package name
    pub name: String,
    /// Package version tied to the name; extended semvar
    pub version: String,
    /// Stringified hash from pypi; this is unique
    pub hash: String,
    /// Identifiers of other packages current instance depends on
    pub depends_on: Vec<i64>,
}

impl Package {
    /// Creates a new package from given values then saves it to the database;
    /// make sure there are not duplicates of this package
    pub async fn new_save(
        pool: &SqlitePool,
        name: String,
        version: String,
        hash: String,
        depends_on: Vec<i64>,
    ) -> Result<Self> {
        validate_name(&name)?;
        validate_version(&version)?;
        validate_hash(&hash)?;

        let id = sqlx::query!(
            "INSERT INTO package (name, version, hash) VALUES (?, ?, ?)",
            name,
            version,
            hash
        )
        .execute(pool)
        .await?
        .last_insert_rowid();

        Ok(Self {
            id,
            name,
            version,
            hash,
            depends_on,
        })
    }

    /// Deeply fetches a package from name and version
    ///
    /// - First, this method tries to search the lockfile for the existing package and returns
    /// - If not, we fetch from the pypi api along with all dependents and store each one
    pub async fn from_namver(
        pool: &SqlitePool,
        name: String,
        version: String,
    ) -> Result<Option<Self>> {
        let _resp = match Self::load_from_namver(pool, name.clone(), version).await? {
            Some(pkg) => return Ok(Some(pkg)),
            None => reqwest::get(format!("{}{}{}", PYPI_URL_PREFIX, name, PYPI_URL_SUFFIX)).await?,
        };

        todo!("decode response")
    }

    /// Loads existing package from id; includes dependents
    pub async fn load_from_id(pool: &SqlitePool, id: i64) -> Result<Option<Self>> {
        Self::from_opt_rawpkg(pool, RawPackage::from_id(pool, id).await?).await
    }

    /// Loads existing package from name and version; used within the [Package::from_namver] method; includes dependents
    pub async fn load_from_namver(
        pool: &SqlitePool,
        name: String,
        version: String,
    ) -> Result<Option<Self>> {
        validate_name(&name)?;
        validate_version(&version)?;

        Self::from_opt_rawpkg(pool, RawPackage::from_namver(pool, name, version).await?).await
    }

    /// Loads package dependents
    async fn load_depends_on(mut self, pool: &SqlitePool) -> Result<Self> {
        let records = sqlx::query!("SELECT target_id FROM depends WHERE id=?", self.id)
            .fetch_all(pool)
            .await?;
        self.depends_on = records.iter().map(|record| record.target_id).collect();
        Ok(self)
    }

    /// Creates a new package from an optional local [RawPackage] struct
    async fn from_opt_rawpkg(
        pool: &SqlitePool,
        opt_rawpkg: impl Into<Option<RawPackage>>,
    ) -> Result<Option<Self>> {
        match opt_rawpkg.into() {
            Some(rawpkg) => Self::from(rawpkg)
                .load_depends_on(pool)
                .await
                .and_then(|pkg| Ok(Some(pkg))),
            None => Ok(None),
        }
    }
}

/// Raw package which can be directly taken from the `package` sql model
#[derive(FromRow)]
struct RawPackage {
    id: i64,
    name: String,
    version: String,
    hash: String,
}

impl RawPackage {
    /// Fetches existing raw package from id
    pub async fn from_id(pool: &SqlitePool, id: i64) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM package WHERE id=?", id)
                .fetch_optional(pool)
                .await?,
        )
    }

    /// Fetches existing raw package from name and version
    pub async fn from_namver(
        pool: &SqlitePool,
        name: String,
        version: String,
    ) -> Result<Option<Self>> {
        Ok(sqlx::query_as!(
            Self,
            "SELECT * FROM package WHERE name=? AND version=?",
            name,
            version
        )
        .fetch_optional(pool)
        .await?)
    }
}

impl From<RawPackage> for Package {
    fn from(rawpkg: RawPackage) -> Self {
        Self {
            id: rawpkg.id,
            name: rawpkg.name,
            version: rawpkg.version,
            hash: rawpkg.hash,
            depends_on: vec![],
        }
    }
}
