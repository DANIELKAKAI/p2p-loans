CREATE TABLE loan_applications (
    id SERIAL PRIMARY KEY,
    amount FLOAT NOT NULL,
    loan_id SERIAL,
    borrower_id SERIAL, 
    FOREIGN KEY (loan_id) REFERENCES loans(id),
    FOREIGN KEY (borrower_id) REFERENCES users(id),
    updated_at timestamp  without time zone default CURRENT_TIMESTAMP not null,
    created_at timestamp  without time zone default CURRENT_TIMESTAMP not null
);