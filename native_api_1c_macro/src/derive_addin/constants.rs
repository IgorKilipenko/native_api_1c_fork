// Базовые типы (существующие)
pub const BOOL_TYPE: &str = "Bool";
pub const I32_TYPE: &str = "Int";
pub const F64_TYPE: &str = "Float";
pub const STRING_TYPE: &str = "Str";
pub const DATE_TYPE: &str = "Date";
pub const BLOB_TYPE: &str = "Blob";
pub const UNTYPED_TYPE: &str = "None";

// Дополнительные типы, поддерживаемые Native API (из types.h)
pub const NULL_TYPE: &str = "Null";
pub const I8_TYPE: &str = "I8";
pub const I16_TYPE: &str = "I16";
pub const I64_TYPE: &str = "I64";
pub const U8_TYPE: &str = "U8";
pub const U16_TYPE: &str = "U16";
pub const U32_TYPE: &str = "U32";
pub const U64_TYPE: &str = "U64";
pub const F32_TYPE: &str = "F32";
pub const DATE_DOUBLE_TYPE: &str = "DateDouble";
pub const ANSI_STRING_TYPE: &str = "AnsiStr";
pub const ERROR_TYPE: &str = "Error";
pub const HRESULT_TYPE: &str = "HResult";
pub const CLS_ID_TYPE: &str = "ClsId";

pub const ALL_RETURN_TYPES: &[&str] = &[
    BOOL_TYPE,
    I32_TYPE,
    F64_TYPE,
    STRING_TYPE,
    DATE_TYPE,
    BLOB_TYPE,
    NULL_TYPE,
    I8_TYPE,
    I16_TYPE,
    I64_TYPE,
    U8_TYPE,
    U16_TYPE,
    U32_TYPE,
    U64_TYPE,
    F32_TYPE,
    DATE_DOUBLE_TYPE,
    ANSI_STRING_TYPE,
    ERROR_TYPE,
    HRESULT_TYPE,
    CLS_ID_TYPE,
    UNTYPED_TYPE,
];

pub const ALL_ARG_TYPES: &[&str] = &[
    BOOL_TYPE,
    I32_TYPE,
    F64_TYPE,
    STRING_TYPE,
    DATE_TYPE,
    BLOB_TYPE,
    NULL_TYPE,
    I8_TYPE,
    I16_TYPE,
    I64_TYPE,
    U8_TYPE,
    U16_TYPE,
    U32_TYPE,
    U64_TYPE,
    F32_TYPE,
    DATE_DOUBLE_TYPE,
    ANSI_STRING_TYPE,
    ERROR_TYPE,
    HRESULT_TYPE,
    CLS_ID_TYPE,
];
