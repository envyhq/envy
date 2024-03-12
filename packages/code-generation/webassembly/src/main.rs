use walrus::{ir::*, ActiveData, ActiveDataLocation, DataKind, InitExpr};
use walrus::{Module, ModuleConfig, ValType};

fn main() -> walrus::Result<()> {
    let config = ModuleConfig::new();
    let mut root_module = Module::with_config(config.clone());

    let memory = root_module.memories.add_local(false, 1, None);

    root_module.data.add(
        DataKind::Active(ActiveData {
            memory,
            location: ActiveDataLocation::Absolute(0),
        }),
        "cool".into(),
    );

    let var_x = root_module
        .globals
        .add_local(ValType::I32, false, InitExpr::Value(Value::I32(10)));

    let var_b = root_module
        .globals
        .add_local(ValType::I32, false, InitExpr::Value(Value::I32(20)));

    root_module.exports.add("var_b", var_b);
    root_module.exports.add("var_x.a", var_x);
    root_module.exports.add("memory", memory);

    root_module.emit_wasm_file("target/out.wasm")
}
