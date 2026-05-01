export const DIRECTION = {
  RL: "RL",
  LR: "LR",
  TB: "TB",
  BT: "BT",
} as const;
export type Direction = (typeof DIRECTION)[keyof typeof DIRECTION];
