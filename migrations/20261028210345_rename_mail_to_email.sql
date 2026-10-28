-- Add migration script here
ALTER TABLE subscriptions RENAME COLUMN mail TO email;
