-- Add up migration script here
CREATE TABLE gh_repo (
    id INTEGER PRIMARY KEY,
    owner_id INTEGER NOT NULL,
    repo_name VARCHAR(255) NOT NULL,
    lang VARCHAR(20) NOT NULL,
    stars INTEGER NOT NULL,
    url TEXT NOT NULL,
    description TEXT,
    readme TEXT,
    created_at DATETIME(3) NOT NULL,
    updated_at DATETIME(3) NOT NULL
);

CREATE TABLE gh_owner (
    id INTEGER PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    avatar_url TEXT NOT NULL,
    url TEXT NOT NULL
);

CREATE TABLE gh_repo_gf_hw_counts (
    repo_id INTEGER PRIMARY KEY,
    good_first_issues_count INTEGER NOT NULL,
    help_wanted_issues_count INTEGER NOT NULL,
    created_at DATETIME(3) NOT NULL,
    updated_at DATETIME(3) NOT NULL
);

CREATE TABLE gh_repo_list_crawling_history (
    id INTEGER PRIMARY KEY,
    lang VARCHAR(20) NOT NULL,
    min_stars INTEGER NOT NULL,
    max_stars INTEGER NOT NULL,
    last_pushed VARCHAR(10) NOT NULL,
    good_first_issues_count INTEGER NOT NULL,
    help_wanted_issues_count INTEGER NOT NULL,
    created_at DATETIME(3) NOT NULL
);

CREATE TABLE gh_repo_crawling_history (
    repo_id INTEGER PRIMARY KEY,
    action_type VARCHAR(30),
    created_at DATETIME(3) NOT NULL,
    updated_at DATETIME(3) NOT NULL
);