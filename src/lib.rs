use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprLit, Lit};

#[proc_macro_derive(FromPsql, attributes(sqlfield))]
/// Proc-macro to generate  `impl TryFrom<tokio_postgres::Row>` for a struct
pub fn generate_tryfrom_impl(input: TokenStream) -> TokenStream {
    let __ast: syn::DeriveInput = syn::parse(input).unwrap();

    let __struct_fields = match __ast.data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(n) => n.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let __field_specific_syn = __struct_fields.iter().map(|f| {
        let __field_name = f.ident.as_ref().unwrap();
        let _type = &f.ty;

        if let Some(attr) = f.attrs.iter().find(|a| a.path().is_ident("sqlfield")) {
            if let Expr::Lit(ExprLit {
                attrs: _,
                lit: Lit::Str(l),
            }) = attr.parse_args().unwrap()
            {
                quote! {
                    #__field_name: row.try_get::<_, #_type>(#l)?
                }
            } else {
                panic!("Only string literals are supported")
            }
        } else if __field_name
            .span()
            .source_text()
            .as_deref()
            .map(|v| v.starts_with("r#"))
            .unwrap_or_default()
        {
            let source_text = __field_name.span().source_text().unwrap();
            let rhs = source_text.trim_start_matches("r#");
            quote! {
                #__field_name: row.try_get::<_, #_type>(#rhs)?
            }
        } else {
            quote! {
                #__field_name: row.try_get::<_, #_type>(stringify!(#__field_name))?
            }
        }
    });

    let __fields_tryget_code = quote!(#(#__field_specific_syn),*);

    let __struct_name = __ast.ident;
    let __final_code = quote! {
        impl std::convert::TryFrom<tokio_postgres::Row> for #__struct_name {
            type Error = tokio_postgres::Error;

            fn try_from(row: tokio_postgres::Row) -> Result<Self, Self::Error> {
                Ok(Self {
                    #__fields_tryget_code
                })
            }
        }
    };

    __final_code.into()
}
