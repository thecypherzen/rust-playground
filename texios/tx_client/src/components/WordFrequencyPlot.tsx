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

export function WordFrequencyPlot({
  chartType = "bar",
  plotLimit = 7,
}: WordFPlotPropsType) {
  const { analysisResult, isPlotting, setIsPlotting, isAnalysing } =
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

  useEffect(() => {}, [isPlotting, isAnalysing]);

  if (!plotData) {
    return <></>;
  }
  return (
    <div className="w-full min-h-[300px] border-1 border-neutral-200 p-5 bg-gray-100 rounded-xl overflow-auto flex flex-col items-center justify-center">
      {isPlotting || isAnalysing ? (
        <Spinner />
      ) : (
        <ResponsiveContainer className="p-4">
          {chartType === "bar" ? (
            <BarChart data={plotData} margin={{ bottom: 18 }} className="p-2">
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
              <Tooltip />
              <Bar dataKey={"f"} fill="#0f52ba" />
            </BarChart>
          ) : (
            <></>
          )}
        </ResponsiveContainer>
      )}
    </div>
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
