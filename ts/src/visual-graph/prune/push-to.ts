export function pushTo(
  map: Map<string, /* mutable */ string[]>,
  key: string,
  value: string,
): void {
  const arr = map.get(key) ?? null;
  if (arr === null) {
    map.set(key, [value]);
  } else {
    arr.push(value);
  }
}
