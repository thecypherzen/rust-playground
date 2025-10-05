import { Button } from "./components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
} from "./components/ui/card";

function App() {
  return (
    <Card>
      <CardHeader>
        <CardDescription className="scr-only"> Upload a file</CardDescription>
      </CardHeader>
      <CardContent></CardContent>
      <CardFooter>
        <Button variant="ghost" className="cursor-pointer">
          {" "}
          Upload{" "}
        </Button>
      </CardFooter>
    </Card>
  );
}

export default App;
