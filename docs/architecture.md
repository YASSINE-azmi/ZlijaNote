# Architecture — ZlijaNote v0.1.0-alpha

How the Rust/Tauri shell, the editor UI, and the file layer fit together, and the
sandboxing model that keeps user content safe.

## Overview

ZlijaNote is a local-first desktop application. The Tauri shell owns the file
system and workspace management; the web UI embeds the GrapesJS visual editor.
Files on disk are the source of truth — GrapesJS is only an editor.

```
┌─────────────────────────────────────────────┐
│                  Tauri shell                │
│                                             │
│  ┌───────────────┐   ┌───────────────────┐  │
│  │  Rust core    │   │  File layer       │  │
│  │  (commands,   │◄──│  (workspace,      │  │
│  │   validation, │   │   projects,       │  │
│  │   history)    │   │   history, trash) │  │
│  └───────┬───────┘   └───────────────────┘  │
│          │ invoke (IPC)                     │
│  ┌───────▼────────────────────────────────┐ │
│  │  Web UI                                │ │
│  │  ├── GrapesJS canvas (design mode)     │ │
│  │  ├── HTML code editor                 │ │
│  │  ├── CSS code editor                  │ │
│  │  └── Sandboxed preview iframe         │ │
│  └────────────────────────────────────────┘ │
└─────────────────────────────────────────────┘
```

## Storage paths

Following the XDG Base Directory specification on Linux (via Tauri's path APIs,
not hardcoded):

| Purpose | Linux path |
|---|---|
| Application configuration | `~/.config/zlijanote/` |
| Application internal data | `~/.local/share/com.zlija.zlijanote/` |
| User workspaces | Anywhere the user chooses (e.g. `~/Documents/ZlijaNote/`) |

Notes never live inside the config directory — workspaces are user-visible,
copyable, and easy to back up.

### First-run onboarding

1. Onboarding explains what a Project and a Note are.
2. User chooses or creates a Workspace (suggesting `Documents/ZlijaNote`).
3. Every new Project is created inside that Workspace.

## The file layer (Rust)

- Owns all read/write access to the workspace (`doc/file-format.md`).
- Save path: on manual Save or real change (auto-save), write the new note file,
  move the previous version into `history/<note-id>/`, keep the last 30 versions
  per note (HTML only, no assets).
- Never rewrites user code: errors surface as clear messages with location hints;
  broken HTML is kept as-is to prevent data loss.
- Delete: notes go to Trash (recoverable); permanent deletion requires
  confirmation; section-sized deletions in the editor require confirmation.
- Backups: manual "Backup Project" ZIP; automatic safety copy of the last clean
  version before sensitive save operations (up to ~10 per note).

## Editor UI (web)

Four modes with a clear contract:

| Mode | Behavior |
|---|---|
| Design Mode | Drag-and-drop with official ZlijaNote components only |
| HTML Mode | Code editing restricted to the supported-tags whitelist |
| CSS Mode | Free CSS, saved exactly as written |
| Preview Mode | Sandboxed rendering of the final result |

There is no separate advanced "HTML preview": in v0.1 Preview serves that role.
Any HTML outside the supported elements shows as an error and is not saved until
the user fixes it.

### GrapesJS integration

- GrapesJS renders the canvas inside an iframe; its parser is configured to
  disallow `<script>` and dangerous event attributes (`onclick`, …) by default.
- Its Storage Manager is not used for persistence (default is `localStorage`):
  saving is bound to writing `.html` files through the Rust/Tauri layer.
- This prevents GrapesJS lock-in and keeps migrating to a custom ZlijaNote Core
  engine feasible later.

### Components (v0.1)

- **Text:** Heading, Paragraph, Quote, Divider
- **Layout:** Section, Container, Row, Column, Spacer
- **Content:** Image, Code Block, Table, Callout
- **Interaction:** Button, Functional Checklist
- **Organization:** Card, Badge/Tag
- **Advanced:** Custom HTML block, Custom CSS class

Not now: video, audio, charts, complex forms, embedded JavaScript, external
plugin components, AI writing.

Visual editing rules: dragging a component generates clean HTML/CSS; changing
padding or color edits a CSS class (never inline styles); manually added classes
are preserved; supported HTML stays visually editable; advanced/unsupported HTML
shows a warning and is not auto-converted; returning to Design Mode explains
which parts will be read-only.

### Delete and Undo

- Delete / Backspace removes the selected element — no confirmation dialog per
  element; a toast "Element deleted — Undo" appears; Ctrl+Z restores.
- Deleting a section with many children may show a confirmation.
- Undo is mandatory.

## Security model (see `doc/security.md` for full details)

- JavaScript inside a note: always rejected. `<script>` rejected. `onclick` /
  `onload` rejected.
- Local iframes: allowed only if the HTML is safe.
- External iframes: only via the dedicated Embed block, checked against a trusted
  domains list starting with YouTube, Google Maps, CodePen, GitHub Gist. More
  domains can be added later from Settings. Arbitrary manual iframes warn or are
  rejected.
- Preview runs inside a sandboxed iframe; device file access and Tauri API access
  are forbidden. `allow-scripts` and `allow-same-origin` are never combined for
  untrusted content.
- v0.1 Preview rule: HTML/CSS allowed, JavaScript forbidden, external resources
  forbidden by default, links open in the external browser after confirmation.

## Editor modes contract

The critical rule across all modes: never repair HTML automatically or write
over user code. The app may suggest a fix or offer Format, but it never changes
the source on its own.

## Tech stack

- Rust + Tauri (desktop shell, filesystem, workspace management)
- GrapesJS (visual editor, v0.1 only)
- Plain HTML/CSS files (storage format)
- Later: ZlijaNote Core, a custom editor engine built after product maturity

## Distribution

First release: v0.1.0-alpha as an AppImage on Linux (Arch first). After the
first version succeeds on Arch: AUR, then consider Flatpak, `.deb`, and `.rpm`
for other distributions. Windows/macOS later.
