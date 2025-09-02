//! Tests for ParamValues struct

use native_api_1c_core::interface::{ParamValue, ParamValues};

#[test]
fn test_param_values_creation() {
    let empty_values = ParamValues::new(vec![]);
    assert!(empty_values.is_empty());
    assert_eq!(empty_values.len(), 0);
    
    let single_value = ParamValues::new(vec![ParamValue::I32(42)]);
    assert!(!single_value.is_empty());
    assert_eq!(single_value.len(), 1);
    
    let multiple_values = ParamValues::new(vec![
        ParamValue::Bool(true),
        ParamValue::String(vec![0x0048, 0x0065, 0x006C, 0x006C, 0x006F]), // "Hello"
        ParamValue::F64(3.14),
    ]);
    assert!(!multiple_values.is_empty());
    assert_eq!(multiple_values.len(), 3);
}

#[test]
fn test_param_values_indexing() {
    let values = ParamValues::new(vec![
        ParamValue::I32(10),
        ParamValue::Bool(true),
        ParamValue::String(vec![0x0054, 0x0065, 0x0073, 0x0074]), // "Test"
    ]);
    
    // Test indexing
    assert_eq!(values[0], ParamValue::I32(10));
    assert_eq!(values[1], ParamValue::Bool(true));
    assert_eq!(values[2], ParamValue::String(vec![0x0054, 0x0065, 0x0073, 0x0074]));
    
    // Test mutable indexing
    let mut mutable_values = values;
    mutable_values[0] = ParamValue::I32(20);
    mutable_values[1] = ParamValue::Bool(false);
    
    assert_eq!(mutable_values[0], ParamValue::I32(20));
    assert_eq!(mutable_values[1], ParamValue::Bool(false));
}

#[test]
fn test_param_values_iteration() {
    let values = ParamValues::new(vec![
        ParamValue::I32(1),
        ParamValue::I32(2),
        ParamValue::I32(3),
    ]);
    
    let mut iter = values.iter();
    
    assert_eq!(iter.next(), Some(&ParamValue::I32(1)));
    assert_eq!(iter.next(), Some(&ParamValue::I32(2)));
    assert_eq!(iter.next(), Some(&ParamValue::I32(3)));
    assert_eq!(iter.next(), None);
    
    // Test collect
    let collected: Vec<&ParamValue> = values.iter().collect();
    assert_eq!(collected.len(), 3);
    assert_eq!(collected[0], &ParamValue::I32(1));
    assert_eq!(collected[1], &ParamValue::I32(2));
    assert_eq!(collected[2], &ParamValue::I32(3));
}

#[test]
fn test_param_values_basic_operations() {
    let values = ParamValues::new(vec![
        ParamValue::Bool(true),
        ParamValue::F64(2.718),
    ]);
    
    // Test basic operations
    assert_eq!(values.len(), 2);
    assert_eq!(values[0], ParamValue::Bool(true));
    assert_eq!(values[1], ParamValue::F64(2.718));
}

#[test]
fn test_param_values_empty() {
    let empty = ParamValues::new(vec![]);
    
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);
    
    let mut iter = empty.iter();
    assert_eq!(iter.next(), None);
}

#[test]
fn test_param_values_single_element() {
    let single = ParamValues::new(vec![ParamValue::I32(42)]);
    
    assert!(!single.is_empty());
    assert_eq!(single.len(), 1);
    
    assert_eq!(single[0], ParamValue::I32(42));
    
    let mut iter = single.iter();
    assert_eq!(iter.next(), Some(&ParamValue::I32(42)));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_param_values_mixed_types() {
    let mixed = ParamValues::new(vec![
        ParamValue::Empty,
        ParamValue::I32(-100),
        ParamValue::F64(0.001),
        ParamValue::Bool(false),
        ParamValue::String(vec![0x0048, 0x0065, 0x006C, 0x006C, 0x006F]), // "Hello"
        ParamValue::Blob(vec![0xAA, 0xBB, 0xCC]),
    ]);
    
    assert_eq!(mixed.len(), 6);
    assert_eq!(mixed[0], ParamValue::Empty);
    assert_eq!(mixed[1], ParamValue::I32(-100));
    assert_eq!(mixed[2], ParamValue::F64(0.001));
    assert_eq!(mixed[3], ParamValue::Bool(false));
    assert_eq!(mixed[4], ParamValue::String(vec![0x0048, 0x0065, 0x006C, 0x006C, 0x006F]));
    assert_eq!(mixed[5], ParamValue::Blob(vec![0xAA, 0xBB, 0xCC]));
}

#[test]
fn test_param_values_mutability() {
    let mut values = ParamValues::new(vec![
        ParamValue::I32(1),
        ParamValue::Bool(true),
    ]);
    
    // Test mutable access
    values[0] = ParamValue::I32(999);
    values[1] = ParamValue::Bool(false);
    
    assert_eq!(values[0], ParamValue::I32(999));
    assert_eq!(values[1], ParamValue::Bool(false));
}

#[test]
fn test_param_values_original_preserved() {
    let original = ParamValues::new(vec![
        ParamValue::I32(1),
        ParamValue::Bool(true),
    ]);
    
    assert_eq!(original[0], ParamValue::I32(1));
    assert_eq!(original[1], ParamValue::Bool(true));
}

#[test]
fn test_param_values_edge_cases() {
    // Test with maximum values
    let max_values = ParamValues::new(vec![
        ParamValue::I32(i32::MAX),
        ParamValue::I32(i32::MIN),
        ParamValue::F64(f64::MAX),
        ParamValue::F64(f64::MIN),
    ]);
    
    assert_eq!(max_values.len(), 4);
    assert_eq!(max_values[0], ParamValue::I32(i32::MAX));
    assert_eq!(max_values[1], ParamValue::I32(i32::MIN));
    assert_eq!(max_values[2], ParamValue::F64(f64::MAX));
    assert_eq!(max_values[3], ParamValue::F64(f64::MIN));
    
    // Test with empty collections
    let empty_string = ParamValues::new(vec![ParamValue::String(vec![])]);
    let empty_blob = ParamValues::new(vec![ParamValue::Blob(vec![])]);
    
    assert_eq!(empty_string.len(), 1);
    assert_eq!(empty_blob.len(), 1);
    
    // Test empty string
    if let ParamValue::String(s) = &empty_string[0] {
        assert!(s.is_empty());
    }
    
    // Test empty blob
    if let ParamValue::Blob(b) = &empty_blob[0] {
        assert!(b.is_empty());
    }
}
