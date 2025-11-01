import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useState, useEffect } from "react";
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

  const next = () => {
    if (step === "welcome") setStep("system");
    else if (step === "system") setStep("install");
  };

  const startInstallation = async () => {
    setStep("install");
    setLogs(["Starting installation..."]);
    setProgress(0);

    await invoke("run_installation");

    listen("install-log", (event) => {
      setLogs((prev) => [...prev, event.payload as string]);
    });

    listen("install-progress", (event) => {
      const data = event.payload as { progress: number };
      setProgress(data.progress);
    });

    listen("install-complete", () => {
      setProgress(100);
      setLogs((prev) => [...prev, "✅ Installation completed!"]);
      setTimeout(() => setStep("success"), 1000);
    });
  };

  return (
    <div className="p-8 text-center">
      {step === "welcome" && <WelcomeScreen onNext={next} />}
      {step === "system" && (
        <div>
          <SystemCheck />
          <button
            className="mt-6 px-6 py-2 bg-green-600 text-white rounded-lg"
            onClick={startInstallation}
          >
            Start Installation
          </button>
        </div>
      )}
      {step === "install" && (
        <div>
          <h2 className="text-xl font-semibold mb-4">Installing components…</h2>
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
