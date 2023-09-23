CREATE TABLE messages (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name TEXT,
  username TEXT NOT NULL,
  body TEXT,
  exercise_type VARCHAR,
  photo BLOB
)
