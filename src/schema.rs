use diesel::{allow_tables_to_appear_in_same_query, joinable};

diesel::table! {
    user (id) {
        id -> Int4,
        contact_id -> Int4,
        created_by -> Int4,
    }
}

diesel::table! {
    contact (id) {
        id -> Int4,
        name -> Text,
    }
}

joinable!(user -> contact (contact_id));

allow_tables_to_appear_in_same_query!(
    user,
    contact,
);
