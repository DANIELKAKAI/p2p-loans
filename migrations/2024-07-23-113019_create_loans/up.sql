CREATE TABLE loans (
  id SERIAL PRIMARY KEY,
  loan_name VARCHAR NOT NULL,
  loan_amount FLOAT NOT NULL,
  lender_id SERIAL,
  FOREIGN KEY (lender_id) REFERENCES users(id),
  interest_rate FLOAT NOT NULL,
  repayment_period INTEGER NOT NULL,
  updated_at timestamp  without time zone default CURRENT_TIMESTAMP not null,
  created_at timestamp  without time zone default CURRENT_TIMESTAMP not null
);