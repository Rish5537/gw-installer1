import React from "react";

interface ProgressBarProps {
  progress: number;
}

export default function ProgressBar({ progress }: ProgressBarProps) {
  return (
    <div
      style={{
        width: "100%",
        background: "var(--gignaati-light)",
        borderRadius: "var(--radius-md)",
        boxShadow: "var(--shadow-sm)",
        overflow: "hidden",
        height: "16px",
        marginBottom: "var(--space-4)",
      }}
    >
      <div
        style={{
          width: `${progress}%`,
          background: "var(--gignaati-gradient-primary)",
          height: "100%",
          transition: "width 0.3s ease",
        }}
      />
    </div>
  );
}
