# Single URL MVP Contract

Single URL MVP defines a minimal `/build` UX contract for a single content URL.

## Scope

This contract covers:

- A single explicit build entrypoint: `/build <target_url>`.
- The projection behavior for BuildSessionView generated from accepted evidence.
- The linear flow `spec -> generate -> preview`.
- Refresh restore behavior for rebuilding the same session from canonical input.

## User Flow

### 1. Submit URL

Operators call `/build` with one URL and receive an in-session path:

- `POST /build <url>` starts a build attempt for that single URL.
- The session is deterministic for the same accepted source inputs.
- Progress is reflected by one visible flow: `spec -> generate -> preview`.

### 2. Render and Refresh Restore

- `/build` must provide a preview artifact only after generation completes.
- Operators may refresh restore the build session by reloading from accepted inputs.
- If cache is missing or stale, the session is rebuilt from source inputs and
  re-rendered through the same `spec -> generate -> preview` path.

## Non-Goals

- No multi-URL orchestration.
- No JavaScript framework.
- No canonical session authority in UI/session/cache layers.

## Canonical Rule

Runtime authority remains with accepted evidence and the execution path defined by
`spec -> generate -> preview`; cache and surface UI are rebuildable projections.
