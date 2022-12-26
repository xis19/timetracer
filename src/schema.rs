// @generated automatically by Diesel CLI.

diesel::table! {
    instantiate_class (name, object) {
        name -> Text,
        object -> Text,
        duration -> Integer,
        count -> Integer,
    }
}

diesel::table! {
    instantiate_function (name, object) {
        name -> Text,
        object -> Text,
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
    parse_class (name, object) {
        name -> Text,
        object -> Text,
        duration -> Integer,
        count -> Integer,
    }
}

diesel::table! {
    parse_template (name, object) {
        name -> Text,
        object -> Text,
        duration -> Integer,
        count -> Integer,
    }
}

diesel::table! {
    source (path, object) {
        path -> Text,
        object -> Text,
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
