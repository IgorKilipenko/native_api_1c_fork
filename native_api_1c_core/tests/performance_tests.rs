//! Тесты производительности для новых VARIANT типов

use native_api_1c_core::interface::ParamValue;
use native_api_1c_core::ffi::provided_types::{TVariant, VariantType};
use native_api_1c_core::safe_wrappers::SafeVariant;
use std::time::Instant;

/// Тест производительности создания и конвертации ParamValue
#[test]
fn test_param_value_creation_performance() {
    let iterations = 100_000;
    
    // Тест создания различных типов ParamValue
    let start = Instant::now();
    for i in 0..iterations {
        let _values = vec![
            ParamValue::Null,
            ParamValue::I8(i as i8),
            ParamValue::I16(i as i16),
            ParamValue::I32(i as i32),
            ParamValue::I64(i as i64),
            ParamValue::U8(i as u8),
            ParamValue::U16(i as u16),
            ParamValue::U32(i as u32),
            ParamValue::U64(i as u64),
            ParamValue::F32(i as f32),
            ParamValue::F64(i as f64),
            ParamValue::Bool(i % 2 == 0),
        ];
    }
    let duration = start.elapsed();
    
    println!("ParamValue creation: {} iterations in {:?}", iterations, duration);
    println!("Average per iteration: {:?}", duration / iterations);
    
    // Проверяем, что время разумное (менее 1 секунды для 100k итераций)
    assert!(duration.as_secs() < 1, "ParamValue creation too slow: {:?}", duration);
}

/// Тест производительности конвертации ParamValue <-> TVariant
#[test]
fn test_param_value_tvariant_conversion_performance() {
    let iterations = 50_000;
    let test_values = vec![
        ParamValue::Null,
        ParamValue::I8(42),
        ParamValue::I16(1234),
        ParamValue::I32(123456),
        ParamValue::I64(123456789),
        ParamValue::U8(255),
        ParamValue::U16(65535),
        ParamValue::U32(4294967295),
        ParamValue::U64(18446744073709551615),
        ParamValue::F32(3.14159),
        ParamValue::F64(2.718281828),
        ParamValue::Bool(true),
    ];
    
    let start = Instant::now();
    for _ in 0..iterations {
        for value in &test_values {
            let safe_variant = SafeVariant::from(value);
            let tvariant = TVariant::from(safe_variant);
            let _converted_back = ParamValue::from(&tvariant);
        }
    }
    let duration = start.elapsed();
    
    println!("ParamValue <-> TVariant conversion: {} iterations in {:?}", 
             iterations * test_values.len(), duration);
    println!("Average per conversion: {:?}", 
             duration / (iterations as u32 * test_values.len() as u32));
    
    // Проверяем, что время разумное
    assert!(duration.as_secs() < 2, "Conversion too slow: {:?}", duration);
}

/// Тест производительности SafeVariant
#[test]
fn test_safe_variant_performance() {
    let iterations = 25_000;
    
    let start = Instant::now();
    for i in 0..iterations {
        let mut safe_variant = SafeVariant::new();
        
        // Тестируем различные операции
        safe_variant.set_i32(i as i32);
        let _value = safe_variant.get_i32().unwrap();
        
        safe_variant.set_f64(i as f64);
        let _value = safe_variant.get_f64().unwrap();
        
        safe_variant.set_bool(i % 2 == 0);
        let _value = safe_variant.get_bool().unwrap();
        
        // Конвертация в ParamValue и обратно
        let param_value = ParamValue::from(safe_variant);
        let _back_to_safe = SafeVariant::from_param_value(&param_value);
    }
    let duration = start.elapsed();
    
    println!("SafeVariant operations: {} iterations in {:?}", iterations, duration);
    println!("Average per iteration: {:?}", duration / iterations);
    
    // Проверяем, что время разумное
    assert!(duration.as_secs() < 2, "SafeVariant operations too slow: {:?}", duration);
}

/// Тест производительности работы с массивами
#[test]
fn test_variant_array_performance() {
    let iterations = 10_000;
    let array_size = 100;
    
    let start = Instant::now();
    for _ in 0..iterations {
        let mut array = native_api_1c_core::safe_wrappers::SafeVariantArray::new();
        
        // Заполняем массив
        for i in 0..array_size {
            let mut variant = SafeVariant::new();
            variant.set_i32(i as i32);
            array.push(variant);
        }
        
        // Читаем из массива
        for i in 0..array_size {
            let _value = array.get(i).unwrap().get_i32().unwrap();
        }
        
        // Конвертируем в Vec и обратно
        let _vec: Vec<SafeVariant> = array.into();
    }
    let duration = start.elapsed();
    
    println!("Variant array operations: {} iterations ({} elements each) in {:?}", 
             iterations, array_size, duration);
    println!("Average per iteration: {:?}", duration / iterations);
    
    // Проверяем, что время разумное
    assert!(duration.as_secs() < 3, "Variant array operations too slow: {:?}", duration);
}

/// Тест производительности сравнения типов
#[test]
fn test_type_comparison_performance() {
    let iterations = 100_000;
    let test_types = vec![
        VariantType::Null,
        VariantType::Int8,
        VariantType::Int16,
        VariantType::Int32,
        VariantType::Int64,
        VariantType::UInt8,
        VariantType::UInt16,
        VariantType::UInt32,
        VariantType::UInt64,
        VariantType::Float,
        VariantType::Double,
        VariantType::Bool,
        VariantType::Date,
        VariantType::Error,
        VariantType::Interface,
    ];
    
    let start = Instant::now();
    for _ in 0..iterations {
        for (i, type1) in test_types.iter().enumerate() {
            for (j, type2) in test_types.iter().enumerate() {
                let _equal = type1 == type2;
                let _index_sum = i + j; // Простая операция для предотвращения оптимизации
            }
        }
    }
    let duration = start.elapsed();
    
    println!("Type comparison: {} iterations in {:?}", iterations, duration);
    println!("Average per iteration: {:?}", duration / iterations);
    
    // Проверяем, что время разумное
    assert!(duration.as_secs() < 2, "Type comparison too slow: {:?}", duration);
}

/// Тест производительности создания больших структур
#[test]
fn test_large_structure_performance() {
    let iterations = 1_000;
    let structure_size = 1_000;
    
    let start = Instant::now();
    for _ in 0..iterations {
        let mut variants = Vec::with_capacity(structure_size);
        
        for i in 0..structure_size {
            let mut variant = SafeVariant::new();
            variant.set_i32(i as i32);
            variants.push(variant);
        }
        
        // Конвертируем в ParamValue массив
        let param_values: Vec<ParamValue> = variants.into_iter()
            .map(|v| ParamValue::from(v))
            .collect();
        
        // Конвертируем обратно
        let _back_to_variants: Vec<SafeVariant> = param_values.iter()
            .map(|pv| SafeVariant::from_param_value(pv))
            .collect();
    }
    let duration = start.elapsed();
    
    println!("Large structure operations: {} iterations ({} elements each) in {:?}", 
             iterations, structure_size, duration);
    println!("Average per iteration: {:?}", duration / iterations);
    
    // Проверяем, что время разумное
    assert!(duration.as_secs() < 5, "Large structure operations too slow: {:?}", duration);
}
