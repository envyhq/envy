use walrus::ir::Value;
// use nv_parser::{AbstractSyntaxNode, AbstractSyntaxTree, DeclarationNode};
use walrus::{ActiveData, ActiveDataLocation, DataKind, InitExpr, ValType};
use walrus::{Module, ModuleConfig};

pub fn generate_webassembly_source() -> walrus::Result<()> {
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

    let cool_locator =
        root_module
            .globals
            .add_local(ValType::I32, false, InitExpr::Value(Value::I32(0)));

    root_module.data.add(
        DataKind::Active(ActiveData {
            memory,
            location: ActiveDataLocation::Absolute("cool".len() as u32),
        }),
        "other_cool".into(),
    );
    let other_cool_locator = root_module.globals.add_local(
        ValType::I32,
        false,
        InitExpr::Value(Value::I32("cool".len() as i32)),
    );

    root_module.exports.add("memory", memory);
    root_module.exports.add("cool", cool_locator);
    root_module.exports.add("other_cool", other_cool_locator);

    root_module.emit_wasm_file("target/out.wasm")
}

// fn traverse_ast(ast: &AbstractSyntaxTree) -> String {
//     traverse_nodes(&ast.root.as_ref().unwrap())
// }

// fn traverse_declaration(node: &DeclarationNode, is_module_var: bool) -> String {
//     match node {
//         DeclarationNode::ModuleDeclaration(module_declaration) => {
//             let declarations: String = module_declaration
//                 .declarations
//                 .iter()
//                 .map(|declaration| traverse_declaration(declaration, true))
//                 .collect();
//             format!(
//                 "const {} = {{ {} }}",
//                 module_declaration.identifier, declarations
//             )
//         }
//         DeclarationNode::VarDeclaration(var_declaration) => {
//             let initializer_keyword = if is_module_var { "" } else { "const " };
//             let assignment_symbol = if is_module_var { ": " } else { " = " };
//             let value = format!("\"{}\"", var_declaration.type_value);
//             let eol = if is_module_var { "," } else { ";" };

//             format!(
//                 "{}{}{}\"{}\"{}",
//                 initializer_keyword, var_declaration.identifier, assignment_symbol, value, eol
//             )
//         }
//         _ => String::new(),
//     }
// }

// fn traverse_nodes(node: &AbstractSyntaxNode) -> String {
//     match node {
//         AbstractSyntaxNode::Declaration(declaration) => traverse_declaration(declaration, false),
//         AbstractSyntaxNode::SourceFile(source_file) => source_file
//             .declarations
//             .iter()
//             .map(|declaration| traverse_declaration(declaration, false))
//             .collect(),
//     }
// }

// pub fn generate_wasm_source(ast: AbstractSyntaxTree) -> String {
//     traverse_ast(&ast)
// }
