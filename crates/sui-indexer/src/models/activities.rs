use crate::models::collections::Collection;
use crate::models::tokens::Token;
use crate::schema::activities;
use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use diesel::insert_into;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(DbEnum, Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::ActivityType"]
#[serde(rename_all = "snake_case")]
pub enum ActivityType {
    Created,
    ///only for collections
    Minted,
    Transferred,
    Listed,
    Canceled,
    Sold,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = activities)]
pub struct Activity {
    pub chain_id: i64,
    pub version: i64,
    pub tx: Option<String>,
    pub event_account_address: String,
    pub event_creation_number: i64,
    pub event_sequence_number: i64,
    pub collection_data_id_hash: String,
    pub token_data_id_hash: String,
    pub property_version: i64,
    pub creator_address: String,
    pub collection_name: String,
    pub name: String,
    pub transfer_type: ActivityType,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub token_amount: i64,
    pub coin_type: Option<String>,
    pub coin_amount: i64,
    pub transaction_timestamp: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn batch_insert(
    connection: &mut PgConnection,
    new: &Vec<Activity>,
) -> Result<usize> {
    insert_into(activities::table)
        .values(new)
        .execute(connection)
        .map_err(|e| anyhow::anyhow!(e.to_string()))
}

impl Activity {
    pub fn new_from_collection_with_type(
        t: ActivityType,
        collection: &Collection,
    ) -> Activity {
        Activity {
            chain_id: collection.chain_id as i64,
            version: collection.version,
            tx: collection.tx.clone(),
            event_account_address: collection.creator_address.clone(),
            event_creation_number: 0,
            event_sequence_number: 0,
            collection_data_id_hash: collection.collection_id.clone(),
            token_data_id_hash: "".to_string(),
            property_version: collection.version,
            creator_address: "".to_string(),
            collection_name: collection.collection_name.clone(),
            name: "".to_string(),
            transfer_type: t,
            from_address: Some(collection.creator_address.clone()),
            to_address: None,
            token_amount: 0,
            coin_type: None,
            coin_amount: 0,
            transaction_timestamp: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn new_from_token_with_type(
        t: ActivityType,
        (token, sender): &(Token, String),
    ) -> Activity {
        Activity {
            chain_id: token.chain_id,
            version: token.version,
            tx: token.tx.clone(),
            event_account_address: token.creator_address.clone(),
            event_creation_number: 0,
            event_sequence_number: 0,
            collection_data_id_hash: token.collection_id.clone(),
            token_data_id_hash: token.token_id.clone(),
            property_version: token.version,
            creator_address: "".to_string(),
            collection_name: token.collection_name.clone(),
            name: token.token_name.clone(),
            transfer_type: t,
            from_address: Some(sender.clone()),
            to_address: token.owner_address.clone(),
            token_amount: 0,
            coin_type: None,
            coin_amount: 0,
            transaction_timestamp: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
