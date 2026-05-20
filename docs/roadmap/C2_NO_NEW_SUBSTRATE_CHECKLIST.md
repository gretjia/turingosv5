# C2 No-New-Substrate Checklist

This checklist keeps the K1 regression narrow: current `main` must not grow a
parallel substrate while Path B-pragmatic remains unproven.

| Forbidden route | Regression guard | Accepted alternative |
| --- | --- | --- |
| `src/cas.rs` default CAS module | `tests/v5_no_new_substrate_policy.rs` asserts the file is absent. | Use the current semantic DevTape development evidence path until an accepted substrate task exists. |
| `src/hash.rs` default hash module | `tests/v5_no_new_substrate_policy.rs` asserts the file is absent. | Keep hashes inside existing accepted data flow or a future approved contract; do not create a competing root. |
| `src/versioned_state.rs` WAL/state module | `tests/v5_no_new_substrate_policy.rs` asserts the file is absent. | Derive views from accepted evidence instead of storing a second authoritative state rail. |
| Any new parallel substrate for CAS/hash/WAL/`HEAD_t` | Review this checklist and the regression before accepting substrate-shaped files. | Follow the accepted task path and required risk gates before introducing canonical substrate behavior. |

The policy target is absence, not replacement. If a future task needs canonical
substrate work, it must arrive through the accepted path rather than by adding a
side-channel module in this atom.
