import { CloudUpload, File } from "lucide-react";
import { Button } from "./ui/button";
import { useEffect, useRef, useState } from "react";
import { Spinner } from "./ui/spinner";
import { UseFileAnalysis } from "@/hooks/UseFileAnalysis";

export function FileSelect() {
  const { file, setFile, fileContent, analyseFile, isAnalysing, isPlotting } =
    UseFileAnalysis();
  const inputRef = useRef<HTMLInputElement>(null);
  const fileTypes = ".txt,.md,.json,.csv,.js,.py,.ts,.sql";
  const [isProcessing, setIsProcessing] = useState<boolean>(false);

  useEffect(() => {
    setIsProcessing(isAnalysing || isPlotting);
  }, [file, isAnalysing, isPlotting, fileContent]);

  return (
    <div className="flex flex-col gap-3 justify-center items-center border-1 border-dashed border-neutral-400 rounded-lg p-6 flex-grow flex-3/4">
      {!file && <Empty />}
      {/* File Preview */}
      {file && (
        <div className="flex flex-col gap-2 justify-center items-center bg-gray-100/70 rounded-lg py-5 px-8 min-w-3/5 max-w-full">
          <div className="size-10 rounded-md bg-white shadow-md shadow-gray-200 flex flex-col items-center justify-center p-2">
            <File />
          </div>
          <h4>{file.name}</h4>
          {fileContent && <p>{fileContent.size}</p>}
        </div>
      )}
      <input
        ref={inputRef}
        type="file"
        className="border-1 border-neutral-100 p-5 hidden"
        accept={fileTypes}
        onChange={(e) => {
          let files = (e.target as HTMLInputElement).files;
          if (files && files.length > 0) {
            setFile(files[0]);
          } else {
            setFile(null);
          }
        }}
      />
      <div className="flex gap-5 justify-center items-center">
        <Button
          variant="outline"
          size="sm"
          className="mt-3 cursor-pointer"
          onClick={() => {
            if (inputRef.current) {
              inputRef.current.click();
            }
          }}
        >
          Select File
        </Button>
        {file && (
          <Button
            size="sm"
            className="mt-3 cursor-pointer flex items-center gap-1"
            onClick={analyseFile}
            disabled={isProcessing}
          >
            {isProcessing && <Spinner className="size-4" />}
            {isProcessing ? "Analysing" : "Analyse"}
          </Button>
        )}
      </div>
    </div>
  );
}

function Empty() {
  return (
    <div className="flex flex-col gap-3 justify-center items-center">
      <CloudUpload className="size-12 text-neutral-300" />
      <div className="flex flex-col gap-1 justify-center items-center">
        <p>Choose file or drag and drop it here</p>
        <p className="text-gray-400 text-xs">
          TXT, PDF, MD, DOC DOCX up to&nbsp;<span>50mb</span>
        </p>
      </div>
    </div>
  );
}
