# UI Flow — ZlijaNote v0.1.0-alpha

The main screens and navigation paths of the application, based on the locked-in
v0.1 decisions: functional hybrid design, eye-friendly Moroccan zellige palette,
English LTR interface (content supports all languages, RTL included).

## 1. Screen map

```
┌─────────────┐     ┌──────────────────┐     ┌─────────────────────────┐
│  Onboarding │────►│ Workspace/       │────►│ Project view            │
│  (1st run)  │     │ Projects home    │     │ (sidebar + notes list)  │
└─────────────┘     └────────┬─────────┘     └──────────┬──────────────┘
                             │                          │
                             │ new project / open       │ open note
                             ▼                          ▼
                     ┌───────────────┐          ┌───────────────────────┐
                     │ New Project   │          │ Editor                │
                     │ dialog        │          │ Design | HTML | CSS   │
                     └───────────────┘          │ | Preview (tabs)      │
                                                └───────────┬───────────┘
                                                            │
                                              ┌─────────────┴──────────────┐
                                              ▼                            ▼
                                    ┌──────────────────┐          ┌──────────────────┐
                                    │ Trash            │          │ History          │
                                    │ (restore/delete) │          │ (restore version)│
                                    └──────────────────┘          └──────────────────┘

Settings (limited) and Backup ZIP dialog are reachable from the app level and
the Project view respectively.
```

## 2. Screens

### 2.1 Onboarding (first run only)

Shown on the very first launch:

1. Explains what a Project and a Note are
2. Explains how to start (first project, first note)
3. Asks to choose or create a Workspace
4. Suggests `Documents/ZlijaNote`
5. Every new Project is created inside that Workspace afterwards

### 2.2 Workspace / Projects home

- Lists the user's Projects (each is a plain folder in the Workspace)
- Project entry shows: title, creation date, and optional banner
- Actions: open Project, create new Project (name + optional banner dialog)

### 2.3 Project view

Sidebar layout:

- Project header: name, banner, creation date
- Search box — searches note titles only
- Notes list
- Tags filter
- Pinned Notes section
- Recent Notes section
- Project settings (limited in v0.1)

Note actions: open, rename, delete (moves to Trash), pin/unpin, edit tags.

### 2.4 Editor

The heart of the app. Four modes with a clear separation, as tabs:

- **Design Mode** — visual drag-and-drop canvas with official ZlijaNote
  components; a note sheet, not a full website page
- **HTML Mode** — code editor restricted to the supported-tags whitelist
- **CSS Mode** — free CSS, saved exactly as written
- **Preview Mode** — sandboxed final result (Desktop only in v0.1)

Layout of Design Mode:

```
┌───────────┬────────────────────────────────────┬───────────────┐
│ Components│            Canvas                  │  Inspector    │
│ palette   │  (note sheet, drag-and-drop,       │  (selected    │
│           │   breadcrumb above it)             │   element:    │
│           │                                    │   color,      │
│           │                                    │   padding,    │
│           │                                    │   border-     │
│           │                                    │   radius,     │
│           │                                    │   CSS class)  │
└───────────┴────────────────────────────────────┴───────────────┘
```

- **Components palette (v0.1):**
  - Text: Heading, Paragraph, Quote, Divider
  - Layout: Section, Container, Row, Column, Spacer
  - Content: Image, Code Block, Table, Callout
  - Interaction: Button, Functional Checklist
  - Organization: Card, Badge/Tag
  - Advanced: Custom HTML block, Custom CSS class
- **Breadcrumb:** selecting an element shows its breadcrumb (path in the
  component tree)
- **Inspector:** changing color, padding, or border-radius writes a CSS class,
  never inline styles
- **Save:** Auto-save + manual Save button

Editor behavior contract (from `doc/product-spec.md`):

- Supported HTML stays visually editable
- Unsupported HTML: warning appears, no automatic conversion, shows in Preview
  (possibly read-only in Design Mode)
- Invalid CSS: clear error, previous version is not lost
- Broken HTML: never auto-fixed; code is kept to prevent data loss
- Returning to Design Mode warns which parts will be read-only

### 2.5 Trash

- Deleted notes appear here, recoverable
- Actions: restore note, permanently delete (requires confirmation)

### 2.6 History

- Previous versions of a note (`history/<note-id>/`, last 30 versions per note)
- Timestamps visible
- Actions: view version, restore version (current version is saved first, so
  restore never destroys it)

### 2.7 Settings

- Limited settings in v0.1
- Theme: Light / Dark
- Later: embed domains list additions

### 2.8 Backup

- "Backup Project" button produces a ZIP of the project to a user-chosen folder

## 3. Key navigation paths

### 3.1 First-run path

```
Launch → Onboarding → choose/create Workspace → Projects home
       → New Project (name + optional banner) → Project view
       → New Note (blank HTML) → Editor (Design Mode)
```

### 3.2 Building a note

```
Design Mode → drag components to canvas → drag-and-drop reorder
            → select element → breadcrumb + Inspector edits
            → switch to HTML Mode (manual content) → CSS Mode (manual styles)
            → Preview Mode (sandboxed result)
```

### 3.3 Delete and undo flow

```
Select element → Delete/Backspace → element removed
               → toast "Element deleted — Undo" → Ctrl+Z restores
Section with many children → confirmation dialog before delete
Delete whole note → goes to Trash (no permanent deletion)
Permanent delete → only from Trash, with confirmation
```

### 3.4 Error handling flow

```
Unsupported HTML added → warning shown → code kept, not deleted
Invalid CSS written → clear error message with location if possible
                      → previous version preserved
Broken HTML → kept as-is → user fixes it themselves (app never auto-fixes)
```

### 3.5 Version history flow

```
Manual Save / real change (auto-save) → new note file written
                                       → previous version moved to history/<note-id>/
                                       → last 30 versions kept
History screen → view timestamps → restore version
               → current version saved first (never destroyed)
```

### 3.6 Trash and restore flow

```
Project view → delete note → note moves to Trash
Trash → restore (back to notes) or permanently delete (confirmation)
```

### 3.7 Backup flow

```
Project view → Backup Project → choose folder → ZIP created locally
```

### 3.8 Open-in-browser flow

```
Outside the app → open any note .html in a browser → content renders correctly
                 (metadata lives in <head>, user content in <body>)
```

## 4. Interface principles (v0.1)

- English UI, LTR; note content supports all languages (RTL included); each
  language uses its popular default font
- Light and Dark mode, eye-friendly Moroccan zellige-inspired palette: calm
  colors, clear contrast, organized edges
- Functional and simple — no crowded or complex interface
- Clear separation between visual and code modes
- The app never forces a fixed shape on the user's content: what you make is
  what you see
