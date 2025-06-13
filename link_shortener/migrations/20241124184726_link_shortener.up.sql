CREATE TABLE links (
    id text NOT NULL PRIMARY KEY,
    original_url text NOT NULL,
    shortened_path text UNIQUE NOT NULL,
    is_active boolean NOT NULL,
    created_at timestamptz NOT NULL,
    expires_at timestamptz NOT NULL
);


CREATE INDEX idx_links_shortened_path_is_active ON links (shortened_path, is_active);
CREATE INDEX idx_links_expires_at ON links USING btree (expires_at);

CREATE TABLE tags (
    id text NOT NULL PRIMARY KEY,
    name text UNIQUE NOT NULL
);

CREATE INDEX idx_tags_name ON tags USING btree (name);

CREATE TABLE links_tags (
    link_id text REFERENCES links(id) ON DELETE CASCADE,
    tag_id text REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (link_id, tag_id)
);

CREATE INDEX idx_links_tags_link_id ON links_tags (link_id);
CREATE INDEX idx_links_tags_tag_id ON links_tags (tag_id);
CREATE INDEX idx_links_tags_link_tag ON links_tags (link_id, tag_id);
