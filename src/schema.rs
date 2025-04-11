use diesel::prelude::*;

table! {
    user (id) {
        id -> Int4,
        name -> Text,
        contact_id -> Int4,
        created_by_id -> Int4,
    }
}

table! {
    contact (id) {
        id -> Int4,
        name -> Text,
    }
}

joinable!(user -> contact (contact_id));

allow_tables_to_appear_in_same_query!(user, contact,);
