# First push to GitHub

This folder is ready to become the `understory` repository. Nothing has been committed yet.

```sh
cd understory
git init -b main
git add .
git commit -s -m "Initial import: version 0.1"
git remote add origin git@github.com:<owner>/understory.git
git push -u origin main
```

Before pushing:

- Replace `OWNER` in `Cargo.toml` (engine) with the GitHub user or organisation.
- Decide whether each repository is public or private. The engine is designed to be public; the game holds unreleased design and is expected to stay private for now.
- "Understory", "Heartlight Heroes" and "Lumina" are working names without a trademark or package-name search.
- The CI workflows in `.github/workflows/` have not run on GitHub yet; expect to adjust them on the first run.
