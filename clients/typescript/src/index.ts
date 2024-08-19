import { promises as fs } from "fs";
import { type Config, parseConfig } from "@envyhq/config";

export async function loadConfig(pathInput: string) {
  const path = pathInput ?? `.nv/config.nvx`;
  const contents = await fs.readFile(path, "utf8");

  return contents;
}

export async function test() {
  const configName = "../../lang/packages/cli/.nv/simple-vars.nvx";

  const result = await loadConfig(configName);

  const lines = result.split("\n");
  const config = lines.reduce(
    (acc, line) => {
      const [key, value] = line.split(":");
      acc[key] = value;
      return acc;
    },
    {} as Record<string, string>,
  );

  const parsedConfig: Config = parseConfig(config);

  console.log(
    config.my_cool_var, // "wow"
    config.my_other_var, // 1234

    parsedConfig.my_cool_var, // "wow"
    parsedConfig.my_other_var, // 1234
    parsedConfig.what, // error
  );
}

test();
