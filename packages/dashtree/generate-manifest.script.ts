import { PackageJSON } from "@npm/types";
import { readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";

(async () => {
  const encoding: BufferEncoding = "utf-8";

  const fileName = "package.json";

  const resultDirectoryName = "dist";

  const sourceFilePath = join(__dirname, fileName);
  const resultFilePath = join(__dirname, resultDirectoryName, fileName);

  const { name, version, main, module, types }: PackageJSON = JSON.parse(
    await readFile(sourceFilePath, {
      encoding,
    }),
  );

  const unnecessarySegment = `${resultDirectoryName}/`;

  const resultFileContent: PackageJSON = {
    name,
    version,
    main:
      typeof main === "string" ? main.replace(unnecessarySegment, "") : main,
    module:
      typeof module === "string"
        ? module.replace(unnecessarySegment, "")
        : module,
    types:
      typeof types === "string" ? types.replace(unnecessarySegment, "") : types,
  };

  await writeFile(resultFilePath, JSON.stringify(resultFileContent), {
    encoding,
  });
})().catch((reason: unknown) => console.error(reason));
