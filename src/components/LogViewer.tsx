import { useEffect, useState } from "react";

export default function LogViewer() {
  const [logs, setLogs] = useState<string[]>([]);

  useEffect(() => {
    const interval = setInterval(() => {
      setLogs((prev) => [...prev, `Installing package ${prev.length + 1}...`]);
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="bg-black text-green-400 p-3 mt-4 rounded-lg h-40 overflow-y-auto text-left text-sm font-mono">
      {logs.map((log, i) => (
        <div key={i}>{log}</div>
      ))}
    </div>
  );
}
