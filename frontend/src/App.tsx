import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import WelcomeScreen from "./components/WelcomeScreen";

interface WorkspaceResponse {
  name: string;
  path: string;
}

export default function App() {
  const [workspace, setWorkspace] = useState<WorkspaceResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  async function handleSelectWorkspace(folderPath: string) {
    setError(null);
    try {
      const res = await invoke<WorkspaceResponse>("open_workspace", {
        request: { workspacePath: folderPath },
      });
      setWorkspace(res);
    } catch (e) {
      setError(String(e));
    }
  }

  if (!workspace) {
    return (
      <div>
        {error && (
          <div className="bg-red-100 text-red-800 text-sm px-4 py-3 text-center font-mono">
            Error: {error}
          </div>
        )}
        <WelcomeScreen onWorkspaceSelect={handleSelectWorkspace} />
      </div>
    );
  }

  return (
    <div className="bg-[#f7f7f5] text-slate-900 min-h-screen p-6 font-sans">
      <div className="max-w-xl mx-auto flex flex-col gap-6">
        <div className="flex items-center gap-4">
          <img
            src="/assets/ZlijaNote-logo/Zlijanote-minimal-logo.svg"
            alt="ZlijaNote logo"
            className="w-12 h-12 rounded-xl shrink-0"
          />
          <h1 className="text-4xl font-black tracking-tight text-black">
            ZlijaNote
          </h1>
        </div>

        <section className="flex flex-col gap-3 font-mono">
          <h2 className="text-xl font-semibold text-black">Active Workspace</h2>
          <p>
            <strong>Name:</strong> {workspace.name}
          </p>
          <p>
            <strong>Path:</strong> {workspace.path}
          </p>
        </section>

        <button
          onClick={() => setWorkspace(null)}
          className="self-start bg-white hover:bg-gray-100 text-black border border-gray-200 px-4 py-2 rounded-lg text-sm font-medium transition-colors cursor-pointer"
        >
          Switch Workspace
        </button>
      </div>
    </div>
  );
}
