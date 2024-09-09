-- Add up migration script here
ALTER TABLE gh_repo ADD INDEX idx_stars(stars);
