-- SQLite schema for todo-cli (matches Todo + optional Location)

CREATE TABLE IF NOT EXISTS todos (
    id          TEXT    PRIMARY KEY NOT NULL,  -- UUID v4 string
    title       TEXT    NOT NULL,
    description TEXT    NOT NULL DEFAULT '',
    completed   INTEGER NOT NULL DEFAULT 0 CHECK (completed IN (0, 1)),
    latitude    REAL,
    longitude   REAL,
    created_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    CHECK (
        (latitude IS NULL AND longitude IS NULL)
        OR (latitude IS NOT NULL AND longitude IS NOT NULL)
    ),
    CHECK (
        latitude IS NULL
        OR (latitude >= -90.0 AND latitude <= 90.0)
    ),
    CHECK (
        longitude IS NULL
        OR (longitude >= -180.0 AND longitude <= 180.0)
    )
);

CREATE INDEX IF NOT EXISTS idx_todos_completed ON todos (completed);
CREATE INDEX IF NOT EXISTS idx_todos_created_at ON todos (created_at);

-- Keep updated_at in sync on UPDATE (optional; app may set explicitly instead)
CREATE TRIGGER IF NOT EXISTS todos_set_updated_at
AFTER UPDATE ON todos
FOR EACH ROW
BEGIN
    UPDATE todos SET updated_at = datetime('now') WHERE id = OLD.id;
END;
