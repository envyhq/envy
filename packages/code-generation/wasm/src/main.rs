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
