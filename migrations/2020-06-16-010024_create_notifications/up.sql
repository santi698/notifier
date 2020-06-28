CREATE TABLE notifications
(
  id uuid NOT NULL,
  user_id uuid NOT NULL,
  read_at timestamp,
  created_at timestamp NOT NULL,
  description varchar NOT NULL
);
