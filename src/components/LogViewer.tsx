import React from "react";

interface LogViewerProps {
  logs: string[];
}

export default function LogViewer({ logs }: LogViewerProps) {
  return (
    <div
      style={{
        background: "var(--gignaati-dark)",
        color: "var(--gignaati-light)",
        fontFamily: "var(--font-primary)",
        borderRadius: "var(--radius-md)",
        padding: "var(--space-4)",
        height: "200px",
        overflowY: "auto",
        textAlign: "left",
        fontSize: "var(--text-sm)",
        boxShadow: "var(--shadow-md)",
      }}
    >
      {logs.length === 0 ? (
        <div style={{ color: "var(--gignaati-medium)" }}>No logs yet...</div>
      ) : (
        logs.map((log, i) => {
          const color =
            log.includes("error") || log.includes("failed")
              ? "var(--gignaati-error)"
              : log.includes("success") || log.includes("completed")
              ? "var(--gignaati-success)"
              : "var(--gignaati-info)";
          return (
            <div key={i} style={{ color, marginBottom: "4px" }}>
              {log}
            </div>
          );
        })
      )}
    </div>
  );
}
