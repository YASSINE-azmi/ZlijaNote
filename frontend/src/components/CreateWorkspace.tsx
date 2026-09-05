import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

interface WorkspaceResponse {
  name: string;
  path: string;
}

interface CreateWorkspaceModalProps {
  onClose: () => void;
  onWorkspaceCreated: (workspacePath: string) => void;
}

export default function CreateWorkspaceModal({
  onClose,
  onWorkspaceCreated,
}: CreateWorkspaceModalProps) {
  const [workspaceName, setWorkspaceName] = useState("");
  const [parentPath, setParentPath] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape" && !busy) {
        onClose();
      }
    };

    window.addEventListener("keydown", handleKeyDown);

    return () => {
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [busy, onClose]);

  const handleBrowse = async () => {
    setError(null);

    try {
      const selectedPath = await open({
        directory: true,
        multiple: false,
        title: "Select Workspace Location",
      });

      if (typeof selectedPath === "string") {
        setParentPath(selectedPath);
      }
    } catch (err) {
      setError(String(err));
    }
  };

  const handleCreate = async () => {
    const name = workspaceName.trim();
    const location = parentPath.trim();

    if (!name) {
      setError("Please enter a workspace name.");
      return;
    }

    if (!location) {
      setError("Please select a location.");
      return;
    }

    setBusy(true);
    setError(null);

    try {
      const workspace = await invoke<WorkspaceResponse>("create_workspace", {
        request: {
          name,
          parentPath: location,
        },
      });

      onWorkspaceCreated(workspace.path);
      onClose();
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/20 p-5 backdrop-blur-[1px]"
      role="presentation"
      onMouseDown={(event) => {
        if (event.target === event.currentTarget && !busy) {
          onClose();
        }
      }}
    >
      <section
        role="dialog"
        aria-modal="true"
        aria-labelledby="create-workspace-title"
        className="w-full max-w-[590px] rounded-2xl border border-[#d9dfda] bg-[#f8faf7] p-5 shadow-2xl"
      >
        <header className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="flex h-9 w-9 items-center justify-center rounded bg-[#087f38] text-xl text-[#f8f7ed]">
              ✦
            </div>

            <h2
              id="create-workspace-title"
              className="font-mono text-[21px] font-bold text-[#1d2821]"
            >
              Create Workspace
            </h2>
          </div>

          <button
            type="button"
            onClick={onClose}
            disabled={busy}
            aria-label="Close create workspace dialog"
            className="cursor-pointer rounded p-1 text-[30px] leading-none text-[#315b45] transition-colors hover:bg-[#e7eee8] hover:text-[#173b2a] disabled:cursor-not-allowed disabled:opacity-50"
          >
            ×
          </button>
        </header>

        <p className="mt-7 max-w-[510px] font-mono text-[16px] leading-relaxed text-[#1d2821]">
          Set up a local folder for your projects, notes, assets, and styles.
        </p>

        <div className="mt-7 flex flex-col gap-2">
          <label
            htmlFor="workspace-name"
            className="font-mono text-[16px] font-medium text-[#1d2821]"
          >
            Workspace name
          </label>

          <input
            id="workspace-name"
            autoFocus
            value={workspaceName}
            onChange={(event) => setWorkspaceName(event.target.value)}
            onKeyDown={(event) => {
              if (event.key === "Enter" && !busy) {
                handleCreate();
              }
            }}
            placeholder="Personal Notes"
            className="h-8 w-full rounded-lg border border-[#cbd3cd] bg-white px-4 font-mono text-[16px] text-[#1d2821] outline-none placeholder:text-[#a6ada8] focus:border-[#087f38] focus:ring-2 focus:ring-[#087f38]/15"
          />
        </div>

        <div className="mt-3 flex flex-col gap-2">
          <label
            htmlFor="workspace-location"
            className="font-mono text-[16px] font-medium text-[#1d2821]"
          >
            Location
          </label>

          <div className="flex gap-2">
            <input
              id="workspace-location"
              value={parentPath}
              readOnly
              placeholder="Select a folder..."
              className="h-9 min-w-0 flex-1 rounded-lg border border-[#cbd3cd] bg-white px-4 font-mono text-[16px] text-[#536159] outline-none placeholder:text-[#a6ada8]"
            />

            <button
              type="button"
              onClick={handleBrowse}
              disabled={busy}
              className="h-12 shrink-0 cursor-pointer rounded-xl border border-[#cbd3cd] bg-white px-4 font-mono text-[16px] text-[#1d2821] transition-colors hover:bg-[#edf1ed] disabled:cursor-not-allowed disabled:opacity-50"
            >
              Browse...
            </button>
          </div>
        </div>

        {error && (
          <p className="mt-3 font-mono text-sm text-red-600">Error: {error}</p>
        )}

        <footer className="mt-14 flex items-center justify-between">
          <button
            type="button"
            onClick={onClose}
            disabled={busy}
            className="cursor-pointer rounded-lg bg-[#d9d9d9] px-5 py-1.5 font-mono text-[16px] font-bold text-[#172019] transition-colors hover:bg-[#c9cec9] disabled:cursor-not-allowed disabled:opacity-50"
          >
            Cancel
          </button>

          <button
            type="button"
            onClick={handleCreate}
            disabled={busy || !workspaceName.trim() || !parentPath.trim()}
            className="cursor-pointer rounded-xl bg-[#087f38] px-6 py-2 font-mono text-[16px] font-bold text-white transition-colors hover:bg-[#066b2f] disabled:cursor-not-allowed disabled:opacity-50"
          >
            {busy ? "Creating..." : "Create Workspace"}
          </button>
        </footer>
      </section>
    </div>
  );
}
