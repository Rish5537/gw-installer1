import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Heading, Text } from "./Typography";
import Button from "./Button";
import "../styles/brand.css";

interface ValidationResult {
  passed: boolean;
  issues: string[];
  warnings: string[];
  os: string;
  ram_gb: number;
  disk_gb: number;
}

interface PortConfig {
  n8n_port: number;
  ollama_port: number;
}

export default function SystemCheck() {
  const [status, setStatus] = useState<ValidationResult | null>(null);
  const [ports, setPorts] = useState<PortConfig | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // ✅ Automatically allocate ports when component mounts
  useEffect(() => {
    invoke<PortConfig>("allocate_ports")
      .then((result) => {
        console.log("Port Allocation Result:", result);
        setPorts(result);
      })
      .catch((err) => {
        console.error("Port allocation failed:", err);
        setError("Failed to allocate ports. Please retry system check.");
      });
  }, []);

  // ✅ Run Rust system validation command
  const runCheck = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<ValidationResult>("validate_requirements", {
        minRam: 8,
        minDisk: 20,
      });
      setStatus(result);
    } catch (err) {
      console.error(err);
      setError("System check failed. Please try again.");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div
      style={{
        background: "var(--gignaati-white)",
        border: "1px solid var(--gignaati-light)",
        borderRadius: "var(--radius-lg)",
        boxShadow: "var(--shadow-md)",
        padding: "var(--space-6)",
        textAlign: "center",
        fontFamily: "var(--font-primary)",
      }}
    >
      <Heading level={2} color="primary">
        System Validation
      </Heading>

      {/* Info message */}
      {!status && !loading && (
        <Text variant="muted" align="center">
          Click below to verify your system meets requirements.
        </Text>
      )}

      {loading && (
        <Text variant="info" align="center">
          Running validation... Please wait.
        </Text>
      )}

      {/* Run system check button */}
      <div style={{ marginTop: "var(--space-6)" }}>
        <Button
          label="Run System Check"
          onClick={runCheck}
          disabled={loading}
        />
      </div>

      {/* Error display */}
      {error && (
        <Text variant="error" align="center">
          {error}
        </Text>
      )}

      {/* ✅ Port Allocation Section */}
      {ports && (
        <div
          style={{
            marginTop: "var(--space-6)",
            textAlign: "left",
            padding: "var(--space-4)",
            background: "var(--gignaati-light)",
            borderRadius: "var(--radius-md)",
          }}
        >
          <Heading level={3} color="secondary">
            Port Allocation
          </Heading>
          <Text variant="success">✅ n8n Port: {ports.n8n_port}</Text>
          <Text variant="success">✅ Ollama Port: {ports.ollama_port}</Text>
        </div>
      )}

      {/* ✅ Validation Results */}
      {status && (
        <div
          style={{
            marginTop: "var(--space-6)",
            textAlign: "left",
            padding: "var(--space-4)",
            background: "var(--gignaati-light)",
            borderRadius: "var(--radius-md)",
          }}
        >
          <Text>
            <strong>OS:</strong> {status.os}
          </Text>
          <Text>
            <strong>RAM:</strong> {status.ram_gb} GB
          </Text>
          <Text>
            <strong>Disk:</strong> {status.disk_gb} GB
          </Text>

          {status.issues.length > 0 && (
            <div style={{ marginTop: "var(--space-4)" }}>
              <Heading level={4} color="error">
                Issues
              </Heading>
              {status.issues.map((issue, idx) => (
                <Text key={idx} variant="error">
                  ✗ {issue}
                </Text>
              ))}
            </div>
          )}

          {status.warnings.length > 0 && (
            <div style={{ marginTop: "var(--space-4)" }}>
              <Heading level={4} color="warning">
                Warnings
              </Heading>
              {status.warnings.map((warn, idx) => (
                <Text key={idx} variant="warning">
                  ⚠ {warn}
                </Text>
              ))}
            </div>
          )}

          {status.passed && status.issues.length === 0 && (
            <div style={{ marginTop: "var(--space-4)" }}>
              <Heading level={4} color="success">
                ✓ System meets all requirements.
              </Heading>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
