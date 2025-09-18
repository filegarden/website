CREATE EXTENSION citext;

-- `timestamptz` has microsecond resolution by default, which needlessly
-- increases the attack surface for timing attacks. `timestamptz(3)` only has
-- millisecond resolution and should always be used instead.
--
-- TODO: Something in CI should guarantee this, and that should have a code
-- comment instead of this database migration comment, because migrations can't
-- be edited once deployed.

CREATE TABLE terms_version (
    constrain_table_to_one_row boolean NOT NULL UNIQUE DEFAULT TRUE
        CHECK (constrain_table_to_one_row),
    updated_at timestamptz(3) PRIMARY KEY
        GENERATED ALWAYS AS (GREATEST(terms_updated_at, privacy_updated_at))
        STORED,
    terms_updated_at timestamptz(3) NOT NULL DEFAULT now(),
    privacy_updated_at timestamptz(3) NOT NULL DEFAULT now(),
    terms_hash bytea NOT NULL,
    privacy_hash bytea NOT NULL
);

CREATE TABLE users (
    created_at timestamptz(3) NOT NULL DEFAULT now(),
    accepted_terms_at timestamptz(3) NOT NULL,
    id bytea PRIMARY KEY,
    email citext NOT NULL UNIQUE,
    name text NOT NULL,
    password_hash text NOT NULL
);

CREATE TABLE totp (
    created_at timestamptz(3) NOT NULL DEFAULT now(),
    user_id bytea PRIMARY KEY REFERENCES users (id) ON DELETE CASCADE,
    secret bytea,
    code_used_last text
        CHECK ((secret IS NULL) = (code_used_last IS NULL)),
    code_used_2nd_to_last text,
    unused_backup_codes text[]
        CHECK ((secret IS NULL) = (unused_backup_codes IS NULL)),
    unverified_secret bytea
        CHECK ((secret IS NULL) != (unverified_secret IS NULL))
);

CREATE TABLE unverified_emails (
    created_at timestamptz(3) NOT NULL DEFAULT now(),
    token_hash bytea PRIMARY KEY,
    user_accepted_terms_at timestamptz(3) NOT NULL DEFAULT now(),
    user_id bytea UNIQUE REFERENCES users (id) ON DELETE CASCADE,
    email citext NOT NULL,
    code_hash text
);

CREATE UNIQUE INDEX unverified_user_emails ON unverified_emails (email)
    WHERE user_id IS NULL;

CREATE TABLE password_resets (
    created_at timestamptz(3) NOT NULL DEFAULT now(),
    token_hash bytea PRIMARY KEY,
    user_id bytea NOT NULL UNIQUE REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE sessions (
    created_at timestamptz(3) NOT NULL DEFAULT now(),
    accessed_at timestamptz(3) NOT NULL DEFAULT now(),
    token_hash bytea PRIMARY KEY,
    user_id bytea NOT NULL REFERENCES users (id) ON DELETE CASCADE
);

CREATE INDEX sessions_by_accessed_at ON sessions (accessed_at);
CREATE INDEX sessions_by_user_id ON sessions (user_id);

CREATE TYPE encoding AS ENUM ('br');

CREATE TABLE files (
    created_at timestamptz(3) NOT NULL DEFAULT now(),
    modified_at timestamptz(3) NOT NULL DEFAULT now(),
    id bytea PRIMARY KEY,
    name text NOT NULL,
    owner_id bytea NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    parent_id_path bytea[] NOT NULL,
    parent_name_path text[] NOT NULL,
    shared boolean NOT NULL DEFAULT FALSE,
    parts integer NOT NULL DEFAULT 1,
    size bigint NOT NULL,
    encoded_size bigint NOT NULL,
    encoding encoding,
    type text NOT NULL,

    UNIQUE (owner_id, parent_name_path, name)
);

CREATE INDEX files_by_id_path ON files (owner_id, parent_id_path, id);

CREATE TABLE folders (
    created_at timestamptz(3) NOT NULL DEFAULT now(),
    id bytea PRIMARY KEY,
    name text NOT NULL,
    owner_id bytea NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    parent_id_path bytea[] NOT NULL,
    parent_name_path text[] NOT NULL,
    browse_key bytea UNIQUE NOT NULL,
    size bigint NOT NULL DEFAULT 0,

    UNIQUE (owner_id, parent_name_path, name)
);

CREATE INDEX folders_by_id_path ON folders (owner_id, parent_id_path, id);
