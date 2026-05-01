import { describe, expect, test } from "vitest";

import { edgeLabelOfRef } from "./edge-label-of-ref.js";
import { makeRef } from "./testing/make-ref.js";

describe("edgeLabelOfRef", () => {
  test.each([
    {
      read: false,
      write: false,
      call: false,
      receiver: false,
      expected: "ref",
    },
    {
      read: true,
      write: false,
      call: false,
      receiver: false,
      expected: "read",
    },
    {
      read: false,
      write: true,
      call: false,
      receiver: false,
      expected: "write",
    },
    {
      read: false,
      write: false,
      call: true,
      receiver: false,
      expected: "call",
    },
    {
      read: true,
      write: true,
      call: false,
      receiver: false,
      expected: "read,write",
    },
    {
      read: true,
      write: false,
      call: true,
      receiver: false,
      expected: "read,call",
    },
    {
      read: false,
      write: true,
      call: true,
      receiver: false,
      expected: "write,call",
    },
    {
      read: true,
      write: true,
      call: true,
      receiver: false,
      expected: "read,write,call",
    },
    { read: false, write: false, call: false, receiver: true, expected: "ref" },
    { read: true, write: false, call: false, receiver: true, expected: "read" },
  ])(
    "flags read=$read write=$write call=$call receiver=$receiver -> $expected",
    ({ read, write, call, receiver, expected }) => {
      expect(
        edgeLabelOfRef(makeRef({ flags: { read, write, call, receiver } })),
      ).toBe(expected);
    },
  );
});
