Overview

  - Axum + SQLx backend providing CRUD over “records” with JSON API, SQLite storage, CLI-configurable host/port/logging, and
    tracing-based logging.

  Structure

  - src/main.rs: CLI (clap) for --config/-c and --verbose/-v; load TOML config (MGBConfig::parse_from_file), init logger,
    connect SQLite via Database::new, build router (create_router), serve with axum::serve on <server_host>:<server_port>.
  - src/config.rs: MGBConfig (log_level/log_file_path/database_url/server_host/server_port) with defaults and custom
    deserialize_log_level; falls back to defaults on read/parse errors.
  - src/logger.rs: init_logger(verbose, log_path, level) uses tracing subscriber; stdout in verbose, file writer otherwise
    (fallback to default log path if custom fails).
  - src/db/mod.rs: Database wraps SqlitePool; ensures directory exists for SQLite file; connects with create-if-missing;
    runs record::create_table; exposes record_repo().
  - src/db/record.rs: Models Record (id/title/content/created_at) and CreateRecord; RecordRepository CRUD (create,
    get_by_id, get_all, update, delete); create_table schema helper; async tests using in-memory SQLite cover CRUD and not-
    found cases.
  - src/api/mod.rs: AppState { db }; builds router with /health (200, {status:"healthy"}) and nested /records routes.
  - src/api/record.rs: Routes
      - GET /records → list
      - POST /records → create (201) from CreateRecord
      - GET /records/:id → fetch (404 if missing)
      - PUT /records/:id → update (404 if missing)
      - DELETE /records/:id → delete (204 or 404)
        Uses structured logging; maps errors to HTTP status; endpoint tests with tower::ServiceExt.

  Supporting

  - Cargo.toml: deps include axum, tokio, sqlx (sqlite, rustls runtime), clap, serde/json, tracing, chrono, tower.
  - mgb.toml: sample config (info level, logs/mgb.log, sqlite:mgb.db, 127.0.0.1:3000).
  - logs/mgb.log runtime output; mgb.db SQLite file.

  Use this as context for Copilot prompts; ask for additions in these files to expand handlers, validation, config, or
  database logic.