CREATE TABLE subscriptions(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    mail TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL
);
