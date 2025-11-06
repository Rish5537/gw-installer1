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

  // 🧩 Ollama metadata from backend
  const [ollamaDetails, setOllamaDetails] = useState<{
    port?: number;
    version?: string;
    model?: string;
  } | null>(null);

  // 🧩 Progress tracking
  const [modelProgress, setModelProgress] = useState<number | null>(null);
  const [modelStatus, setModelStatus] = useState<string>("Idle");
  const [downloadActive, setDownloadActive] = useState(false);

  // ⚙️ Agentic Platform states
  const [n8nStatus, setN8nStatus] = useState<string>("Unknown");
  const [isLaunchingN8n, setIsLaunchingN8n] = useState(false);
  const [isCheckingN8n, setIsCheckingN8n] = useState(false);
  const [isStoppingN8n, setIsStoppingN8n] = useState(false);

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

      // Ollama progress tracking
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

      if (message.includes("❌ Model pull failed")) {
        setLastFailedModel(selectedModel);
      }

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

      // Check for AJV error and suggest a fix
      if (message.includes("Cannot find module 'ajv/dist/core'")) {
        setLogs((prev) => [
          ...prev,
          "[n8n] ⚠ Detected missing 'ajv/dist/core'. Try reinstalling n8n with 'npm install -g n8n@latest --omit=optional'.",
        ]);
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

  // 🧩 Auto-check Ollama status on mount
  useEffect(() => {
    const checkOllama = async () => {
      try {
        const status: string = await invoke("get_ollama_status");
        setOllamaRunning(true);
        setLogs(prev => [...prev, `[Ollama] ${status}`]);

        const details = await invoke<{
          ollama_port?: number;
          ollama_version?: string;
          ollama_default_model?: string;
        }>("get_ollama_details");

        setOllamaDetails({
          port: details.ollama_port,
          version: details.ollama_version,
          model: details.ollama_default_model,
        });
      } catch {
        setOllamaRunning(false);
        setLogs(prev => [...prev, "[Ollama] ❌ Not running"]);
      }
    };

    checkOllama();
  }, []);

  // 📊 Aggregate progress
  const aggregatedPercent = useMemo(() => {
    const vals = Object.values(components);
    if (!vals.length) return 0;
    const sum = vals.reduce((s, v) => s + (v.percent ?? 0), 0);
    return Math.round(sum / vals.length);
  }, [components]);

  // 🚀 Smart Installer core controls
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

  // --- ⚙️ Agentic Platform controls ---
  const launchAgenticPlatform = async () => {
    try {
      setIsLaunchingN8n(true);
      setLogs((prev) => [...prev, "[n8n] 🚀 Launching Agentic Platform..."]);
      await invoke("launch_agentic_platform");
      setLogs((prev) => [...prev, "[n8n] ✅ Agentic Platform launched successfully"]);
    } catch (err) {
      setLogs((prev) => [...prev, `[n8n] ❌ Launch failed: ${String(err)}`]);
    } finally {
      setIsLaunchingN8n(false);
    }
  };

  const checkN8nHealth = async () => {
    try {
      setIsCheckingN8n(true);
      const result: string = await invoke("check_n8n_health");
      setN8nStatus(result);
      setLogs((prev) => [...prev, `[n8n] ${result}`]);
    } catch (err) {
      setN8nStatus("❌ n8n Unreachable");
      setLogs((prev) => [...prev, `[n8n] ❌ Health check failed: ${String(err)}`]);
    } finally {
      setIsCheckingN8n(false);
    }
  };

  const stopN8n = async () => {
    try {
      setIsStoppingN8n(true);
      await invoke("stop_n8n");
      setLogs((prev) => [...prev, "[n8n] 🛑 Stopped successfully"]);
    } catch (err) {
      setLogs((prev) => [...prev, `[n8n] ⚠ Stop failed: ${String(err)}`]);
    } finally {
      setIsStoppingN8n(false);
    }
  };

  return (
    <div className="p-6 bg-white rounded-md" style={{ maxWidth: 900, margin: "0 auto" }}>
      <h2 style={{ color: "var(--gignaati-primary)" }}>🚀 Smart Installer</h2>

      {/* ---- Core Installer Controls ---- */}
      <div style={{ marginTop: 12, display: "flex", gap: 10, flexWrap: "wrap" }}>
        <button onClick={start} disabled={running} style={btnStyle("var(--gignaati-primary)", running)}>
          {running ? "Installing…" : "Start Smart Installation"}
        </button>

        <button onClick={cleanup} disabled={running} style={btnStyle("crimson", running)}>
          🧹 Cleanup Installation
        </button>

        <button onClick={repair} disabled={running} style={btnStyle("darkorange", running)}>
          🔧 Repair Installation
        </button>

        {nodeDownloadUrl && (
          <button onClick={openNodeDownload} style={btnStyle("var(--gignaati-secondary)", false)}>
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

        {/* ✅ Show live Ollama info */}
        {ollamaDetails && (
          <div style={{ marginTop: 8, fontSize: 14, color: "gray" }}>
            <strong>🧩 Ollama Info:</strong>{" "}
            Port {ollamaDetails.port ?? "—"} | Version {ollamaDetails.version ?? "—"} | Model{" "}
            {ollamaDetails.model ?? "—"}
          </div>
        )}

        {/* === Agentic Platform Management Section === */}
        <div style={{ marginTop: 28, borderTop: "1px solid #ddd", paddingTop: 16 }}>
          <h3 style={{ color: "var(--gignaati-primary)" }}>⚙️ Agentic Platform (n8n + Ollama)</h3>

          <div style={{ display: "flex", gap: 10, flexWrap: "wrap", marginTop: 10 }}>
            <button onClick={launchAgenticPlatform} disabled={isLaunchingN8n}>
              {isLaunchingN8n ? "Launching..." : "🚀 Launch Agentic Platform"}
            </button>
            <button onClick={checkN8nHealth} disabled={isCheckingN8n}>
              {isCheckingN8n ? "Checking..." : "🔍 Check n8n Health"}
            </button>
            <button onClick={stopN8n} disabled={isStoppingN8n}>
              {isStoppingN8n ? "Stopping..." : "🛑 Stop n8n"}
            </button>
          </div>

          <div style={{ marginTop: 8, fontSize: 14 }}>
            <strong>Status:</strong>{" "}
            <span
              style={{
                color: n8nStatus.includes("✅")
                  ? "green"
                  : n8nStatus.includes("❌")
                  ? "red"
                  : "gray",
              }}
            >
              {n8nStatus}
            </span>
          </div>
        </div>
      </div>

      {/* ---- Log console ---- */}
      <div style={{ marginTop: 12 }}>
        <pre
          style={{
            background: "#0b0b0b",
            color: "#dfffd8",
            padding: 12,
            height: 250,
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

// 🔧 Helper: button style
const btnStyle = (color: string, disabled: boolean) => ({
  padding: "8px 14px",
  background: disabled ? "gray" : color,
  color: "white",
  borderRadius: 8,
  border: "none",
  cursor: disabled ? "not-allowed" : "pointer",
});
