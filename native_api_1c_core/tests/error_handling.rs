//! Tests for error handling

use native_api_1c_core::{
    errors::{NativeApiError, PropertyError, MethodError},
    interface::{AddInWrapper, ParamValue, ParamValues},
};

/// Mock AddIn that can simulate errors
struct ErrorProneAddIn {
    should_fail_init: bool,
    should_fail_props: bool,
    should_fail_methods: bool,
}

impl ErrorProneAddIn {
    fn new() -> Self {
        Self {
            should_fail_init: false,
            should_fail_props: false,
            should_fail_methods: false,
        }
    }

    fn with_init_failure() -> Self {
        Self {
            should_fail_init: true,
            should_fail_props: false,
            should_fail_methods: false,
        }
    }

    fn with_prop_failures() -> Self {
        Self {
            should_fail_init: false,
            should_fail_props: true,
            should_fail_methods: false,
        }
    }

    fn with_method_failures() -> Self {
        Self {
            should_fail_init: false,
            should_fail_props: false,
            should_fail_methods: true,
        }
    }
}

impl AddInWrapper for ErrorProneAddIn {
    fn init(&mut self, _interface: &'static native_api_1c_core::ffi::connection::Connection) -> bool {
        !self.should_fail_init
    }

    fn done(&mut self) {}

    fn register_extension_as(&mut self) -> &[u16] {
        &utf16_lit::utf16_null!("ErrorProneAddIn")
    }

    fn get_n_props(&self) -> usize {
        if self.should_fail_props {
            0
        } else {
            1
        }
    }

    fn find_prop(&self, name: &[u16]) -> Option<usize> {
        if self.should_fail_props || name.is_empty() {
            None
        } else {
            // Only return Some(0) for "Test" string
            let test_string = [0x0054, 0x0065, 0x0073, 0x0074]; // "Test"
            if name == test_string {
                Some(0)
            } else {
                None
            }
        }
    }

    fn get_prop_name(&self, num: usize, alias: usize) -> Option<Vec<u16>> {
        if self.should_fail_props || num > 0 || alias > 1 {
            None
        } else {
            Some(vec![0x0045, 0x0072, 0x0072, 0x006F, 0x0072]) // "Error"
        }
    }

    fn get_prop_val(&self, _num: usize) -> Result<ParamValue, NativeApiError> {
        if self.should_fail_props {
            Err(PropertyError::not_readable(_num).into())
        } else {
            Ok(ParamValue::I32(42))
        }
    }

    fn set_prop_val(&mut self, _num: usize, _val: ParamValue) -> Result<(), NativeApiError> {
        if self.should_fail_props {
            Err(PropertyError::not_writable(_num).into())
        } else {
            Ok(())
        }
    }

    fn is_prop_readable(&self, _num: usize) -> bool {
        !self.should_fail_props
    }

    fn is_prop_writable(&self, _num: usize) -> bool {
        !self.should_fail_props
    }

    fn get_n_methods(&self) -> usize {
        if self.should_fail_methods {
            0
        } else {
            1
        }
    }

    fn find_method(&self, name: &[u16]) -> Option<usize> {
        if self.should_fail_methods || name.is_empty() {
            None
        } else {
            // Only return Some(0) for "Test" string
            let test_string = [0x0054, 0x0065, 0x0073, 0x0074]; // "Test"
            if name == test_string {
                Some(0)
            } else {
                None
            }
        }
    }

    fn get_method_name(&self, num: usize, alias: usize) -> Option<Vec<u16>> {
        if self.should_fail_methods || num > 0 || alias > 1 {
            None
        } else {
            Some(vec![0x0045, 0x0072, 0x0072, 0x006F, 0x0072]) // "Error"
        }
    }

    fn get_n_params(&self, _method_num: usize) -> usize {
        if self.should_fail_methods {
            0
        } else {
            0
        }
    }

    fn get_param_def_value(&self, _method_num: usize, _param_num: usize) -> Option<ParamValue> {
        if self.should_fail_methods {
            None
        } else {
            None
        }
    }

    fn has_ret_val(&self, _method_num: usize) -> bool {
        !self.should_fail_methods
    }

    fn call_as_proc(&mut self, _method_num: usize, _params: &mut ParamValues) -> Result<(), NativeApiError> {
        if self.should_fail_methods {
            Err(MethodError::ExecutionFailed { message: "Method execution failed".to_string() }.into())
        } else {
            Ok(())
        }
    }

    fn call_as_func(&mut self, _method_num: usize, _params: &mut ParamValues) -> Result<ParamValue, NativeApiError> {
        if self.should_fail_methods {
            Err(MethodError::ExecutionFailed { message: "Method execution failed".to_string() }.into())
        } else {
            Ok(ParamValue::I32(42))
        }
    }

    fn set_locale(&mut self, _loc: &[u16]) {}

    fn set_user_interface_language_code(&mut self, _lang: &[u16]) {}
}

#[test]
fn test_init_failure() {
    let mut addin = ErrorProneAddIn::with_init_failure();
    
    // Mock connection
    let connection = unsafe { 
        std::mem::transmute(0x1 as *const native_api_1c_core::ffi::connection::Connection) 
    };
    
    let result = addin.init(connection);
    assert!(!result);
}

#[test]
fn test_init_success() {
    let mut addin = ErrorProneAddIn::new();
    
    // Mock connection
    let connection = unsafe { 
        std::mem::transmute(0x1 as *const native_api_1c_core::ffi::connection::Connection) 
    };
    
    let result = addin.init(connection);
    assert!(result);
}

#[test]
fn test_property_failures() {
    let addin = ErrorProneAddIn::with_prop_failures();
    
    // Properties should fail
    assert_eq!(addin.get_n_props(), 0);
    assert_eq!(addin.find_prop(&[]), None);
    assert_eq!(addin.get_prop_name(0, 0), None);
    assert!(addin.get_prop_val(0).is_err());
    assert!(!addin.is_prop_readable(0));
    assert!(!addin.is_prop_writable(0));
}

#[test]
fn test_property_success() {
    let addin = ErrorProneAddIn::new();
    
    // Properties should work
    assert_eq!(addin.get_n_props(), 1);
    assert_eq!(addin.find_prop(&[0x0054, 0x0065, 0x0073, 0x0074]), Some(0)); // "Test"
    assert!(addin.get_prop_name(0, 0).is_some());
    assert!(addin.get_prop_val(0).is_ok());
    assert!(addin.is_prop_readable(0));
    assert!(addin.is_prop_writable(0));
}

#[test]
fn test_method_failures() {
    let addin = ErrorProneAddIn::with_method_failures();
    
    // Methods should fail
    assert_eq!(addin.get_n_methods(), 0);
    assert_eq!(addin.find_method(&[]), None);
    assert_eq!(addin.get_method_name(0, 0), None);
    assert_eq!(addin.get_n_params(0), 0);
    assert_eq!(addin.get_param_def_value(0, 0), None);
    assert!(!addin.has_ret_val(0));
}

#[test]
fn test_method_success() {
    let addin = ErrorProneAddIn::new();
    
    // Methods should work
    assert_eq!(addin.get_n_methods(), 1);
    assert_eq!(addin.find_method(&[0x0054, 0x0065, 0x0073, 0x0074]), Some(0)); // "Test"
    assert!(addin.get_method_name(0, 0).is_some());
    assert_eq!(addin.get_n_params(0), 0);
    assert_eq!(addin.get_param_def_value(0, 0), None);
    assert!(addin.has_ret_val(0));
}

#[test]
fn test_error_propagation() {
    let mut addin = ErrorProneAddIn::with_method_failures();
    
    // Test that errors propagate correctly
    let mut params = ParamValues::new(vec![]);
    
    let proc_result = addin.call_as_proc(0, &mut params);
    assert!(proc_result.is_err());
    
    let func_result = addin.call_as_func(0, &mut params);
    assert!(func_result.is_err());
}

#[test]
fn test_success_propagation() {
    let mut addin = ErrorProneAddIn::new();
    
    // Test that success propagates correctly
    let mut params = ParamValues::new(vec![]);
    
    let proc_result = addin.call_as_proc(0, &mut params);
    assert!(proc_result.is_ok());
    
    let func_result = addin.call_as_func(0, &mut params);
    assert!(func_result.is_ok());
    assert_eq!(func_result.unwrap(), ParamValue::I32(42));
}

#[test]
fn test_edge_cases() {
    let addin = ErrorProneAddIn::new();
    
    // Test edge cases
    assert_eq!(addin.find_prop(&[0x0049, 0x006E, 0x0076, 0x0061, 0x006C, 0x0069, 0x0064]), None); // "Invalid"
    assert_eq!(addin.get_prop_name(999, 0), None); // Invalid index
    assert_eq!(addin.get_prop_name(0, 999), None); // Invalid alias
    assert_eq!(addin.find_method(&[0x0049, 0x006E, 0x0076, 0x0061, 0x006C, 0x0069, 0x0064]), None); // "Invalid"
    assert_eq!(addin.get_method_name(999, 0), None); // Invalid index
    assert_eq!(addin.get_method_name(0, 999), None); // Invalid alias
}
