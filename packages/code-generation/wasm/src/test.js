const fs = require("fs");

async function main() {
  const bytes = fs.readFileSync("target/out.wasm");
  const { instance } = await WebAssembly.instantiate(bytes);
  console.log(instance.exports.mahGlobal.value);
}

main();
