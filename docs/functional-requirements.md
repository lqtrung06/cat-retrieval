# Functional Requirements

## Scope

This document defines the functional requirements for the Hound MVP: a lightweight SaaS shell around WISE that lets users upload large picture and video libraries, manually start indexing, and search indexed media with WISE-powered multimodal retrieval.

## Personas

### Casual Explorer

A non-technical user who wants to try AI media search on a meaningful personal library.

Needs:

- Easy signup.
- Simple upload.
- Clear upload and indexing progress.
- Search results that feel impressive quickly.

### Media Hoarder

A user with many unorganized pictures or videos.

Needs:

- Batch upload.
- Search across many files.
- Preview and locate original files.
- Basic filters.

### Small Creator

A creator with reusable media assets.

Needs:

- Find images/videos by visual content.
- Preview results quickly.
- Inspect result details and metadata.
- Avoid manual tagging.

## MVP Functional Requirements

### Account Management

FR-001: Users can create an account with username and password.

Acceptance criteria:

- User can register with username and password.
- User cannot register with an already-used username.
- User can log in with username and password.
- Username is used as display identity.
- Passwords are stored securely as hashes, never as plain text.
- User can log in and log out.
- Self-service password reset is not required in v1.
- Password recovery is handled manually by the admin setting a new user password.
- Sessions use secure HTTP-only cookies.
- OAuth is not required for v1.

FR-002: Users can view basic account information and policy status.

Acceptance criteria:

- Account page shows username.
- Account page shows quota usage.
- Account page shows the current free-tier policy.
- Account page shows the early-stage storage policy warning.
- Account page provides logout.
- Account page can include a placeholder for future billing.
- Real billing is not required in v1.

FR-003: Users can access only their own media library.

Acceptance criteria:

- Authenticated users see only their uploaded media.
- Unauthenticated users cannot access upload, search, or library pages.
- Direct media/result URLs require authorization.
- Disabled user accounts cannot log in.
- Self-service account deletion is not required in v1.

### WISE Integration

FR-004: Hound uses WISE as the core indexing and search engine.

Acceptance criteria:

- Hound delegates multimodal indexing and retrieval to WISE or WISE-compatible services.
- Hound owns account, upload, quota, storage, user isolation, and product UI concerns.
- Hound does not require users to understand or operate WISE directly.
- Hound can expose any WISE search capability that is configured and available at runtime.
- In v1, WISE runs locally on the same server as Hound, either as a backend service or subprocess.
- Hound stores each user's media under a separate server-side directory or namespace.
- Hound indexing jobs call WISE against the current user's batch or library path.
- Hound search requests are scoped to the current user's WISE project or index.

FR-005: Hound follows WISE's search UX as the main product reference.

Acceptance criteria:

- Search modes and result layouts should be inspired by WISE's practical media-search interface.
- The local WISE frontend reference is documented in `docs/wise-frontend-reference.md`.
- Hound may adapt the UI for account, quota, upload, and storage lifecycle workflows.
- Hound should avoid a custom social or consumer-gallery UI that hides the search-first purpose.

FR-006: Hound provides clear top-level navigation.

Acceptance criteria:

- V1 top-level navigation includes Search, Explore, Library, Uploads, and Account.
- Search opens the WISE-style search UI.
- Explore opens WISE facets/clustering.
- Library opens uploaded/indexed media management and delete controls.
- Uploads opens active uploads, batches, indexing status, and Start indexing controls.
- Account opens quota, settings, and logout.

### Media Upload

FR-007: Users can upload image files.

Acceptance criteria:

- Supported MVP formats: JPG, JPEG, PNG, WebP.
- User can select multiple files in one upload.
- Folder upload is not supported in v1.
- Upload UI shows progress, success, and failure states.
- Invalid file types are rejected with a clear message.

FR-008: Users can upload video files.

Acceptance criteria:

- Supported MVP formats: MP4 and MOV.
- Video uploads are accepted behind a stricter size limit than images.
- Upload UI explains that video indexing may take longer.

FR-009: Users can upload ZIP archives containing media files.

Acceptance criteria:

- ZIP upload is supported in v1.
- ZIP files count toward the same per-file and per-batch upload limits.
- The configured per-file limit applies to the ZIP archive itself.
- The configured per-file limit also applies to each extracted media file inside the ZIP archive.
- Hound extracts supported media files from ZIP archives server-side.
- Hound does not need to show ZIP contents before upload.
- After extraction, Hound shows a summary with extracted count, skipped count, and failed count.
- Unsupported files inside a ZIP are skipped and reported as invalid.
- Nested ZIP archives are not recursively extracted in v1.
- Nested ZIP archives are treated as unsupported files and skipped or reported.
- ZIP extraction must block unsafe paths such as `../` traversal and absolute paths.
- ZIP extraction must never write files outside the current user's storage namespace.
- Hidden system metadata files inside ZIPs can be skipped where reasonable.
- Broken or unreadable ZIP archives are marked failed.
- Extracted media belongs to the same upload batch created by the ZIP upload.
- Folder structure inside ZIP can be ignored for v1 unless stored as optional metadata.
- Duplicate filenames inside a ZIP must be preserved as separate media items.

FR-010: Uploads run asynchronously inside the web app.

Acceptance criteria:

- Users can start a file or ZIP upload and continue navigating within Hound while upload continues.
- A persistent global upload indicator shows active uploads, progress, completed files, and failed files.
- Internal Hound navigation must not cancel active uploads.
- If the user attempts to refresh, close the tab, navigate to an external page, or otherwise unload the app during active uploads, Hound warns that unfinished uploads may be cancelled and asks for confirmation.
- Upload completion and indexing are represented as separate states.
- Users can cancel active uploads.
- Cancelled uploads are marked cancelled or removed from the active upload list.
- Resumable uploads are not required in v1.
- Interrupted, cancelled, or failed uploads must be restarted by the user.
- Uploads use temporary staging storage before completed files become part of the user's library.
- Temporary upload files are cleaned after success, failure, or cancellation.

FR-011: Each upload creates a new batch.

Acceptance criteria:

- A batch is a collection of uploaded files created by one upload action.
- Batches are used for upload progress, readiness, indexing controls, and failure reporting.
- Users do not need to understand batches as a separate library concept.
- Batch names are not user-editable in v1.
- Hound can label batches automatically with upload date/time and file count.
- All uploaded and indexed media belongs to the user's same searchable library.
- Search runs across the user's indexed library, not only one batch, unless the user explicitly applies a future filter.
- Files with duplicate original filenames must not overwrite each other.
- Hound assigns unique internal storage names or IDs and preserves the original filename as metadata.

FR-012: The system enforces free-tier limits.

Acceptance criteria:

- Free users initially receive a configurable default storage quota, currently intended as 10GB.
- Free-tier limits are configurable and may change later.
- Each upload batch is limited by a configurable max batch size, currently intended as 2GB.
- Each individual uploaded file is limited by a configurable max file size, currently intended as 500MB.
- ZIP archives and extracted ZIP entries both respect the configured max file size.
- File count is constrained by storage and batch size rather than a separate v1 file-count limit.
- Uploads that exceed limits are rejected before upload or before final acceptance.
- The user sees the current usage count and remaining allowance.
- Quota, batch-size, file-size, and operational-cap values must come from configuration rather than hardcoded constants.
- Global default limits are read from environment variables or configuration at server startup.
- Global default limits are visible in the admin page as read-only system settings.
- Changing global default limits requires changing environment/configuration and restarting the server.

FR-013: The system respects the server storage operating cap.

Acceptance criteria:

- Hound uses server local storage for uploaded media in v1.
- Hound must treat the configured operational storage cap, currently intended as 200GB, as authoritative even if the server has more physical free space.
- When storage approaches the cap, Hound can pause new uploads and indexing.
- The operator may manually dump stored data to HDD, lower future quota policy, and notify affected users that old uploaded data cannot be processed again and must be re-uploaded.

### Media Indexing

FR-014: Uploaded media is not indexed automatically.

Acceptance criteria:

- After upload completes, media is marked as ready to index.
- The user must explicitly click Start indexing to begin WISE processing.
- The Start indexing action is unavailable for a batch while any file in that batch is still uploading.
- The user can start indexing only after the relevant upload batch has fully completed.
- Uploaded media is not searchable until indexing has started and completed.
- The UI clearly communicates when uploaded media is stored but not yet searchable.
- Users cannot manually re-index an already indexed batch in v1.
- If WISE model/configuration changes require re-indexing, the admin handles it operationally.

FR-015: Indexing runs as a backend background process.

Acceptance criteria:

- After the user starts indexing, processing runs on the backend.
- The user can continue using the web app while indexing runs.
- A persistent status surface shows indexing progress, indexed count, failed count, and whether search coverage is complete or partial.
- Users can search already-indexed media while other media continues processing.
- Users cannot cancel an indexing job in v1 after it starts.
- Users can delete unwanted media after indexing completes.
- Indexing jobs run through a backend queue.
- A user's indexing jobs run sequentially in v1.
- Global indexing concurrency is configurable.

FR-016: Hound indexes all supported media types through WISE.

Acceptance criteria:

- Images and videos are supported from v1.
- Audio, object, face, transcript, metadata, and multimodal capabilities can be exposed when WISE provides them for the indexed media.
- Hound should not artificially restrict WISE capabilities unless a capability is unavailable, misconfigured, or operationally unsafe.

FR-017: Users can see indexing status.

Acceptance criteria:

- Media items show statuses: uploaded, ready to index, indexing, indexed, failed.
- Search excludes media that is not indexed.
- If a file fails processing, Hound marks it as failed, excludes it from search, and continues processing the rest of the batch.
- No automatic retry is required for failed processing, because failed files are assumed to be broken or unsupported.
- The user may delete a failed item and upload another copy manually.

### Search

FR-018: Users can search their media with WISE-powered search modes.

Acceptance criteria:

- User can enter text queries.
- User can add image file or image URL query terms when supported by the selected WISE mode.
- User can add audio file or audio URL query terms when supported by the selected WISE mode.
- User can add internal image result query terms from search results.
- User can add negative internal image result query terms from search results.
- User can combine query terms into compound multimodal searches.
- Compound search is limited to 5 query terms, matching the WISE frontend behavior.
- System returns ranked image, video, audio, object, face, transcript, metadata, or multimodal results when available.
- Empty queries are rejected.
- Searches run only against the current user's indexed media.

FR-019: Users can filter search results.

Acceptance criteria:

- MVP filters include media type: images, videos, or all.
- MVP filters include upload date range.
- WISE-provided filters such as shot scale and metadata should be exposed when configured.
- A safety filter for video result imagery should be available if the WISE frontend behavior is retained.
- Search can be submitted with or without filters.

FR-020: Users can inspect search results.

Acceptance criteria:

- Results follow a WISE-inspired layout and show thumbnail or preview, filename, media type, match type, and relevance order.
- Video results show frame, segment, or timestamp details when available.
- Video results support frame, segment, and video-level views when available.
- User can open a result details modal.
- User can run visual similarity search from an image result.
- User can add an image result as an additional query.
- User can add an image result as a negative query.
- User can report media.
- User can export search results to JSON.
- Video details can show a player near the matched timestamp, occurrences within the same video, transcripts, filename, and metadata when available.
- Search result grids do not include delete controls in v1; delete belongs to the library-management surface.
- Search result details do not include original-file download controls in v1.

FR-021: Users can rerun recent searches.

Acceptance criteria:

- Hound stores recent searches for the current user account.
- Recent searches store only text query strings.
- Recent searches do not store filter settings.
- Recent searches do not store image, audio, internal-result, negative-query, or metadata query payloads in v1.
- Recent searches appear in the Search surface.
- Users can rerun a recent search.
- Recent searches are product history, not a full analytics system.
- Clearing recent searches can be added later and is not required for v1.

FR-022: Users can use WISE facets and clustering views when configured.

Acceptance criteria:

- The product navigation label for WISE facets/clustering is Explore.
- Hound exposes WISE facets/clustering UI in v1 when WISE provides facet data for the user's indexed library.
- Users can open a facets index page listing available facets.
- Users can open a facet cluster overview with paginated cluster cards.
- Users can change cluster preview layout when WISE supports it.
- Cluster cards show label, instance count, media count when available, and representative thumbnails.
- Users can open a cluster detail view grouped by media file.
- Cluster detail can show metadata, bounding boxes, and video context near the occurrence timestamp when available.
- Facets and clusters are scoped to the current user's indexed library.

### Library Management

FR-023: Users can browse uploaded media.

Acceptance criteria:

- Library page lists uploaded media.
- User can sort by upload date.
- User can filter by media type and indexing status.
- Original-file download is not supported in v1.

FR-024: Users can delete uploaded media.

Acceptance criteria:

- Users can delete media in ready to index, indexed, or failed states.
- Users cannot delete media while it is actively uploading; they must cancel the upload instead.
- Users cannot delete media while it is actively indexing in v1; they must wait for indexing to finish.
- Deleting media removes the original file from storage.
- Deleting media removes related embeddings, thumbnails, extracted frames, transcripts, and WISE-derived artifacts when present.
- User must confirm deletion.
- Users can delete an entire batch if the batch is not actively uploading or indexing.
- Batch deletion removes all media and derived WISE artifacts associated with that batch.

### Freemium And Ads

FR-025: The system supports a free tier.

Acceptance criteria:

- Free-tier limits are configurable.
- The UI communicates limits before the user hits them.
- The system can block uploads above the limit.

FR-026: The system can display ads on free-tier pages.

Acceptance criteria:

- Ads can appear only on non-critical surfaces such as bottom or sidebar placements in Search results and Explore.
- Ads are not shown inside result detail modals or private media previews.
- Ads are not shown in upload progress, indexing status, auth pages, or Account policy pages.
- Ads do not block upload, indexing, search, preview, or deletion controls.
- Ads can be disabled by configuration.

### Notifications

FR-027: The system provides in-app notifications for important events.

Acceptance criteria:

- In-app notifications are required for v1.
- Hound notifies users in-app when uploads fail, indexing completes, indexing has failures, quota is reached, storage is temporarily unavailable, free-tier policy changes, or previously uploaded data has been removed and must be re-uploaded.
- Notifications should be visible without requiring users to stay on the upload or indexing page.

### Admin And Operations

FR-028: Operators can manage users and system capacity.

Acceptance criteria:

- V1 assumes a single admin/operator.
- Admin/operator access is protected and not available to normal users.
- Admin account is created manually through environment/configuration or a database seed.
- Public admin signup is not supported.
- Operators can view all users.
- Operators can search users by username.
- Operators can view per-user storage usage.
- Operators can view per-user upload and indexing status.
- Operators can set a user's new password by username.
- Operators can change a user's quota.
- Operators cannot change global default limits at runtime in v1.
- Operators can disable or enable a user account.
- Operators can delete a user's uploaded media if needed.
- Operators do not need to hard-delete user accounts in v1.
- Operators can pause uploads globally.
- Operators can pause indexing globally.
- Operators can see active upload and indexing queues.
- Operators can view total server storage usage against the configured operational cap.
- Operators can view configured global default limits as read-only system settings.
- Operators cannot view user passwords.
- Admin-set passwords become the user's active password immediately; no forced password change flow is required in v1.
- Admin password changes do not need to notify the user in v1.
- Admin password changes can be logged simply with target user and timestamp.
- A full multi-admin audit system is not required in v1.
- Impersonation is not required in v1.

### Abuse Protection

FR-029: The system applies basic rate limits.

Acceptance criteria:

- Login attempts are rate-limited.
- Upload start requests are rate-limited.
- Search requests are rate-limited.
- Rate limits are configurable.
- Rate-limit failures return clear user-facing messages.

## Non-Functional Requirements

NFR-001: Search should feel responsive for large libraries within the practical limits of WISE and the current server.

NFR-002: Upload and indexing failures should be recoverable without contacting support.

NFR-003: Private media must not be visible to other users.

NFR-004: The MVP should run on the operator's server local storage with a configurable operational cap and strict free-tier limits.

NFR-005: The product should be usable by non-technical users without explaining embeddings, models, or vector databases in the main UI.

NFR-006: Temporary upload, extraction, and indexing artifacts should be cleaned up after success, failure, cancellation, or deletion.

## Non-Goals For MVP

- Full Google Photos, iCloud, Drive, Dropbox, desktop sync, or NAS integration.
- Building a custom retrieval engine from scratch instead of using WISE.
- Durable archival storage guarantees.
- Original-file download.
- Automatic indexing immediately after upload.
- User-triggered re-indexing of already indexed batches.
- Resumable uploads.
- Self-service password reset.
- Self-service account deletion.
- Hard deletion of user accounts.
- Saved searches.
- Collaborative team workspaces.
- Enterprise permissions.

## Open Questions

- None for the current docs pass.
