CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    "posts" (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
        title VARCHAR(255) NOT NULL,
        content TEXT NOT NULL,
        photo VARCHAR(255) NOT NULL,
        user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );
