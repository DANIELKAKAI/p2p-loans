CREATE TABLE loan_applications (
    id SERIAL PRIMARY KEY,
    amount FLOAT NOT NULL,
    loan_id SERIAL,
    user_id SERIAL, 
    FOREIGN KEY (loan_id) REFERENCES loans(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);