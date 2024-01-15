-- add slug column to posts table

ALTER TABLE posts ADD COLUMN slug VARCHAR(255) NOT NULL;
