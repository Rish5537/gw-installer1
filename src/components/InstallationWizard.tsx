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

  useEffect(() => {
    // Attach event listeners only once
    const unlistenFns: (() => void)[] = [];

    (async () => {
      unlistenFns.push(
        await listen("install-log", (event) => {
          setLogs((prev) => [...prev, event.payload as string]);
        })
      );

      unlistenFns.push(
        await listen("install-progress", (event) => {
          const data = event.payload as { progress: number };
          setProgress(data.progress);
        })
      );

      unlistenFns.push(
        await listen("install-complete", () => {
          setProgress(100);
          setLogs((prev) => [...prev, "✅ Installation completed!"]);
          setTimeout(() => setStep("success"), 1000);
        })
      );
    })();

    return () => {
      unlistenFns.forEach((un) => un());
    };
  }, []);

  const next = () => {
    if (step === "welcome") setStep("system");
    else if (step === "system") startInstallation();
  };

  const startInstallation = async () => {
    setStep("install");
    setLogs(["Starting installation..."]);
    setProgress(0);
    await invoke("run_installation");
  };

  return (
    <div style={{ textAlign: "center" }}>
      {step === "welcome" && <WelcomeScreen onNext={next} />}

      {step === "system" && (
        <div>
          <Heading level={2}>System Check</Heading>
          <SystemCheck />
          <div style={{ marginTop: "var(--space-6)" }}>
            <Button label="Proceed to Install" onClick={next} />
          </div>
        </div>
      )}

      {step === "install" && (
        <div>
          <Heading level={2}>Installing Components</Heading>
          <ProgressBar progress={progress} />
          <LogViewer logs={logs} />
        </div>
      )}

      {step === "success" && (
        <SuccessScreen onRestart={() => setStep("welcome")} />
      )}
    </div>
  );
}
