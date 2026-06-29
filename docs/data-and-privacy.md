# Data And Privacy

## Privacy Position

Hound handles private user media. Even as a side project, the product should behave as if uploaded pictures and videos are sensitive.

## Data Types

User account data:

- User ID
- Username
- Password hash
- Joined date
- Subscription or tier

Media data:

- Original uploaded files
- Generated thumbnails
- Extracted video frames
- Media metadata such as filename, size, format, upload date, and duration
- Embeddings generated from images, frames, and text queries
- WISE-generated indexes and derived search artifacts

Usage data:

- Upload counts
- Storage usage
- Search count
- Indexing job status
- Error logs

## Data Handling Requirements

DR-001: Original media files must be private by default.

DR-002: Media URLs must not be public permanent links unless explicitly designed and authorized.

DR-003: Deleting media must also delete generated thumbnails, sampled frames, and embeddings.

DR-004: Passwords must be hashed with a modern password hashing algorithm.

DR-004a: Sessions must use secure HTTP-only cookies.

DR-005: Logs must not store raw private media content.

DR-006: Search queries may be stored for product analytics only if the privacy policy says so.

DR-007: Admin access to user media should be avoided or tightly limited.

DR-008: Each user's library must be isolated by account, storage namespace, metadata records, and search scope.

DR-009: Search must never return another user's media.

DR-010: WISE may run as shared backend infrastructure, but Hound must constrain indexing and search to the current user's library.

DR-011: ZIP extraction must not recursively extract nested archives in v1.

DR-012: ZIP extraction must block path traversal, absolute paths, and writes outside the current user's storage namespace.

DR-013: Temporary upload and extraction artifacts must be cleaned after success, failure, cancellation, deletion, or storage-dump operations.

## Free-Tier Retention

Early side-project policy:

- Free users initially receive a configurable default storage quota, currently intended as 10GB.
- Hound uses server local storage with a configurable operational cap, currently intended as 200GB.
- Free-tier limits may change later.
- If storage pressure requires manual dumping to HDD or deletion, affected users may lose searchable uploaded data and need to re-upload.
- Affected users should be notified in-app.
- Hound does not promise durable archival storage.

## Ads

If ads are used:

- Show ads only on non-critical surfaces such as bottom or sidebar placements in Search results and Explore.
- Do not show ads inside media preview modals, upload progress, indexing status, auth pages, or Account policy pages.
- Do not send private media content to ad networks.
- Avoid ad placements that make the product feel unsafe for personal photos.
- Keep ads configurable so they can be disabled during development.

## Model And Embedding Notes

Embeddings are derived from user media and should be treated as user data. They may not reconstruct the original file directly, but they still represent private content and must follow deletion and access-control rules.

## Basic Privacy Copy

Draft copy for the app:

> Your uploaded media is used to create a private search index for your account. Other users cannot search or view your files. Hound is an early side-project service and does not guarantee permanent archival storage. If storage policy changes or old data is removed, you may need to upload your media again.
