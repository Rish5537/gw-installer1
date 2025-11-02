import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, Event } from "@tauri-apps/api/event";

export default function SmartInstaller() {
  const [logs, setLogs] = useState<string[]>([]);
  const [nodeUrl, setNodeUrl] = useState<string | null>(null);

  useEffect(() => {
    // ✅ Properly typed event listeners (no window.__TAURI__)
    const unlistenLogPromise = listen<string>("install-log", (e: Event<string>) =>
      setLogs((l) => [...l, e.payload])
    );

    const unlistenNodePromise = listen<string>("node-missing", (e: Event<string>) =>
      setNodeUrl(e.payload)
    );

    // cleanup
    return () => {
      unlistenLogPromise.then((unlisten) => unlisten());
      unlistenNodePromise.then((unlisten) => unlisten());
    };
  }, []);

  const startInstall = async () => {
    setLogs([]);
    try {
      await invoke("smart_installer");
    } catch (err) {
      console.error("Installer error:", err);
    }
  };

  const openNodePage = async () => {
    if (!nodeUrl) return;
    try {
      await invoke("tauri_plugin_opener::open", { path: nodeUrl });
    } catch (err) {
      console.error("Failed to open browser:", err);
    }
  };

  const launchPlatform = async () => {
    try {
      await invoke("launch_platform");
    } catch (err) {
      console.error("Failed to launch platform:", err);
    }
  };

  return (
    <div className="p-6 text-white bg-gray-900 rounded-2xl">
      <h2 className="text-xl mb-3 font-bold">🚀 Gignaati Smart Installer</h2>

      <button
        className="bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded"
        onClick={startInstall}
      >
        Start Smart Installation
      </button>

      {nodeUrl && (
        <div className="mt-3">
          <p className="text-red-400 mb-2">⚠ Node.js not detected!</p>
          <button
            onClick={openNodePage}
            className="bg-green-600 hover:bg-green-700 px-3 py-1 rounded"
          >
            Download Node.js
          </button>
        </div>
      )}

      <pre className="mt-4 bg-black/40 p-3 rounded h-64 overflow-y-scroll text-sm">
        {logs.length > 0 ? logs.join("\n") : "Waiting for installer logs..."}
      </pre>

      <button
        onClick={launchPlatform}
        className="bg-purple-600 hover:bg-purple-700 px-4 py-2 mt-3 rounded"
      >
        Launch Gignaati Workbench
      </button>
    </div>
  );
}
