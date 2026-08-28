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

    let mut columns = Vec::new();
    let mut pkeys = Vec::new();
    let mut non_pkeys = Vec::new();

    let mut binds = Vec::new();
    let mut pkey_binds = Vec::new();
    let mut data_binds = Vec::new();

    for field in &fields {
        let ident = field.ident.as_ref().unwrap();

        let mut skip = false;
        let mut pkey = false;
        let mut column_name = ident.to_string();

        for attr in &field.attrs {
            if attr.path().is_ident("sqlext") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("skip") {
                        skip = true;
                        return Ok(());
                    }

                    if meta.path.is_ident("pkey") {
                        pkey = true;
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

        columns.push(column_name.clone());

        binds.push(quote! {
            query = query.bind(&self.#ident);
        });

        if pkey {
            pkeys.push(column_name);
            pkey_binds.push(quote! {
                query = query.bind(&self.#ident);
            });
        } else {
            non_pkeys.push(column_name);
            data_binds.push(quote! {
                query = query.bind(&self.#ident);
            });
        }
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

            fn pkey_columns() -> &'static [&'static str] {
                &[#(#pkeys),*]
            }

            fn data_columns() -> &'static [&'static str] {
                &[#(#non_pkeys),*]
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

            fn pkey_bind<'q>(
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
                #(#pkey_binds)*
                query
            }

            fn data_bind<'q>(
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
                #(#data_binds)*
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

            fn pkey_columns() -> &'static [&'static str] {
                &[#(#pkeys),*]
            }

            fn data_columns() -> &'static [&'static str] {
                &[#(#non_pkeys),*]
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

            fn pkey_bind<'q>(
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
                #(#pkey_binds)*
                query
            }

            fn data_bind<'q>(
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
                #(#data_binds)*
                query
            }
        }
    };

    expanded.into()
}
