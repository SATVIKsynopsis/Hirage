use syn::{ImplItem, Item};

pub fn extract_functions(source: &str) -> Vec<String> {
    let mut funcs = Vec::new();

    let ast = match syn::parse_file(source) {
        Ok(ast) => ast,
        Err(_) => return funcs,
    };

    for item in ast.items {
        match item {
            Item::Fn(f) => {
                funcs.push(f.sig.ident.to_string());
            }

            Item::Impl(imp) => {
                for item in imp.items {
                    if let ImplItem::Fn(method) = item {
                        funcs.push(method.sig.ident.to_string());
                    }
                }
            }

            _ => {}
        }
    }

    funcs
}
