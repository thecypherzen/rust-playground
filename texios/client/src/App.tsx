//import { Button } from "./components/ui/button";
import { CloudUpload, File, X } from "lucide-react";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  //CardFooter,
  CardHeader,
  CardTitle,
} from "./components/ui/card";
import { Button } from "./components/ui/button";
import { Progress } from "./components/ui/progress";
import { useEffect, useState } from "react";

function App() {
  return (
    <div className="m-auto flex h-screen flex-col items-center justify-center p-3">
      <Card className="min-h-3/4 w-full md:w-2/3 lg:w-1/2 shadow-none gap-2">
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
          {/* Upload Action */}
          <div className="flex flex-col gap-3 justify-center items-center border-1 border-dashed border-gray-300 rounded-lg p-6 flex-grow flex-3/4">
            <CloudUpload className="size-12 text-neutral-300" />
            <div className="flex flex-col gap-1 justify-center items-center">
              <p>Choose file or drag and drop it here</p>
              <p className="text-gray-400 text-xs">
                TXT, PDF, MD, DOC DOCX up to&nbsp;<span>50mb</span>
              </p>
            </div>
            <Button variant="outline" size="sm" className="mt-3 cursor-pointer">
              Choose File
            </Button>
          </div>
        </CardContent>
        {/* Uploaded Content */}
        <CardFooter className="flex flex-col gap-1">
          {/* Uploading State */}
          <div className="flex flex-col gap-2 justify-center items-center bg-neutral-100 rounded-lg p-8 w-full">
            {/* header */}
            <div className="flex w-full justify-between items-start mb-2 border-1 border-neutral-500">
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
          {/* Uploaded State */}
          <div className="flex flex-col gap-2 justify-center items-center bg-neutral-100 rounded-lg p-8 w-full"></div>
        </CardFooter>
      </Card>
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

export default App;
