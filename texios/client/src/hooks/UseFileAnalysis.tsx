import {
  useContext,
  createContext,
  useState,
  useEffect,
  useCallback,
  useRef,
} from "react";
import { analyse } from "@/pkg/text_analyser";
// default values
const supportedFileTypes = [
  ".txt",
  ".md",
  ".json",
  ".csv",
  ".js",
  ".py",
  ".ts",
  ".sql",
] as const;

const FileAnalysisContext = createContext<AnalysisContextType | undefined>(
  undefined
);

export function FileAnalysisProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const [file, setFile] = useState<FileType>(null);
  const [isAnalysing, setIsAnalysing] = useState<boolean>(false);
  const [isPlotting, setIsPlotting] = useState<boolean>(false);
  const [isUploading, setIsUploading] = useState<boolean>(false);
  const [isReadingFile, setIsReadingFile] = useState<boolean>(false);
  const [isBusy, setIsBusy] = useState<boolean>(false);
  const [fileContent, setFileContent] = useState<string | null>(null);
  const [error, setError] = useState<ErrorType>(null);
  const [analysisResult, setAnalysisResult] =
    useState<TextAnalysisResult | null>(null);
  const justAnalysedRef = useRef<string | null>(null);

  /**
   * State reset function
   */
  const resetState = useCallback((delay: number = 0) => {
    const timeout = setTimeout(() => {
      setIsAnalysing(false);
      setIsPlotting(false);
      setIsReadingFile(false);
      setFileContent(null);
      setError(null);
      setAnalysisResult(null);
      justAnalysedRef.current = null;
    }, delay);
    return () => clearTimeout(timeout);
  }, []);

  /**
   * File Content Reader (Text only)
   */
  const readFileText = useCallback(async () => {
    if (!file) return;
    setIsReadingFile(true);
    const timeout = setTimeout(() => {
      const fileReader = new FileReader();
      if (
        file.type.startsWith("text") ||
        (!file.type && supportedFileTypes.some((v) => file.name.endsWith(v)))
      ) {
        fileReader.onload = (e) => setFileContent(e.target?.result as string);
        fileReader.readAsText(file);
      } else {
        setFileContent(null);
        setError({ message: "File type not supported" });
        return;
      }
      setIsReadingFile(false);
    }, 1000);
    return () => clearTimeout(timeout);
  }, [file]);

  const handleFileAnalysis = () => {
    if (!fileContent || justAnalysedRef.current == fileContent) {
      return;
    }
    analyseFile(fileContent);
  };

  const analyseFile = useCallback(
    async (text: string): Promise<TextAnalysisResult | null> => {
      setIsAnalysing(true);
      let res: TextAnalysisResult | null = null;

      const timeout = setTimeout(async () => {
        try {
          const result = analyse(text) as WasmResultValue;
          const wFreqs: WordFreqArray = Array.from(
            result.get("word_freqs") ?? new Map()
          ).sort((a, b) => b[1] - a[1]);
          const wPos = (result.get("word_pos") ?? new Map()) as WordPosMap;
          const stats = (result.get("stats") ?? new Map()) as StatsMap;
          res = {
            positions: wPos,
            frequencies: wFreqs,
            stats,
          };
          justAnalysedRef.current = text;
        } catch (e: any) {
          setError({ message: "Analysing file failed" });
          console.error(e);
        } finally {
          setAnalysisResult(res);
          setIsAnalysing(false);
          clearTimeout(timeout);
        }
      }, 2000);
      return res;
    },
    []
  );

  // when file hanges or first load
  useEffect(() => {
    // avoid for empty file
    if (!file) {
      resetState();
      return;
    }
    // Read file text and handle error
    readFileText();
  }, [file]);

  useEffect(() => {}, [fileContent, analysisResult]);

  useEffect(() => {
    setIsBusy(isAnalysing || isPlotting || isUploading || isReadingFile);
  }, [isAnalysing, isPlotting, isUploading, isReadingFile]);

  return (
    <FileAnalysisContext.Provider
      value={{
        isBusy,
        isAnalysing,
        isUploading,
        setIsUploading,
        isPlotting,
        setIsPlotting,
        isReadingFile,
        file,
        setFile,
        supportedFileTypes,
        fileContent,
        analysisResult,
        analyseFile: handleFileAnalysis,
        error,
      }}
    >
      {children}
    </FileAnalysisContext.Provider>
  );
}

export function UseFileAnalysis() {
  const context = useContext(FileAnalysisContext);
  if (!context) {
    throw new Error(
      "UseAnalysisContext must be used within FileAnalysisProvider"
    );
  }
  return context;
}

/*
 *  Types
 */
export type AnalysisContextType = {
  file: FileType;
  setFile: React.Dispatch<React.SetStateAction<FileType>>;
  isBusy: boolean;
  isAnalysing: boolean;
  isPlotting: boolean;
  setIsPlotting: React.Dispatch<React.SetStateAction<boolean>>;
  isUploading: boolean;
  setIsUploading: React.Dispatch<React.SetStateAction<boolean>>;
  isReadingFile: boolean;
  fileContent: string | null;
  supportedFileTypes: typeof supportedFileTypes;
  analysisResult: TextAnalysisResult | null;
  error: ErrorType | null;
  analyseFile: () => void;
};
type ErrorType = { message: string } | null;
export type WasmStatsKey = "word_count" | "char_count";
export type WasmResultKey = "word_pos" | "stats" | "word_freqs";
export type WordFreqMap = Map<string, number>;
export type WordFreqArray = [string, number][];
export type StatsMap = Map<WasmStatsKey, number>;
export type WordPosMap = Map<string, Array<number>>;
export type FileType = File | null;
export type WasmResultValue = Map<
  WasmResultKey,
  WordFreqMap | StatsMap | WordPosMap
>;
export type SupportedFileTypes = (typeof supportedFileTypes)[number];
export type WordFreqPlotDataType = {
  word: string;
  f: number;
}[];
export type TextAnalysisResult = {
  positions: WordPosMap;
  frequencies: WordFreqArray;
  stats: StatsMap;
};
