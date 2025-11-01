import React, { ReactNode } from "react";
import "../styles/brand.css";

/* ============================= */
/* 🔤 HEADING COMPONENT           */
/* ============================= */

interface HeadingProps {
  level?: 1 | 2 | 3 | 4;
  children: ReactNode;
  align?: "left" | "center" | "right";
  color?: "primary" | "secondary" | "dark" | "light" | string; // ✅ now supports CSS variables
}

export function Heading({
  level = 1,
  children,
  align = "center",
  color = "primary",
}: HeadingProps) {
  const Tag = `h${level}` as keyof React.JSX.IntrinsicElements;

  const colorMap = {
    primary: "var(--gignaati-primary)",
    secondary: "var(--gignaati-secondary)",
    dark: "var(--gignaati-dark)",
    light: "var(--gignaati-light)",
  };

  const sizeMap = {
    1: "var(--text-3xl)",
    2: "var(--text-2xl)",
    3: "var(--text-xl)",
    4: "var(--text-lg)",
  };

  const resolvedColor =
    typeof color === "string" && color.startsWith("var(")
      ? color // ✅ allow direct CSS variables like var(--gignaati-primary)
      : colorMap[color as keyof typeof colorMap] ?? "var(--gignaati-dark)";

  return (
    <Tag
      style={{
        color: resolvedColor,
        fontSize: sizeMap[level],
        fontFamily: "var(--font-primary)",
        textAlign: align,
        marginBottom: "var(--space-4)",
      }}
    >
      {children}
    </Tag>
  );
}

/* ============================= */
/* 📝 TEXT COMPONENT              */
/* ============================= */

interface TextProps {
  children: ReactNode;
  variant?: "body" | "muted" | "success" | "error" | "warning";
  align?: "left" | "center" | "right";
  size?: "xs" | "sm" | "base" | "lg";
  color?: string; // ✅ added to support custom CSS variable colors
}

export function Text({
  children,
  variant = "body",
  align = "left",
  size = "base",
  color,
}: TextProps) {
  const colorMap = {
    body: "var(--gignaati-dark)",
    muted: "var(--gignaati-medium)",
    success: "var(--gignaati-success)",
    error: "var(--gignaati-error)",
    warning: "var(--gignaati-warning)",
  };

  const sizeMap = {
    xs: "var(--text-xs)",
    sm: "var(--text-sm)",
    base: "var(--text-base)",
    lg: "var(--text-lg)",
  };

  return (
    <p
      style={{
        color: color ?? colorMap[variant],
        fontSize: sizeMap[size],
        fontFamily: "var(--font-primary)",
        textAlign: align,
        lineHeight: 1.6,
        marginBottom: "var(--space-3)",
      }}
    >
      {children}
    </p>
  );
}
