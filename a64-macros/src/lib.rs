use core::range::RangeInclusive;
use proc_macro::TokenStream as ProcTokenStream;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::ExprMatch;
use syn::visit_mut::VisitMut;

#[derive(Clone, Copy)]
enum Pattern {
    Value(u32),
    Range(RangeInclusive<u32>),
    Wildcard,
}

fn to_int_pat(value: u32) -> syn::Pat {
    syn::Pat::Lit(syn::parse_str(&value.to_string()).unwrap())
}

impl ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Value(x) => {
                let tok = to_int_pat(*x);
                tokens.extend(tok.to_token_stream());
            }
            Self::Range(range) => {
                let start = to_int_pat(range.start);
                let last = to_int_pat(range.last);
                tokens.extend(quote::quote! { #start..=#last });
            }
            Self::Wildcard => tokens.extend(quote::quote!(_)),
        }
    }
}

fn expand(pattern: &str, out: &mut Vec<Pattern>) {
    if pattern.contains("_") {
        if pattern.matches("_").count() == pattern.len() {
            out.push(Pattern::Wildcard);
            return;
        }

        let zero = pattern.replacen("_", "0", 1);
        let one = pattern.replacen("_", "1", 1);

        expand(&zero, out);
        expand(&one, out);
    } else {
        let value = u32::from_str_radix(pattern, 2).unwrap();
        out.push(Pattern::Value(value));
    }
}

fn compress(seq: &[Pattern]) -> Vec<Pattern> {
    let mut result = Vec::new();
    for pat in seq.iter().copied() {
        match pat {
            Pattern::Value(x) => {
                if let Some(last) = result.last_mut() {
                    match last {
                        Pattern::Value(prev) if x == *prev + 1 => {
                            let prev = *prev;
                            *last = Pattern::Range(RangeInclusive {
                                start: prev,
                                last: x,
                            });
                        }
                        Pattern::Range(range) if x == range.last + 1 => range.last = x,
                        Pattern::Wildcard => unreachable!(),
                        _ => result.push(pat),
                    }
                } else {
                    result.push(pat)
                }
            }
            _ => result.push(pat),
        }
    }

    result
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

            let compressed = compress(&expanded);
            *pat = syn::parse_quote! { (#(#compressed)|*) };
        }

        syn::visit_mut::visit_pat_mut(self, pat);
    }
}

#[proc_macro]
pub fn bit_match(item: ProcTokenStream) -> ProcTokenStream {
    let mut match_ = syn::parse_macro_input!(item as ExprMatch);
    ExpandPattern.visit_expr_match_mut(&mut match_);
    match_.into_token_stream().into()
}
