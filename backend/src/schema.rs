// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Uuid,
        avatar -> Varchar,
        level -> Varchar,
        preference_id -> Uuid,
        role_ids -> Array<Nullable<Uuid>>,
        sensitivity_ids -> Array<Nullable<Uuid>>,
        survey_results_id -> Uuid,
    }
}

diesel::table! {
    conduct_codes (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Array<Nullable<Text>>,
        qualifications -> Array<Nullable<Text>>,
        restrictions -> Array<Nullable<Text>>,
        examples -> Array<Nullable<Text>>,
        sensitivity_ids -> Array<Nullable<Uuid>>,
    }
}

diesel::table! {
    dress_codes (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Array<Nullable<Text>>,
        qualifications -> Array<Nullable<Text>>,
        restrictions -> Array<Nullable<Text>>,
        links -> Array<Nullable<Text>>,
        examples -> Array<Nullable<Text>>,
        sensitivity_ids -> Array<Nullable<Uuid>>,
    }
}

diesel::table! {
    events (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Array<Nullable<Text>>,
        imgs -> Array<Nullable<Text>>,
        links -> Array<Nullable<Text>>,
        ticketing -> Array<Nullable<Text>>,
        location -> Array<Nullable<Text>>,
        directions -> Array<Nullable<Text>>,
        map_images -> Array<Nullable<Text>>,
        start_time -> Timestamp,
        end_time -> Timestamp,
        conduct_code_ids -> Array<Nullable<Uuid>>,
        dress_code_ids -> Array<Nullable<Uuid>>,
        other_expectations -> Array<Nullable<Text>>,
        account_ids -> Array<Nullable<Uuid>>,
        diet_ids -> Array<Nullable<Uuid>>,
        tasting_session_ids -> Array<Nullable<Uuid>>,
        env_feature_ids -> Array<Nullable<Uuid>>,
        sensitivity_ids -> Array<Nullable<Uuid>>,
    }
}

diesel::table! {
    logins (id) {
        id -> Uuid,
        email -> Varchar,
        pw_salt -> Varchar,
        pw_hash -> Varchar,
        mfa_salt -> Varchar,
        mfa_hash -> Varchar,
    }
}

diesel::table! {
    preferences (id) {
        id -> Uuid,
        browser_theme -> Varchar,
        display_name -> Varchar,
        pronouns -> Varchar,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        title -> Varchar,
        description -> Varchar,
        responsibility -> Varchar,
        discount -> Varchar,
        seen_by_role -> Array<Nullable<Uuid>>,
    }
}

diesel::table! {
    sensitivities (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Array<Nullable<Text>>,
        links -> Array<Nullable<Text>>,
        precautions -> Array<Nullable<Text>>,
        imparing -> Bool,
        life_threatening -> Bool,
        dietary -> Bool,
        environmental -> Bool,
        social -> Bool,
    }
}

diesel::table! {
    survey_results (id) {
        id -> Uuid,
        aesthetics -> Array<Nullable<Text>>,
        cognitive -> Array<Nullable<Text>>,
        cosmology -> Array<Nullable<Text>>,
        environmental -> Array<Nullable<Text>>,
        epistemology -> Array<Nullable<Text>>,
        ethics -> Array<Nullable<Text>>,
        history -> Array<Nullable<Text>>,
        isms -> Array<Nullable<Text>>,
        law -> Array<Nullable<Text>>,
        logic -> Array<Nullable<Text>>,
        maths -> Array<Nullable<Text>>,
        ontology -> Array<Nullable<Text>>,
        political -> Array<Nullable<Text>>,
        rhetoric -> Array<Nullable<Text>>,
        science -> Array<Nullable<Text>>,
        theology -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        address -> Array<Nullable<Text>>,
        address_verified -> Array<Nullable<Bool>>,
        email -> Array<Nullable<Text>>,
        email_verified -> Array<Nullable<Bool>>,
        phone -> Array<Nullable<Text>>,
        phone_verified -> Array<Nullable<Bool>>,
        taint -> Varchar,
        login_ids -> Array<Nullable<Uuid>>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    conduct_codes,
    dress_codes,
    events,
    logins,
    preferences,
    roles,
    sensitivities,
    survey_results,
    users,
);
