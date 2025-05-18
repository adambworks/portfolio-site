-- Projects table
CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    date_started DATE NOT NULL,
    overview TEXT,
    slug TEXT UNIQUE NOT NULL,
    image TEXT
);

-- Chapters table
CREATE TABLE chapters (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    date_started DATE,
    index INTEGER UNIQUE NOT NULL
);

-- Entries table
CREATE TABLE entries (
    id SERIAL PRIMARY KEY,
    chapter_id INTEGER NOT NULL REFERENCES chapters(id) ON DELETE CASCADE,
    text TEXT,
    image TEXT, -- could be a URL or file path
    date DATE,  -- optional
    index INTEGER UNIQUE NOT NULL

);
