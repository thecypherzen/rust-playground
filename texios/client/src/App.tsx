//import { Button } from "./components/ui/button";
import { lazy, Suspense } from "react";
import { CloudUpload, EllipsisVertical, File, X } from "lucide-react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "./components/ui/card";
import { Button } from "@/components/ui/button";
import { Progress } from "@/components/ui/progress";
import { useEffect, useState } from "react";
import { FileSelect } from "./components/FileSelect";
const WordFrequencyPlot = lazy(() => import("./components/WordFrequencyPlot"));
import { UseFileAnalysis } from "./hooks/UseFileAnalysis";
import { cn } from "./lib/utils";

function App() {
  const { analysisResult } = UseFileAnalysis();
  useEffect(() => {}, [analysisResult]);

  return (
    <div
      className={cn(
        "m-auto flex min-h-screen lg:h-screen flex-col lg:flex-row items-center justify-center lg:items-start p-3 w-full md:max-w-4/5  gap-6 lg:max-w-[1400px]"
      )}
    >
      <Card className="w-full lg:flex-2/5 2xl:flex-1/2 shadow-none gap-2 flex-grow lg:max-w-[600px]">
        <CardHeader className="border-b-1 border-gray-200 flex gap-3 items-center pt-2 pb-3">
          <div className="rounded-full h-full w-auto aspect-square p-2 flex flex-col items-center justify-center border-1 border-neutral-200">
            <CloudUpload />
          </div>
          <div>
            <CardTitle className="text-xl font-medium">Texios</CardTitle>
            <CardDescription className="text-gray-400 text-md font-light">
              Select or upload file to analyse
            </CardDescription>
          </div>
        </CardHeader>
        <CardContent className="flex flex-col gap-4 border-0 border-neutral-300 flex-1 pt-5">
          <FileSelect />
        </CardContent>
      </Card>
      {/* Graphs */}
      {analysisResult && (
        <div className="w-full lg:flex-3/5 2xl:flex-1/2 flex flex-col min-h-3/5 max-h-full items-center lg:items-end flex-grow p-2 md:p-5 bg-gray-100 rounded-2xl">
          <h3 className="font-semibold text-2xl self-center p-3">Your Plots</h3>
          <div className="w-full overflow-y-scroll flex flex-col items-center lg:items-end flex-grow gap-5 p-2 md:p-5">
            <Suspense fallback={<></>}>
              <WordFrequencyPlot chartType="bar" />
            </Suspense>
          </div>
        </div>
      )}
    </div>
  );
}

function ProgressBar() {
  const [progress, setProgress] = useState(10);
  useEffect(() => {
    const interval = setInterval(() => {
      setProgress((oldProgress) => {
        if (oldProgress === 80) {
          clearInterval(interval);
          return 90;
        }
        const diff = Math.random() * 10;
        return Math.min(oldProgress + diff, 80);
      });
    }, 500);
    return () => {
      clearInterval(interval);
    };
  }, []);
  return <Progress value={progress} className="" />;
}

export function UplodedState() {
  return (
    <div className="flex flex-col gap-2 justify-center items-center bg-gray-100 rounded-lg p-4 w-full">
      {/* header */}
      <div className="flex w-full justify-between items-start mb-2 border-0 border-neutral-500">
        <div className="flex gap-2 items-center">
          {/* Icon */}
          <File size={24} />
          <p className="font-semibold text-sm">Filename.ext</p>
        </div>
        {/* close Btn */}
        <div className="p-1 rounded-full hover:border-1 hover:border-gray-500 cursor-pointer transition-all duration-100 ease-in-out">
          <EllipsisVertical size={16} />
        </div>
      </div>
      <div className="flex gap-5 justify-between w-full items-start">
        <p className="text-xs text-gray-500 flex-grow flex-2/3">
          Click analyse to begin processing your file or download it using
          the&nbsp;
          <span className="font-bold bg-gray-400 p-0 text-white py-[1px] rounded-xs">
            ︙
          </span>
          &nbsp;icon above.
        </p>
        <div className="flex flex-1/3 flex-col items-end justify-start">
          <Button
            variant={"default"}
            className="cursor-pointer shadow-sm shadow-gray-200 !bg-white !text-foreground hover:scale-105"
          >
            Analyse
          </Button>
        </div>
      </div>
    </div>
  );
}

export function UploadingState() {
  return (
    <div className="flex flex-col gap-2 justify-center items-center bg-gray-100 rounded-lg p-4 w-full">
      {/* header */}
      <div className="flex w-full justify-between items-start mb-1 border-0 border-neutral-500">
        <div className="flex gap-3 items-center">
          {/* Icon */}
          <div className="size-10 rounded-md bg-white shadow-md shadow-gray-200 flex flex-col items-center justify-center p-2">
            <File />
          </div>
          {/* file info */}
          <div className="flex flex-col justify-center">
            <p className="font-medium text-sm">Filename.ext</p>
            <p className="font-normal text-xs text-gray-400">3 MB</p>
          </div>
        </div>
        {/* close Btn */}
        <div className="p-1 rounded-full hover:border-1 hover:border-gray-500 cursor-pointer transition-all duration-100 ease-in-out">
          <X size={16} />
        </div>
      </div>
      <ProgressBar />
    </div>
  );
}

export type TextAnalysisRes = {
  word_pos: WordPosMap;
  word_freqs: WordFreqArray;
  stats: StatsMap;
};

type WasmStatsKey = "word_count" | "char_count";

type WordFreqArray = [string, number][];
type StatsMap = Map<WasmStatsKey, number>;
type WordPosMap = Map<string, Array<number>>;

export default App;
