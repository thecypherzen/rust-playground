import { lazy, Suspense } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
const App = lazy(() => import("./App.tsx"));
import { FileAnalysisProvider } from "./hooks/UseFileAnalysis.tsx";
import { Spinner } from "./components/ui/spinner.tsx";

createRoot(document.getElementById("root")!).render(
  <FileAnalysisProvider>
    <Suspense
      fallback={
        <Spinner
          className="size-12 text-neutral-400"
          strokeWidth={1.2}
          wrap="screen"
        />
      }
    >
      <App />
    </Suspense>
  </FileAnalysisProvider>
);
