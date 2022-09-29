table! {
    students (uid) {
        uid -> Int4,
        name -> Varchar,
        age -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    teachers (uid) {
        uid -> Int4,
        name -> Varchar,
        age -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        uid -> Varchar,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    students,
    teachers,
    users,
);
