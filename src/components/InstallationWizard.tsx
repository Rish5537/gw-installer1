import { useState } from "react";
import WelcomeScreen from "./WelcomeScreen";
import SystemCheck from "./SystemCheck";
import ProgressBar from "./ProgressBar";
import LogViewer from "./LogViewer";
import SuccessScreen from "./SuccessScreen";

export type WizardStep = "welcome" | "system" | "install" | "success";

export default function InstallationWizard() {
  const [step, setStep] = useState<WizardStep>("welcome");

  const next = () => {
    if (step === "welcome") setStep("system");
    else if (step === "system") setStep("install");
    else if (step === "install") setStep("success");
  };

  const back = () => {
    if (step === "success") setStep("install");
    else if (step === "install") setStep("system");
    else if (step === "system") setStep("welcome");
  };

  return (
    <div className="p-8 text-center">
      {step === "welcome" && <WelcomeScreen onNext={next} />}
      {step === "system" && <SystemCheck />}
      {step === "install" && (
        <div>
          <h2 className="text-xl font-semibold mb-4">Installing components …</h2>
          <ProgressBar progress={65} />
          <LogViewer />
          <button className="mt-4 px-4 py-2 bg-blue-600 text-white rounded-lg"
                  onClick={next}>Simulate Success</button>
        </div>
      )}
      {step === "success" && <SuccessScreen onRestart={() => setStep("welcome")} />}

      <div className="mt-6 flex justify-center gap-4">
        {step !== "welcome" && step !== "success" && (
          <button onClick={back}
                  className="px-4 py-2 bg-gray-300 rounded-lg">Back</button>
        )}
        {step !== "success" && (
          <button onClick={next}
                  className="px-4 py-2 bg-green-600 text-white rounded-lg">Next</button>
        )}
      </div>
    </div>
  );
}
