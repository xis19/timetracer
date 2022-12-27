// @generated automatically by Diesel CLI.

diesel::table! {
    instantiate_class (name) {
        name -> Text,
        duration -> Integer,
        count -> Integer,
    }
}

diesel::table! {
    instantiate_function (name) {
        name -> Text,
        duration -> Integer,
        count -> Integer,
    }
}

diesel::table! {
    objects (path) {
        path -> Text,
        total_time -> Integer,
        frontend -> Integer,
        backend -> Integer,
    }
}

diesel::table! {
    parse_class (name) {
        name -> Text,
        duration -> Integer,
        count -> Integer,
    }
}

diesel::table! {
    parse_template (name) {
        name -> Text,
        duration -> Integer,
        count -> Integer,
    }
}

diesel::table! {
    source (path) {
        path -> Text,
        duration -> Integer,
        count -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    instantiate_class,
    instantiate_function,
    objects,
    parse_class,
    parse_template,
    source,
);
