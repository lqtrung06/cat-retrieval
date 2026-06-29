# MVP Roadmap

## Phase 0: WISE Deployment Spike

Goal:

- Prove that Hound can run WISE on the target server and index/search uploaded image and video libraries.

Build:

- Local WISE deployment.
- WISE running on the same server as Hound.
- Server local media storage.
- Manual test library with images and videos.
- Verification of available WISE search modes.
- Notes on required CPU/GPU/storage constraints.

Exit criteria:

- WISE can index and search a realistic media batch on the target machine.

## Phase 1: Web MVP Shell

Goal:

- Build the account, upload, quota, and library shell around WISE.

Build:

- Username/password signup and login.
- User-owned media library.
- Strict per-user media isolation.
- Browser upload for files and ZIP archives.
- Background in-app upload with global progress.
- Browser unload warning during active uploads.
- No resumable uploads in v1.
- Manual Start indexing action.
- Backend indexing status.
- Backend indexing queue.
- No user-triggered re-indexing in v1.
- WISE-inspired search page.
- Result preview.
- Delete media.
- Configurable free-tier limit, initially intended as 10GB.
- Configurable server storage operating cap, initially intended as 200GB.
- Global defaults loaded from environment variables or configuration at server startup.
- Read-only display of global defaults in the admin page.

Exit criteria:

- A new user can sign up, upload a large media batch, manually start indexing, search through WISE, preview results, and delete media.

## Phase 2: WISE Capability Exposure

Goal:

- Expose WISE capabilities through Hound without hiding useful search modes.

Build:

- Natural-language search.
- Visual similarity search.
- Image plus text multimodal search.
- Video frame or segment results.
- Audio search if configured.
- Object search if configured.
- Face search if configured.
- Transcript search if configured.
- Metadata search and filters.
- Facets and clustering views when configured.

Exit criteria:

- Hound can surface whatever WISE capabilities are available for the indexed library.

## Phase 3: Freemium Polish

Goal:

- Make the side project ready for public testing.

Build:

- Usage meter.
- Free-tier limits.
- Policy-change copy for free-tier limits.
- Optional ad placements for free users.
- Privacy copy.
- Better empty states and error states.

Exit criteria:

- Users understand what they can do for free and what happens when they hit limits.

## Phase 4: Retention Experiments

Goal:

- Learn whether users come back.

Build:

- Recent searches.
- Search suggestions.
- "Try another query" prompts.
- Basic analytics for upload, indexing, search, and result clicks.

Exit criteria:

- You can measure activation and repeat usage.

## Suggested MVP Limits

Free tier:

- Configurable per-user quota, initially intended as 10GB.
- Configurable maximum per upload batch, initially intended as 2GB.
- Configurable maximum per individual file, initially intended as 500MB.
- Policy may change later.
- Upload and indexing can be paused when server storage approaches the configured operational cap.

Development limits:

- Use a configurable operational cap, initially intended as 200GB of the available 400GB server free space.
- Do not promise permanent backup or archival storage.
- If local data is dumped to HDD or removed, affected users may need to re-upload.
