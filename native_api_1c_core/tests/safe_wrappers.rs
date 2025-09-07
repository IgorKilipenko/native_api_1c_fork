//! Тесты для безопасных оберток TVariant

use native_api_1c_core::safe_wrappers::{SafeVariant, SafeVariantArray};
use native_api_1c_core::interface::ParamValue;
use native_api_1c_core::ffi::provided_types::VariantType;

#[cfg(test)]
mod safe_variant_tests {
    use super::*;

    #[test]
    fn test_safe_variant_creation() {
        let variant = SafeVariant::new();
        assert!(variant.is_empty());
        assert_eq!(variant.get_type(), VariantType::Empty);
    }

    #[test]
    fn test_safe_variant_bool() {
        let mut variant = SafeVariant::new();
        
        // Устанавливаем булево значение
        variant.set_bool(true);
        assert_eq!(variant.get_type(), VariantType::Bool);
        assert_eq!(variant.get_bool().unwrap(), true);
        
        variant.set_bool(false);
        assert_eq!(variant.get_bool().unwrap(), false);
    }

    #[test]
    fn test_safe_variant_i32() {
        let mut variant = SafeVariant::new();
        
        // Устанавливаем целое число
        variant.set_i32(42);
        assert_eq!(variant.get_type(), VariantType::Int32);
        assert_eq!(variant.get_i32().unwrap(), 42);
        
        variant.set_i32(-100);
        assert_eq!(variant.get_i32().unwrap(), -100);
    }

    #[test]
    fn test_safe_variant_f64() {
        let mut variant = SafeVariant::new();
        
        // Устанавливаем число с плавающей точкой
        variant.set_f64(3.14159);
        assert_eq!(variant.get_type(), VariantType::Double);
        assert_eq!(variant.get_f64().unwrap(), 3.14159);
        
        variant.set_f64(-2.71828);
        assert_eq!(variant.get_f64().unwrap(), -2.71828);
    }

    #[test]
    fn test_safe_variant_date() {
        let mut variant = SafeVariant::new();
        
        // Устанавливаем дату (Windows DATE format)
        variant.set_date(44197.0); // Пример даты
        assert_eq!(variant.get_type(), VariantType::Date);
        assert_eq!(variant.get_date().unwrap(), 44197.0);
    }

    #[test]
    fn test_safe_variant_error() {
        let mut variant = SafeVariant::new();
        
        // Устанавливаем код ошибки
        variant.set_error(1001);
        assert_eq!(variant.get_type(), VariantType::Error);
        assert_eq!(variant.get_error().unwrap(), 1001);
    }


    #[test]
    fn test_safe_variant_cls_id() {
        let mut variant = SafeVariant::new();
        
        // Устанавливаем CLSID
        let cls_id = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        variant.set_cls_id(cls_id);
        assert_eq!(variant.get_type(), VariantType::ClsID);
        assert_eq!(variant.get_cls_id().unwrap(), cls_id);
    }

    #[test]
    fn test_safe_variant_null() {
        let mut variant = SafeVariant::new();
        
        // Устанавливаем NULL
        variant.set_null();
        assert!(variant.is_null());
        assert_eq!(variant.get_type(), VariantType::Null);
    }

    #[test]
    fn test_safe_variant_type_mismatch() {
        let mut variant = SafeVariant::new();
        
        // Устанавливаем булево значение
        variant.set_bool(true);
        
        // Пытаемся получить как i32 - должна быть ошибка
        assert!(variant.get_i32().is_err());
        assert!(variant.get_f64().is_err());
        assert!(variant.get_date().is_err());
    }

    #[test]
    fn test_safe_variant_param_value_conversion() {
        // Тест преобразования в ParamValue
        let mut variant = SafeVariant::new();
        variant.set_i32(123);
        
        let param_value = variant.to_param_value();
        match param_value {
            ParamValue::I32(value) => assert_eq!(value, 123),
            _ => panic!("Expected I32 variant"),
        }
        
        // Тест создания из ParamValue
        let param_value = ParamValue::Bool(true);
        let variant = SafeVariant::from_param_value(&param_value);
        assert_eq!(variant.get_type(), VariantType::Bool);
        assert_eq!(variant.get_bool().unwrap(), true);
    }

    #[test]
    fn test_safe_variant_from_tvariant() {
        let mut variant = SafeVariant::new();
        variant.set_f64(3.14);
        
        let tvariant = variant.into_tvariant();
        let new_variant = SafeVariant::from_tvariant(tvariant);
        
        assert_eq!(new_variant.get_type(), VariantType::Double);
        assert_eq!(new_variant.get_f64().unwrap(), 3.14);
    }
}

#[cfg(test)]
mod safe_variant_array_tests {
    use super::*;

    #[test]
    fn test_safe_variant_array_creation() {
        let array = SafeVariantArray::new();
        assert!(array.is_empty());
        assert_eq!(array.len(), 0);
    }

    #[test]
    fn test_safe_variant_array_with_capacity() {
        let array = SafeVariantArray::with_capacity(10);
        assert!(array.is_empty());
        assert_eq!(array.len(), 0);
    }

    #[test]
    fn test_safe_variant_array_push_and_get() {
        let mut array = SafeVariantArray::new();
        
        // Добавляем элементы
        let mut variant1 = SafeVariant::new();
        variant1.set_i32(100);
        array.push(variant1);
        
        let mut variant2 = SafeVariant::new();
        variant2.set_bool(true);
        array.push(variant2);
        
        assert_eq!(array.len(), 2);
        assert!(!array.is_empty());
        
        // Получаем элементы
        let first = array.get(0).unwrap();
        assert_eq!(first.get_i32().unwrap(), 100);
        
        let second = array.get(1).unwrap();
        assert_eq!(second.get_bool().unwrap(), true);
        
        // Проверяем несуществующий индекс
        assert!(array.get(2).is_none());
    }

    #[test]
    fn test_safe_variant_array_mut_access() {
        let mut array = SafeVariantArray::new();
        
        let mut variant = SafeVariant::new();
        variant.set_i32(50);
        array.push(variant);
        
        // Изменяем элемент
        let first = array.get_mut(0).unwrap();
        first.set_i32(75);
        
        // Проверяем изменение
        let first = array.get(0).unwrap();
        assert_eq!(first.get_i32().unwrap(), 75);
    }

    #[test]
    fn test_safe_variant_array_from_vec() {
        let mut variant1 = SafeVariant::new();
        variant1.set_i32(10);
        
        let mut variant2 = SafeVariant::new();
        variant2.set_f64(2.5);
        
        let vec = vec![variant1, variant2];
        let array = SafeVariantArray::from_vec(vec);
        
        assert_eq!(array.len(), 2);
        assert_eq!(array.get(0).unwrap().get_i32().unwrap(), 10);
        assert_eq!(array.get(1).unwrap().get_f64().unwrap(), 2.5);
    }

    #[test]
    fn test_safe_variant_array_into_vec() {
        let mut array = SafeVariantArray::new();
        
        let mut variant = SafeVariant::new();
        variant.set_bool(false);
        array.push(variant);
        
        let vec = array.into_vec();
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0].get_bool().unwrap(), false);
    }
}
