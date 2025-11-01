import React from "react";
import "../styles/brand.css";

interface ButtonProps {
  label: string;
  onClick?: () => void;
  variant?: "primary" | "secondary" | "danger";
  disabled?: boolean;
  style?: React.CSSProperties; // ✅ allow inline style overrides
}

export default function Button({
  label,
  onClick,
  variant = "primary",
  disabled = false,
  style = {},
}: ButtonProps) {
  const colorMap = {
    primary: {
      background: "var(--gignaati-primary)",
      hover: "var(--gignaati-secondary)",
    },
    secondary: {
      background: "var(--gignaati-secondary)",
      hover: "var(--gignaati-accent)",
    },
    danger: {
      background: "var(--gignaati-error)",
      hover: "#dc2626",
    },
  };

  const { background, hover } = colorMap[variant];

  return (
    <button
      onClick={onClick}
      disabled={disabled}
      style={{
        background,
        color: "var(--gignaati-white)",
        border: "none",
        borderRadius: "var(--radius-md)",
        padding: "var(--space-3) var(--space-6)",
        cursor: disabled ? "not-allowed" : "pointer",
        opacity: disabled ? 0.6 : 1,
        fontWeight: 500,
        transition: "background 0.3s ease",
        fontFamily: "var(--font-primary)",
        ...style, // ✅ merge custom inline styles safely
      }}
      onMouseEnter={(e) => {
        if (!disabled) (e.target as HTMLButtonElement).style.background = hover;
      }}
      onMouseLeave={(e) => {
        if (!disabled)
          (e.target as HTMLButtonElement).style.background = background;
      }}
    >
      {label}
    </button>
  );
}
