# Changelog

All notable changes are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0]

### Changed

- Upgrade to napi-rs v3. Public TypeScript API is preserved; the regenerated
  loader and bindings ship the same `focusWindow(handleBuffer: Buffer): void`
  signature. Existing consumers can upgrade with no code changes.
- Modernize CI workflow to current package-template conventions: dedicated
  lint job (`cargo fmt` + `clippy`), updated GitHub Actions versions, Node 24
  (Node 22 retained for i686 testing since Node 24 dropped 32-bit Windows
  binaries), `napi create-npm-dirs` runs at publish time, and automated
  GitHub releases populated from this CHANGELOG.
- Switch publish-content control from `.npmignore` blacklist to
  `package.json` `files` whitelist. Per-platform `npm/` sub-package dirs are
  no longer tracked in git — they are generated in CI.
- Pin toolchain versions via `mise.toml` (Rust 1.95.0, Node 24.15.0).

## [0.0.10] and earlier

See git history.
