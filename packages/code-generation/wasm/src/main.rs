//! This example constructs a Wasm module from scratch with Walrus.
//!
//! The module we are building implements and exports the `factorial` function,
//! and imports a `env.log` function to observe incremental results.
//!
//! You can run the built Wasm module using Node.js (for example) like this:
//!
//! ```js
//! const fs = require("fs");
//!
//!
//! async function main() {
//!   const bytes = fs.readFileSync("target/out.wasm");
//!   const env = { log: val => console.log(`logged ${val}`), };
//!   const { instance } = await WebAssembly.instantiate(
//!     bytes,
//!     {
//!       env: {
//!         log(val) {
//!           console.log(`log saw ${val}`);
//!         }
//!       }
//!     }
//!   );
//!   const result = instance.exports.factorial(5);
//!   console.log(`factorial(5) = ${result}`);
//! }
//!
//! main();
//! ```

use walrus::{ir::*, InitExpr};
use walrus::{Module, ModuleConfig, ValType};

fn main() -> walrus::Result<()> {
    let config = ModuleConfig::new();
    let mut module = Module::with_config(config);

    let res = module
        .globals
        .add_local(ValType::I32, false, InitExpr::Value(Value::I32(1)));

    module.exports.add("mahGlobal", res);

    module.emit_wasm_file("target/out.wasm")
}
