const fs = require("fs");

async function main() {
  const bytes = fs.readFileSync("target/out.wasm");
  const { instance } = await WebAssembly.instantiate(bytes);

  let decoder = new TextDecoder();

  console.log(
    instance.exports.cool.value,
    instance.exports.other_cool.value,
    `|${decoder.decode(new Uint8Array(instance.exports.memory.buffer, 0, 4))}|`,
    `>${decoder.decode(new Uint8Array(instance.exports.memory.buffer, 4, 14))}<`
  );
}

main();
