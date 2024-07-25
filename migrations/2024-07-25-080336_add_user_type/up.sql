CREATE TYPE user_type_enum AS ENUM ('LENDER', 'BORROWER');

ALTER TABLE users ADD COLUMN user_type user_type_enum NOT NULL DEFAULT 'BORROWER';
