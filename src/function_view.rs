use syn::{ImplItemFn, ItemFn, visit::Visit};

pub struct FunctionFinder {
    pub target: String,
    pub result: Option<String>,
}

impl<'ast> Visit<'ast> for FunctionFinder {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        if node.sig.ident == self.target {
            self.result = Some(quote::quote!(#node).to_string());
        }
    }

    fn visit_impl_item_fn(&mut self, node: &'ast ImplItemFn) {
        if node.sig.ident == self.target {
            self.result = Some(quote::quote!(#node).to_string());
        }
    }
}

pub fn extract_function_source(source: &str, name: &str) -> Option<String> {
    let ast = syn::parse_file(source).ok()?;

    let mut finder = FunctionFinder {
        target: name.to_string(),
        result: None,
    };

    finder.visit_file(&ast);

    finder.result
}
