# V4 Snapshot

V5 was bootstrapped from TuringOS V4 commit:

```text
300fb563ae57d971610b923d83fc55ab083ae245
```

Source repo:

```text
/home/zephryj/projects/turingosv4
```

Archived lineage artifact:

```text
/home/zephryj/projects/turingosv5_legacy_archive/v4_archive_300fb563ae57d971610b923d83fc55ab083ae245.tar.gz
```

Archive SHA-256:

```text
68f45ce258bb6fe42d59ce40a215ce1f2ccc1108ef5cee88338adac6c2a1d433
```

Bootstrap method:

```text
git archive --format=tar --output=/tmp/turingosv5_from_v4_main.tar HEAD
tar -xf /tmp/turingosv5_from_v4_main.tar -C /home/zephryj/projects/turingosv5
git init -b main
```

Boundary:

- V4 is development harness only.
- V4 handover/evidence is not V5 production evidence.
- V4 `genesis_payload.toml` is not V5 production genesis.
- V4 ChainTape HEAD is not V5 runtime `HEAD_t`.
- V5 runtime must not depend on V4 evidence/genesis/local paths.
- `experiments/minif2f_v4` was intentionally not carried forward as a V5
  product asset; MiniF2F remains a V4 development/evaluation corpus.
