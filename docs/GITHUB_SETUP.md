# GitHub repository setup

Recommended settings for this **public** repository. They protect the code,
the releases and your account, and keep the automation working.
Paths refer to the repository's **Settings** tab unless stated otherwise.

## 1. Your account

- **Two-factor authentication**: *Account settings → Password and
  authentication*. Prefer a passkey or an authenticator app over SMS.
- **E-mail privacy**: *Account settings → Emails* → enable *Keep my email
  addresses private* and *Block command line pushes that expose my email*.
  Commits in this repository already use the `users.noreply.github.com` address.
- **Signed commits (optional)**: add an SSH signing key
  (*SSH and GPG keys → New SSH key → Signing key*), then
  `git config --global gpg.format ssh`,
  `git config --global user.signingkey <path to .pub>` and
  `git config --global commit.gpgsign true`. Enable *Vigilant mode* to flag
  unsigned commits made in your name.

## 2. General

*Settings → General*

- **Description and topics**: add a short description and topics such as
  `discord`, `rich-presence`, `rust`, `windows`, `egui`.
- **Features**: disable *Wikis* (documentation lives in `docs/`) and
  *Projects* if unused; keep *Issues*. *Discussions* is optional.
- **Pull Requests**: allow *squash merging* only, with *Pull request title* as
  the default message; enable *Always suggest updating pull request branches*
  and *Automatically delete head branches*.
- **Releases**: enable **Release immutability**, so published files can never
  be replaced.

## 3. Rulesets

*Settings → Rules → Rulesets → New ruleset → New branch ruleset*

**Protect master**

- Enforcement status: *Active*. Target: *Include default branch*.
- *Restrict deletions* and *Block force pushes*.
- *Require linear history*.
- *Require status checks to pass*, with these checks (they appear after the
  first CI run): `Format, lint and test`, `Minimum supported Rust version`,
  `Dependency audit`, `Build and test the installer`.
- *Require a pull request before merging* with **0** required approvals (a
  solo maintainer cannot approve their own pull request). Add *Repository
  admin* to the bypass list if you still want to push small fixes directly to
  `master`.

**Protect release tags** (*New tag ruleset*)

- Target: *Include by pattern* `v*`.
- *Restrict updates*, *Restrict deletions* and *Block force pushes*, so a
  published version tag can never be moved.

## 4. Actions

*Settings → Actions → General*

- **Actions permissions**: *Allow Yatsuuw, and select non-Yatsuuw, actions and
  reusable workflows* → check *Allow actions created by GitHub* and list:

  ```text
  Swatinem/rust-cache@*,
  EmbarkStudios/cargo-deny-action@*
  ```

  Enable *Require actions to be pinned to a full-length commit SHA* (the
  workflows already pin every action; Dependabot keeps the pins up to date).
- **Fork pull request workflows**: *Require approval for all external
  contributors*.
- **Workflow permissions**: *Read repository contents and packages
  permissions*, and leave *Allow GitHub Actions to create and approve pull
  requests* **unchecked**. Jobs that need more (the release) request it
  explicitly.

*Settings → Environments → New environment* named **`release`**:

- *Deployment branches and tags* → *Selected branches and tags* → add the tag
  rule `v*`. Only version tags can then run the release job.
- Optional: add yourself as *Required reviewer* to approve each release
  manually before it is published.

No secret is needed: the workflows only use the automatic `GITHUB_TOKEN`.

## 5. Security

*Settings → Advanced Security* (called *Code security* on some accounts)

- **Private vulnerability reporting**: enable (used by `SECURITY.md`).
- **Dependency graph**, **Dependabot alerts**, **Dependabot security
  updates** and **Grouped security updates**: enable.
  Version updates are configured in `.github/dependabot.yml`.
- **Code scanning → CodeQL analysis → Set up → Default**: enable it for the
  *Rust* and *GitHub Actions* languages.
- **Secret scanning** and **Push protection**: enable, so a token
  accidentally committed is blocked before it becomes public.

## 6. Day-to-day habits

- Never commit secrets, `.env` files or personal data; the repository is public.
- Merge Dependabot pull requests once the CI is green.
- Publish releases only through tags (see [RELEASING.md](RELEASING.md)), never
  by uploading files by hand.
