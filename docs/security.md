# Security — ZlijaNote v0.1.0-alpha

The security model for user content: what can run, what can be embedded, and how
untrusted content is isolated. Rules are strict in v0.1 and can be relaxed
deliberately later — never silently.

## 1. Core distinction

| Content | Decision |
|---|---|
| User JavaScript (inside a note) | Forbidden |
| External iframe | May contain JavaScript — but that JavaScript runs inside the external site's own context, not the note |

An embedded YouTube iframe runs YouTube's own JavaScript inside the frame. The
user never wrote JavaScript inside the note.

## 2. JavaScript and event handlers

Always rejected inside note content:

- `<script>` elements
- Event handler attributes: `onclick`, `onload`, and all other `on*`
- External scripts
- Form submission

GrapesJS renders the canvas inside an iframe, and its parser has settings that
disallow `<script>` and dangerous event attributes by default — aligned with the
no-JavaScript decision. The Storage Manager is never used for persistence: HTML
files in the Project remain the source of truth.

## 3. iframe policy (v0.1.0-alpha)

We allow iframes, but with rules:

| Rule | Behavior |
|---|---|
| JavaScript inside a note | Always rejected |
| `<script>` | Rejected |
| `onclick` / `onload` | Rejected |
| Local iframe | Allowed if the HTML is safe |
| External iframe | Allowed only via a dedicated Embed block |
| iframe links | Checked against an allowed domains list |
| Arbitrary manual iframe | We show a warning or reject it |
| Preview | Runs inside a sandboxed iframe |
| Access to device files | Forbidden |
| Access to Tauri APIs | Forbidden |

### Trusted domains list

v0.1 does not allow the user to place any URL directly inside an iframe. We start
with a small trusted list:

- YouTube
- Google Maps
- CodePen
- GitHub Gist

More domains can be added later from Settings. This is not censorship of the
user; it protects the app from annoying or unexpected embedded content.

## 4. Preview

Opening external notes or pasted HTML must happen in an isolated environment:
an iframe with `sandbox` that imposes restrictions on embedded content, lifting
specific restrictions only when needed. Never combine `allow-scripts` and
`allow-same-origin` for untrusted content — that can severely weaken isolation.

v0.1 Preview rule:

- HTML/CSS: allowed
- JavaScript: forbidden
- External resources: forbidden by default
- iframe: disabled or heavily sandboxed
- Links: open in the external browser after confirmation

This keeps "runs without internet" true: the note does not depend on external
sites, fonts, or scripts.

## 5. Isolation boundaries

- Notes never execute JavaScript
- Notes cannot access device files
- Notes cannot access Tauri APIs
- Embedded content is confined to its sandboxed frame
- External links leave the app and open in the user's browser, after confirmation

## 6. Data integrity

- The app never silently rewrites, deletes, or "fixes" user code
- Broken HTML is kept, never replaced — errors are shown with location when
  possible
- A safety copy of the last clean version is kept before sensitive save
  operations
- Deletion is never permanent first: notes go to Trash; permanent deletion
  requires confirmation
- Restoring a history version first saves the current version, so restore never
  destroys data

## 7. Out of scope for v0.1

- Note encryption (a separate Encrypted Workspace mode may come later; in v0.1
  notes stay plain HTML so they remain openable in any browser)
- Cloud sync (OneDrive / Google Drive / ZlijaNote Drive)
- Real collaboration between users
- Network is optional: allowed for links and iframes, not required to run the app

## 8. Related documents

- `doc/architecture.md` — sandboxing in the editor and preview architecture
- `doc/file-format.md` — supported HTML whitelist and embed restrictions
- `doc/product-spec.md` — v0.1.0-alpha scope and definition of done
