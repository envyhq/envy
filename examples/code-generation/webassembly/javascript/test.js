const fs = require("fs");

async function main() {
  const bytes = fs.readFileSync("target/out.wasm");
  const { instance } = await WebAssembly.instantiate(bytes);

  let decoder = new TextDecoder();

  console.log(
    instance.exports["var_x.a"].value,
    instance.exports.var_b.value,
    decoder.decode(new Uint8Array(instance.exports.memory.buffer, 0, 4))
  );
}

main();
