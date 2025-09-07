use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use super::constants::{
    BLOB_TYPE, BOOL_TYPE, CLS_ID_TYPE, DATE_TYPE, ERROR_TYPE, F32_TYPE, F64_TYPE, HRESULT_TYPE,
    I16_TYPE, I32_TYPE, I64_TYPE, I8_TYPE, NULL_TYPE, STRING_TYPE, U16_TYPE, U32_TYPE, U64_TYPE,
    U8_TYPE,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ParamType {
    // Базовые типы (существующие)
    Bool,
    I32,
    F64,
    String,
    Date,
    Blob,
    
    // Дополнительные типы, поддерживаемые Native API
    Null,
    I8,
    I16,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    Error,
    HResult,
    ClsId,
}

const META_TYPE_ERR: &str = "expected string literal or path";

impl FromMeta for ParamType {
    fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
        let meta_type_err = darling::Error::custom(META_TYPE_ERR);
        let expr_string = match expr {
            syn::Expr::Lit(str_lit) => match str_lit.lit {
                syn::Lit::Str(ref str) => str.value(),
                _ => return Err(meta_type_err),
            },
            syn::Expr::Path(path) => path.path.segments.first().unwrap().ident.to_string(),
            _ => return Err(meta_type_err),
        };
        Self::from_string(&expr_string)
    }

    fn from_string(value: &str) -> darling::Result<Self> {
        let joined_allowed_types = crate::derive_addin::constants::ALL_ARG_TYPES.join(", ");
        Self::try_from(value).map_err(|_| {
            darling::Error::custom(format!(
                "unknown type `{value}`. Must be one of: {joined_allowed_types}",
            ))
        })
    }
}

impl TryFrom<&str> for ParamType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            // Базовые типы
            BOOL_TYPE => Ok(ParamType::Bool),
            I32_TYPE => Ok(ParamType::I32),
            F64_TYPE => Ok(ParamType::F64),
            STRING_TYPE => Ok(ParamType::String),
            DATE_TYPE => Ok(ParamType::Date),
            BLOB_TYPE => Ok(ParamType::Blob),
            
            // Дополнительные типы
            NULL_TYPE => Ok(ParamType::Null),
            I8_TYPE => Ok(ParamType::I8),
            I16_TYPE => Ok(ParamType::I16),
            I64_TYPE => Ok(ParamType::I64),
            U8_TYPE => Ok(ParamType::U8),
            U16_TYPE => Ok(ParamType::U16),
            U32_TYPE => Ok(ParamType::U32),
            U64_TYPE => Ok(ParamType::U64),
            F32_TYPE => Ok(ParamType::F32),
            ERROR_TYPE => Ok(ParamType::Error),
            HRESULT_TYPE => Ok(ParamType::HResult),
            CLS_ID_TYPE => Ok(ParamType::ClsId),
            
            _ => Err(())
        }
    }
}

impl ToTokens for ParamType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        *tokens = match self {
            // Базовые типы
            ParamType::Bool => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::Bool }
            }
            ParamType::I32 => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::I32 }
            }
            ParamType::F64 => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::F64 }
            }
            ParamType::Date => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::Date }
            }
            ParamType::String => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::String }
            }
            ParamType::Blob => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::Blob }
            }
            
            // Дополнительные типы
            ParamType::Null => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::Null }
            }
            ParamType::I8 => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::I8 }
            }
            ParamType::I16 => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::I16 }
            }
            ParamType::I64 => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::I64 }
            }
            ParamType::U8 => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::U8 }
            }
            ParamType::U16 => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::U16 }
            }
            ParamType::U32 => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::U32 }
            }
            ParamType::U64 => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::U64 }
            }
            ParamType::F32 => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::F32 }
            }
            ParamType::Error => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::Error }
            }
            ParamType::HResult => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::HResult }
            }
            ParamType::ClsId => {
                quote! { native_api_1c::native_api_1c_core::interface::ParamValue::ClsId }
            }
        }
    }
}

#[derive(Debug)]
pub enum PropName {
    StringLiteral(syn::LitStr),
    Ident(syn::ExprPath),
}

impl FromMeta for PropName {
    fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
        match expr {
            syn::Expr::Lit(lit) => match &lit.lit {
                syn::Lit::Str(str_lit) => Ok(PropName::StringLiteral(str_lit.clone())),
                _ => Err(darling::Error::custom("expected string literal").with_span(expr)),
            },
            syn::Expr::Path(path) => Ok(PropName::Ident(path.clone())),
            _ => Err(darling::Error::custom("expected string literal or path").with_span(expr)),
        }
    }
}

impl From<PropName> for proc_macro2::TokenStream {
    fn from(prop_name: PropName) -> proc_macro2::TokenStream {
        match prop_name {
            PropName::StringLiteral(str_lit) => str_lit.to_token_stream(),
            PropName::Ident(ident) => ident.to_token_stream(),
        }
    }
}
