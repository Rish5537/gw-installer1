import "./styles/brand.css"; // 🎨 Brand color system
import "./App.css"; // Optional custom overrides
import Layout from "./components/Layout"; // 🧱 Unified window layout
import InstallationWizard from "./components/InstallationWizard"; // ⚙️ Main installer logic

function App() {
  return (
    <Layout title="Gignaati Workbench Installer">
      <InstallationWizard />
    </Layout>
  );
}

export default App;
