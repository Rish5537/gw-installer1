import { ReactNode } from "react";
import "../styles/brand.css";

interface LayoutProps {
  title?: string;
  children: ReactNode;
}

export default function Layout({ title, children }: LayoutProps) {
  return (
    <div
      style={{
        minHeight: "100vh",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        background: "var(--gignaati-gradient-secondary)",
        fontFamily: "var(--font-primary)",
      }}
    >
      <div
        style={{
          background: "var(--gignaati-white)",
          borderRadius: "var(--radius-lg)",
          boxShadow: "var(--shadow-lg)",
          padding: "var(--space-8)",
          width: "90%",
          maxWidth: "850px",
          minHeight: "500px",
        }}
      >
        {title && (
          <h1
            style={{
              color: "var(--gignaati-primary)",
              textAlign: "center",
              fontSize: "var(--text-3xl)",
              marginBottom: "var(--space-6)",
            }}
          >
            {title}
          </h1>
        )}

        <div>{children}</div>
      </div>
    </div>
  );
}
