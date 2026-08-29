# Project File Format — ZlijaNote v0.1.0-alpha

This document defines the on-disk format for ZlijaNote: the workspace layout, the
`project.zlija.json` file, and the HTML note format with its metadata.

## Guiding rules

- The `.html` file is the source of truth. GrapesJS is only an editor; its internal
  storage is never the primary storage.
- Notes must remain portable: openable in any browser, copyable, movable, and
  editable outside the app.
- No lock-in. The format stays simple enough that moving to a custom
  ZlijaNote Core engine later remains possible.

## Workspace layout

```
Workspace/
│
├── project-zlijanote/
│   ├── project.zlija.json
│   ├── notes/
│   │   ├── architecture.html
│   │   ├── rust-roadmap.html
│   │   └── ui-ideas.html
│   │
│   ├── assets/
│   │   ├── architecture-diagram.png
│   │   ├── zellij-background.jpg
│   │   └── rust-logo.svg
│   │
│   ├── history/
│   │   ├── architecture/
│   │   ├── rust-roadmap/
│   │   └── ui-ideas/
│   │
│   └── trash/
│
└── university-notes/
    ├── project.zlija.json
    ├── notes/
    ├── assets/
    ├── history/
    └── trash/
```

The Workspace location is user-selected (e.g. `~/Documents/ZlijaNote/`). Each
Project is a plain folder inside it.

## Folder responsibilities

| Path | Responsibility |
|---|---|
| `project.zlija.json` | Project name, banner, creation date, pinned notes, tags, and project settings |
| `notes/` | The real note files, in `.html` format |
| `assets/` | Images and files shared across all notes in the project |
| `history/` | Previous versions of each note (`history/<note-id>/`) |
| `trash/` | Deleted notes, recoverable |

Application configuration and internal data live outside the workspace (see
`doc/architecture.md`), so the workspace contains only user files.

## The note format

Every note is a complete HTML document — full HTML content in `<body>`, not a
fragment. The CSS lives inside the same file, in a `<style>` element in the
`<head>` (chosen over a separate `note-name.css` for v0.1: one self-contained
document that is easy to move, copy, and open outside the app).

```
HTML document
├── head
│   ├── ZlijaNote metadata
│   ├── title
│   ├── created date
│   ├── updated date
│   ├── tags
│   └── style
└── body
    └── user content, fully designable
```

Metadata is organized in the `<head>`, never inside the `<body>` the user edits.
When the user opens the file in a browser they see the note rendered correctly;
when ZlijaNote opens it, it reads the metadata and shows the note in the sidebar
and Project view.

A new note starts as a blank page (only the document skeleton with metadata).

### Supported HTML (v0.1 whitelist)

Code mode accepts a defined list; anything outside it is shown as an error and is
not saved until the user fixes it. Preview is broader and displays unsupported
HTML read-only.

- **Document and layout:** `main`, `section`, `div`, `article`, `header`, `footer`
- **Text:** `h1`–`h6`, `p`, `span`, `strong`, `em`, `mark`, `small`, `blockquote`, `hr`
- **Lists:** `ul`, `ol`, `li`
- **Links and images:** `a`, `img`
- **Tables:** `table`, `thead`, `tbody`, `tr`, `th`, `td`
- **Developer content:** `pre`, `code`
- **Interactive:** `button`, `input[type="checkbox"]`, `label`
- **Embed:** `iframe` — restricted and only via the dedicated Embed block with a
  trusted domains list (see `doc/security.md`)

Not allowed in v0.1: `script`, event handlers (`onclick`, `onload`, …), form
submission, external scripts, and JavaScript of any kind.

Links: both local (`./another-note.html`) and external URLs are allowed.

### Manual HTML/CSS

- Manual HTML is allowed within the whitelist.
- Manual CSS is allowed and saved exactly as the user wrote it.
- User-added CSS classes are preserved; the app never deletes or rewrites them.
- The app never silently fixes broken HTML/CSS. On error: a clear message with the
  error location when possible; broken code is kept, never replaced (no data loss).
- Visual changes (padding, color, …) are saved as CSS classes, not inline styles.

## `project.zlija.json`

One per project folder. Holds:

- name
- banner
- creation date
- pinned notes
- tags
- project settings

## Version history (v0.1)

No full "Git inside each note" system. Simple rules:

- On manual Save or a real change via auto-save, the app saves the new note file.
- The previous version is moved into `history/<note-id>/`.
- A maximum of the last 30 versions per note is kept.
- One version per real change — never per keystroke.
- Versions store the HTML only (not `assets/`), so history never fills the disk
  with duplicated images.
- Timestamps are visible; restoring a version is allowed and first saves the
  current version as a new entry (restore never destroys the current version).

## Trash and backup

- Deleting a note moves it to `trash/` — it is never permanently deleted first.
- Final deletion from Trash requires confirmation.
- Manual backup: a "Backup Project" button produces a ZIP of the project to a
  user-chosen folder.

## Out of scope for v0.1

- Encryption (a separate Encrypted Workspace mode may come later; in v0.1 notes
  stay plain HTML so browsers can open them)
- Cloud sync (OneDrive / Google Drive / ZlijaNote Drive)
- Video and audio elements
- Global CSS templates, custom editor engine, plugins marketplace, AI writing,
  full-content HTML search, note thumbnails
