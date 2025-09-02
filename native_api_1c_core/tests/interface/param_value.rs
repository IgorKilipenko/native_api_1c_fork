//! Tests for ParamValue enum

use native_api_1c_core::interface::ParamValue;
use native_api_1c_core::ffi::provided_types::Tm;

#[test]
fn test_param_value_creation() {
    let empty = ParamValue::Empty;
    let bool_val = ParamValue::Bool(true);
    let int_val = ParamValue::I32(42);
    let float_val = ParamValue::F64(3.14);
    let string_val = ParamValue::String(vec![0x0048, 0x0065, 0x006C, 0x006C, 0x006F]); // "Hello"
    let blob_val = ParamValue::Blob(vec![0xAA, 0xBB, 0xCC, 0xDD]);
    
    // Test that all variants can be created
    assert!(matches!(empty, ParamValue::Empty));
    assert!(matches!(bool_val, ParamValue::Bool(true)));
    assert!(matches!(int_val, ParamValue::I32(42)));
    assert!(matches!(float_val, ParamValue::F64(3.14)));
    assert!(matches!(string_val, ParamValue::String(_)));
    assert!(matches!(blob_val, ParamValue::Blob(_)));
}

#[test]
fn test_param_value_setters() {
    let mut value = ParamValue::Empty;
    
    // Test bool setter
    value.set_bool(true);
    assert!(matches!(value, ParamValue::Bool(true)));
    
    // Test i32 setter
    value.set_i32(123);
    assert!(matches!(value, ParamValue::I32(123)));
    
    // Test f64 setter
    value.set_f64(2.718);
    assert!(matches!(value, ParamValue::F64(2.718)));
    
    // Test string setter
    let test_string = vec![0x0054, 0x0065, 0x0073, 0x0074]; // "Test"
    value.set_str(test_string.clone());
    assert!(matches!(value, ParamValue::String(_)));
    if let ParamValue::String(s) = &value {
        assert_eq!(s, &test_string);
    }
    
    // Test blob setter
    let test_blob = vec![0x11, 0x22, 0x33, 0x44];
    value.set_blob(test_blob.clone());
    assert!(matches!(value, ParamValue::Blob(_)));
    if let ParamValue::Blob(b) = &value {
        assert_eq!(b, &test_blob);
    }
}

#[test]
fn test_param_value_date() {
    let mut value = ParamValue::Empty;
    
    // Create a test date
    let test_date = Tm {
        sec: 30,
        min: 15,
        hour: 12,
        mday: 25,
        mon: 11, // December (0-based)
        year: 2023 - 1900, // Years since 1900
        wday: 1, // Monday
        yday: 358, // Day of year
        isdst: 0,
        #[cfg(target_family = "unix")]
        gmtoff: 0,
        #[cfg(target_family = "unix")]
        zone: 0,
    };
    
    value.set_date(test_date);
    assert!(matches!(value, ParamValue::Date(_)));
    
    if let ParamValue::Date(date) = value {
        assert_eq!(date.year, test_date.year);
        assert_eq!(date.mon, test_date.mon);
        assert_eq!(date.mday, test_date.mday);
    }
}

#[test]
fn test_param_value_partial_eq() {
    let empty1 = ParamValue::Empty;
    let empty2 = ParamValue::Empty;
    let bool_true = ParamValue::Bool(true);
    let bool_false = ParamValue::Bool(false);
    let int_42 = ParamValue::I32(42);
    let int_100 = ParamValue::I32(100);
    
    // Empty values should be equal
    assert_eq!(empty1, empty2);
    
    // Different types should not be equal
    assert_ne!(empty1, bool_true);
    assert_ne!(bool_true, int_42);
    
    // Same type, different values should not be equal
    assert_ne!(bool_true, bool_false);
    assert_ne!(int_42, int_100);
    
    // Same type, same values should be equal
    assert_eq!(bool_true, ParamValue::Bool(true));
    assert_eq!(int_42, ParamValue::I32(42));
}

#[test]
fn test_param_value_clone() {
    let original = ParamValue::String(vec![0x0048, 0x0065, 0x006C, 0x006C, 0x006F]); // "Hello"
    let cloned = original.clone();
    
    assert_eq!(original, cloned);
    
    // Modify the clone and verify they're different
    let mut modified = cloned;
    modified.set_str(vec![0x0057, 0x006F, 0x0072, 0x006C, 0x0064]); // "World"
    
    assert_ne!(original, modified);
}

#[test]
fn test_param_value_debug() {
    let values = vec![
        ParamValue::Empty,
        ParamValue::Bool(true),
        ParamValue::I32(42),
        ParamValue::F64(3.14),
        ParamValue::String(vec![0x0048, 0x0065, 0x006C, 0x006C, 0x006F]), // "Hello"
        ParamValue::Blob(vec![0xAA, 0xBB, 0xCC]),
    ];
    
    // Test that all values can be formatted with Debug
    for value in values {
        let debug_str = format!("{:?}", value);
        assert!(!debug_str.is_empty());
    }
}

#[test]
fn test_param_value_edge_cases() {
    // Test empty string
    let mut value = ParamValue::Empty;
    value.set_str(vec![]);
    assert!(matches!(value, ParamValue::String(s) if s.is_empty()));
    
    // Test empty blob
    let mut value2 = ParamValue::Empty;
    value2.set_blob(vec![]);
    assert!(matches!(value2, ParamValue::Blob(b) if b.is_empty()));
    
    // Test large values
    let large_int = ParamValue::I32(i32::MAX);
    assert!(matches!(large_int, ParamValue::I32(i32::MAX)));
    
    let large_float = ParamValue::F64(f64::MAX);
    assert!(matches!(large_float, ParamValue::F64(f64::MAX)));
    
    // Test negative values
    let negative_int = ParamValue::I32(-42);
    assert!(matches!(negative_int, ParamValue::I32(-42)));
    
    let negative_float = ParamValue::F64(-3.14);
    assert!(matches!(negative_float, ParamValue::F64(-3.14)));
}
