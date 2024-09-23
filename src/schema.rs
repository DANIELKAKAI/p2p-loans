// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_type_enum"))]
    pub struct UserTypeEnum;
}

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
        amount_deposited -> Nullable<Bool>,
    }
}

diesel::table! {
    payment (id) {
        id -> Int4,
        payment_code -> Varchar,
        amount -> Float8,
        loan_id -> Int4,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserTypeEnum;

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
        user_type -> UserTypeEnum,
    }
}

diesel::joinable!(loan_applications -> loans (loan_id));
diesel::joinable!(loan_applications -> users (borrower_id));
diesel::joinable!(loans -> users (lender_id));
diesel::joinable!(payment -> loans (loan_id));

diesel::allow_tables_to_appear_in_same_query!(
    loan_applications,
    loans,
    payment,
    users,
);
