# Worktree Policy

Multiple CLI workers must not share one working directory.

Canonical layout:

```text
/home/zephryj/projects/turingosv5
/home/zephryj/projects/turingosv5-worktrees/<worker_slot>/<atom_id>
```

The main checkout is for intake and control view. Task code edits happen only
inside the canonical task worktree path created from latest `origin/main`.

One worker slot binds to one branch at a time. One branch binds to one worker
slot at a time. Meta may inspect all worktrees, but workers must not mix
directories or reuse stale sibling checkouts.
