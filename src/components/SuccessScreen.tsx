interface Props {
    onRestart: () => void;
  }
  
  export default function SuccessScreen({ onRestart }: Props) {
    return (
      <div className="text-center">
        <h2 className="text-2xl font-semibold text-green-600 mb-4">
          ✅ Installation Completed Successfully!
        </h2>
        <p className="text-gray-600 mb-6">
          Your Gignaati Workbench environment is ready to use.
        </p>
        <button
          onClick={onRestart}
          className="px-6 py-2 bg-blue-600 text-white rounded-lg shadow-md hover:bg-blue-700 transition"
        >
          Restart Wizard
        </button>
      </div>
    );
  }
  