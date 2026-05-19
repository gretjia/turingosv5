# Worktree Policy

Multiple CLI workers must not share one working directory.

Suggested layout:

```text
/home/zephryj/projects/turingosv5
/home/zephryj/projects/turingosv5-worker-a
/home/zephryj/projects/turingosv5-worker-b
/home/zephryj/projects/turingosv5-worker-c
```

One worker binds to one branch at a time. One branch binds to one worker at a
time. Meta may inspect all worktrees, but workers must not mix directories.
