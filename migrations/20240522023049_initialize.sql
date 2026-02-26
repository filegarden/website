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
    updated_at timestamptz(3)
        GENERATED ALWAYS AS (GREATEST(terms_updated_at, privacy_updated_at))
        VIRTUAL,
    terms_updated_at timestamptz(3) NOT NULL DEFAULT now(),
    privacy_updated_at timestamptz(3) NOT NULL DEFAULT now(),
    terms_hash bytea NOT NULL,
    privacy_hash bytea NOT NULL
);

CREATE TABLE unverified_users (
    created_at timestamptz(3) NOT NULL DEFAULT now(),
    token_hash bytea PRIMARY KEY,
    accepted_terms_at timestamptz(3) NOT NULL DEFAULT now(),
    email citext NOT NULL UNIQUE,
    code_hash text
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
    secret bytea NOT NULL,
    otp_used_last text NOT NULL,
    otp_used_2nd_to_last text,
    unused_backup_codes text[] NOT NULL
);

CREATE TABLE email_change_requests (
    created_at timestamptz(3) NOT NULL DEFAULT now(),
    token_hash bytea PRIMARY KEY,
    user_id bytea NOT NULL UNIQUE REFERENCES users (id) ON DELETE CASCADE,
    email citext NOT NULL
);

CREATE INDEX unverified_user_emails ON email_change_requests (email);

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
CREATE INDEX sessions_by_user ON sessions (user_id);

CREATE TYPE encoding AS ENUM ('br', 'lep', 'wv');

CREATE TABLE file_contents (
    created_at timestamptz(3) NOT NULL DEFAULT now(),
    modified_at timestamptz(3) NOT NULL DEFAULT now(),
    id bytea PRIMARY KEY,
    complete boolean NOT NULL,
    partial_hash bytea
        CHECK ((partial_hash IS NULL) = complete),
    hash bytea
        CHECK ((hash IS NULL) = NOT complete),
    original_size bigint NOT NULL,
    encoding encoding,
    encoded_size bigint NOT NULL DEFAULT 0,
    part_count integer NOT NULL DEFAULT 0,
    decoded_part_size integer
        CHECK (
            decoded_part_size IS NULL
            OR NOT complete
            OR part_count = original_size / decoded_part_size
        ),
    decoded_part_sizes integer[]
        CHECK (
            decoded_part_sizes IS NULL
            OR part_count = cardinality(decoded_part_sizes)
        )
        CHECK (decoded_part_sizes IS NULL OR decoded_part_size IS NULL),

    UNIQUE (id, complete, original_size)
);

CREATE INDEX file_contents_by_hash ON file_contents (hash);

CREATE TABLE files (
    created_at timestamptz(3) NOT NULL,
    modified_at timestamptz(3) NOT NULL DEFAULT now(),
    id bytea NOT NULL,
    complete boolean NOT NULL DEFAULT FALSE,
    name text NOT NULL,
    owner_id bytea NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    parent_id_path bytea[] NOT NULL,
    parent_name_path text[] NOT NULL,
    size bigint NOT NULL,
    content_id bytea NOT NULL,
    type text NOT NULL,
    shared boolean NOT NULL DEFAULT FALSE,

    PRIMARY KEY (id, complete),
    FOREIGN KEY (content_id, complete, size)
        REFERENCES file_contents (id, complete, original_size)
        MATCH FULL
        ON UPDATE CASCADE,
    UNIQUE (owner_id, parent_id_path, id, complete),
    UNIQUE (owner_id, parent_name_path, name, complete)
);

CREATE INDEX files_by_content_id ON files (content_id);

CREATE TABLE files_processing (
    created_at timestamptz(3) NOT NULL DEFAULT now(),
    required_before_deduping_file_id bytea UNIQUE
        REFERENCES files (id) ON DELETE CASCADE
        CHECK ((required_before_deduping_file_id IS NULL) !=
            (source_content_hash IS NULL)),
    source_content_hash bytea UNIQUE,
    encoding encoding NOT NULL,
    output_content_id bytea UNIQUE REFERENCES file_contents (id),
    failed_at timestamptz(3)
        CHECK ((failed_at IS NULL) != (output_content_id IS NULL)),

    UNIQUE NULLS NOT DISTINCT
        (required_before_deduping_file_id, source_content_hash, encoding)
);

CREATE TABLE folders (
    created_at timestamptz(3) NOT NULL DEFAULT now(),
    id bytea PRIMARY KEY,
    name text NOT NULL,
    owner_id bytea NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    parent_id_path bytea[] NOT NULL,
    parent_name_path text[] NOT NULL,
    browse_key bytea UNIQUE NOT NULL,
    size bigint NOT NULL DEFAULT 0,

    UNIQUE (owner_id, parent_id_path, id),
    UNIQUE (owner_id, parent_name_path, name)
);
