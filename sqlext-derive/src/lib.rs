use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Meta, parse_macro_input};

#[proc_macro_derive(ToRow, attributes(sqlext))]
pub fn derive_torow(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let mut table_override: Option<String> = None;

    for attr in input.attrs.iter().filter(|a| a.path().is_ident("sqlext")) {
        let meta = attr.meta.clone();

        if let Meta::List(list) = meta {
            list.parse_nested_meta(|meta| {
                if meta.path.is_ident("table") {
                    let value = meta.value()?.parse::<syn::LitStr>()?;
                    table_override = Some(value.value());
                    return Ok(());
                }

                Ok(())
            })
            .unwrap();
        }
    }

    let fields = match input.data {
        syn::Data::Struct(data) => match data.fields {
            syn::Fields::Named(fields) => fields.named,
            _ => panic!("ToRow only supports named fields"),
        },
        _ => panic!("ToRow only supports structs"),
    };

    let mut binds = Vec::new();
    let mut columns = Vec::new();

    for field in fields.iter() {
        let ident = field.ident.as_ref().unwrap();

        let mut skip = false;
        let mut column_name = ident.to_string();

        for attr in &field.attrs {
            if attr.path().is_ident("sqlext") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("skip") {
                        skip = true;
                        return Ok(());
                    }

                    if meta.path.is_ident("column") {
                        let value = meta.value()?.parse::<syn::LitStr>()?;
                        column_name = value.value();
                        return Ok(());
                    }

                    Ok(())
                })
                .unwrap();
            }
        }

        if skip {
            continue;
        }

        columns.push(column_name);

        binds.push(quote! {
            query = query.bind(&self.#ident);
        });
    }

    let table_name = table_override.unwrap_or_else(|| name.to_string().to_lowercase());

    let expanded = quote! {
        impl sqlext::ToRow<sqlx::Postgres> for #name {
            fn table_name() -> &'static str {
                #table_name
            }

            fn columns() -> &'static [&'static str] {
                &[#(#columns),*]
            }

            fn bind<'q>(
                &'q self,
                mut query: sqlx::query::Query<
                    'q,
                    sqlx::Postgres,
                    sqlx::postgres::PgArguments,
                >,
            ) -> sqlx::query::Query<
                'q,
                sqlx::Postgres,
                sqlx::postgres::PgArguments,
            > {
                #(#binds)*
                query
            }
        }

        impl sqlext::ToRow<sqlx::Sqlite> for #name {
            fn table_name() -> &'static str {
                #table_name
            }

            fn columns() -> &'static [&'static str] {
                &[#(#columns),*]
            }

            fn bind<'q>(
                &'q self,
                mut query: sqlx::query::Query<
                    'q,
                    sqlx::Sqlite,
                    sqlx::sqlite::SqliteArguments<'q>,
                >,
            ) -> sqlx::query::Query<
                'q,
                sqlx::Sqlite,
                sqlx::sqlite::SqliteArguments<'q>,
            > {
                #(#binds)*
                query
            }
        }
    };

    expanded.into()
}
