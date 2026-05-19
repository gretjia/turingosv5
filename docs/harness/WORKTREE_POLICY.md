# Worktree Policy

Multiple CLI workers must not share one working directory.

Suggested layout:

```text
/home/zephryj/projects/turingosv5
/home/zephryj/projects/turingosv5-claude
/home/zephryj/projects/turingosv5-gemini
/home/zephryj/projects/turingosv5-codex-worker
```

One worker binds to one branch at a time. One branch binds to one worker at a
time. Meta may inspect all worktrees, but workers must not mix directories.
