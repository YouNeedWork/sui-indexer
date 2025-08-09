// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "activity_type"))]
    pub struct ActivityType;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "list_type"))]
    pub struct ListType;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "market_type"))]
    pub struct MarketType;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "offer_type"))]
    pub struct OfferType;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "order_type"))]
    pub struct OrderType;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "token_status"))]
    pub struct TokenStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ActivityType;

    activities (id) {
        id -> Int8,
        chain_id -> Int8,
        version -> Int8,
        tx -> Nullable<Text>,
        event_account_address -> Text,
        event_creation_number -> Int8,
        event_sequence_number -> Int8,
        collection_data_id_hash -> Text,
        token_data_id_hash -> Text,
        property_version -> Int8,
        creator_address -> Text,
        collection_name -> Text,
        name -> Text,
        transfer_type -> ActivityType,
        from_address -> Nullable<Text>,
        to_address -> Nullable<Text>,
        token_amount -> Int8,
        coin_type -> Nullable<Text>,
        coin_amount -> Nullable<Int8>,
        transaction_timestamp -> Timestamptz,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    check_point (chain_id) {
        chain_id -> Int8,
        version -> Int8,
    }
}

diesel::table! {
    collections (collection_id) {
        chain_id -> Int4,
        #[max_length = 255]
        slug -> Nullable<Varchar>,
        #[max_length = 255]
        collection_id -> Varchar,
        #[max_length = 255]
        collection_type -> Varchar,
        #[max_length = 255]
        creator_address -> Varchar,
        #[max_length = 255]
        royaltie -> Nullable<Varchar>,
        #[max_length = 255]
        display_name -> Nullable<Varchar>,
        #[max_length = 255]
        website -> Nullable<Varchar>,
        #[max_length = 255]
        discord -> Nullable<Varchar>,
        #[max_length = 255]
        twitter -> Nullable<Varchar>,
        #[max_length = 255]
        icon -> Nullable<Varchar>,
        #[max_length = 255]
        banner -> Nullable<Varchar>,
        #[max_length = 255]
        collection_name -> Varchar,
        description -> Varchar,
        supply -> Int8,
        version -> Int8,
        metadata_uri -> Text,
        #[max_length = 255]
        tx -> Nullable<Varchar>,
        metadata -> Text,
        verify -> Bool,
        last_metadata_sync -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ListType;
    use super::sql_types::MarketType;

    lists (id) {
        id -> Int4,
        chain_id -> Int8,
        coin_id -> Int4,
        #[max_length = 255]
        list_id -> Varchar,
        list_time -> Timestamptz,
        #[max_length = 255]
        token_id -> Varchar,
        #[max_length = 255]
        seller_address -> Varchar,
        seller_value -> Int8,
        expire_time -> Nullable<Timestamptz>,
        list_type -> ListType,
        market_type -> MarketType,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OfferType;

    offers (id) {
        id -> Int4,
        chain_id -> Int8,
        coin_id -> Int4,
        #[max_length = 255]
        offer_id -> Varchar,
        #[max_length = 255]
        list_id -> Varchar,
        #[max_length = 255]
        buyer_address -> Varchar,
        offer_value -> Int8,
        offer_type -> OfferType,
        expire_time -> Timestamptz,
        offer_time -> Timestamptz,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OrderType;

    orders (id) {
        id -> Int4,
        chain_id -> Int8,
        coin_id -> Int4,
        #[max_length = 255]
        list_id -> Varchar,
        #[max_length = 255]
        token_id -> Varchar,
        #[max_length = 255]
        offer_id -> Nullable<Varchar>,
        #[max_length = 255]
        seller_address -> Varchar,
        #[max_length = 255]
        buyer_address -> Varchar,
        value -> Int8,
        order_type -> OrderType,
        sell_time -> Timestamptz,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TokenStatus;

    tokens (token_id) {
        chain_id -> Int8,
        #[max_length = 255]
        token_id -> Varchar,
        #[max_length = 255]
        collection_id -> Varchar,
        #[max_length = 255]
        creator_address -> Varchar,
        #[max_length = 255]
        collection_type -> Varchar,
        #[max_length = 255]
        collection_name -> Varchar,
        #[max_length = 255]
        token_name -> Varchar,
        attributes -> Nullable<Text>,
        version -> Int8,
        payee_address -> Varchar,
        royalty_points_numerator -> Int8,
        royalty_points_denominator -> Int8,
        owner_address -> Nullable<Varchar>,
        metadata_uri -> Varchar,
        metadata_json -> Nullable<Text>,
        image -> Nullable<Varchar>,
        tx -> Nullable<Varchar>,
        status -> Nullable<TokenStatus>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    activities,
    check_point,
    collections,
    lists,
    offers,
    orders,
    tokens,
);
