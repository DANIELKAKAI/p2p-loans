CREATE TABLE payment (
    id SERIAL PRIMARY KEY,
    payment_code VARCHAR NOT NULL,
    amount FLOAT NOT NULL,
    loan_id SERIAL,
    FOREIGN KEY (loan_id) REFERENCES loans(id),
    updated_at timestamp  without time zone default CURRENT_TIMESTAMP not null,
    created_at timestamp  without time zone default CURRENT_TIMESTAMP not null
);