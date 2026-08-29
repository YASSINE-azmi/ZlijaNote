# RFC-0001: Workspace and Project Core

- **Feature Name:** `workspace_project_core`
- **Start Date:** 2026-08-28
- **Author(s):** @YASSINE-azmi
- **RFC PR:** TBD
- **Status:** Accepted

---

## 1. Summary

This RFC proposes the Workspace and Project Core for ZlijaNote.

A Workspace is a user-selected directory that contains one or more ZlijaNote Projects. A Project is a self-contained directory that stores its metadata, Notes, assets, history snapshots, and trashed Notes. The Workspace and Project Core will provide the first persistent, user-owned data foundation for ZlijaNote.

After this RFC is implemented, a user will be able to select or create a Workspace, create a Project inside it, close ZlijaNote, reopen the application, and safely discover and open the same Project again.

This RFC does not introduce the Note editor, HTML/CSS editing, GrapesJS, CodeMirror, assets import, Version History behavior, or Trash behavior. It only creates the folders and metadata structure required by those future features.

---

## 2. Motivation

ZlijaNote is a local-first visual note editor. Its most important product principle is that user data belongs to the user and remains understandable outside the application.

Before implementing Notes, HTML/CSS editing, or visual drag-and-drop features, ZlijaNote needs a reliable answer to these questions:

- Where do user Projects live?
- How does the application identify a valid Project?
- How does ZlijaNote restore the user’s last Workspace after restart?
- How can the application create a Project without risking partial or corrupted user data?
- How can future features such as Notes, assets, history, trash, backup, and search share a stable directory structure?

Without a Workspace and Project foundation, every future feature would need to invent its own storage rules. That would make the application harder to maintain and increase the risk of data loss.

### Primary use cases

#### Create a new Workspace

A user opens ZlijaNote for the first time and chooses a directory such as:

```text
/home/yassine/Documents/ZlijaNote/
```

ZlijaNote treats this directory as a Workspace where the user can create and manage Projects.

#### Create a Project

The user creates a Project named:

```text
Rust Learning
```

ZlijaNote creates the required Project folders and metadata file without creating any Note yet.

#### Reopen an existing Workspace

The user closes ZlijaNote and opens it again. The application remembers the most recently opened Workspace and discovers the existing Projects inside it.

#### Move or back up user data

Because a Project is a normal directory with normal files, a user can copy, move, compress, or back up a Workspace without depending on a cloud service or a hidden database.

---

## 3. Detailed Design

### 3.1 Core concepts

This RFC defines three storage concepts:

```text
Application configuration
→ Stores ZlijaNote application state.

Workspace
→ A user-selected directory containing Projects.

Project
→ A self-contained directory containing Project metadata and future Note-related data.
```

### 3.2 Application configuration

Application configuration is private application state. It is not the primary location of user content.

It stores information such as:

```text
- Last opened Workspace path
- Recent Workspace paths
- Selected application theme
- Future application preferences
```

It must not store:

```text
- Project contents
- Note contents
- HTML/CSS note source
- Assets
- Version History snapshots
```

Projects remain inside the user-selected Workspace so they remain visible, portable, and user-owned.

### 3.3 Workspace structure

A Workspace is a directory selected or created by the user.

Example:

```text
/home/yassine/Documents/ZlijaNote/
```

A Workspace can contain multiple Projects:

```text
ZlijaNote/
├── rust-learning/
├── university-notes/
├── fitness-plans/
└── zlijanote-development/
```

The Workspace itself does not require a metadata file in this RFC.

A directory is considered a Workspace when the user selects it in ZlijaNote. It may be empty when first selected.

### 3.4 Project directory structure

Each Project is represented by one directory inside a Workspace.

The required structure is:

```text
<workspace>/<project-folder>/
├── project.zlija.json
├── notes/
├── assets/
├── history/
└── trash/
```

Each entry has a clear responsibility:

| Entry                | Responsibility                                                      |
| -------------------- | ------------------------------------------------------------------- |
| `project.zlija.json` | Project metadata, schema version, identity, and display information |
| `notes/`             | Future HTML Note files                                              |
| `assets/`            | Future images and other local assets shared by Notes in the Project |
| `history/`           | Future Note version snapshots                                       |
| `trash/`             | Future deleted Notes that can be restored                           |

This RFC creates all required directories even though Notes, assets, history, and trash behavior are implemented in later milestones.

### 3.5 Project metadata

Each Project contains a metadata file named:

```text
project.zlija.json
```

The initial metadata model must include:

```text
schema_version
project_id
name
description
banner_path
created_at
updated_at
```

The conceptual JSON shape is:

```json
{
  "schema_version": 1,
  "project_id": "stable-project-identifier",
  "name": "Rust Learning",
  "description": "",
  "banner_path": null,
  "created_at": "2026-08-28T00:00:00Z",
  "updated_at": "2026-08-28T00:00:00Z"
}
```

### 3.6 Metadata rules

The metadata file must follow these rules:

- `schema_version` is required for future migrations.
- `project_id` is generated once and must never change.
- `name` is the human-readable Project name.
- `description` is optional and may be empty.
- `banner_path` is optional and must be `null` until banner support is implemented.
- `created_at` is set when the Project is created.
- `updated_at` changes when Project metadata changes.
- The metadata must be valid JSON.
- The metadata must be readable after restarting the application.

### 3.7 Project naming and folder names

A Project has two related identities:

```text
Display name
→ What the user sees in the interface.

Folder name
→ The directory name on disk.
```

Example:

```text
Display name: Rust Learning
Folder name: rust-learning
```

The display name should support Unicode text, including Arabic:

```text
Display name: دروس Rust
```

However, the folder name must be safe for the filesystem.

The application must reject unsafe names or paths such as:

```text
.
..
../other-folder
project/name
project\name
```

The implementation must also reject:

```text
- Empty names
- Whitespace-only names
- Names that resolve outside the selected Workspace
- Duplicate Project folder names
```

The initial implementation should generate a safe folder name from the display name. Manual folder-name editing is outside the scope of this RFC and may be added later.

### 3.8 Project discovery

When a Workspace is opened, ZlijaNote must inspect its direct child directories.

A directory is considered a valid Project when:

```text
- It is a directory.
- It contains project.zlija.json.
- project.zlija.json can be parsed.
- The metadata contains a supported schema version.
```

The application must safely ignore:

```text
- Regular files inside the Workspace.
- Directories without project.zlija.json.
- Directories with invalid or unsupported Project metadata.
```

Malformed metadata must not crash the application.

The UI may show a recoverable warning for malformed Projects in a future improvement, but the first implementation may ignore them safely and log a structured error for debugging.

### 3.9 Project creation flow

Creating a Project must follow a safe sequence:

```text
1. Receive the Project display name.
2. Trim and validate the name.
3. Generate a safe folder name.
4. Resolve the final Project path inside the selected Workspace.
5. Verify that the target path does not already exist.
6. Generate a stable Project ID.
7. Build ProjectMetadata in memory.
8. Create the Project directory.
9. Create notes/, assets/, history/, and trash/.
10. Write project.zlija.json through a temporary file.
11. Atomically rename the temporary file to project.zlija.json.
12. Return the created Project to the application layer.
```

If an error occurs during creation, the application should avoid leaving a partially initialized Project directory whenever possible.

### 3.10 Layered architecture

The Workspace and Project Core must follow the ZlijaNote architecture:

```text
Frontend
→ Tauri command
→ Application service
→ File-system repository
→ Workspace and Project files
```

#### Domain layer

The domain layer defines concepts and rules without depending on framework or filesystem details.

It may contain concepts such as:

```text
Workspace
WorkspacePath
Project
ProjectId
ProjectName
ProjectMetadata
ProjectPath
```

The domain layer must not depend on:

```text
- Tauri
- React
- GrapesJS
- CodeMirror
- JSON parsing
- File dialogs
- Filesystem APIs
```

#### Application layer

The application layer contains use cases such as:

```text
OpenWorkspace
CreateWorkspace
ListProjects
CreateProject
OpenProject
GetRecentWorkspaces
SetLastOpenedWorkspace
```

This layer coordinates domain validation and infrastructure implementations.

#### Infrastructure layer

The infrastructure layer performs filesystem and persistence operations.

It may contain responsibilities such as:

```text
- Create directories
- Read project.zlija.json
- Write project.zlija.json
- Persist application configuration
- Validate resolved paths
- Write files atomically
- Convert I/O failures into application errors
```

#### Tauri command layer

Tauri commands expose small, typed operations to the frontend.

Possible commands include:

```text
choose_workspace
create_workspace
open_workspace
list_projects
create_project
open_project
```

Commands must remain thin. They must not contain filesystem logic, JSON parsing, or Project business rules.

### 3.11 UI requirements

The initial user interface must provide:

```text
Welcome screen
→ Select existing Workspace
→ Create new Workspace

Workspace screen
→ Display known Projects
→ Create Project action
→ Empty state
→ Loading state
→ Error state
```

The UI does not need final visual polish in this RFC.

The user must be able to understand:

```text
- Which Workspace is currently open.
- Which Projects exist inside that Workspace.
- How to create a Project.
- Why an invalid Project name was rejected.
- Why a Workspace could not be opened.
```

### 3.12 Testing requirements

This RFC requires tests for both domain rules and filesystem behavior.

#### Domain tests

```text
- Valid Project names are accepted.
- Empty Project names are rejected.
- Whitespace-only Project names are rejected.
- Unsafe path-like names are rejected.
- Generated Project IDs are unique.
```

#### Integration tests

Integration tests must use temporary directories.

```text
- A Project directory structure is created correctly.
- project.zlija.json is valid and readable.
- A Project can be discovered after creation.
- Existing valid Projects are listed.
- Unrelated folders are ignored.
- Malformed project.zlija.json does not crash discovery.
- Duplicate Project creation is rejected.
- Failed creation does not overwrite existing data.
```

---

## 4. Drawbacks & Trade-offs

### Added initial complexity

This RFC introduces multiple layers, metadata files, path validation, and filesystem tests before the application can create a single Note.

This is intentional. ZlijaNote is a local-first application, so reliable user data storage is more important than quickly building an editor UI.

### Folder-name generation is imperfect

Generating safe folder names from display names may produce less readable paths for Arabic or other non-Latin names.

For example:

```text
Display name: دروس Rust
Generated folder name: rust-project
```

This is not ideal, but it is safer than allowing arbitrary user-controlled filesystem paths in the first implementation.

### No Workspace metadata file

This RFC does not define a Workspace metadata file.

That keeps the first implementation simple, but future features such as Workspace-level settings, global templates, or Workspace-wide search may eventually need one.

### Partial creation cleanup is best-effort

Filesystem operations can fail for many reasons: insufficient permissions, disk failures, invalid paths, or interrupted processes.

The application should clean up partial Project directories when possible, but it cannot guarantee cleanup in every operating-system failure scenario.

---

## 5. Alternatives

### Alternative 1: Store everything in a SQLite database

Under this approach, Projects and future Notes would live in a database file managed by ZlijaNote.

This approach was rejected because:

- ZlijaNote is designed around user-owned HTML files.
- Users should be able to inspect, copy, move, and back up their Projects without database tooling.
- Future Notes must remain openable outside ZlijaNote.
- A database would create stronger application lock-in.

SQLite may be used later for an optional search index, but it will not become the source of truth for Project or Note content.

### Alternative 2: Store all Projects inside the application data directory

Under this approach, ZlijaNote would place all user Projects inside a hidden application directory.

This approach was rejected because:

- User data would be less visible.
- Manual backup would be less intuitive.
- Moving Projects between machines would be less obvious.
- Users could confuse application configuration with their own content.

Instead, users select their own Workspace.

### Alternative 3: Create only folders when a feature needs them

Under this approach, a new Project would initially contain only `project.zlija.json`, and folders such as `history/` or `trash/` would appear later.

This approach was rejected because:

- Project structure would become inconsistent.
- Future code would need repeated “does this folder exist?” logic.
- A complete Project structure is easier for users and contributors to understand.
- Creating empty folders is inexpensive.

### Alternative 4: Allow arbitrary Project folder names

Under this approach, the user would type both a display name and the exact directory path.

This approach was rejected for the first version because:

- It increases the risk of path traversal.
- It makes validation more difficult.
- It adds unnecessary UI complexity.
- It can cause inconsistent Project structures.

The application will generate safe folder names initially. Advanced folder customization can be considered later.

### Alternative 5: Keep Workspace state only in memory

Under this approach, ZlijaNote would ask the user to select a Workspace every time the application opens.

This approach was rejected because:

- It creates unnecessary friction.
- It makes the application feel unfinished.
- The last opened Workspace is safe and useful application state.

ZlijaNote will persist the last opened Workspace and a recent Workspace list.

---

## 6. Breaking Changes

This RFC does not introduce breaking changes because ZlijaNote has not released a stable Workspace or Project format yet.

The initial schema version is:

```text
schema_version: 1
```

Future changes to `project.zlija.json` must use explicit schema versions and migration logic where necessary.

Future migrations must follow these rules:

```text
- Never silently delete user data.
- Create a safe backup before destructive migrations.
- Preserve compatibility whenever practical.
- Show clear errors when an unsupported Project version is opened.
```

---

## 7. Unresolved Questions

The following questions should be resolved during implementation or future RFCs:

1. Should the application allow users to manually customize the generated Project folder name?

2. What exact slug-generation behavior should be used for Arabic, Amazigh, Japanese, Chinese, and other non-Latin Project names?

3. Should malformed Projects be hidden silently, shown as warnings, or displayed in a dedicated recovery screen?

4. Should Workspace metadata be introduced in a future RFC for Workspace-level preferences and global templates?

5. How many recent Workspaces should the application store by default?

6. Should recent Workspaces that no longer exist on disk be removed automatically or retained with a “missing” state?

7. Should a Project include a default empty Note immediately after creation, or should the Project begin with zero Notes?

8. Should Project descriptions and banners be implemented in this milestone or only represented in metadata until the UI feature exists?

9. What exact application-config location should be used on Linux, Windows, and macOS?

10. Should the implementation create a hidden lock file while a Project is open to reduce concurrent-write risks in a future multi-window or multi-process scenario?
