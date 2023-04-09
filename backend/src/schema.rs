// @generated automatically by Diesel CLI.

diesel::table! {
    fixture (id) {
        id -> Int4,
        home_team -> Nullable<Varchar>,
        away_team -> Nullable<Varchar>,
        home_score -> Nullable<Int4>,
        away_score -> Nullable<Int4>,
        result -> Nullable<Int4>,
    }
}

diesel::table! {
    player (id) {
        id -> Int4,
        username -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        is_active -> Nullable<Bool>,
        points -> Nullable<Int4>,
    }
}

diesel::table! {
    prediction (id) {
        id -> Int4,
        fixture_id -> Nullable<Int4>,
        player_id -> Nullable<Int4>,
        home_score -> Nullable<Int4>,
        away_score -> Nullable<Int4>,
        result -> Nullable<Int4>,
    }
}

diesel::table! {
    team (id) {
        id -> Int4,
        full_name -> Nullable<Varchar>,
        short_name -> Nullable<Varchar>,
        three_letter_name -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    fixture,
    player,
    prediction,
    team,
);
