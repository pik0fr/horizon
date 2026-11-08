# Horizon fork for Horizon-Luau development

This fork starts from official Nightly 2026-11-05, commit
`66f3981f522b24c250b21523a70080965b940424`.
Work branch: `codex/luau-native-support` in `pik0fr/horizon`.

## Changes

- A `horizon-mod-deps` workspace member records optional `mlua` 0.11.6
  (`luau`, `vendored`), `notify` 7 and `toml` 0.8 in the engine lockfile.
  Its default features are empty. The game does not depend on this catalog.
- The native launcher's source, hook catalog and commit date lookups use
  `pik0fr/horizon`. Downloads of official game versions remain upstream.
  The source archive is pinned to the native mod's exact commit.
- The development pack-registration bridge, `mod_api::LUAU_RESOURCE_PACKS`,
  allows Horizon-Luau startup declarations to add immutable resource pack roots
  before options/resources are read.

The source allowlist, exact registry checksums, pinned revisions and native-code
trust prompts remain enabled. This is a development fork, not an official
Horizon release and not an installed Horizon-Luau mod.

## Check

```sh
cargo check -p horizon-mod-deps --locked
cargo test -p horizon-mod-deps --features luau --locked
cargo clippy -p horizon-mod-deps --all-features --locked -- -D warnings
cargo check --manifest-path launcher/Cargo.toml --locked
```

`horizon-mod-deps` tests execute two different Luau scripts without a Rust build
between them and verify VM globals stay isolated. In the Horizon-Luau repository,
`python tools/verify_horizon_fork.py --fork forks/horizon` additionally resolves
and tests all four runtime crates using this fork's lockfile and verifies that
no registry package outside the fork allowlist was added.

## Next integration step

Build the fork launcher, generate a native Horizon-Luau bundle pinned to a
published commit of this branch, and test the entire native-build/install/play
flow in an isolated launcher home. The native runtime owns the Horizon hooks
and the in-game manager; scripts remain in `Horizon-Luau-Mods` and do not require
native rebuilds when edited. A dependency check alone does not prove that flow.

The launcher used to build mods must include the native source-repository change;
the existing official launcher will still request source from the upstream repo.
Changing only a mod's Cargo.lock does not select this fork.
