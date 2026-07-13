# Portfolio

James Kevius Tribble's terminal-style portfolio, built with Expo, React Native,
React Native Web, and TypeScript. Expo statically renders each route for Vercel.

## Run locally

Install Node.js, then run:

```bash
npm install
npm run web
```

Expo prints the local URL and reloads the site when source files change.

## Edit site information

All portfolio copy, identity, metadata, projects, and social links live in one
file: `src/content/site.ts`. TypeScript checks that required fields remain
present. After editing it, run `npm run typecheck`.

Static files remain easy to replace:

- `public/resume.pdf` is served at `/resume.pdf`.
- `public/mascot.png` is used as the site and app icon.

## Commands and routes

The terminal supports `/help`, `/about`, `/projects`, `/resume`, `/socials`,
`/toggle`, and `/clear`. Section commands navigate to shareable static routes.
On desktop, Up/Down or hover changes the selected project/social and Enter opens
it. On mobile, tapping an entry opens its primary link directly.

## Verify and deploy

```bash
npm run check
```

This runs TypeScript validation, unit tests, and the static Expo export. The
result is written to `dist/`. Vercel installs dependencies, runs `npm run build`,
and serves clean section URLs such as `/projects` and `/about`.
