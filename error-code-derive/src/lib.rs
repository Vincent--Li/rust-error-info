mod error_info;

use darling::{
    ast::{Data, Fields},
    util, FromDeriveInput, FromField, FromVariant,
};
use error_info::process_error_info;
use proc_macro::TokenStream;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(error_info))]
struct ErrorData {
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
    use anyhow::Result;

    #[test]
    fn test_daling_data_struct() -> Result<()> {
        let input = r#"
      #[derive(thiserror::Error, ToErrorInfo)]
      #[error_info(app_type="http::StatusCode", prefix="01")]
      pub enum MyError {
      #[error("Invalid command: {0}")]
      #[error_info(code="IC", app_code="400")]
      InvalidCommand(String),

      #[error("Invalid argument: {0}")]
      #[error_info(code="IA", app_code="400", client_msg="friendly msg")]
      InvalidArgument(String),

      #[error("{0}")]
      #[error_info(code="RE", app_code="500")]
      RespError(#[from] RespError),
      }
      "#;

        let parsed = syn::parse_str(input).unwrap();
        let info = ErrorData::from_derive_input(&parsed).unwrap();
        println!("{:?}", info);
        Ok(())
    }
}
