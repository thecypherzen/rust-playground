//import { Button } from "./components/ui/button";
import { CloudUpload, Upload } from "lucide-react";
import {
  Card,
  CardContent,
  CardDescription,
  //CardFooter,
  CardHeader,
  CardTitle,
} from "./components/ui/card";

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
        <CardContent></CardContent>
      </Card>
    </div>
  );
}

export default App;
