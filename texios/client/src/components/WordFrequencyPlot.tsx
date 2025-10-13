import { UseFileAnalysis } from "@/hooks/UseFileAnalysis";
import { useEffect, useRef, useState } from "react";
import {
  Bar,
  BarChart,
  Label,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { Spinner } from "./ui/spinner";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { cn } from "@/lib/utils";

export default function WordFrequencyPlot({
  chartType = "bar",
  plotLimit = 7,
}: WordFPlotPropsType) {
  const { analysisResult, isPlotting, setIsPlotting, isAnalysing, file } =
    UseFileAnalysis();
  const [plotData, setPlotData] = useState<WordFPlotDataType | null>(null);
  const plottedDataRef = useRef<WordFPlotDataType | null>(null);

  useEffect(() => {
    if (!analysisResult) {
      if (plotData) {
        setPlotData(null);
      }
      if (isPlotting) {
        setIsPlotting(false);
      }
    } else {
      if (
        plottedDataRef.current &&
        plotData &&
        plottedDataRef.current == plotData
      ) {
        return;
      }
      setIsPlotting(true);
      const timeout = setTimeout(() => {
        const array: WordFPlotDataType = analysisResult?.frequencies
          .slice(0, plotLimit)
          .map(([word, f]) => {
            return { word, f };
          });
        setPlotData(array);
        plottedDataRef.current = array;
        setIsPlotting(false);
        clearTimeout(timeout);
      }, 2000);
    }
  }, [analysisResult, plotData]);

  useEffect(() => {}, [isPlotting, isAnalysing, file]);

  return (
    <Card
      className={cn(
        "w-full rounded-xl pt-0",
        (!plotData || isPlotting || isAnalysing) &&
          "bg-transparent shadow-none border-none"
      )}
    >
      <CardHeader>
        <CardTitle className="sr-only">{`Word frequency ${chartType} chart for ${file?.name}`}</CardTitle>
      </CardHeader>
      <CardContent className="w-full flex flex-col items-center justify-center">
        {!plotData || isPlotting || isAnalysing ? (
          <div className="flex gap-2 items-center">
            <Spinner />
            <p>Hold on a sec</p>
          </div>
        ) : (
          <div className="w-full flex flex-col items-center justify-center">
            <h5 className="text-gray-400 font-semibold text-lg capitalize">
              {`word frequency ${chartType} chart`}
            </h5>
            <ResponsiveContainer className="p-4" height={310}>
              {chartType === "bar" ? (
                <BarChart
                  data={plotData}
                  margin={{ bottom: 18 }}
                  className="p-2"
                >
                  <XAxis
                    dataKey={"word"}
                    label={{
                      value: "Words",
                      position: "insideBottom",
                      offset: -18,
                    }}
                    className="text-sm"
                  />
                  <YAxis className="text-sm">
                    <Label
                      value="Count"
                      angle={-90}
                      offset={10}
                      position="insideLeft"
                      style={{ textAnchor: "middle", fill: "#777" }}
                    />
                  </YAxis>
                  <Tooltip cursor={{ fill: "rgba(0,0,0,0.05)" }} />
                  <Bar dataKey={"f"} fill="#0f52ba" />
                </BarChart>
              ) : (
                <></>
              )}
            </ResponsiveContainer>
          </div>
        )}
      </CardContent>
    </Card>
  );
}

type WordFPlotPropsType = {
  chartType?: string;
  plotLimit?: number;
};

type WordFPlotDataType = {
  word: string;
  f: number;
}[];
