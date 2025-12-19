Context: Rust 2024 service using Axum 0.7 + SQLx (SQLite) as above. Keep module structure and logging/error-handling
  style. When proposing changes, apply this review checklist:

  - Behavior: Are features correct? Any regressions or missing edge cases? Validate HTTP statuses and payloads.
  - Errors: Are failures surfaced with meaningful messages/logs? Avoid panics in request path; map DB errors to HTTP.
  - Concurrency/async: Any race/await issues? Avoid blocking calls in async handlers.
  - Data/DB: Are queries correct and safe? Migrations/schema (create_table) aligned with models?
  - API: Do handlers validate inputs? Are status codes and JSON shapes consistent?
  - Config: Respect defaults/fallbacks; no silent misconfig; sane parsing of log levels/paths/DB URL.
  - Logging/observability: Useful info/error logs without leaking secrets; file vs stdout behavior preserved.
  - Tests: Are there tests for new paths and error cases? Do existing tests remain valid (in-memory SQLite, tower oneshot)?
  - Performance: Any obvious inefficiencies for the hot paths?
  - Style/structure: Matches existing module layout and tracing style; idiomatic Rust.

  Generate diffs/PR guidance or code following these conventions and checklist.