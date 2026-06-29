# User Flows

## Flow 1: First-Time User Tries Hound

1. User lands on the app.
2. User creates an account or logs in.
3. User sees top-level navigation: Search, Explore, Library, Uploads, and Account.
4. User sees an empty library or uploads state with an upload action.
5. User uploads a large batch of image, video, or ZIP files within the free quota.
6. System shows upload progress.
7. User continues navigating the app while upload runs in the background.
8. Uploaded media becomes ready to index.
9. User clicks Start indexing.
10. Backend indexing starts through WISE.
11. User sees indexing status and can keep using the app.
12. User searches indexed media with available WISE search modes.
13. System returns ranked results.
14. User opens a result preview.

Success condition:

- User uploads media, explicitly starts indexing, and reaches relevant WISE-powered search results.

## Flow 2: Upload Media

1. User opens the library page.
2. User selects files from their device.
3. System validates file type, size, and account limit.
4. Valid files upload to storage.
5. Invalid files are listed with reasons.
6. Uploads continue in the background while the user navigates inside Hound.
7. A global upload indicator shows active, complete, and failed uploads.
8. If the user tries to close, refresh, or leave Hound during active uploads, the app warns that unfinished uploads may be cancelled.
9. Successfully uploaded files become ready to index only after the full batch finishes uploading.
10. The batch is an operational upload group; indexed media still belongs to the user's main searchable library.
11. If a ZIP was uploaded, Hound extracts supported media files server-side.
12. After extraction, Hound shows extracted count, skipped count, failed count, and resulting media items in the batch.

Failure states:

- Unsupported file type.
- File too large.
- Broken or unreadable ZIP archive.
- Account storage limit exceeded.
- Upload interrupted.
- Browser tab closed before upload completion.
- Indexing failed.

Upload recovery:

- Resumable uploads are not required in v1.
- Interrupted, cancelled, or failed uploads must be restarted by the user.

## Flow 3: Start Indexing

1. User opens a batch or library view containing uploaded media.
2. If any file in the batch is still uploading, Start indexing is disabled.
3. After the full batch finishes uploading, the system shows the batch as ready to index.
4. User clicks Start indexing.
5. Backend indexing starts through WISE.
6. User can navigate elsewhere in Hound.
7. Global status shows indexing progress, indexed count, and failed count.
8. Failed files are marked failed and skipped.
9. Indexed files become searchable.
10. Search can include indexed media from all of the user's batches.

Indexing restrictions:

- An already indexed batch cannot be manually re-indexed by the user in v1.
- Admin handles operational re-indexing if WISE model/configuration changes require it.

## Flow 4: Search Media

1. User chooses or enters a query using the available WISE-inspired search interface.
2. User can combine supported query terms such as text, image, audio, internal image, negative internal image, and metadata filters.
3. User optionally selects filters.
4. System sends the query to WISE for the user's indexed library.
5. System returns ranked media results.
6. User can switch result views when available, such as frames, segments, and videos.
7. User opens a result details modal.

Empty states:

- No indexed media yet.
- No results found.
- Indexing still in progress.
- Uploaded media exists but Start indexing has not been clicked.

## Flow 5: Preview Result

1. User clicks a search result.
2. System opens a preview view.
3. Preview shows the image or video player.
4. Preview shows filename and metadata when available.
5. Video preview starts near the matched frame, segment, or timestamp if available.
6. Video preview shows occurrences and transcript navigation when available.
7. User can report media when reporting is enabled.
8. Delete is not shown in the WISE-style result preview; deletion is handled in the library flow.
9. Original-file download is not supported in v1.

## Flow 6: Manage Limits

1. User uploads media near the free-tier limit.
2. UI shows usage and remaining allowance.
3. If user exceeds the limit, upload is blocked.
4. User can delete existing media or wait for future policy changes.

## Flow 7: Delete Media

1. User selects a media item that is ready to index, indexed, or failed.
2. User clicks delete.
3. System asks for confirmation.
4. System deletes original media, thumbnails, extracted frames, embeddings, transcripts, and WISE-derived artifacts when present.
5. The item disappears from library and search results.

Delete restrictions:

- Active uploads must be cancelled, not deleted.
- Active indexing jobs cannot be deleted in v1.

## Flow 8: Delete Batch

1. User selects a batch that is not actively uploading or indexing.
2. User clicks delete batch.
3. System asks for confirmation.
4. System deletes all media, thumbnails, extracted frames, embeddings, transcripts, and WISE-derived artifacts associated with the batch.
5. Batch media disappears from library and search results.

## Flow 9: Operator Storage Dump

1. Server storage approaches the configured operational cap.
2. Operator pauses new uploads or indexing if needed.
3. Operator manually dumps stored uploaded data to HDD or removes old data.
4. Hound can lower future free-user quotas.
5. Affected users are notified that previous uploaded data cannot be processed again and must be re-uploaded.
