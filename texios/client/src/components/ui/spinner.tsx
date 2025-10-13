import { Loader2Icon } from "lucide-react";

import { cn } from "@/lib/utils";

function Spinner({
  className,
  wrap,
  ...props
}: React.ComponentProps<"svg"> & { wrap?: "screen" | "parent" }) {
  const e = (
    <Loader2Icon
      role="status"
      aria-label="Loading"
      className={cn("size-4 animate-spin", className)}
      {...props}
    />
  );
  return wrap ? (
    <div
      className={cn(
        "w-full flex flex-col items-center justify-center",
        wrap === "screen" ? "h-screen" : "h-full"
      )}
    >
      {e}
    </div>
  ) : (
    e
  );
}

export { Spinner };
