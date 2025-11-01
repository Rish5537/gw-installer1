interface Props {
  logs: string[];
}

export default function LogViewer({ logs }: Props) {
  return (
    <div className="bg-black text-green-400 p-3 mt-4 rounded-lg h-40 overflow-y-auto text-left text-sm font-mono">
      {logs.length === 0 ? (
        <div className="text-gray-400 italic">Waiting for installation logs...</div>
      ) : (
        logs.map((log, i) => (
          <div key={i}>{log}</div>
        ))
      )}
    </div>
  );
}
