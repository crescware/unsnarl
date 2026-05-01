export function pushTo(
  map: Map<string, /* mutable */ string[]>,
  key: string,
  value: string,
): void {
  const arr = map.get(key);
  if (arr === undefined) {
    map.set(key, [value]);
  } else {
    arr.push(value);
  }
}
