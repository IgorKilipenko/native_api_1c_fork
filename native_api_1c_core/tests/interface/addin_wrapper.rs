//! Tests for AddInWrapper trait

use std::sync::Arc;
use native_api_1c_core::{
    ffi::connection::Connection,
    interface::{AddInWrapper, ParamValue, ParamValues},
};

/// Simple AddIn implementation for testing
struct TestAddIn {
    connection: Arc<Option<&'static Connection>>,
    properties: Vec<(String, ParamValue, bool, bool)>, // (name, value, readable, writable)
    methods: Vec<(String, usize, bool)>, // (name, param_count, has_return)
}

impl TestAddIn {
    fn new() -> Self {
        Self {
            connection: Arc::new(None),
            properties: vec![
                ("TestProp".to_string(), ParamValue::I32(42), true, true),
                ("ReadOnlyProp".to_string(), ParamValue::Bool(true), true, false),
                ("WriteOnlyProp".to_string(), ParamValue::String(vec![0x0048, 0x0065, 0x006C, 0x006C, 0x006F]), false, true),
            ],
            methods: vec![
                ("TestMethod".to_string(), 2, true),
                ("TestProcedure".to_string(), 1, false),
            ],
        }
    }
}

impl AddInWrapper for TestAddIn {
    fn init(&mut self, interface: &'static Connection) -> bool {
        self.connection = Arc::new(Some(interface));
        true
    }

    fn done(&mut self) {
        // Cleanup if needed
    }

    fn register_extension_as(&mut self) -> &[u16] {
        &utf16_lit::utf16_null!("TestAddIn")
    }

    fn get_n_props(&self) -> usize {
        self.properties.len()
    }

    fn find_prop(&self, name: &[u16]) -> Option<usize> {
        let name_str = String::from_utf16_lossy(name);
        self.properties.iter().position(|(prop_name, _, _, _)| prop_name == &name_str)
    }

    fn get_prop_name(&self, num: usize, _alias: usize) -> Option<Vec<u16>> {
        self.properties.get(num).map(|(name, _, _, _)| {
            name.encode_utf16().collect()
        })
    }

    fn get_prop_val(&self, num: usize) -> Result<ParamValue, ()> {
        self.properties.get(num)
            .filter(|(_, _, readable, _)| *readable)
            .map(|(_, value, _, _)| value.clone())
            .ok_or(())
    }

    fn set_prop_val(&mut self, num: usize, val: ParamValue) -> Result<(), ()> {
        if let Some((_, _, _, writable)) = self.properties.get(num) {
            if *writable {
                if let Some((_, ref mut value, _, _)) = self.properties.get_mut(num) {
                    *value = val;
                    Ok(())
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    fn is_prop_readable(&self, num: usize) -> bool {
        self.properties.get(num).map(|(_, _, readable, _)| *readable).unwrap_or(false)
    }

    fn is_prop_writable(&self, num: usize) -> bool {
        self.properties.get(num).map(|(_, _, _, writable)| *writable).unwrap_or(false)
    }

    fn get_n_methods(&self) -> usize {
        self.methods.len()
    }

    fn find_method(&self, name: &[u16]) -> Option<usize> {
        let name_str = String::from_utf16_lossy(name);
        self.methods.iter().position(|(method_name, _, _)| method_name == &name_str)
    }

    fn get_method_name(&self, num: usize, _alias: usize) -> Option<Vec<u16>> {
        self.methods.get(num).map(|(name, _, _)| {
            name.encode_utf16().collect()
        })
    }

    fn get_n_params(&self, method_num: usize) -> usize {
        self.methods.get(method_num).map(|(_, param_count, _)| *param_count).unwrap_or(0)
    }

    fn get_param_def_value(&self, _method_num: usize, _param_num: usize) -> Option<ParamValue> {
        None // No default values for simplicity
    }

    fn has_ret_val(&self, method_num: usize) -> bool {
        self.methods.get(method_num).map(|(_, _, has_return)| *has_return).unwrap_or(false)
    }

    fn call_as_proc(&mut self, method_num: usize, _params: &mut ParamValues) -> Result<(), ()> {
        if method_num < self.methods.len() {
            Ok(()) // Valid method
        } else {
            Err(()) // Invalid method number
        }
    }

    fn call_as_func(&mut self, method_num: usize, _params: &mut ParamValues) -> Result<ParamValue, ()> {
        if let Some((_, _, has_return)) = self.methods.get(method_num) {
            if *has_return {
                Ok(ParamValue::I32(42)) // Return test value
            } else {
                Err(()) // Procedure has no return value
            }
        } else {
            Err(()) // Invalid method number
        }
    }

    fn set_locale(&mut self, _loc: &[u16]) {
        // Locale setting not implemented for simplicity
    }

    fn set_user_interface_language_code(&mut self, _lang: &[u16]) {
        // Language setting not implemented for simplicity
    }
}

#[test]
fn test_addin_wrapper_basic() {
    let addin = TestAddIn::new();
    
    // Test basic properties
    assert_eq!(addin.get_n_props(), 3);
    assert_eq!(addin.get_n_methods(), 2);
    
    // Test property finding
    let test_prop_name = "TestProp".encode_utf16().collect::<Vec<u16>>();
    assert_eq!(addin.find_prop(&test_prop_name), Some(0));
    
    let invalid_prop_name = "InvalidProp".encode_utf16().collect::<Vec<u16>>();
    assert_eq!(addin.find_prop(&invalid_prop_name), None);
}

#[test]
fn test_addin_wrapper_properties() {
    let addin = TestAddIn::new();
    
    // Test property names
    assert_eq!(addin.get_prop_name(0, 0), Some("TestProp".encode_utf16().collect()));
    assert_eq!(addin.get_prop_name(1, 0), Some("ReadOnlyProp".encode_utf16().collect()));
    assert_eq!(addin.get_prop_name(2, 0), Some("WriteOnlyProp".encode_utf16().collect()));
    
    // Test property values
    assert_eq!(addin.get_prop_val(0), Ok(ParamValue::I32(42)));
    assert_eq!(addin.get_prop_val(1), Ok(ParamValue::Bool(true)));
    assert_eq!(addin.get_prop_val(2), Err(())); // Write-only property
    
    // Test property permissions
    assert!(addin.is_prop_readable(0));
    assert!(addin.is_prop_writable(0));
    assert!(addin.is_prop_readable(1));
    assert!(!addin.is_prop_writable(1));
    assert!(!addin.is_prop_readable(2));
    assert!(addin.is_prop_writable(2));
}

#[test]
fn test_addin_wrapper_methods() {
    let addin = TestAddIn::new();
    
    // Test method names
    assert_eq!(addin.get_method_name(0, 0), Some("TestMethod".encode_utf16().collect()));
    assert_eq!(addin.get_method_name(1, 0), Some("TestProcedure".encode_utf16().collect()));
    
    // Test method parameters
    assert_eq!(addin.get_n_params(0), 2);
    assert_eq!(addin.get_n_params(1), 1);
    
    // Test return values
    assert!(addin.has_ret_val(0));
    assert!(!addin.has_ret_val(1));
}

#[test]
fn test_addin_wrapper_calls() {
    let mut addin = TestAddIn::new();
    
    // Test procedure call
    let mut params = ParamValues::new(vec![ParamValue::I32(10)]);
    let proc_result = addin.call_as_proc(1, &mut params);
    assert!(proc_result.is_ok());
    
    // Test function call
    let func_result = addin.call_as_func(0, &mut params);
    assert!(func_result.is_ok());
    assert_eq!(func_result.unwrap(), ParamValue::I32(42));
    
    // Test calling procedure as function (should fail)
    let func_result = addin.call_as_func(1, &mut params);
    assert!(func_result.is_err());
}

#[test]
fn test_addin_wrapper_edge_cases() {
    let mut addin = TestAddIn::new();
    
    // Test invalid indices
    assert_eq!(addin.get_prop_name(999, 0), None);
    assert_eq!(addin.get_method_name(999, 0), None);
    assert_eq!(addin.get_n_params(999), 0);
    assert!(!addin.has_ret_val(999));
    
    // Test property operations on invalid indices
    assert!(addin.get_prop_val(999).is_err());
    assert!(addin.set_prop_val(999, ParamValue::I32(0)).is_err());
    assert!(!addin.is_prop_readable(999));
    assert!(!addin.is_prop_writable(999));
    
    // Test method operations on invalid indices
    let mut params = ParamValues::new(vec![]);
    assert!(addin.call_as_proc(999, &mut params).is_err());
    assert!(addin.call_as_func(999, &mut params).is_err());
}

#[test]
fn test_addin_wrapper_connection() {
    let mut addin = TestAddIn::new();
    
    // Mock connection
    let connection = unsafe { 
        std::mem::transmute(0x1 as *const Connection) 
    };
    
    // Test initialization
    let init_result = addin.init(connection);
    assert!(init_result);
    
    // Test that connection was set
    assert!(addin.connection.is_some());
}
