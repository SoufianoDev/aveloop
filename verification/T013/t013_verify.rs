use tree_sitter::Parser;

fn main() {
    let results = vec![
        ("c17_parse_roundtrip", verify_c17_parse()),
        ("cpp20_parse_roundtrip", verify_cpp20_parse()),
        ("c17_node_kinds", verify_c17_node_kinds()),
        ("cpp20_node_kinds", verify_cpp20_node_kinds()),
    ];

    let mut failed = 0;
    for (name, ok) in &results {
        println!("[T013] {}: {}", name, if *ok { "PASS" } else { "FAIL" });
        if !ok {
            failed += 1;
        }
    }
    if failed > 0 {
        eprintln!("[T013] {} check(s) failed", failed);
        std::process::exit(1);
    }
    println!("[T013] all checks passed");
}

fn verify_c17_parse() -> bool {
    let mut parser = Parser::new();
    let lang: tree_sitter::Language = tree_sitter_c::LANGUAGE.into();
    if parser.set_language(&lang).is_err() {
        eprintln!("[T013] failed to set C language");
        return false;
    }
    let src = b"int main(void) { return 0; }";
    let tree = match parser.parse(src, None) {
        Some(t) => t,
        None => {
            eprintln!("[T013] C parse returned None");
            return false;
        }
    };
    let root = tree.root_node();
    !root.has_error() && root.kind() == "translation_unit"
}

fn verify_cpp20_parse() -> bool {
    let mut parser = Parser::new();
    let lang: tree_sitter::Language = tree_sitter_cpp::LANGUAGE.into();
    if parser.set_language(&lang).is_err() {
        eprintln!("[T013] failed to set C++ language");
        return false;
    }
    let src = b"#include <cstdio>\nint main() { return 0; }";
    let tree = match parser.parse(src, None) {
        Some(t) => t,
        None => {
            eprintln!("[T013] C++ parse returned None");
            return false;
        }
    };
    let root = tree.root_node();
    !root.has_error() && root.kind() == "translation_unit"
}

fn verify_c17_node_kinds() -> bool {
    let mut parser = Parser::new();
    let lang: tree_sitter::Language = tree_sitter_c::LANGUAGE.into();
    parser.set_language(&lang).unwrap();
    let src = b"void foo(int x) { if (x > 0) { x = x - 1; } }";
    let tree = parser.parse(src, None).unwrap();
    let sexp = tree.root_node().to_sexp();
    sexp.contains("function_definition") && sexp.contains("if_statement")
}

fn verify_cpp20_node_kinds() -> bool {
    let mut parser = Parser::new();
    let lang: tree_sitter::Language = tree_sitter_cpp::LANGUAGE.into();
    parser.set_language(&lang).unwrap();
    let src = b"template<typename T> T add(T a, T b) { return a + b; }";
    let tree = parser.parse(src, None).unwrap();
    let sexp = tree.root_node().to_sexp();
    sexp.contains("template_declaration") && sexp.contains("function_definition")
}
