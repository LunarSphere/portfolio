# Portfolio

Portfolio website of James Kevius Tribble, built as a Rust/WebAssembly browser
TUI with Ratzilla, Ratatui, and Trunk.

## Run Locally

Install the WASM target and Trunk:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --version 0.21.14
```

Start the development server:

```bash
trunk serve
```

If your shell exports `NO_COLOR=1` and Trunk rejects it, run:

```bash
NO_COLOR=true trunk serve
```

## Build

```bash
trunk build --release
```

The static output is written to `dist/`. Vercel uses `install.sh`, which installs
Rust tooling when needed and runs the same release build.

## Edit Content

Portfolio content is loaded at compile time from JSON:

- `data/projects.json` for project entries.
- `data/socials.json` for social/profile links.

After editing either file, rebuild or restart `trunk serve`.

## Static Assets

- Replace `public/resume.pdf` with the current resume. The app exposes it at
  `/resume.pdf`.
- Replace `public/mascot.png` with a PNG or animated GIF from Aseprite if
  desired, then update the image path in `index.html` if the extension changes.

## Commands

The interactive shell supports `/help`, `/about`, `/projects`, `/resume`,
`/socials`, `/toggle`, and `/clear`. In project and social panels, use Up/Down
or mouse hover to select an item; press Enter or click to open its primary URL.
