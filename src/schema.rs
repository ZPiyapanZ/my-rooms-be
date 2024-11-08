// @generated automatically by Diesel CLI.

diesel::table! {
    customer_contacts (id) {
        id -> Int4,
        #[max_length = 100]
        full_name -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 15]
        phone_number -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    reservations (id) {
        id -> Int4,
        room_id -> Int4,
        customer_contact_id -> Int4,
        check_in_date -> Date,
        check_out_date -> Date,
        total_price -> Int4,
        #[max_length = 20]
        status -> Varchar,
        created_by -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_by -> Nullable<Int4>,
        updated_at -> Timestamptz,
        confirmed_by -> Nullable<Int4>,
        confirmed_at -> Nullable<Timestamptz>,
        cancelled_by -> Nullable<Int4>,
        cancelled_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    room_types (id) {
        id -> Int4,
        #[max_length = 50]
        type_name -> Varchar,
        description -> Nullable<Text>,
        price_per_night -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        created_by -> Nullable<Int4>,
        updated_by -> Nullable<Int4>,
    }
}

diesel::table! {
    rooms (id) {
        id -> Int4,
        #[max_length = 100]
        room_name -> Varchar,
        type_id -> Nullable<Int4>,
        capacity -> Int4,
        is_available -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        created_by -> Nullable<Int4>,
        updated_by -> Nullable<Int4>,
    }
}

diesel::table! {
    staff (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        position -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(reservations -> customer_contacts (customer_contact_id));
diesel::joinable!(reservations -> rooms (room_id));
diesel::joinable!(rooms -> room_types (type_id));

diesel::allow_tables_to_appear_in_same_query!(
    customer_contacts,
    reservations,
    room_types,
    rooms,
    staff,
);
