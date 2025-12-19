You are working in an existing Rust (edition 2024) Axum + SQLx (SQLite) project.

Goal
- Add a brand-new CRUD resource implemented in the same style/pattern as the existing “Record” resource.
- It must support: create, get by id, get all, update, delete.
- Follow the existing module structure and conventions exactly.

Existing reference implementation to copy
- API: src/api/record.rs
- DB repo: src/db/record.rs
- DB wiring: src/db/mod.rs
- Router wiring: src/api/mod.rs

Requirements
1) Database layer
- Create: src/db/<endpoint_name>.rs
- Define structs similar to Record:
  - <endpoint_name> { id: i64, name: String, created_at: String }
  - Create<endpoint_name> { name: String }
- Implement <endpoint_name>Repository with methods:
  - create(Create<endpoint_name>) -> <endpoint_name>
  - get_by_id(id: i64) -> <endpoint_name>
  - get_all() -> Vec<<endpoint_name>>
  - update(id: i64, Create<endpoint_name>) -> <endpoint_name>
  - delete(id: i64) -> ()
- Add create_table(pool) for “<endpoint_name>s” table (CREATE TABLE IF NOT EXISTS).
- Update src/db/mod.rs to:
  - mod <endpoint_name>;
  - pub use <endpoint_name>::{Create<endpoint_name>, <endpoint_name>Repository};
  - create tables in Database::new(): call <endpoint_name>::create_table(&pool).await? (like record)
  - add Database::<endpoint_name>_repo(&self) -> <endpoint_name>Repository

2) API layer
- Create: src/api/<endpoint_name>.rs
- Implement routes(state: AppState) -> Router
  - "/" : GET get_all_<endpoint_name>s, POST create_<endpoint_name>
  - "/:id" : GET get_<endpoint_name>, PUT update_<endpoint_name>, DELETE delete_<endpoint_name>
- Mirror response behavior/status codes from record endpoints:
  - create -> 201
  - get -> 200 or 404
  - get_all -> 200
  - update -> 200 or 404
  - delete -> 204 or 404
- Use tracing (info/error) similarly.

3) Router wiring
- Update src/api/mod.rs:
  - mod <endpoint_name>;
  - nest “/<endpoint_name>s” with <endpoint_name>::routes(state.clone()) just like records.

4) Tests
- Add unit tests in src/db/<endpoint_name>.rs like record tests (sqlite::memory: + create_table).
- Add endpoint tests in src/api/<endpoint_name>.rs similar to src/api/record.rs tests.
  - Ensure the router used in tests mounts the routes at "/" (like record tests do) and uses sqlite::memory:.

Acceptance criteria
- `cargo build` passes.
- `cargo fmt` passes.
- `cargo clippy` passes.
- `cargo test` passes.
- No unused imports / warnings introduced.
- Naming, file layout, and error JSON shape match the existing Record implementation.
- Keep changes minimal and consistent (copy patterns rather than inventing new ones).