CREATE TABLE links (
    code TEXT PRIMARY KEY,
    url TEXT NOT NULL,
    clicks INTEGER NOT NULL DEFAULT 0,
    created_at DATE NOT NULL DEFAULT current_timestamp
);