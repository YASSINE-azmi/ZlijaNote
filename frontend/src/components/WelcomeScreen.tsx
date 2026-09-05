import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import CreateWorkspaceModal from "./CreateWorkspace";

interface AppConfig {
  recentWorkspaces: string[];
}

interface WelcomeScreenProps {
  onWorkspaceSelect: (folderPath: string) => void;
}

export default function WelcomeScreen({
  onWorkspaceSelect,
}: WelcomeScreenProps) {
  const [recentWorkspaces, setRecentWorkspaces] = useState<string[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);

  useEffect(() => {
    invoke<AppConfig>("load_app_config")
      .then((config) => {
        setRecentWorkspaces(config.recentWorkspaces);
      })
      .catch((err) => {
        console.error("Failed to load recent workspaces:", err);
      });
  }, []);

  const handleOpenFolder = async () => {
    setError(null);

    try {
      const selectedPath = await open({
        directory: true,
        multiple: false,
        title: "Select Workspace Folder",
      });

      if (typeof selectedPath === "string") {
        onWorkspaceSelect(selectedPath);
      }
    } catch (err) {
      setError(String(err));
    }
  };

  return (
    <div className="relative min-h-screen bg-[#F7F8F6] px-6 py-6 font-sans text-slate-900">
      <button
        className="absolute right-6 top-6 cursor-pointer p-2 text-slate-900 transition-opacity hover:opacity-70"
        aria-label="Settings"
      >
        <img src="/assets/setting_light.svg" alt="" className="h-6 w-6" />
      </button>

      <main className="mx-auto flex w-full max-w-[944px] flex-col items-start pt-0 sm:pt-20 lg:pt-20">
        <div className="w-full">
          <img
            src="/assets/ZlijaNote-logo/Zlijanote-full-logo-nobg.svg"
            alt="ZlijaNote"
            className="h-auto w-full -ml-[60px]"
          />
        </div>

        <section className="-mt-[100px] flex w-full flex-col gap-4">
          <h2 className="text-[26px] font-medium tracking-tight text-[#1b2520]">
            Welcome to ZlijaNote.
          </h2>

          <p className="text-[20px] font-medium leading-relaxed text-[#202923]">
            A local-first workspace for your notes, projects, HTML, and CSS.
          </p>

          {error && (
            <p className="font-mono text-sm text-red-600">Error: {error}</p>
          )}

          <div className="mt-4 flex flex-col items-start gap-5 font-mono">
            <button
              onClick={() => setIsCreateModalOpen(true)}
              className="cursor-pointer rounded-2xl bg-[#087f38] px-4 py-2 font-mono text-[22px] font-bold text-white transition-colors hover:bg-[#0a6331]"
            >
              Create Workspace
            </button>

            <button
              onClick={handleOpenFolder}
              className="cursor-pointer rounded-2xl border border-gray-300 bg-transparent px-4 py-2 font-mono text-[22px] font-bold text-[#202923] transition-colors hover:bg-gray-100"
            >
              Open Workspace
            </button>
          </div>
        </section>

        <section className="mt-12 flex w-full flex-col gap-3 font-mono">
          <h3 className="text-[20px] font-medium text-[#202923]">
            Recent workspaces
          </h3>

          {recentWorkspaces.length === 0 ? (
            <p className="text-[20px] text-[#6d7771]">
              Your recent workspaces will appear here.
            </p>
          ) : (
            <ul className="flex w-full flex-col gap-2">
              {recentWorkspaces.map((path) => (
                <li key={path}>
                  <button
                    onClick={() => onWorkspaceSelect(path)}
                    className="w-full cursor-pointer truncate rounded-lg px-3 py-2 text-left text-lg text-slate-800 transition-colors hover:bg-gray-200/50"
                  >
                    {path}
                  </button>
                </li>
              ))}
            </ul>
          )}
        </section>
      </main>

      {isCreateModalOpen && (
        <CreateWorkspaceModal
          onClose={() => setIsCreateModalOpen(false)}
          onWorkspaceCreated={(workspacePath) => {
            setIsCreateModalOpen(false);
            onWorkspaceSelect(workspacePath);
          }}
        />
      )}
    </div>
  );
}
