import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, Event } from "@tauri-apps/api/event";

type ComponentProgress = {
  component: string;
  percent: number;
  status: "running" | "failed" | "done" | string;
  message: string;
  eta_seconds?: number | null;
};

type ComponentLog = {
  component: string;
  message: string;
};

export default function SmartInstaller() {
  const [running, setRunning] = useState(false);
  const [components, setComponents] = useState<Record<string, ComponentProgress>>({});
  const [logs, setLogs] = useState<string[]>([]);
  const [nodeDownloadUrl, setNodeDownloadUrl] = useState<string | null>(null);
  const [ollamaWaiting, setOllamaWaiting] = useState(false);
  const [checkingOllama, setCheckingOllama] = useState(false);

  // 🧠 Ollama control states
  const [ollamaRunning, setOllamaRunning] = useState(false);
  const [availableModels, setAvailableModels] = useState<string[]>([]);
  const [selectedModel, setSelectedModel] = useState("llama3");
  const [pullingModel, setPullingModel] = useState(false);
  const [lastFailedModel, setLastFailedModel] = useState<string | null>(null);

  // 🧩 Progress tracking
  const [modelProgress, setModelProgress] = useState<number | null>(null);
  const [modelStatus, setModelStatus] = useState<string>("Idle");
  const [downloadActive, setDownloadActive] = useState(false);

  // 🕘 Recent models (persistent)
  const [recentModels, setRecentModels] = useState<string[]>(() => {
    const stored = localStorage.getItem("recent_models");
    return stored ? JSON.parse(stored) : [];
  });
  useEffect(() => {
    localStorage.setItem("recent_models", JSON.stringify(recentModels));
  }, [recentModels]);

  // 🔊 Subscribe to backend logs
  useEffect(() => {
    const unP = listen<ComponentProgress>("component-progress", (e: Event<ComponentProgress>) => {
      setComponents((prev) => ({ ...prev, [e.payload.component]: e.payload }));
    });

    const unL = listen<ComponentLog>("component-log", (e: Event<ComponentLog>) => {
      const { component, message } = e.payload;
      const fullMsg = `[${component}] ${message}`;
      setLogs((prev) => [...prev, fullMsg]);

      // 🎯 Smarter progress updates
      if (component.includes("Ollama Model Pull")) {
        const match = message.match(/(\d+)%/);
        if (match) {
          const pct = Number(match[1]);
          setModelProgress((prev) => (prev !== null && pct < prev ? prev : pct));
          setModelStatus(`📦 Downloading: ${pct}%`);
          setDownloadActive(true);
        }

        if (message.includes("✅ Finished pulling")) {
          setModelProgress(100);
          setModelStatus("✅ Model pulled successfully!");
          setDownloadActive(false);

          setRecentModels((prev) => {
            const updated = [selectedModel, ...prev.filter((m) => m !== selectedModel)];
            return updated.slice(0, 5);
          });

          fetchModels();
          setTimeout(() => setModelProgress(null), 4000);
        }

        if (message.includes("❌") || message.includes("cancelled")) {
          setModelStatus("❌ Download cancelled or failed.");
          setDownloadActive(false);
          setTimeout(() => setModelProgress(null), 4000);
        }
      }

      // Detect failed pulls for repair button
      if (message.includes("❌ Model pull failed")) {
        setLastFailedModel(selectedModel);
      }

      // Handle manual installation prompt
      if (
        message.includes("Ollama not found") ||
        message.includes("download Ollama manually") ||
        message.includes("waiting for manual installation")
      ) {
        setOllamaWaiting(true);
      }

      if (message.includes("✅ Ollama detected") || message.includes("✅ Already installed")) {
        setOllamaWaiting(false);
      }
    });

    const unC = listen<ComponentLog>("smart-complete", (e: Event<ComponentLog>) => {
      setLogs((prev) => [...prev, `[${e.payload.component}] ${e.payload.message}`]);
      setRunning(false);
    });

    const unNode = listen<string>("node-missing", (e) => setNodeDownloadUrl(e.payload || null));

    return () => {
      unP.then((u) => u());
      unL.then((u) => u());
      unC.then((u) => u());
      unNode.then((u) => u());
    };
  }, [selectedModel]);

  // 📊 Aggregate progress
  const aggregatedPercent = useMemo(() => {
    const vals = Object.values(components);
    if (!vals.length) return 0;
    const sum = vals.reduce((s, v) => s + (v.percent ?? 0), 0);
    return Math.round(sum / vals.length);
  }, [components]);

  // 🚀 Smart Installer controls
  const start = async () => {
    if (running) return;
    setLogs([]);
    setComponents({});
    setRunning(true);
    try {
      await invoke("smart_installer");
    } catch (err) {
      setLogs((l) => [...l, `Error starting installer: ${String(err)}`]);
      setRunning(false);
    }
  };

  const cleanup = async () => {
    if (running) return;
    setLogs(["🧹 Starting cleanup..."]);
    setRunning(true);
    try {
      await invoke("cleanup_installation");
    } catch (err) {
      setLogs((l) => [...l, `Error during cleanup: ${String(err)}`]);
    } finally {
      setRunning(false);
    }
  };

  const repair = async () => {
    if (running) return;
    setLogs(["🔧 Attempting repair of existing setup..."]);
    setRunning(true);
    try {
      await invoke("smart_installer");
    } catch (err) {
      setLogs((l) => [...l, `Repair failed: ${String(err)}`]);
    } finally {
      setRunning(false);
    }
  };

  // 🌐 Node.js download link
  const openNodeDownload = async () => {
    if (!nodeDownloadUrl) return;
    try {
      await invoke("open", { path: nodeDownloadUrl });
    } catch {}
  };

  // --- 🧠 Ollama runtime control ---
  const startOllama = async () => {
    try {
      setLogs((prev) => [...prev, "[Ollama] 🚀 Starting Ollama server..."]);
      await invoke("start_ollama_server");
      setOllamaRunning(true);
      setLogs((prev) => [...prev, "[Ollama] ✅ Server started successfully"]);
    } catch (err) {
      setLogs((prev) => [...prev, `[Ollama] ❌ Start failed: ${String(err)}`]);
    }
  };

  const stopOllama = async () => {
    try {
      setLogs((prev) => [...prev, "[Ollama] 🛑 Stopping Ollama server..."]);
      await invoke("stop_ollama_server");
      setOllamaRunning(false);
      setLogs((prev) => [...prev, "[Ollama] ✅ Server stopped"]);
    } catch (err) {
      setLogs((prev) => [...prev, `[Ollama] ❌ Stop failed: ${String(err)}`]);
    }
  };

  const fetchModels = async () => {
    try {
      const models: string[] = await invoke("list_ollama_models");
      setAvailableModels(models);
      setLogs((prev) => [...prev, `[Ollama] 📦 Available models:\n${models.join(", ")}`]);
    } catch (err) {
      setLogs((prev) => [...prev, `[Ollama] ❌ Failed to list models: ${String(err)}`]);
    }
  };

  const pullModel = async (name?: string) => {
    const modelName = name || selectedModel;
    try {
      setPullingModel(true);
      setLastFailedModel(null);
      setDownloadActive(true);
      setModelProgress(0);
      setModelStatus(`⬇ Pulling '${modelName}'...`);
      await invoke("pull_ollama_model", { modelName });
    } catch (err) {
      setLogs((prev) => [...prev, `[Ollama] ❌ Pull failed: ${String(err)}`]);
      setLastFailedModel(modelName);
      setDownloadActive(false);
    } finally {
      setPullingModel(false);
    }
  };

  const cancelModelPull = async () => {
    try {
      await invoke("cancel_ollama_download");
      setLogs((prev) => [...prev, "[Ollama] ⏹ Download cancelled by user."]);
      setDownloadActive(false);
      setModelStatus("⏹ Download cancelled");
    } catch (err) {
      setLogs((prev) => [...prev, `[Ollama] ❌ Cancel failed: ${String(err)}`]);
    }
  };

  const removeModel = async (name?: string) => {
    const modelToRemove = name || selectedModel;
    try {
      await invoke("remove_ollama_model", { modelName: modelToRemove });
      setLogs((prev) => [...prev, `[Ollama] 🗑 Model '${modelToRemove}' removed.`]);
      setRecentModels((prev) => prev.filter((m) => m !== modelToRemove));
      fetchModels();
    } catch (err) {
      setLogs((prev) => [...prev, `[Ollama] ❌ Remove failed: ${String(err)}`]);
    }
  };

  const repairModel = async () => {
    if (!lastFailedModel) return;
    try {
      setLogs((prev) => [...prev, `[Ollama] 🔄 Repairing model '${lastFailedModel}'...`]);
      await invoke("repair_ollama_model", { modelName: lastFailedModel });
      setLogs((prev) => [...prev, `[Ollama] ✅ Repair started.`]);
    } catch (err) {
      setLogs((prev) => [...prev, `[Ollama] ❌ Repair failed: ${String(err)}`]);
    }
  };

  return (
    <div className="p-6 bg-white rounded-md" style={{ maxWidth: 900, margin: "0 auto" }}>
      <h2 style={{ color: "var(--gignaati-primary)" }}>🚀 Smart Installer</h2>

      {/* ---- Controls ---- */}
      <div style={{ marginTop: 12, display: "flex", gap: 10, flexWrap: "wrap" }}>
        <button
          onClick={start}
          disabled={running}
          style={{
            padding: "8px 14px",
            background: running ? "gray" : "var(--gignaati-primary)",
            color: "white",
            borderRadius: 8,
            border: "none",
            cursor: running ? "not-allowed" : "pointer",
          }}
        >
          {running ? "Installing…" : "Start Smart Installation"}
        </button>

        <button
          onClick={cleanup}
          disabled={running}
          style={{
            padding: "8px 14px",
            background: "crimson",
            color: "white",
            borderRadius: 8,
            border: "none",
            cursor: running ? "not-allowed" : "pointer",
          }}
        >
          🧹 Cleanup Installation
        </button>

        <button
          onClick={repair}
          disabled={running}
          style={{
            padding: "8px 14px",
            background: "darkorange",
            color: "white",
            borderRadius: 8,
            border: "none",
            cursor: running ? "not-allowed" : "pointer",
          }}
        >
          🔧 Repair Installation
        </button>

        {nodeDownloadUrl && (
          <button
            onClick={openNodeDownload}
            style={{
              padding: "8px 14px",
              background: "var(--gignaati-secondary)",
              color: "white",
              borderRadius: 8,
              border: "none",
            }}
          >
            Download Node.js
          </button>
        )}
      </div>

      {/* ---- Ollama Runtime Panel ---- */}
      <div style={{ marginTop: 20, borderTop: "1px solid #eee", paddingTop: 14 }}>
        <h3 style={{ color: "var(--gignaati-primary)" }}>🧠 Ollama Local Models</h3>

        <div style={{ display: "flex", gap: 10, flexWrap: "wrap", marginTop: 8 }}>
          <button onClick={startOllama} disabled={ollamaRunning || running}>🚀 Start Ollama</button>
          <button onClick={stopOllama} disabled={!ollamaRunning || running}>🛑 Stop Ollama</button>
          <button onClick={fetchModels} disabled={!ollamaRunning}>📦 List Models</button>
        </div>

        {/* Model pull section */}
        <div style={{ marginTop: 10, display: "flex", gap: 10, alignItems: "center" }}>
          <input
            value={selectedModel}
            onChange={(e) => setSelectedModel(e.target.value)}
            placeholder="Enter model name (e.g. llama3)"
            style={{
              flex: 1,
              padding: "6px 10px",
              borderRadius: 6,
              border: "1px solid #ddd",
            }}
          />
          <button
            onClick={() => pullModel()}
            disabled={!ollamaRunning || pullingModel || downloadActive}
            style={{
              padding: "8px 14px",
              background: pullingModel ? "gray" : "var(--gignaati-primary)",
              color: "white",
              borderRadius: 8,
              border: "none",
            }}
          >
            {pullingModel ? "Pulling..." : "⬇ Pull Model"}
          </button>

          {downloadActive && (
            <button
              onClick={cancelModelPull}
              style={{
                padding: "8px 14px",
                background: "#b52e31",
                color: "white",
                borderRadius: 8,
                border: "none",
              }}
            >
              ⏹ Cancel
            </button>
          )}
        </div>

        {/* Progress bar */}
        {modelProgress !== null && (
          <div style={{ marginTop: 14 }}>
            <div style={{ display: "flex", justifyContent: "space-between" }}>
              <span>{modelStatus}</span>
              <span>{modelProgress}%</span>
            </div>
            <div
              style={{
                width: "100%",
                height: 10,
                background: "#eee",
                borderRadius: 5,
                overflow: "hidden",
                marginTop: 5,
              }}
            >
              <div
                style={{
                  width: `${modelProgress}%`,
                  height: "100%",
                  background:
                    modelProgress === 100 ? "#27ae60" : "var(--gignaati-primary)",
                  transition: "width 0.3s ease",
                }}
              />
            </div>
          </div>
        )}

        {/* Recent Models */}
        {recentModels.length > 0 && (
          <div style={{ marginTop: 18 }}>
            <strong>🕘 Recent Models:</strong>
            <ul style={{ marginTop: 8, paddingLeft: 20 }}>
              {recentModels.map((m) => (
                <li
                  key={m}
                  style={{
                    display: "flex",
                    justifyContent: "space-between",
                    alignItems: "center",
                    fontSize: 13,
                    marginBottom: 4,
                  }}
                >
                  <span>{m}</span>
                  <div style={{ display: "flex", gap: 6 }}>
                    <button
                      onClick={() => pullModel(m)}
                      style={{
                        padding: "3px 8px",
                        background: "#2980b9",
                        color: "white",
                        borderRadius: 4,
                        border: "none",
                        fontSize: 12,
                      }}
                    >
                      🔁 Re-Pull
                    </button>
                    <button
                      onClick={() => removeModel(m)}
                      style={{
                        padding: "3px 8px",
                        background: "#e74c3c",
                        color: "white",
                        borderRadius: 4,
                        border: "none",
                        fontSize: 12,
                      }}
                    >
                      🗑 Delete
                    </button>
                  </div>
                </li>
              ))}
            </ul>
          </div>
        )}

        {availableModels.length > 0 && (
          <div style={{ marginTop: 10, fontSize: 13 }}>
            <strong>Installed Models:</strong> {availableModels.join(", ")}
          </div>
        )}
      </div>

      {/* ---- Log console ---- */}
      <div style={{ marginTop: 12 }}>
        <pre
          style={{
            background: "#0b0b0b",
            color: "#dfffd8",
            padding: 12,
            height: 220,
            overflowY: "auto",
            borderRadius: 8,
            whiteSpace: "pre-wrap",
          }}
        >
          {logs.length > 0
            ? logs.map((line, i) => (
                <div
                  key={i}
                  style={{
                    color: line.includes("✅")
                      ? "#00ff9c"
                      : line.includes("⚠")
                      ? "#ffb84d"
                      : line.includes("❌")
                      ? "#ff6666"
                      : "#dfffd8",
                  }}
                >
                  {line}
                </div>
              ))
            : "Waiting for logs..."}
        </pre>
      </div>
    </div>
  );
}
