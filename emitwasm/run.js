const fs = require("fs");

async function main() {
  const bytes = fs.readFileSync("out.wasm");
  const env = { negate: val => -val };
  const module = await WebAssembly.instantiate(bytes, { env }).then(
    results => results.instance.exports
  );
  const result = module.main();
  console.log("main:", result);
  // const result = module.sqrt(25);
  // const buffer = new Uint8Array(module.memory.buffer, 0);
  // const decoder = new TextDecoder();
  // const str = decoder.decode(buffer);
  // console.log(module.memory.buffer);
  // console.log(str);
  // console.log(result);
  // const negate1_result = module.negate1();
  // console.log("negate1:", negate1_result);
}

main();
