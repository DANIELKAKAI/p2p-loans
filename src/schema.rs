// @generated automatically by Diesel CLI.

diesel::table! {
    loan_applications (id) {
        id -> Int4,
        amount -> Float8,
        loan_id -> Int4,
        borrower_id -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    loans (id) {
        id -> Int4,
        loan_name -> Varchar,
        loan_amount -> Float8,
        lender_id -> Int4,
        interest_rate -> Float8,
        repayment_period -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 256]
        first_name -> Varchar,
        #[max_length = 256]
        last_name -> Varchar,
        #[max_length = 256]
        email -> Varchar,
        #[max_length = 256]
        password -> Varchar,
        active -> Bool,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::joinable!(loan_applications -> loans (loan_id));
diesel::joinable!(loan_applications -> users (borrower_id));
diesel::joinable!(loans -> users (lender_id));

diesel::allow_tables_to_appear_in_same_query!(
    loan_applications,
    loans,
    users,
);
