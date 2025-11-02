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

  // 🔊 Subscribe to backend events
  useEffect(() => {
    const unP = listen<ComponentProgress>("component-progress", (e: Event<ComponentProgress>) => {
      setComponents((prev) => {
        const copy = { ...prev };
        copy[e.payload.component] = e.payload;
        return copy;
      });
    });

    const unL = listen<ComponentLog>("component-log", (e: Event<ComponentLog>) => {
      setLogs((prev) => [...prev, `[${e.payload.component}] ${e.payload.message}`]);
    });

    const unC = listen<ComponentLog>("smart-complete", (e: Event<ComponentLog>) => {
      setLogs((prev) => [...prev, `[${e.payload.component}] ${e.payload.message}`]);
      setRunning(false);
    });

    const unNode = listen<string>("node-missing", (e) => {
      setNodeDownloadUrl(e.payload || null);
    });

    return () => {
      unP.then((u) => u());
      unL.then((u) => u());
      unC.then((u) => u());
      unNode.then((u) => u());
    };
  }, []);

  // 📊 Aggregate overall progress
  const aggregatedPercent = useMemo(() => {
    const vals = Object.values(components);
    if (vals.length === 0) return 0;
    const sum = vals.reduce((s, v) => s + (v.percent ?? 0), 0);
    return Math.round(sum / vals.length);
  }, [components]);

  // 🚀 Start the smart installer
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

  // 🧹 Cleanup installation
  const cleanup = async () => {
    if (running) return;
    setLogs(["🧹 Starting cleanup..."]);
    setComponents({});
    setRunning(true);
    try {
      await invoke("cleanup_installation");
    } catch (err) {
      setLogs((l) => [...l, `Error during cleanup: ${String(err)}`]);
    } finally {
      setRunning(false);
    }
  };

  // 🔧 Repair existing installation (same simulation as installer)
  const repair = async () => {
    if (running) return;
    setLogs(["🔧 Attempting repair of existing setup..."]);
    setComponents({});
    setRunning(true);
    try {
      await invoke("smart_installer");
    } catch (err) {
      setLogs((l) => [...l, `Repair failed: ${String(err)}`]);
    } finally {
      setRunning(false);
    }
  };

  // 🌐 Open Node.js download page if missing
  const openNodeDownload = async () => {
    if (!nodeDownloadUrl) return;
    try {
      await invoke("open", { path: nodeDownloadUrl }).catch(() => {});
    } catch {
      // ignore
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

      {/* ---- Overall progress ---- */}
      <div style={{ marginTop: 18 }}>
        <div style={{ height: 14, background: "#eee", borderRadius: 8 }}>
          <div
            style={{
              width: `${aggregatedPercent}%`,
              height: "100%",
              background:
                "linear-gradient(90deg,var(--gignaati-secondary),var(--gignaati-primary))",
              borderRadius: 8,
              transition: "width 200ms linear",
            }}
          />
        </div>
        <div style={{ marginTop: 6, fontSize: 13, color: "#333" }}>
          Total progress: {aggregatedPercent}% — components:{" "}
          {Object.keys(components).length || "0"}
        </div>
      </div>

      {/* ---- Per-component breakdown ---- */}
      <div style={{ marginTop: 12 }}>
        {Object.values(components).map((c) => (
          <div
            key={c.component}
            style={{
              marginBottom: 10,
              border: "1px solid #eee",
              padding: 8,
              borderRadius: 8,
            }}
          >
            <div
              style={{
                display: "flex",
                justifyContent: "space-between",
                alignItems: "center",
              }}
            >
              <strong>{c.component}</strong>
              <span style={{ fontSize: 12 }}>
                {c.status === "running"
                  ? "⏳"
                  : c.status === "done"
                  ? "✅"
                  : "⚠"}{" "}
                {c.percent}%
              </span>
            </div>
            <div style={{ fontSize: 12, color: "#555", marginTop: 6 }}>
              {c.message}
            </div>
            {typeof c.eta_seconds === "number" && c.eta_seconds !== null && (
              <div style={{ fontSize: 12, color: "#777", marginTop: 4 }}>
                ETA: {c.eta_seconds}s
              </div>
            )}
          </div>
        ))}
      </div>

      {/* ---- Log Console ---- */}
      <div style={{ marginTop: 12 }}>
        <pre
          style={{
            background: "#0b0b0b",
            color: "#dfffd8",
            padding: 12,
            height: 220,
            overflowY: "auto",
            borderRadius: 8,
          }}
        >
          {logs.length > 0 ? logs.join("\n") : "Waiting for logs..."}
        </pre>
      </div>
    </div>
  );
}
