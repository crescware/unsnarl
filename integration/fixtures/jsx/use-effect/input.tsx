import { useEffect } from "react";

const Comp = ({ name }: { name: string }) => {
  useEffect(() => {
    console.log("mount:", name);
    return () => {
      console.log("unmount:", name);
    };
  }, [name]);
  return null;
};
