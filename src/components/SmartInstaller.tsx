import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, Event } from "@tauri-apps/api/event";

interface ProgressEvent {
  step: string;
  percent: number;
  eta_seconds: number;
  message: string;
}

export default function SmartInstaller() {
  const [logs, setLogs] = useState<string[]>([]);
  const [progress, setProgress] = useState<number>(0);
  const [step, setStep] = useState<string>("Initializing...");
  const [eta, setEta] = useState<number>(0);
  const [isComplete, setIsComplete] = useState<boolean>(false);
  const [nodeUrl, setNodeUrl] = useState<string | null>(null);
  const [isRunning, setIsRunning] = useState<boolean>(false);

  useEffect(() => {
    // ✅ Event listeners
    const unlistenProgress = listen<ProgressEvent>(
      "progress-update",
      (e: Event<ProgressEvent>) => {
        const data = e.payload;
        setStep(data.step);
        setProgress(data.percent);
        setEta(data.eta_seconds);
        setLogs((l) => [...l, data.message]);
      }
    );

    const unlistenComplete = listen<ProgressEvent>(
      "progress-complete",
      (e: Event<ProgressEvent>) => {
        const data = e.payload;
        setLogs((l) => [...l, data.message]);
        setProgress(100);
        setIsComplete(true);
        setIsRunning(false);
      }
    );

    const unlistenLogs = listen<string>("install-log", (e: Event<string>) =>
      setLogs((l) => [...l, e.payload])
    );

    const unlistenNode = listen<string>("node-missing", (e: Event<string>) =>
      setNodeUrl(e.payload)
    );

    return () => {
      unlistenProgress.then((f) => f());
      unlistenComplete.then((f) => f());
      unlistenLogs.then((f) => f());
      unlistenNode.then((f) => f());
    };
  }, []);

  // 🚀 Start installation
  const startInstall = async () => {
    if (isRunning) return; // Prevent duplicate starts
    setLogs(["🚀 Starting Smart Installation..."]);
    setIsRunning(true);
    setIsComplete(false);
    setProgress(0);
    try {
      await invoke("start_progress_tracking");
    } catch (err) {
      console.error("Installer error:", err);
      setLogs((l) => [...l, "❌ Installation failed."]);
      setIsRunning(false);
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
    <div className="p-6 text-white bg-gray-900 rounded-2xl max-w-3xl mx-auto">
      <h2 className="text-2xl font-bold mb-4 text-blue-400">
        🚀 Gignaati Smart Installer
      </h2>

      <button
        className={`px-5 py-2 rounded text-white ${
          isRunning
            ? "bg-gray-600 cursor-not-allowed"
            : "bg-blue-600 hover:bg-blue-700"
        }`}
        disabled={isRunning}
        onClick={startInstall}
      >
        {isRunning ? "Installing..." : "Start Smart Installation"}
      </button>

      {nodeUrl && (
        <div className="mt-4">
          <p className="text-red-400 mb-2 font-medium">
            ⚠ Node.js not detected on your system.
          </p>
          <button
            onClick={openNodePage}
            className="bg-green-600 hover:bg-green-700 px-4 py-2 rounded"
          >
            Download Node.js
          </button>
        </div>
      )}

      <div className="mt-6 w-full bg-gray-800 rounded-full h-5">
        <div
          className="bg-green-500 h-5 rounded-full transition-all duration-300 ease-in-out"
          style={{ width: `${progress}%` }}
        />
      </div>
      <p className="text-sm text-gray-400 mt-2">
        {step} — {progress}% complete ({eta}s remaining)
      </p>

      <pre className="mt-4 bg-black/50 p-3 rounded-lg h-64 overflow-y-auto text-xs font-mono whitespace-pre-wrap">
        {logs.length > 0 ? logs.join("\n") : "Waiting for installer logs..."}
      </pre>

      {isComplete && (
        <button
          onClick={launchPlatform}
          className="bg-purple-600 hover:bg-purple-700 px-5 py-2 mt-4 rounded"
        >
          Launch Gignaati Workbench
        </button>
      )}
    </div>
  );
}
