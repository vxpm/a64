use proc_macro::TokenStream;
use quote::ToTokens;
use syn::ExprMatch;
use syn::visit_mut::VisitMut;

fn expand(pattern: &str, out: &mut Vec<syn::Pat>) {
    if pattern.contains("_") {
        if pattern.matches("_").count() == pattern.len() {
            out.push(syn::parse_quote!(_));
            return;
        }

        let zero = pattern.replacen("_", "0", 1);
        let one = pattern.replacen("_", "1", 1);

        expand(&zero, out);
        expand(&one, out);
    } else {
        let value = u32::from_str_radix(pattern, 2).unwrap();
        out.push(syn::Pat::Lit(syn::parse_str(&value.to_string()).unwrap()));
    }
}

struct ExpandPattern;

impl VisitMut for ExpandPattern {
    fn visit_pat_mut(&mut self, pat: &mut syn::Pat) {
        if let syn::Pat::Lit(lit) = pat
            && let syn::Lit::Str(s) = &mut lit.lit
        {
            let pattern = s.value();
            let mut expanded = Vec::new();
            expand(&pattern, &mut expanded);
            *pat = syn::parse_quote! { (#(#expanded)|*) };
        }

        syn::visit_mut::visit_pat_mut(self, pat);
    }
}

#[proc_macro]
pub fn bit_match(item: TokenStream) -> TokenStream {
    let mut match_ = syn::parse_macro_input!(item as ExprMatch);
    ExpandPattern.visit_expr_match_mut(&mut match_);
    match_.into_token_stream().into()
}
