export function View(props: { enabled: boolean }) {
  const on = "on";
  const off = "off";
  return <div>{props.enabled ? on : off}</div>;
}
