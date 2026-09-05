import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { attachConsole } from "@tauri-apps/plugin-log";
import App from "./App";
import "./index.css";

attachConsole();

createRoot(document.getElementById("root") as HTMLElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
