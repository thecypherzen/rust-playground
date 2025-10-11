import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import { FileAnalysisProvider } from "./hooks/UseFileAnalysis.tsx";

createRoot(document.getElementById("root")!).render(
  <FileAnalysisProvider>
    <App />
  </FileAnalysisProvider>
);
