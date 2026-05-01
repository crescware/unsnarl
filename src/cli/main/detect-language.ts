export function detectLanguage(
  path: string | null,
  fallback: "ts" | "tsx" | "js" | "jsx",
): "ts" | "tsx" | "js" | "jsx" {
  if (path === null) {
    return fallback;
  }
  if (path.endsWith(".tsx")) {
    return "tsx";
  }
  if (path.endsWith(".jsx")) {
    return "jsx";
  }
  if (path.endsWith(".js")) {
    return "js";
  }
  return "ts";
}
