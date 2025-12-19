Code in Rust 2024 using Axum 0.7 and SQLx (SQLite, Tokio). Follow existing style:

  - Modules: src/api, src/db, src/config, src/logger, src/main.rs.
  - Use tracing for structured logs; info for happy path, error on failures.
  - Axum handlers: function per route, take State/Path/Json, return (StatusCode, Json(...)).
  - DB access: through RecordRepository on a SqlitePool; async CRUD; create_table sets schema; prefer ? error propagation
    and map to HTTP codes in handlers.
  - Config: deserialize from TOML with defaults; keep sensible fallbacks rather than panicking.
  - Logging: stdout when verbose, file otherwise; no ANSI in file mode.
  - Tests: async #[tokio::test], use in-memory SQLite; cover CRUD and not-found cases; for endpoints use tower::ServiceExt
    + oneshot.
    Add new endpoints/features consistent with this structure and conventions.