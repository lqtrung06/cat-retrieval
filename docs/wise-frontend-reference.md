# WISE Frontend Reference

This document records the WISE frontend behavior Hound should use as product inspiration.

Source inspected locally:

- `/Users/lequangtrung123/foundry/wise/frontend`
- Key files: `src/App.tsx`, `src/WiseHeader.tsx`, `src/SearchResults.tsx`, `src/DataService.ts`, `src/misc/ImageDetailsModal.tsx`, `src/misc/VideoOccurrencesView.tsx`, `src/misc/ReportImageModal.tsx`

## Search Inputs

WISE supports compound multimodal queries.

Observed query inputs:

- Text query.
- Image file query.
- Image URL query.
- Internal image query from an existing result.
- Negative internal image query.
- Audio file query.
- Audio URL query.
- Metadata filter query when metadata is supported.

The frontend limits compound searches to a maximum of 5 query terms.

## Search Target Modes

WISE chooses the default target mode from configured project search targets.

Observed target modes:

- Image.
- Video visual track.
- Audio track of video, represented as `VideoAudio`.
- Metadata mode when configured.

For video results, WISE supports result view modes:

- Frames.
- Segments.
- Videos.

## Filters

Observed filters:

- Shot-scale filter when configured.
- Metadata filter when metadata is supported.
- Safety filter for video result imagery, applying blur/grayscale to potentially graphic images.

## Result Layout And Actions

WISE uses a visual result grid and paginates results at 50 items per page.

Observed result actions:

- Open a result details modal.
- Find visually similar images from an image result.
- Add an image result as an additional query.
- Add an image result as a negative query.
- Report image/media.
- Export current search results to JSON.
- For videos, switch between frame, segment, and video-level views.
- For videos, open a player near the matched timestamp and view occurrences within the same video.
- For videos with transcripts, click transcript entries to seek the player.
- View filename and external metadata in the details modal.

WISE search results do not provide a delete action. Hound should keep delete actions in the library-management surface, not the WISE-style result grid, unless a future product decision changes this.

## Facets And Clustering

WISE includes a separate facets frontend.

Observed facets behavior:

- Facets index page listing available facets.
- Facet cards with preview thumbnails.
- Cluster overview page with paginated cluster cards.
- Cluster layout selector: 1x1, 2x2, 3x2, and 3x3 preview grids.
- Cluster cards show label, instance count, approximate unique media count, and representative thumbnails.
- Cluster detail page groups occurrences by media file.
- Cluster detail can show metadata.
- Face/object occurrences can show bounding boxes.
- Video context modal can play the source media near the occurrence timestamp.

## Hound Adaptation

Hound should reuse WISE's search interaction model where practical, while adding SaaS-specific surfaces:

- Account management.
- Per-user library isolation.
- Large library upload.
- Async upload progress.
- Manual Start indexing control.
- Quota and storage policy.
- In-app notifications.
- Library deletion controls.

In Hound navigation, WISE facets/clustering should be labeled `Explore`.
