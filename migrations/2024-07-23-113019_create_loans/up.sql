CREATE TABLE loans (
  id SERIAL PRIMARY KEY,
  loan_name VARCHAR NOT NULL,
  loan_amount FLOAT NOT NULL,
  interest_rate FLOAT NOT NULL,
  repayment_period INTEGER NOT NULL
);