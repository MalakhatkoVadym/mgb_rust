## Description

Implements a complete CRUD endpoint for Artists resource, resolving issue #1.

## Changes

- Added `src/db/artist.rs` with Artist data structure and ArtistRepository
- Added `src/api/artist.rs` with full REST API endpoints
- Updated `src/db/mod.rs` to wire the artists database layer
- Updated `src/api/mod.rs` to mount artists routes at `/artists`

## Type of Change

- [x] New feature (non-breaking change which adds functionality)

## Acceptance Criteria

- [x] `cargo build` passes
- [x] `cargo fmt` passes
- [x] `cargo clippy` passes
- [x] `cargo test` passes (17 tests passing)
- [x] No unused imports or warnings
- [x] Follows existing Record pattern
- [x] API endpoints:
  - `POST /artists` - Create artist (201)
  - `GET /artists` - Get all artists (200)
  - `GET /artists/:id` - Get artist by id (200 or 404)
  - `PUT /artists/:id` - Update artist (200 or 404)
  - `DELETE /artists/:id` - Delete artist (204 or 404)

## Testing

All unit and integration tests pass:
- Database layer tests for CRUD operations
- API endpoint tests for HTTP operations
- Error handling tests for non-existent resources
