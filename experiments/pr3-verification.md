# PR #3: trusted publishing and CI verification

## Scope

PR #3 has no linked issue (`closingIssuesReferences` is empty), required sub-issues,
inline comments, or reviews. Its edited description is the acceptance source:

- Both automatic and manual releases attempt trusted publishing using job-scoped
  `id-token: write` and the pinned crates.io auth action.
- Cargo reads the OIDC token through `CARGO_REGISTRY_TOKEN`, falling back to
  `CARGO_REGISTRY_TOKEN` and then `CARGO_TOKEN` secrets when auth is unavailable.
- Keep fallback secrets; remove the inline `--token` argument and global token.
- Publisher registration is explicitly a post-merge setup step.

Main (`da24e09`) was already an ancestor of this branch at verification time.

## Failure and reproduction

Both failed runs were created after their respective head commits:

| Run | Created (UTC) | Head | Error location in saved log |
| --- | --- | --- | --- |
| 38417795250 | 2026-10-13 14:57:04 | 0eba317 | lines 679–694 |
| 38428855368 | 2026-10-13 16:14:26 | 9a16104 | lines 851–866 |

Full logs are preserved locally as ignored `ci-logs/pipeline-<run>.log` files.
Both failures report `clippy::assert_is_empty` at `tests/integration_test.rs:800`.
The existing `test_version_is_not_empty` reproduces this CI lint failure with:

```sh
CARGO_BUILD_JOBS=1 CARGO_PROFILE_DEV_DEBUG=0 RUSTFLAGS=-Dwarnings \
  cargo clippy --all-targets --all-features
```

Before the fix, this exits 101. Replacing `assert!(!VERSION.is_empty())` with
`assert_ne!(VERSION, "")` retains the assertion and makes the same command pass.
No lints are disabled.

## Local verification

Using Rust 1.99.0, `RUSTFLAGS=-Dwarnings`, one Cargo build job, and development
debug information disabled to respect the container memory budget:

- `cargo fmt --all -- --check`: passed.
- `cargo clippy --all-targets --all-features`: passed after reproducing the failure.
- `node scripts/check-file-size.mjs`: passed.
- `cargo test --all-features --verbose`: 43 integration tests and 4 doc tests passed.
- `cargo test --doc --verbose`: 4 doc tests passed.
- `cargo build --release --verbose`: passed.
- `cargo package --list --allow-dirty`: passed (before committing the fix).
- `actionlint` v1.7.12 with `-shellcheck=`: passed (workflow syntax, permissions,
  action references, and expressions; ShellCheck was not installed locally).

Reviewed both release paths against the pinned action's `README.md` and
`action.yml`: the token output is named `token`; both jobs grant `id-token: write`;
only auth may continue on error; the publish steps use OIDC first, then the two
secret fallbacks, and retain their existing release conditions. Publishing itself
is reserved for release events and was not triggered by this verification.

Register the publisher for `.github/workflows/release.yml` after merge using the
PR's documented `package-registry-manager setup --registry crates-io --migrate
--execute` command. Keep the token secrets as fallbacks. Live OIDC exchange requires
that registration; PR checks validate the build and tests without publishing.
