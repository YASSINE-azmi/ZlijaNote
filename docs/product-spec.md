# Product Specification — ZlijaNote v0.1.0-alpha

This is the adopted specification for the first release, based on the locked-in
decisions for ZlijaNote.

## 1. Product definition

| Field             | Decision                                                                                                                                       |
| ----------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| Name              | ZlijaNote                                                                                                                                      |
| Type              | Visual HTML/CSS Note Editor                                                                                                                    |
| Primary user      | Programmer or technical user                                                                                                                   |
| Goal              | Make writing and reading notes fun, beautiful, and easy                                                                                        |
| Design philosophy | User freedom — what you make is what you see                                                                                                   |
| Short description | Visual note editor for developers                                                                                                              |
| Product promise   | Make writing and reading notes fun, beautiful, and fully customizable                                                                          |
| README tagline    | "ZlijaNote is a local-first visual note editor for developers. Create beautiful HTML/CSS notes with drag-and-drop or write the code yourself." |

The idea is not "another note app": it is code freedom combined with the ease of
visual building.

## 2. Target user

- A programmer or technical user
- Wants notes for study and project documentation
- Knows HTML/CSS, or wants to learn them gradually
- Does not want a crowded or complicated interface
- Wants to see exactly what they made, without the app forcing a fixed shape

## 3. Identity

- Desktop application
- Works locally, no account, no telemetry
- Always open source
- English UI in v0.1; content supports Arabic and all languages (RTL included),
  each language gets its popular default font (with a font import feature later)
- Light and Dark mode
- Eye-friendly visual style inspired by Moroccan zellige colors: calm colors,
  clear contrast, organized edges — not heavy ornamentation
- Interface is LTR in v0.1

## 4. Note form

| Field           | Decision                                            |
| --------------- | --------------------------------------------------- |
| Note shape      | A real `.html` file                                 |
| Written HTML    | Full HTML content (in `<body>`), not a fragment     |
| CSS             | Inside the same HTML file, in `<style>` in `<head>` |
| Note start      | Blank page                                          |
| Title           | Mandatory Metadata, separate from the HTML          |
| JavaScript      | Forbidden in v0.1                                   |
| Links           | Local and external allowed                          |
| Images          | Supported via Image block and clipboard             |
| Video and audio | Postponed                                           |
| Manual HTML     | Allowed within a defined list of tags               |
| Manual CSS      | Allowed                                             |
| Save            | Auto-save + manual Save button                      |
| Errors          | Clear message; user code is never deleted           |

If the user opens the file in a browser they see the note rendered correctly; if
ZlijaNote opens it, it reads the metadata and shows the note in the sidebar and
Project view.

## 5. Editing

- **Visual editor:** GrapesJS at the start; after product maturity, our own
  ZlijaNote Core engine
- **Canvas:** a note sheet, not a full website page
- **Preview:** Desktop only in v0.1

### Mode contract

| Mode         | Behavior                             |
| ------------ | ------------------------------------ |
| Design Mode  | Only official ZlijaNote components   |
| HTML Mode    | Only HTML from the supported list    |
| CSS Mode     | Manual CSS, saved exactly as written |
| Preview Mode | Shows the final result               |

In v0.1 there is no separate advanced "HTML Preview" — Preview fills that role.
Any HTML outside the supported elements shows as an error and is not saved until
the user corrects it.

### Official components (v0.1)

- **Document and layout:** `main`, `section`, `div`, `article`, `header`, `footer`
- **Text:** `h1`–`h6`, `p`, `span`, `strong`, `em`, `mark`, `small`, `blockquote`, `hr`
- **Lists:** `ul`, `ol`, `li`
- **Links and images:** `a`, `img`
- **Tables:** `table`, `thead`, `tbody`, `tr`, `th`, `td`
- **Developer content:** `pre`, `code`
- **Interactive:** `button`, `input[type="checkbox"]`, `label`
- **Embed:** `iframe` (restricted, via Embed block with trusted domains)

### Canvas components (v0.1)

- **Text:** Heading, Paragraph, Quote, Divider
- **Layout:** Section, Container, Row, Column, Spacer
- **Content:** Image, Code Block, Table, Callout
- **Interaction:** Button, Functional Checklist
- **Organization:** Card, Badge/Tag
- **Advanced:** Custom HTML block, Custom CSS class

Not now: video, audio, charts, complex forms, embedded JavaScript, external
plugin components, AI writing.

### Visual/Code mode rules

| Situation                           | ZlijaNote behavior                                                  |
| ----------------------------------- | ------------------------------------------------------------------- |
| User drags an element visually      | Clean HTML and CSS are generated                                    |
| User changes padding or color       | App changes a CSS class, not inline style                           |
| User adds a class manually          | It stays as-is; the app never deletes it                            |
| User edits supported HTML           | The element stays visually editable                                 |
| User adds advanced/unsupported HTML | A warning appears; no automatic conversion                          |
| Unsupported HTML                    | Shows in Preview; may not be editable in Design mode                |
| HTML/CSS contains an error          | Clear message and error location when possible                      |
| HTML completely broken              | Never replaced or "fixed" automatically; code is kept, no data loss |
| User tries to return to Design mode | Explain which parts will be read-only or not visually editable      |

The critical point: never repair HTML automatically or write over user code. The
app may suggest a fix or offer Format, but never changes the source on its own.

### Delete and Undo

- Delete element = allowed; Undo = mandatory
- No confirmation dialog when deleting every paragraph (too annoying):
  - Delete / Backspace deletes the selected element
  - A small toast appears: "Element deleted — Undo"
  - Ctrl+Z restores the deletion
- Deleting a section containing many elements may show a confirmation
- Deleting a whole note moves it to Trash
- Permanent deletion only from Trash, and it requires confirmation

## 6. Organization

| Field        | Decision                                                   |
| ------------ | ---------------------------------------------------------- |
| Projects     | Traditional folders instead of special Projects            |
| Assets       | One shared folder per Project                              |
| Search       | In note title only                                         |
| Organization | Tags + Pinned Notes + Recent Notes                         |
| Theme        | Light and Dark, eye-friendly, Moroccan zellige inspired    |
| Language     | English UI first; content supports all languages           |
| Storage      | Several Projects inside a user-chosen Workspace            |
| Deletion     | Trash + restore                                            |
| Copy         | Version History (essential in v0.1)                        |
| Cloud backup | Postponed                                                  |
| Encryption   | Postponed                                                  |
| Network      | Allowed for links and iframes; not required to run the app |

## 7. Workspace and projects

- Workspace location is chosen by the user (e.g. `~/Documents/ZlijaNote/`)
- Each Project is a folder: `project.zlija.json`, `notes/`, `assets/`, `history/`,
  `trash/` (details in `doc/file-format.md`)
- Images and files live in the project's `assets/` folder
- Project has a name, creation date, and a banner property
- First run: onboarding explains Project and Note, asks to choose or create a
  Workspace, then new Projects are created inside it
- "Add people to project" in v0.1 means local names/contributors inside the
  project metadata — real collaboration is postponed

## 8. Data, safety, and release

- Version History: simple system, not full Git inside each note (see
  `doc/file-format.md`)
- Manual backup: "Backup Project" button produces a ZIP to a user-chosen folder
- Automatic safety copy: keeps the last clean version before sensitive save
  operations (limited count, e.g. last 10 per note)
- Trash: deletion never erases the note permanently; restorable or permanently
  deletable
- v0.1.0-alpha as the first official release (alpha, not v0.1.0, because the
  editor is a first experimental version)
- First platform: Arch Linux / Linux
- First distribution: AppImage; AUR after the first version succeeds on Arch;
  then consider Flatpak, `.deb`, and `.rpm`; Windows and macOS later

## 9. Out of scope for v0.1.0-alpha

- JavaScript inside notes
- Running external scripts
- Note encryption
- OneDrive or Google Drive sync
- ZlijaNote Drive
- Real collaboration between users
- "People" inside Project (as a real feature)
- Video and audio as independent elements
- Mobile support
- Windows and macOS releases
- Global CSS templates
- Custom editor engine (replacing GrapesJS)
- Plugins marketplace
- AI writing
- Full search inside HTML content
- Note thumbnails in the list

## 10. Definition of done

v0.1 is not "done when it works for me". It is done only when a brand-new user
succeeds at these scenarios:

1. Installs the app on Arch Linux and opens it successfully
2. Chooses a Workspace on first run
3. Creates a new Project with a name and optional banner
4. Creates an empty Note in HTML format
5. Drags a Heading, Paragraph, Card, Image, and Checklist to the Canvas
6. Reorders an element inside a Container using drag-and-drop
7. Selects an element and sees its breadcrumb
8. Changes color, padding, and border-radius from the Inspector; the change is
   saved as a CSS class
9. Opens HTML mode and adds content manually
10. Opens CSS mode and adds styles manually
11. Sees the changes in Preview, directly or after safe save
12. Adds unsupported HTML: a warning appears and their code is not deleted
13. Writes invalid CSS: gets a clear error and the previous version is not lost
14. Adds an image from file picker or clipboard inside an Image block
15. Saves a note, closes the app, opens it, and the note appears as it was
16. Finds a note via title search
17. Adds tags and pins a note
18. Deletes a note and restores it from Trash
19. Creates a local Backup of the Project
20. Opens the `.html` file outside the app in a browser and sees the core
    content correctly
21. No note can execute JavaScript or access device files
22. The app is built as an AppImage and works on another Linux machine

## 11. Tech stack

- Rust + Tauri — desktop shell, filesystem, workspace management
- GrapesJS — visual editor (files stay the source of truth; ZlijaNote Core, the
  project's own engine, comes later to get rid of GrapesJS)
- Plain HTML/CSS files — the storage format
