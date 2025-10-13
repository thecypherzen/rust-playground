import { CloudUpload, File } from "lucide-react";
import { Button } from "./ui/button";
import { useEffect, useRef, useState } from "react";
import { Spinner } from "./ui/spinner";
import { supportedFileTypes, UseFileAnalysis } from "@/hooks/UseFileAnalysis";

const fileTypes = supportedFileTypes.join(", ");

export function FileSelect() {
  const { file, setFile, analyseFile, isAnalysing, isPlotting } =
    UseFileAnalysis();
  const inputRef = useRef<HTMLInputElement>(null);
  const [isProcessing, setIsProcessing] = useState<boolean>(false);

  useEffect(() => {
    setIsProcessing(isAnalysing || isPlotting);
  }, [file, isAnalysing, isPlotting]);

  return (
    <div className="flex flex-col gap-3 justify-center items-center border-1 border-dashed border-neutral-400 rounded-lg py-6 flex-grow flex-1">
      {!file && <Empty />}
      {/* File Preview */}
      {file && (
        <div className="flex flex-col gap-2 justify-center items-center bg-gray-100/70 rounded-lg py-5 px-8 w-9/10">
          <div className="size-10 rounded-md bg-white shadow-md shadow-gray-200 flex flex-col items-center justify-center p-2">
            <File />
          </div>
          <h4 className="text-center text-wrap">
            <span className="font-medium">{`${file.name}`}</span>
            <span className="text-sm">
              &nbsp;{` - ${getFileSize(file.size)}`}
            </span>
          </h4>
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
      <div className="flex gap-5 justify-center items-center flex-wrap">
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
    <div className="flex flex-col gap-3 justify-center items-center w-full">
      <CloudUpload className="size-12 text-neutral-300" />
      <div className="flex flex-col gap-1 justify-center items-center text-center sm:text-left w-full">
        <p className="w-full">Choose file or drag and drop it here</p>
        <p className="text-gray-400 text-xs md:text-sm w-full whitespace-break-spaces">
          <span className="uppercase">{`${fileTypes}`}</span>&nbsp;up to&nbsp;
          <span>20mb</span>
        </p>
      </div>
    </div>
  );
}

function getFileSize(bytes: number) {
  const units: Record<number, "KB" | "MB" | "GB"> = {
    1000: "KB",
    1000000: "MB",
    1000000000: "GB",
  };

  let div = 1000;
  while (bytes / div > 1000) {
    div *= 1000;
  }

  const value = Math.round((bytes / div) * 100) / 100;
  return `${value} ${units[div]}`;
}
