CREATE TABLE shards (
    id SERIAL PRIMARY KEY,
    hostname VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    region_tag VARCHAR NOT NULL,
    slug VARCHAR NOT NULL
)