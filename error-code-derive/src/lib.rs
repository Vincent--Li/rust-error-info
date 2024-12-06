mod error_info;

use darling::{ast::{Data, Fields}, util, FromDeriveInput, FromField, FromVariant};
use proc_macro::TokenStream;
use error_info::process_error_info;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(error_info))]
struct EnumFromDarling {
  ident: syn::Ident,
  generics: syn::Generics,
  data: Data<EnumVariants, ()>,
  app_type: syn::Type,
  prefix: String,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(error_info))]
struct EnumVariants {
  ident: syn::Ident,
  fields: Fields<util::Ignored>,
  code: String,
  #[darling(default)]
  app_code: String,
  #[darling(default)]
  client_msg: String,
}

#[derive(Debug, FromField)]
struct EnumVariantFields {
  ty: syn::Type,
}



#[proc_macro_derive(ToErrorInfo, attributes(error_info))]
pub fn derive_to_error_info(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    
    
    process_error_info(input).into()
} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daling_data_struct() -> Result<()> {
        
        
        Ok(())
    }
}