import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useState, useEffect } from "react";
import { Heading, Text } from "./Typography";
import Button from "./Button";
import WelcomeScreen from "./WelcomeScreen";
import SystemCheck from "./SystemCheck";
import ProgressBar from "./ProgressBar";
import LogViewer from "./LogViewer";
import SuccessScreen from "./SuccessScreen";

export type WizardStep = "welcome" | "system" | "install" | "success";

export default function InstallationWizard() {
  const [step, setStep] = useState<WizardStep>("welcome");
  const [logs, setLogs] = useState<string[]>([]);
  const [progress, setProgress] = useState<number>(0);

  // ✅ Attach & clean event listeners safely
  useEffect(() => {
    let unlistenFns: (() => void)[] = [];

    (async () => {
      unlistenFns = [
        await listen("install-log", (event) => {
          setLogs((prev) => [...prev, event.payload as string]);
        }),

        await listen("install-progress", (event) => {
          const data = event.payload as { progress: number };
          setProgress(data.progress);
        }),

        await listen("install-complete", () => {
          setProgress(100);
          setLogs((prev) => [...prev, "✅ Installation completed successfully!"]);
          setTimeout(() => setStep("success"), 1000);
        }),
      ];
    })();

    return () => {
      unlistenFns.forEach((un) => un());
    };
  }, []);

  // ✅ Step navigation logic
  const next = () => {
    if (step === "welcome") setStep("system");
    else if (step === "system") startInstallation();
  };

  // ✅ Trigger Rust-side installation logic
  const startInstallation = async () => {
    setStep("install");
    setLogs(["🚀 Starting installation..."]);
    setProgress(0);
    await invoke("run_installation");
  };

  // ✅ Common wrapper styles
  const containerStyle: React.CSSProperties = {
    textAlign: "center",
    fontFamily: "var(--font-primary)",
    color: "var(--gignaati-dark)",
    padding: "var(--space-6)",
  };

  return (
    <div style={containerStyle}>
      {step === "welcome" && <WelcomeScreen onNext={next} />}

      {step === "system" && (
        <div
          style={{
            background: "var(--gignaati-white)",
            borderRadius: "var(--radius-lg)",
            boxShadow: "var(--shadow-md)",
            padding: "var(--space-6)",
          }}
        >
          <Heading level={2} color="primary">
            System Check
          </Heading>
          <Text variant="muted">
            We’re verifying your system meets all minimum requirements.
          </Text>

          <div style={{ marginTop: "var(--space-6)" }}>
            <SystemCheck />
            <div style={{ marginTop: "var(--space-6)" }}>
              <Button label="Proceed to Installation" onClick={next} />
            </div>
          </div>
        </div>
      )}

      {step === "install" && (
        <div>
          <Heading level={2} color="primary">
            Installing Components
          </Heading>
          <Text variant="muted">
            Please wait while we set everything up.
          </Text>
          <div style={{ marginTop: "var(--space-6)" }}>
            <ProgressBar progress={progress} />
            <LogViewer logs={logs} />
          </div>
        </div>
      )}

      {step === "success" && (
        <SuccessScreen onRestart={() => setStep("welcome")} />
      )}
    </div>
  );
}
