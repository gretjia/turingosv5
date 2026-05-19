# V5 Genesis Plan

V5 production genesis is intentionally not initialized in R0.

R0 creates only the clean repo, harness entry files, Task Broadcast, PR/CI
scaffolding, and boundary gates. V4 `genesis_payload.toml` may exist in the
bootstrap archive for compatibility during migration, but it is not V5
production genesis.

Before V5 production genesis:

1. Freeze contracts in V5-C0.
2. Define production evidence surfaces.
3. Define accepted and rejected paths.
4. Create a genesis proposal.
5. Run CI/review/Veto.
6. Require exact Class 4 human ratification if trust-root authority changes.
