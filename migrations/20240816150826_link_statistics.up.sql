CREATE TABLE IF NOT EXISTS link_statistics
(
  id serial PRIMARY KEY,
  link_id text NOT NULL,
  referer text,
  user_agent text,
  CONSTRAINT fk_links
  FOREIGN KEY (link_id)
  REFERENCES links (id)
);

CREATE INDEX idx_link_statistics_link_id on link_statistics USING btree (link_id);