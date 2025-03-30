use chrono::{DateTime, Utc};
use ring::rand::SystemRandom;
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection, Result as SqliteResult};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("Failed to generate key pair: {0}")]
    KeyGeneration(String),

    #[error("Failed to read key pair: {0}")]
    KeyReading(String),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Wallet not found")]
    NotFound,

    #[error("Date parsing error: {0}")]
    DateParsing(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletData {
    pub id: Option<i64>,
    pub address: String,
    pub private_key_bytes: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub last_used: DateTime<Utc>,
    pub label: String,
}

pub struct WalletManager {
    conn: Connection,
}

impl WalletManager {
    pub fn new(db_path: &str) -> Result<Self, WalletError> {
        let conn = Connection::open(db_path)?;

        // Create the wallets table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS wallets (
                id INTEGER PRIMARY KEY,
                address TEXT NOT NULL UNIQUE,
                private_key_bytes BLOB NOT NULL,
                created_at TEXT NOT NULL,
                last_used TEXT NOT NULL,
                label TEXT NOT NULL
            )",
            [],
        )?;

        Ok(WalletManager { conn })
    }

    pub fn create_wallet(&self, label: &str) -> Result<WalletData, WalletError> {
        // Generate new keypair
        let rng = SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)
            .map_err(|e| WalletError::KeyGeneration(format!("{:?}", e)))?;

        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
            .map_err(|e| WalletError::KeyReading(format!("{:?}", e)))?;

        let address = hex::encode(key_pair.public_key().as_ref());
        let now = Utc::now();

        let wallet = WalletData {
            id: None,
            address,
            private_key_bytes: pkcs8_bytes.as_ref().to_vec(),
            created_at: now,
            last_used: now,
            label: label.to_string(),
        };

        // Insert the wallet into the database
        self.conn.execute(
            "INSERT INTO wallets (address, private_key_bytes, created_at, last_used, label)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                wallet.address,
                wallet.private_key_bytes,
                wallet.created_at.to_rfc3339(),
                wallet.last_used.to_rfc3339(),
                wallet.label,
            ],
        )?;

        // Get the ID of the newly inserted wallet
        let id = self.conn.last_insert_rowid();

        let mut wallet_with_id = wallet.clone();
        wallet_with_id.id = Some(id);

        Ok(wallet_with_id)
    }

    pub fn get_wallet_by_address(&self, address: &str) -> Result<WalletData, WalletError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, address, private_key_bytes, created_at, last_used, label
             FROM wallets
             WHERE address = ?1"
        )?;

        let wallet = stmt.query_row(params![address], |row| {
            let created_at_str: String = row.get(3)?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc);

            let last_used_str: String = row.get(4)?;
            let last_used = DateTime::parse_from_rfc3339(&last_used_str)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(4, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc);

            Ok(WalletData {
                id: Some(row.get(0)?),
                address: row.get(1)?,
                private_key_bytes: row.get(2)?,
                created_at,
                last_used,
                label: row.get(5)?,
            })
        }).map_err(|e| {
            match e {
                rusqlite::Error::QueryReturnedNoRows => WalletError::NotFound,
                e => WalletError::Database(e),
            }
        })?;

        Ok(wallet)
    }

    pub fn get_wallet_by_label(&self, label: &str) -> Result<WalletData, WalletError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, address, private_key_bytes, created_at, last_used, label
             FROM wallets
             WHERE label = ?1"
        )?;

        let wallet = stmt.query_row(params![label], |row| {
            let created_at_str: String = row.get(3)?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc);

            let last_used_str: String = row.get(4)?;
            let last_used = DateTime::parse_from_rfc3339(&last_used_str)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(4, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc);

            Ok(WalletData {
                id: Some(row.get(0)?),
                address: row.get(1)?,
                private_key_bytes: row.get(2)?,
                created_at,
                last_used,
                label: row.get(5)?,
            })
        }).map_err(|e| {
            match e {
                rusqlite::Error::QueryReturnedNoRows => WalletError::NotFound,
                e => WalletError::Database(e),
            }
        })?;

        Ok(wallet)
    }

    pub fn list_wallets(&self) -> Result<Vec<WalletData>, WalletError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, address, private_key_bytes, created_at, last_used, label
             FROM wallets
             ORDER BY created_at DESC"
        )?;

        let wallet_iter = stmt.query_map([], |row| {
            let created_at_str: String = row.get(3)?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc);

            let last_used_str: String = row.get(4)?;
            let last_used = DateTime::parse_from_rfc3339(&last_used_str)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(4, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc);

            Ok(WalletData {
                id: Some(row.get(0)?),
                address: row.get(1)?,
                private_key_bytes: row.get(2)?,
                created_at,
                last_used,
                label: row.get(5)?,
            })
        })?;

        let mut wallets = Vec::new();
        for wallet in wallet_iter {
            wallets.push(wallet?);
        }

        Ok(wallets)
    }

    pub fn update_last_used(&self, address: &str) -> Result<(), WalletError> {
        let now = Utc::now();

        self.conn.execute(
            "UPDATE wallets SET last_used = ?1 WHERE address = ?2",
            params![now.to_rfc3339(), address],
        )?;

        Ok(())
    }

    pub fn get_keypair(&self, address: &str) -> Result<Ed25519KeyPair, WalletError> {
        let wallet = self.get_wallet_by_address(address)?;

        let key_pair = Ed25519KeyPair::from_pkcs8(&wallet.private_key_bytes)
            .map_err(|e| WalletError::KeyReading(format!("{:?}", e)))?;

        // Update last used time
        self.update_last_used(address)?;

        Ok(key_pair)
    }
}

impl WalletData {
    pub fn get_keypair(&self) -> Result<Ed25519KeyPair, WalletError> {
        Ed25519KeyPair::from_pkcs8(&self.private_key_bytes)
            .map_err(|e| WalletError::KeyReading(format!("{:?}", e)))
    }
}