ALTER TABLE users ADD COLUMN salt text NOT NULL DEFAULT 'default_salt';
