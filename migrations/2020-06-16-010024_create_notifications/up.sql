CREATE TABLE notifications
(
  id SERIAL NOT NULL,
  done boolean NOT NULL,
  created_at timestamp NOT NULL,
  description varchar NOT NULL
);
