// @generated automatically by Diesel CLI.

diesel::table! {
    employees (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        department -> Varchar,
        salary -> Int4,
        age -> Int4,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    employees,
    posts,
);
