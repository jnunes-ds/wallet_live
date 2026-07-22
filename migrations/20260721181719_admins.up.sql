-- Add up migration script here
CREATE TABLE IF NOT EXISTS admins (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    user_id BIGSERIAL REFERENCES users(id)
);