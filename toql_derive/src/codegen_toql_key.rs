
use proc_macro2::{TokenStream, Span};
use crate::sane::{FieldKind, SqlTarget};
use syn::{Ident};

pub(crate) struct GeneratedToqlKey<'a> {

    rust_struct: &'a crate::sane::Struct,
    key_columns_code: Vec<TokenStream>,
    key_params_code: Vec<TokenStream>,
    key_types: Vec<TokenStream>,
    key_fields: Vec<TokenStream>,
    key_setters: Vec<TokenStream>
}


impl<'a> GeneratedToqlKey<'a> {
    pub(crate) fn from_toql(toql: &crate::sane::Struct) -> GeneratedToqlKey {
        GeneratedToqlKey {
            rust_struct: &toql,
            key_columns_code: Vec::new(),
            key_params_code: Vec::new(),
            key_types: Vec::new(),
            key_fields: Vec::new(),
            key_setters: Vec::new()
          }
   }

   pub fn add_key_field(&mut self, field: &crate::sane::Field) -> darling::error::Result<()> {


        let rust_type_ident = &field.rust_type_ident;
        let rust_type_name = &field.rust_type_name;
        let rust_field_ident = &field.rust_field_ident;
        let rust_field_name = &field.rust_field_name;

        
         match &field.kind {
            FieldKind::Regular(ref regular_attrs) => {
                if !regular_attrs.key {
                    return Ok(());
                }

                 if let SqlTarget::Column(ref column) = &regular_attrs.sql_target {
                        self.key_columns_code
                            .push(quote!( columns.push( String::from(#column)); ));
                    } else {
                        // error only
                    }

                    self.key_types.push(quote!( #rust_type_ident));

                    if field.number_of_options > 0 {
                        let value = quote!(self. #rust_field_ident .as_ref() .ok_or(toql::error::ToqlError::ValueMissing( String::from(# rust_type_name)))? .to_owned());
                        self.key_fields.push(value);

                        /*  self.key_params_code.push( quote!(
                        params.push(self. #rust_field_ident .as_ref()
                         .ok_or(toql::error::ToqlError::ValueMissing( String::from(# rust_field_name)))? .to_owned().to_string());
                         )); */

                        let index = syn::Index::from(self.key_types.len() - 1);
                        self.key_setters
                            .push(quote!(self. #rust_field_ident = Some( key . #index  ); ))
                    } else {
                        self.key_fields
                            .push(quote!(self. #rust_field_ident .to_owned()));

                        let index = syn::Index::from(self.key_types.len() - 1);

                        self.key_setters
                            .push(quote!(self. #rust_field_ident = key . #index;))
                    }

                    /* if let SqlTarget::Column(ref sql_column) = &regular_attrs.sql_target {
                        let key_expr = format!("{}.{} = ?", self.sql_table_alias, sql_column);
                        self.select_keys.push(quote!(#key_expr)); */
                  /*   } else {
                        // error only
                    }
 */
                    let key_index = syn::Index::from(self.key_fields.len() - 1);
                   

                    self.key_params_code
                        .push(quote!(params.push(key . #key_index .to_owned().to_string()); ));

                  /*   self.select_keys_params.push(quote! {
                     params.push( key . #key_index .to_string());
                    }); */


            },
                FieldKind::Join(ref join_attrs) => {

                    if !join_attrs.key {
                        return Ok(());
                    }


                    let key_index = syn::Index::from(self.key_types.len() - 1);
                   self.key_types
                        .push(quote!( <#rust_type_ident as toql::key::Key>::Key));
                    let key_index = syn::Index::from(self.key_types.len() - 1);

                    /*  let unwrap = match field.number_of_options {
                        1 if !field.preselect => quote!(.as_ref().unwrap()),
                        _ => quote!()

                    }; */

                    self.key_columns_code.push( quote!( columns.extend_from_slice(&<#rust_type_ident as toql::key::Key>::columns());));
                    self.key_params_code.push( quote!( params.extend_from_slice(&<#rust_type_ident as toql::key::Key>::params(& key. #key_index));));

                    //let function_ident = Ident::new(&format!("{}_key_predicate",&field.rust_field_name), Span::call_site());

                  
                    // Select key predicate
                    if field.number_of_options > 0 {
                        self.key_fields.push( quote!(
                                < #rust_type_ident as toql::key::Key>::get_key(
                                    self. #rust_field_ident .as_ref()
                                        .ok_or(toql::error::ToqlError::ValueMissing( String::from(#rust_field_name)))?
                                    )?
                            ));

                        self.key_setters.push( quote!(
                                        < #rust_type_ident as toql::key::Key>::set_key(self. #rust_field_ident .as_mut()
                                            .ok_or(toql::error::ToqlError::ValueMissing( String::from(#rust_field_name)))? , key . #key_index )?;
                            ));
                    } else {
                        self.key_fields.push( quote!(
                                < #rust_type_ident as toql::key::Key>::get_key(  &self. #rust_field_ident )?
                            ));

                        self.key_setters.push( quote!(
                                    < #rust_type_ident as toql::key::Key>::set_key(&mut self. #rust_field_ident,key . #key_index)?;
                            ));
                    }
                 //   let aliased_column_format = format!("{}.{{}} = ?", &self.sql_table_alias);
                 /*    self.select_keys.push(quote!( {
                        &<#rust_type_ident as toql::key::Key>::columns().iter()
                        .map(|other_column|{
                            #default_self_column_code;
                            let self_column = #columns_map_code;
                            format!(#aliased_column_format, self_column )
                        }).collect::<Vec<String>>().join(" AND ")
                    }
                    )); */
                /*     self.select_keys_params.push(  quote! {
                            params.extend_from_slice( &<#rust_type_ident as toql::key::Key>::params( &key. #key_index));
                        });
 */


                },
                _ => {}
            }
            Ok(())

   }

   pub fn key_missing(&self) -> bool {
        self.key_types.is_empty()
   }
}



impl<'a> quote::ToTokens for GeneratedToqlKey<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {


            let vis = &self.rust_struct.rust_struct_visibility;
            let rust_stuct_ident = &self.rust_struct.rust_struct_ident;

            let struct_key_ident = Ident::new(&format!("{}Key", &rust_stuct_ident), Span::call_site());
            let key_columns_code = &self.key_columns_code;
            let key_params_code = &self.key_params_code;

            let key_types = &self.key_types;

            let key_type_code = quote!(  #(pub #key_types),* );


             let key_fields = &self.key_fields;
          
            let key_getter = quote!( #(#key_fields  ),* );
            let key_setters = &self.key_setters;

           let key =  quote! {

            #[derive(Debug, Eq, PartialEq, Hash)]
               #vis struct #struct_key_ident ( #key_type_code);

                impl toql::key::Key for #rust_stuct_ident {
                    type Key = #struct_key_ident;

                    fn get_key(&self) -> toql::error::Result<Self::Key> {
                       Ok(  #struct_key_ident (#key_getter) )
                    }
                    fn set_key(&mut self, key: Self::Key) -> toql::error::Result<()> {
                      #( #key_setters)*
                      Ok(())
                    }
                    fn columns() ->Vec<String> {
                         let mut columns: Vec<String>= Vec::new();

                        #(#key_columns_code)*
                        columns
                    }
                    fn params(key: &Self::Key) ->Vec<String> {
                         let mut params: Vec<String>= Vec::new();

                        #(#key_params_code)*
                        params
                    }
                }

                impl std::convert::TryFrom<#rust_stuct_ident> for #struct_key_ident
                {
                    type Error = toql::error::ToqlError;
                    fn try_from(entity: #rust_stuct_ident) -> toql::error::Result<Self> {
                        <#rust_stuct_ident as toql::key::Key>::get_key(&entity)
                    }
                }

                // Impl to supprt HashSets
                impl std::hash::Hash for #rust_stuct_ident {
                    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                        <#rust_stuct_ident as toql::key::Key>::get_key(self).ok().hash(state);
                    }
                }
            };

             log::debug!(
            "Source code for `{}`:\n{}",
            rust_stuct_ident,
            key.to_string()
        );
        tokens.extend(key);

    }
}