import React, { ReactNode } from "react";
import "../styles/brand.css";

interface HeadingProps {
  level?: 1 | 2 | 3 | 4;
  children: ReactNode;
  align?: "left" | "center" | "right";
  color?: "primary" | "secondary" | "dark" | "light";
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

  return (
    <Tag
      style={{
        color: colorMap[color],
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

interface TextProps {
  children: ReactNode;
  variant?: "body" | "muted" | "success" | "error" | "warning";
  align?: "left" | "center" | "right";
}

export function Text({
  children,
  variant = "body",
  align = "left",
}: TextProps) {
  const colorMap = {
    body: "var(--gignaati-dark)",
    muted: "var(--gignaati-medium)",
    success: "var(--gignaati-success)",
    error: "var(--gignaati-error)",
    warning: "var(--gignaati-warning)",
  };

  return (
    <p
      style={{
        color: colorMap[variant],
        fontSize: "var(--text-base)",
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
