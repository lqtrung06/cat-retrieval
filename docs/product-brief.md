# Product Brief

## Product Name

Hound

## One-Liner

Search your pictures and videos by describing what you remember.

## Product Summary

Hound helps users with large photo and video collections find specific media without manually opening files one by one. Users upload media through the web app, explicitly start indexing, and then search the indexed library with the multimodal search capabilities provided by WISE.

This is a lightweight side-project SaaS, but the first version should still promise true large-library search. Hound is not a tiny demo uploader. It is a hosted product shell around WISE for users who want an easy web app instead of running WISE themselves.

## Target Users

Primary target:

- Curious non-technical users who have many pictures or videos and want an easy way to try AI-powered search.

Better early adopters:

- People with messy personal media folders.
- Small creators with reused photo/video assets.
- Families managing years of digital memories.
- Hobby photographers who do not want to tag files manually.

## Problem

Large media folders become hard to search because filenames, folders, and timestamps rarely describe what is inside the image or video. Users often remember visual content but not the exact file name or storage location.

## Proposed Solution

Hound stores uploaded media on server local storage, lets the user start WISE indexing manually, and exposes WISE-style multimodal search over the user's private library. Search can include natural language, visual similarity, video moments, audio, object, face, transcript, and metadata capabilities when those capabilities are configured and available from WISE.

## Differentiation

- Hosted SaaS shell around WISE.
- Large-library search from v1.
- No manual tagging required.
- WISE-inspired UI and search experience.
- User accounts, per-user isolation, quotas, uploads, storage lifecycle, and operational controls.

## Monetization Assumption

Initial monetization uses a freemium model:

- Free tier starts with a configurable default quota, initially intended to be 10GB per user during the early side-project phase.
- The free-tier policy can change later.
- Ads can be tested on free-tier surfaces if they do not reduce trust.
- Paid tier can be added later for larger storage or better processing policy.

For a side project, the first success metric should be return usage and search satisfaction, not immediate revenue.

## Success Metrics

- New users complete upload, manually start indexing, and run a first search.
- Users run multiple searches in one session.
- Users return with more media.
- Search results feel relevant enough that users open details, refine queries, or export results.
- Indexing completes without the user needing technical knowledge.

## Key Assumptions

- Users are willing to upload large media collections into a side-project service.
- WISE provides the core multimodal indexing and retrieval capabilities.
- A configurable free tier, initially intended as 10GB per user, can fit within the current configurable operational storage cap while the user base is small.
- If storage pressure becomes real, the operator can manually archive or delete local data and change future quota policy.
