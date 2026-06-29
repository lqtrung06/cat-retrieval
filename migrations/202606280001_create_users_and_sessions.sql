CREATE TABLE IF NOT EXISTS users (
    id uuid PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('admin', 'user_free', 'user_pro')),
    status TEXT NOT NULL CHECK (status IN ('active', 'disabled')),
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS sessions (
    id uuid PRIMARY KEY,
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS sessions_user_id_idx ON sessions(user_id);
CREATE INDEX IF NOT EXISTS sessions_expires_at_idx ON sessions(expires_at);

INSERT INTO users (id, username, email, password_hash, role, status)
VALUES (
    '00000000-0000-0000-0000-000000000001',
    'admin',
    NULL,
    '$2b$12$bVL0BkmRCBinAvN4Wky8oO6upduInO70A/Spl/jJYYMHaLeQU7fWy',
    'admin',
    'active'
)
ON CONFLICT (username) DO NOTHING;
