//! Тесты для поддержки VARIANT типов из 1C Native API
//! 
//! Этот модуль содержит тесты для всех поддерживаемых типов VARIANT,
//! включая новые типы, которые планируется добавить.

use native_api_1c_core::ffi::provided_types::{TVariant, VariantType, VariantValue};
use native_api_1c_core::interface::ParamValue;
use native_api_1c_core::errors::NativeApiError;

/// Тесты для базовых типов VARIANT
#[cfg(test)]
mod basic_types {
    use super::*;

    #[test]
    fn test_vtype_empty() {
        let variant = TVariant::default();
        assert_eq!(variant.vt, VariantType::Empty);
    }

    #[test]
    fn test_vtype_bool() {
        let mut variant = TVariant::default();
        variant.update_to_bool(true);
        assert_eq!(variant.vt, VariantType::Bool);
        assert_eq!(unsafe { variant.value.bool }, true);
    }

    #[test]
    fn test_vtype_i32() {
        let mut variant = TVariant::default();
        variant.update_to_i32(42);
        assert_eq!(variant.vt, VariantType::Int32);
        assert_eq!(unsafe { variant.value.i32 }, 42);
    }

    #[test]
    fn test_vtype_f64() {
        let mut variant = TVariant::default();
        variant.update_to_f64(3.14);
        assert_eq!(variant.vt, VariantType::Double);
        assert_eq!(unsafe { variant.value.f64 }, 3.14);
    }
}

/// Тесты для новых типов VARIANT (планируемые)
#[cfg(test)]
mod new_types {
    use super::*;

    #[test]
    fn test_vtype_null() {
        let mut variant = TVariant::default();
        variant.update_to_null();
        assert_eq!(variant.vt, VariantType::Null);
    }

    #[test]
    fn test_vtype_i8() {
        let mut variant = TVariant::default();
        variant.update_to_i8(127);
        assert_eq!(variant.vt, VariantType::Int8);
        assert_eq!(unsafe { variant.value.i8 }, 127);
    }

    #[test]
    fn test_vtype_i16() {
        let mut variant = TVariant::default();
        variant.update_to_i16(32000);
        assert_eq!(variant.vt, VariantType::Int16);
        assert_eq!(unsafe { variant.value.i16 }, 32000);
    }

    #[test]
    fn test_vtype_i64() {
        let mut variant = TVariant::default();
        variant.update_to_i64(9223372036854775807);
        assert_eq!(variant.vt, VariantType::Int64);
        assert_eq!(unsafe { variant.value.i64 }, 9223372036854775807);
    }

    #[test]
    fn test_vtype_u8() {
        let mut variant = TVariant::default();
        variant.update_to_u8(255);
        assert_eq!(variant.vt, VariantType::UInt8);
        assert_eq!(unsafe { variant.value.u8 }, 255);
    }

    #[test]
    fn test_vtype_u16() {
        let mut variant = TVariant::default();
        variant.update_to_u16(65535);
        assert_eq!(variant.vt, VariantType::UInt16);
        assert_eq!(unsafe { variant.value.u16 }, 65535);
    }

    #[test]
    fn test_vtype_u32() {
        let mut variant = TVariant::default();
        variant.update_to_u32(4294967295);
        assert_eq!(variant.vt, VariantType::UInt32);
        assert_eq!(unsafe { variant.value.u32 }, 4294967295);
    }

    #[test]
    fn test_vtype_u64() {
        let mut variant = TVariant::default();
        variant.update_to_u64(18446744073709551615);
        assert_eq!(variant.vt, VariantType::UInt64);
        assert_eq!(unsafe { variant.value.u64 }, 18446744073709551615);
    }

    #[test]
    fn test_vtype_f32() {
        let mut variant = TVariant::default();
        variant.update_to_f32(3.14f32);
        assert_eq!(variant.vt, VariantType::Float);
        assert_eq!(unsafe { variant.value.f32 }, 3.14f32);
    }

    #[test]
    fn test_vtype_date() {
        let mut variant = TVariant::default();
        variant.update_to_date_double(44197.5); // 2021-01-01 12:00:00
        assert_eq!(variant.vt, VariantType::Date);
        assert_eq!(unsafe { variant.value.date }, 44197.5);
    }

    #[test]
    fn test_vtype_error() {
        let mut variant = TVariant::default();
        variant.update_to_error(-2147024809); // E_INVALIDARG
        assert_eq!(variant.vt, VariantType::Error);
        assert_eq!(unsafe { variant.value.error }, -2147024809);
    }

    #[test]
    fn test_vtype_cls_id() {
        let mut variant = TVariant::default();
        let uuid = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 
                   0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0];
        variant.update_to_cls_id(uuid);
        assert_eq!(variant.vt, VariantType::ClsID);
        assert_eq!(unsafe { variant.value.cls_id }, uuid);
    }
}

/// Тесты для конвертации между TVariant и ParamValue
#[cfg(test)]
mod conversion_tests {
    use super::*;

    #[test]
    fn test_tvariant_to_param_value() {
        // Тест конвертации TVariant -> ParamValue для новых типов
        let mut variant = TVariant::default();
        
        // Тест I8
        variant.update_to_i8(127);
        let param = ParamValue::from(&variant);
        assert_eq!(param, ParamValue::I8(127));
        
        // Тест U32
        variant.update_to_u32(4294967295);
        let param = ParamValue::from(&variant);
        assert_eq!(param, ParamValue::U32(4294967295));
        
        // Тест F32
        variant.update_to_f32(3.14f32);
        let param = ParamValue::from(&variant);
        assert_eq!(param, ParamValue::F32(3.14f32));
        
        // Тест DateDouble
        variant.update_to_date_double(44197.5);
        let param = ParamValue::from(&variant);
        assert_eq!(param, ParamValue::DateDouble(44197.5));
        
        // Тест Error
        variant.update_to_error(-2147024809);
        let param = ParamValue::from(&variant);
        assert_eq!(param, ParamValue::Error(-2147024809));
        
        // Тест ClsId
        let uuid = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 
                   0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0];
        variant.update_to_cls_id(uuid);
        let param = ParamValue::from(&variant);
        assert_eq!(param, ParamValue::ClsId(uuid));
    }
}

/// Тесты для валидации типов
#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_type_validation() {
        // TODO: Реализовать валидацию типов
        // let mut variant = TVariant::default();
        // variant.update_to_i8(127);
        // assert!(variant.validate_type().is_ok());
        
        // variant.vt = VariantType::Int8;
        // variant.value.i32 = 1000; // Неверное значение для i8
        // assert!(variant.validate_type().is_err());
    }
}

/// Тесты производительности
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_conversion_performance() {
        // TODO: Реализовать тесты производительности
        // let start = Instant::now();
        // for _ in 0..10000 {
        //     let param = ParamValue::I64(9223372036854775807);
        //     let _variant = param.to_tvariant().unwrap();
        // }
        // let duration = start.elapsed();
        // assert!(duration.as_millis() < 10); // Менее 10ms для 10000 конвертаций
    }
}

/// Интеграционные тесты с макросами
#[cfg(test)]
mod macro_integration_tests {
    use super::*;

    #[test]
    fn test_macro_support_new_types() {
        // TODO: Протестировать поддержку новых типов в макросах
        // #[derive(AddIn)]
        // struct TestAddIn {
        //     #[prop(ty = I64, name = "TestI64")]
        //     pub test_i64: i64,
        //     
        //     #[prop(ty = U32, name = "TestU32")]
        //     pub test_u32: u32,
        // }
    }
}
