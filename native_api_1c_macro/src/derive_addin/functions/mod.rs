use std::fmt::Display;

use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

use super::{
    constants::{
        ANSI_STRING_TYPE, BLOB_TYPE, BOOL_TYPE, CLS_ID_TYPE, DATE_DOUBLE_TYPE, DATE_TYPE, ERROR_TYPE,
        F32_TYPE, F64_TYPE, HRESULT_TYPE, I16_TYPE, I32_TYPE, I64_TYPE, I8_TYPE, NULL_TYPE, STRING_TYPE,
        U16_TYPE, U32_TYPE, U64_TYPE, U8_TYPE,
    },
    parsers::ParamType,
};

pub mod collectors;
pub mod generate;
pub mod parse;

#[derive(Debug)]
pub struct FuncDesc {
    pub ident: Ident,

    pub name_literal: TokenStream,
    pub name_ru_literal: TokenStream,

    pub params: Vec<FuncArgumentDesc>,
    pub return_value: ReturnTypeDesc,
}

impl FuncDesc {
    pub fn get_1c_params(&self) -> Vec<&FuncArgumentDesc> {
        self.params
            .iter()
            .filter(|param| !matches!(param.ty, FuncParamType::SelfType))
            .collect()
    }

    pub fn has_self_param(&self) -> bool {
        self.params
            .iter()
            .any(|param| matches!(param.ty, FuncParamType::SelfType))
    }
}

#[derive(Debug)]
pub struct FuncArgumentDesc {
    pub ty: FuncParamType,
    pub default: Option<TokenStream>,
    pub out_param: bool,
}

#[derive(Debug)]
pub struct ReturnTypeDesc {
    pub ty: Option<ParamType>,
    pub result: bool,
}
const META_TYPE_ERR: &str = "expected string literal or path";

#[derive(Clone, Debug, PartialEq)]
pub enum FuncParamType {
    SelfType,
    PlatformType(ParamType),
}

impl Display for FuncParamType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_str = match self {
            FuncParamType::SelfType => "Self".to_string(),
            FuncParamType::PlatformType(param_type) => format!("{param_type:?}"),
        };
        write!(f, "{}", type_str)
    }
}

impl FromMeta for FuncParamType {
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

impl TryFrom<&str> for FuncParamType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            // Базовые типы
            BOOL_TYPE => Ok(FuncParamType::PlatformType(ParamType::Bool)),
            I32_TYPE => Ok(FuncParamType::PlatformType(ParamType::I32)),
            F64_TYPE => Ok(FuncParamType::PlatformType(ParamType::F64)),
            STRING_TYPE => Ok(FuncParamType::PlatformType(ParamType::String)),
            DATE_TYPE => Ok(FuncParamType::PlatformType(ParamType::Date)),
            BLOB_TYPE => Ok(FuncParamType::PlatformType(ParamType::Blob)),
            
            // Дополнительные типы
            NULL_TYPE => Ok(FuncParamType::PlatformType(ParamType::Null)),
            I8_TYPE => Ok(FuncParamType::PlatformType(ParamType::I8)),
            I16_TYPE => Ok(FuncParamType::PlatformType(ParamType::I16)),
            I64_TYPE => Ok(FuncParamType::PlatformType(ParamType::I64)),
            U8_TYPE => Ok(FuncParamType::PlatformType(ParamType::U8)),
            U16_TYPE => Ok(FuncParamType::PlatformType(ParamType::U16)),
            U32_TYPE => Ok(FuncParamType::PlatformType(ParamType::U32)),
            U64_TYPE => Ok(FuncParamType::PlatformType(ParamType::U64)),
            F32_TYPE => Ok(FuncParamType::PlatformType(ParamType::F32)),
            DATE_DOUBLE_TYPE => Ok(FuncParamType::PlatformType(ParamType::DateDouble)),
            ANSI_STRING_TYPE => Ok(FuncParamType::PlatformType(ParamType::AnsiString)),
            ERROR_TYPE => Ok(FuncParamType::PlatformType(ParamType::Error)),
            HRESULT_TYPE => Ok(FuncParamType::PlatformType(ParamType::HResult)),
            CLS_ID_TYPE => Ok(FuncParamType::PlatformType(ParamType::ClsId)),
            
            _ => Err(())
        }
    }
}

impl ToTokens for FuncParamType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        *tokens = match self {
            FuncParamType::SelfType => panic!("type not supported for selection"),
            FuncParamType::PlatformType(param_type) => match param_type {
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
                ParamType::String => {
                    quote! { native_api_1c::native_api_1c_core::interface::ParamValue::String }
                }
                ParamType::Date => {
                    quote! { native_api_1c::native_api_1c_core::interface::ParamValue::Date }
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
                ParamType::DateDouble => {
                    quote! { native_api_1c::native_api_1c_core::interface::ParamValue::DateDouble }
                }
                ParamType::AnsiString => {
                    quote! { native_api_1c::native_api_1c_core::interface::ParamValue::AnsiString }
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
            },
        }
    }
}
