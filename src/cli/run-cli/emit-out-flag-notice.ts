import { extname } from "node:path";

import type { NormalizedCliOptions } from "./normalized-cli-options.js";

/**
 * Tell the user when `-o foo.json` would be silently treated as a
 * directory name. The dot in the basename is the heuristic for
 * "looks like a filename"; an empty extname means no notice.
 */
export function emitOutFlagNotice(opts: NormalizedCliOptions): void {
  if (opts.out === null || opts.out.mode !== "dir") {
    return;
  }
  if (extname(opts.out.path) === "") {
    return;
  }
  process.stderr.write(
    `uns: notice: -o '${opts.out.path}' is treated as a directory name; use --out-file to write to that path as a file.\n`,
  );
}
