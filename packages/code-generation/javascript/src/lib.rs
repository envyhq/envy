use nv_parser::{AbstractSyntaxNode, AbstractSyntaxTree, DeclarationNode};

fn traverse_ast(ast: &AbstractSyntaxTree) -> String {
    traverse_nodes(&ast.root.as_ref().unwrap())
}

fn traverse_declaration(node: &DeclarationNode, is_module_var: bool) -> String {
    match node {
        DeclarationNode::ModuleDeclaration(module_declaration) => {
            let declarations: String = module_declaration
                .declarations
                .iter()
                .map(|declaration| traverse_declaration(declaration, true))
                .collect();
            format!(
                "const {} = {{ {} }}",
                module_declaration.identifier, declarations
            )
        }
        DeclarationNode::VarDeclaration(var_declaration) => {
            let initializer_keyword = if is_module_var { "" } else { "const " };
            let assignment_symbol = if is_module_var { ": " } else { " = " };
            let value = format!("\"{}\"", var_declaration.type_value);
            let eol = if is_module_var { "," } else { ";" };

            format!(
                "{}{}{}\"{}\"{}",
                initializer_keyword, var_declaration.identifier, assignment_symbol, value, eol
            )
        }
        _ => String::new(),
    }
}

fn traverse_nodes(node: &AbstractSyntaxNode) -> String {
    match node {
        AbstractSyntaxNode::Declaration(declaration) => traverse_declaration(declaration, false),
        AbstractSyntaxNode::SourceFile(source_file) => source_file
            .declarations
            .lock()
            .unwrap()
            .iter()
            .map(|declaration| traverse_declaration(declaration, false))
            .collect(),
    }
}

pub fn generate_javascript_source(ast: AbstractSyntaxTree) -> String {
    traverse_ast(&ast)
}
