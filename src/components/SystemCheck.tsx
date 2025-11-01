import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface SystemInfo {
  node?: string;
  git?: string;
  python?: string;
  os: string;
}

export default function SystemCheck() {
  const [info, setInfo] = useState<SystemInfo | null>(null);

  useEffect(() => {
    invoke<SystemInfo>("detect_system")
      .then(setInfo)
      .catch(console.error);
  }, []);

  if (!info) return <div>Detecting system configuration...</div>;

  return (
    <div className="p-6 text-gray-800">
      <h2 className="text-xl font-semibold mb-4">System Environment</h2>
      <ul className="space-y-2">
        <li>🧩 OS: {info.os}</li>
        <li>📦 Node.js: {info.node || "Not Found"}</li>
        <li>🔧 Git: {info.git || "Not Found"}</li>
        <li>🐍 Python: {info.python || "Not Found"}</li>
      </ul>
    </div>
  );
}
