ALTER TABLE users ADD COLUMN updated_at timestamp  without time zone default CURRENT_TIMESTAMP not null;
ALTER TABLE users ADD COLUMN created_at timestamp  without time zone default CURRENT_TIMESTAMP not null;

ALTER TABLE loans ADD COLUMN updated_at timestamp  without time zone default CURRENT_TIMESTAMP not null;
ALTER TABLE loans ADD COLUMN created_at timestamp  without time zone default CURRENT_TIMESTAMP not null;

ALTER TABLE loan_applications ADD COLUMN updated_at timestamp  without time zone default CURRENT_TIMESTAMP not null;
ALTER TABLE loan_applications ADD COLUMN created_at timestamp  without time zone default CURRENT_TIMESTAMP not null;