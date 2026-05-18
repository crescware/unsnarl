const PLUGIN_PREFIX = "unsnarl-plugin-";

export function collectPlugins(
  value: string,
  prev: readonly string[],
): readonly string[] {
  const names = value
    .split(",")
    .map((v) => v.trim())
    .filter((v) => v.length > 0)
    .map((v) =>
      v.startsWith(PLUGIN_PREFIX) ? v.slice(PLUGIN_PREFIX.length) : v,
    );
  return [...new Set([...prev, ...names])];
}
