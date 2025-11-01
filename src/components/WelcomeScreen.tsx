interface Props {
    onNext: () => void;
  }
  
  export default function WelcomeScreen({ onNext }: Props) {
    return (
      <div>
        <h1 className="text-3xl font-bold mb-4 text-blue-700">
          Welcome to Gignaati Workbench Installer
        </h1>
        <p className="text-gray-600 mb-6">
          This wizard will guide you through the setup process.
        </p>
        <button
          onClick={onNext}
          className="px-6 py-2 bg-blue-600 text-white rounded-lg shadow-md hover:bg-blue-700 transition"
        >
          Begin Setup
        </button>
      </div>
    );
  }
  