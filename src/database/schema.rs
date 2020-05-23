table! {
    author (author_id) {
        author_id -> Integer,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        DOB -> Nullable<Date>,
        DOD -> Nullable<Date>,
        POB -> Nullable<Integer>,
    }
}

table! {
    author_alias (a_ali_id) {
        a_ali_id -> Integer,
        author_id -> Nullable<Integer>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
    }
}

table! {
    book (book_id) {
        book_id -> Integer,
        title -> Nullable<Varchar>,
        pub_year -> Nullable<Integer>,
        edition -> Nullable<Varchar>,
        language -> Nullable<Integer>,
        publisher -> Nullable<Integer>,
        ean13 -> Nullable<Integer>,
        category -> Nullable<Varchar>,
    }
}

table! {
    book_author (ba_id) {
        ba_id -> Integer,
        a_ali_id -> Nullable<Integer>,
        book_id -> Nullable<Integer>,
    }
}

table! {
    book_rate (bo_rate_id) {
        bo_rate_id -> Integer,
        user -> Nullable<Integer>,
        rating_cat_id -> Nullable<Integer>,
        score -> Nullable<Integer>,
        book_id -> Nullable<Integer>,
    }
}

table! {
    book_rating (bo_rating_id) {
        bo_rating_id -> Integer,
        book_id -> Nullable<Integer>,
        rating_cat_id -> Nullable<Integer>,
        rating -> Nullable<Float>,
        total -> Nullable<Integer>,
        count -> Nullable<Integer>,
    }
}

table! {
    rating_cat (rating_cat_id) {
        rating_cat_id -> Integer,
        name -> Nullable<Varchar>,
        _desc -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        salt1 -> Varchar,
        salt2 -> Varchar,
        created_by -> Varchar,
        created_at -> Timestamp,
        updated_by -> Varchar,
        updated_at -> Timestamp,
    }
}

joinable!(author_alias -> author (author_id));
joinable!(book_author -> author_alias (a_ali_id));
joinable!(book_author -> book (book_id));
joinable!(book_rate -> rating_cat (rating_cat_id));
joinable!(book_rating -> rating_cat (rating_cat_id));

allow_tables_to_appear_in_same_query!(
    author,
    author_alias,
    book,
    book_author,
    book_rate,
    book_rating,
    rating_cat,
    users,
);
