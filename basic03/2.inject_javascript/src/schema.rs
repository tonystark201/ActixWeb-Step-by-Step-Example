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

allow_tables_to_appear_in_same_query!(
    students,
    teachers,
);
