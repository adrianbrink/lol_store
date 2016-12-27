CREATE TABLE games (
    id SERIAL PRIMARY KEY,
    game_id BIGINT NOT NULL,
    game_length BIGINT NOT NULL,
    game_mode VARCHAR NOT NULL,
    game_queue_config_id BIGINT NOT NULL,
    game_start_time BIGINT NOT NULL,
    game_type VARCHAR NOT NULL,
    map_id BIGINT NOT NULL,
    platform_id VARCHAR NOT NULL
)