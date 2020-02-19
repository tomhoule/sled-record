extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(Record)]
pub fn derive_record(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    impl_record(input)
}

fn impl_record(input: DeriveInput) -> TokenStream {
    let body: syn::DataStruct = match input.data {
        syn::Data::Struct(body) => body,
        syn::Data::Enum(_) => todo!("error"),
        syn::Data::Union(_) => todo!("error"),
    };

    let ident = input.ident;
    let key_struct_ident =
        syn::Ident::new(&format!("{}Key", ident), proc_macro2::Span::call_site());
    let key_reader_struct_ident = syn::Ident::new(
        &format!("{}KeyReader", ident),
        proc_macro2::Span::call_site(),
    );
    let value_struct_ident =
        syn::Ident::new(&format!("{}Value", ident), proc_macro2::Span::call_site());
    let value_reader_struct_ident = syn::Ident::new(
        &format!("{}ValueReader", ident),
        proc_macro2::Span::call_site(),
    );

    // TODO: add support for custom table names via annotation.
    let table_name = ident.to_string();

    let mut fields = body.fields.iter();

    // TODO: add support for custom key fields via annotation.
    let id_field = fields
        .next()
        .expect("TODO: return a proper error for empty structs");

    let key_field_name = &id_field.ident.as_ref().expect("TODO: error handling");
    let key_struct_field_type = &id_field.ty;

    let value_fields: Vec<_> = fields.collect();
    let value_field_definitions = value_fields.iter().map(|f| {
        let ident = f
            .ident
            .as_ref()
            .expect("TODO: good error for unnamed fields");

        let tpe = &f.ty;

        quote!(#ident: &'a #tpe)
    });
    let value_reader_field_definitions = value_fields.iter().map(|f| {
        let ident = f
            .ident
            .as_ref()
            .expect("TODO: good error for unnamed fields");

        let tpe = &f.ty;

        quote!(#ident: #tpe)
    });

    let value_field_names: Vec<_> = value_fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect();

    let trait_impl = quote! {
        #[derive(serde::Serialize)]
        struct #key_struct_ident<'a>(&'a #key_struct_field_type);
        #[derive(serde::Deserialize)]
        struct #key_reader_struct_ident(#key_struct_field_type);

        #[derive(serde::Serialize)]
        struct #value_struct_ident<'a> {
            #(#value_field_definitions),*
        }

        #[derive(serde::Deserialize)]
        struct #value_reader_struct_ident {
            #(#value_reader_field_definitions),*
        }

        impl sled_record::Record for #ident {
            const TABLE_NAME: &'static str = #table_name;

            fn write_key_bytes(&self, buf: &mut Vec<u8>) -> std::result::Result<(), bincode::Error> {
                let key_struct = #key_struct_ident(&self.#key_field_name);
                buf.reserve(bincode::serialized_size(&key_struct)? as usize);
                bincode::serialize_into(buf, &key_struct)
            }

            fn write_value_bytes(&self, buf: &mut Vec<u8>) -> std::result::Result<(), bincode::Error> {
                let value_struct = #value_struct_ident {
                    #(#value_field_names: &self.#value_field_names),*
                };
                buf.reserve(bincode::serialized_size(&value_struct)? as usize);
                bincode::serialize_into(buf, &value_struct)
            }

            fn from_kv(key_bytes: &sled::IVec, value_bytes: &sled::IVec) -> std::result::Result<Self, bincode::Error> {
                let key_fields: #key_reader_struct_ident = bincode::deserialize(key_bytes)?;
                let value_fields: #value_reader_struct_ident = bincode::deserialize(value_bytes)?;

                Ok(#ident {
                    #key_field_name: key_fields.0,
                    #(
                        #value_field_names: value_fields.#value_field_names,
                    )*
                })
            }
        }
    };

    trait_impl.into()
}
