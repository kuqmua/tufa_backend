CREATE TABLE idempotency (
   user_id uuid NOT NULL REFERENCES users(user_id),
   idempotency_key TEXT NOT NULL,
   created_at timestamptz NOT NULL,
   PRIMARY KEY(user_id, idempotency_key)
);