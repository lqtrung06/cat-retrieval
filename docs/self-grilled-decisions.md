# Self-Grilled Decisions

This document records v1 choices made after the initial grill session. These are pragmatic defaults for a side-project SaaS and can change later.

## Search Backend Integration

- Hound should not couple its product/domain model directly to WISE.
- Hound is the public product and API boundary; search/index engines are internal backend adapters.
- The browser should call Hound APIs, not raw engine-specific APIs.
- Hound should define a backend-facing search/indexing contract around Hound concepts: user library, media item, upload batch, indexing job, query, search result, preview, and metadata.
- WISE can be the first implementation of that contract, but WISE-specific project paths, media IDs, routes, and process details should stay inside the WISE adapter.
- Hound should enforce authentication, authorization, quota, and media ownership before calling any search backend.
- V1 should maintain one cumulative searchable index per user library, regardless of the underlying backend implementation.
- Upload batches are Hound lifecycle/progress concepts, not separate search indexes.
- When a batch is indexed, the configured search backend appends that batch into the user's cumulative searchable index.
- Search runs against the user's cumulative searchable index, covering all indexed batches for that user.
- Hound should not rely on frontend access to backend project paths or unauthenticated backend service URLs.
- Indexing should run through Hound backend jobs; search can call an internal backend service or library adapter.

## WISE Adapter

- The WISE adapter can map each Hound user library to a WISE project internally.
- WISE append support should be used to add each indexed batch into the user's cumulative WISE-backed index.
- WISE search can run through an internal HTTP service, while indexing jobs can call WISE CLI/library functionality.
- WISE-specific media IDs or project paths can be stored as implementation references, but they should not leak into Hound's public API or domain language.

## Storage Layout

- Each user gets a private server-side storage namespace such as `storage/users/{user_id}`.
- Hound-owned durable source media should live separately from search-backend-owned derived index data.
- Uploaded files should move through temporary staging before becoming accepted library media.
- Accepted media should use unique internal filenames or IDs so duplicate original filenames never overwrite each other.
- Original filename, MIME type, size, hash, batch ID, and state should be stored in Postgres.
- Backend-derived index data should live under that user's namespace, for example `storage/users/{user_id}/backends/{backend_name}`.
- Backend-derived index data is rebuildable from Hound-owned source media; source media is the durable product-owned library data.
- Optional Hound-derived assets such as previews, normalized files, or logs can live outside backend-owned index directories.

## Product State Authority

- Postgres is the source of truth for Hound product state.
- Hound should not infer upload, batch, media, quota, or indexing state by scanning the filesystem or reading WISE databases.
- Hound should track users, upload batches, media items, indexing jobs, and search backend metadata in Postgres.
- Media state should cover at least uploaded, ready to index, indexing, indexed, failed, and deleted.
- Indexing job state should cover at least queued, running, succeeded, and failed.
- Backend-specific media IDs or project paths can be stored as implementation references, but Hound owns the user-visible state machine.

## Indexing

- Users cannot manually re-index an already indexed batch in v1.
- If WISE model/configuration changes require re-indexing, the single admin handles it operationally.
- Users can delete and re-upload media if they need a fresh user-controlled index.
- Indexing jobs should run through a backend queue.
- A user's indexing jobs should run sequentially to avoid confusing progress and WISE project state.
- Global indexing concurrency should be configurable.

## Uploads

- Resumable uploads are not required in v1.
- If a browser refresh, tab close, network failure, or cancellation interrupts an upload, the affected files must be uploaded again.
- Uploads and ZIP extraction should use temporary staging paths before files become part of the user's library.
- Temporary upload and extraction files should be cleaned after success, failure, or cancellation.

## Authentication

- V1 uses username and password.
- Email is not a v1 concern.
- Self-service password reset is not required in v1.
- Password recovery is handled manually by the admin setting a new user password.
- Sessions should use secure HTTP-only cookies.

## Admin

- V1 assumes one admin/operator.
- Admin is created manually through config, environment, or database seed.
- Admin can set a user's new password directly; no forced password-change flow.
- No public admin signup.
- No impersonation.
- No full multi-admin audit system.
- Disabled users cannot log in.
- Hard deletion of user accounts is not required in v1.
- Self-service account deletion is not required in v1.

## Search History

- Recent searches store only text query strings.
- Recent searches do not store filters or complex WISE query payloads.
- Saved searches are not part of v1.

## Limits

- Global defaults are loaded from environment/configuration at startup.
- Global defaults are read-only in the admin UI.
- Runtime per-user quota changes are allowed from the admin UI.
- Global default changes require config/env change and server restart.

## Safety

- ZIP extraction blocks path traversal and absolute paths.
- Nested ZIPs are not recursively extracted.
- Duplicate filenames are preserved as separate media items using unique internal IDs.
- Rate limits should exist for auth, upload start, and search requests.
