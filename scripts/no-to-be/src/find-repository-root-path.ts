import { existsSync } from "node:fs";
import path from "node:path";

export function findRepositoryRootPath(startPath: string): string {
  let currentPath = startPath;

  while (true) {
    const workspaceFilePath = path.join(currentPath, "pnpm-workspace.yaml");
    const sourceDirectoryPath = path.join(currentPath, "src");
    if (existsSync(workspaceFilePath) && existsSync(sourceDirectoryPath)) {
      return currentPath;
    }

    const parentPath = path.dirname(currentPath);
    if (parentPath === currentPath) {
      throw new Error("Repository root not found from scripts/no-to-be");
    }

    currentPath = parentPath;
  }
}
