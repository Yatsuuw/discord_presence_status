# Releasing

Releases are fully automated by `.github/workflows/release.yml`: pushing a
version tag builds, tests, packages and publishes everything.

## Steps

1. Make sure `master` is green in the CI.
2. Update the version in `Cargo.toml` (and run `cargo check` to refresh
   `Cargo.lock`), following [Semantic Versioning](https://semver.org/):
   - `0.1.0 → 0.1.1` for fixes,
   - `0.1.0 → 0.2.0` for new features,
   - `1.0.0 → 2.0.0` for breaking changes (e.g. incompatible `activity.toml`).
3. Move the *Unreleased* entries of `CHANGELOG.md` under the new version.
4. Commit and tag:

   ```powershell
   git commit -am "Release 0.2.0"
   git tag -a v0.2.0 -m "Release 0.2.0"
   git push origin master v0.2.0
   ```

5. Follow the *Release* workflow in the **Actions** tab. When it succeeds, the
   release appears on the **Releases** page with:
   - `DiscordPresenceStatus-<version>-setup-x64.exe` — the installer;
   - `DiscordPresenceStatus-<version>-portable-x64.zip` — the portable version;
   - `SHA256SUMS.txt` — checksums;
   - a build-provenance attestation for each file.

The workflow refuses to publish if the tag does not match the version in
`Cargo.toml`. Tags with a suffix (`v0.2.0-beta.1`) are published as
pre-releases.

## If something goes wrong

- **The workflow failed before publishing**: fix the problem on `master`,
  delete the tag (`git push --delete origin v0.2.0` and `git tag -d v0.2.0`),
  then tag again.
- **A published release is broken**: do not replace its files. Publish a new
  patch version instead; with immutable releases enabled, assets cannot be
  changed after publication anyway.

## Code signing (optional)

Executables are currently not code-signed, so Windows SmartScreen shows a
warning on first launch until the application builds a reputation. To remove
it, sign the executable and the installer in the release workflow, for example
with [SignPath Foundation](https://signpath.org/) (free for open-source
projects) or [Azure Artifact Signing](https://learn.microsoft.com/azure/trusted-signing/).
