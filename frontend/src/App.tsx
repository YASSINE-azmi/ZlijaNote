import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import "./index.css";

type Theme = "light" | "dark";

function getInitialTheme(): Theme {
  return window.matchMedia("(prefers-color-scheme: dark)").matches
    ? "dark"
    : "light";
}

function App() {
  const [theme, setTheme] = useState<Theme>(getInitialTheme);
  const [backendMessage, setBackendMessage] = useState(
    "Checking Rust backend connection...",
  );

  useEffect(() => {
    document.documentElement.dataset.theme = theme;
  }, [theme]);

  useEffect(() => {
    let isMounted = true;

    async function checkBackendConnection() {
      if (!("__TAURI_INTERNALS__" in window)) {
        if (isMounted) {
          setBackendMessage("Frontend preview: Rust backend is unavailable.");
        }

        return;
      }

      try {
        const message = await invoke<string>("get_backend_status");

        if (isMounted) {
          setBackendMessage(message);
        }
      } catch {
        if (isMounted) {
          setBackendMessage("Could not connect to the Rust backend.");
        }
      }
    }

    void checkBackendConnection();

    return () => {
      isMounted = false;
    };
  }, []);

  function toggleTheme() {
    setTheme((currentTheme) => (currentTheme === "dark" ? "light" : "dark"));
  }

  return (
    <main className="app-shell">
      <header className="topbar">
        <a className="brand" href="/" aria-label="ZlijaNote home">
          ZlijaNote
        </a>

        <button
          className="theme-toggle"
          type="button"
          onClick={toggleTheme}
          aria-label={`Switch to ${theme === "dark" ? "light" : "dark"} theme`}
        >
          {theme === "dark" ? "☀ Light" : "☾ Dark"}
        </button>
      </header>

      <section className="welcome-panel" aria-labelledby="welcome-title">
        <p className="eyebrow">Desktop note workspace</p>

        <h1 id="welcome-title">Welcome to ZlijaNote</h1>

        <p className="welcome-copy">
          A local-first workspace for developers to shape ideas, technical
          notes, and visual documents.
        </p>

        <button className="placeholder-action" type="button" disabled>
          Notes are coming soon
        </button>

        <p className="status-message">{backendMessage}</p>
      </section>
    </main>
  );
}

export default App;
