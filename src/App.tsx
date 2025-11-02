import "./styles/brand.css";
import "./App.css";
import Layout from "./components/Layout";
import InstallationWizard from "./components/InstallationWizard";
import SmartInstaller from "./components/SmartInstaller";
import { useState } from "react";

function App() {
  const [useSmart, setUseSmart] = useState(false);

  return (
    <Layout title="Gignaati Workbench Installer">
      {useSmart ? (
        <SmartInstaller />
      ) : (
        <div className="flex flex-col items-center text-center">
          <InstallationWizard />
          <button
            onClick={() => setUseSmart(true)}
            className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 mt-6 rounded shadow-md"
          >
            🚀 Switch to Smart Installer
          </button>
        </div>
      )}
    </Layout>
  );
}

export default App;
