//! Тесты для новых VARIANT типов в макросах

use native_api_1c::native_api_1c_core::interface::ParamValue;
use native_api_1c::native_api_1c_core::errors::NativeApiError;

// Простая структура для тестирования новых типов ParamValue
struct TestNewVariantTypes {
    pub test_i8: i8,
    pub test_u32: u32,
    pub test_i64: i64,
    pub test_f32: f32,
    pub test_error: i32,
    pub test_hresult: i32,
    pub test_cls_id: [u8; 16],
}

impl TestNewVariantTypes {
    // Тест новых типов в функциях
    pub fn get_i8(&self) -> Result<i8, NativeApiError> {
        Ok(self.test_i8)
    }
    
    pub fn get_u32(&self) -> Result<u32, NativeApiError> {
        Ok(self.test_u32)
    }
    
    pub fn get_i64(&self) -> Result<i64, NativeApiError> {
        Ok(self.test_i64)
    }
    
    pub fn get_f32(&self) -> Result<f32, NativeApiError> {
        Ok(self.test_f32)
    }
    
    pub fn get_error(&self) -> Result<i32, NativeApiError> {
        Ok(self.test_error)
    }
    
    pub fn get_hresult(&self) -> Result<i32, NativeApiError> {
        Ok(self.test_hresult)
    }
    
    pub fn get_cls_id(&self) -> Result<[u8; 16], NativeApiError> {
        Ok(self.test_cls_id)
    }
    
    // Тест функций с параметрами новых типов
    pub fn set_i8(&mut self, value: i8) -> Result<(), NativeApiError> {
        self.test_i8 = value;
        Ok(())
    }
    
    pub fn set_u32(&mut self, value: u32) -> Result<(), NativeApiError> {
        self.test_u32 = value;
        Ok(())
    }
    
    pub fn set_i64(&mut self, value: i64) -> Result<(), NativeApiError> {
        self.test_i64 = value;
        Ok(())
    }
    
    pub fn set_f32(&mut self, value: f32) -> Result<(), NativeApiError> {
        self.test_f32 = value;
        Ok(())
    }
    
    pub fn set_error(&mut self, value: i32) -> Result<(), NativeApiError> {
        self.test_error = value;
        Ok(())
    }
    
    pub fn set_hresult(&mut self, value: i32) -> Result<(), NativeApiError> {
        self.test_hresult = value;
        Ok(())
    }
    
    pub fn set_cls_id(&mut self, value: [u8; 16]) -> Result<(), NativeApiError> {
        self.test_cls_id = value;
        Ok(())
    }
}

impl Default for TestNewVariantTypes {
    fn default() -> Self {
        Self {
            test_i8: 127,
            test_u32: 4294967295,
            test_i64: 9223372036854775807,
            test_f32: 3.14f32,
            test_error: -2147024809, // E_INVALIDARG
            test_hresult: 0x80004005u32 as i32, // E_FAIL
            test_cls_id: [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 
                         0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_variant_types_properties() {
        let addin = TestNewVariantTypes::default();
        
        // Проверяем, что свойства доступны
        assert_eq!(addin.test_i8, 127);
        assert_eq!(addin.test_u32, 4294967295);
        assert_eq!(addin.test_i64, 9223372036854775807);
        assert_eq!(addin.test_f32, 3.14f32);
        assert_eq!(addin.test_error, -2147024809);
        assert_eq!(addin.test_hresult, 0x80004005u32 as i32);
        assert_eq!(addin.test_cls_id, [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 
                                      0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0]);
    }

    #[test]
    fn test_new_variant_types_functions() {
        let addin = TestNewVariantTypes::default();
        
        // Проверяем функции получения значений
        assert_eq!(addin.get_i8().unwrap(), 127);
        assert_eq!(addin.get_u32().unwrap(), 4294967295);
        assert_eq!(addin.get_i64().unwrap(), 9223372036854775807);
        assert_eq!(addin.get_f32().unwrap(), 3.14f32);
        assert_eq!(addin.get_error().unwrap(), -2147024809);
        assert_eq!(addin.get_hresult().unwrap(), 0x80004005u32 as i32);
        assert_eq!(addin.get_cls_id().unwrap(), [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 
                                                0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0]);
    }

    #[test]
    fn test_new_variant_types_setters() {
        let mut addin = TestNewVariantTypes::default();
        
        // Тестируем функции установки значений
        addin.set_i8(100).unwrap();
        assert_eq!(addin.test_i8, 100);
        
        addin.set_u32(1000).unwrap();
        assert_eq!(addin.test_u32, 1000);
        
        addin.set_i64(1000000).unwrap();
        assert_eq!(addin.test_i64, 1000000);
        
        addin.set_f32(2.71f32).unwrap();
        assert_eq!(addin.test_f32, 2.71f32);
        
        addin.set_error(0).unwrap();
        assert_eq!(addin.test_error, 0);
        
        addin.set_hresult(0).unwrap();
        assert_eq!(addin.test_hresult, 0);
        
        let new_uuid = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 
                       0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];
        addin.set_cls_id(new_uuid).unwrap();
        assert_eq!(addin.test_cls_id, new_uuid);
    }

    #[test]
    fn test_param_value_new_types() {
        // Тестируем создание новых типов ParamValue
        let i8_val = ParamValue::I8(127);
        let u32_val = ParamValue::U32(4294967295);
        let i64_val = ParamValue::I64(9223372036854775807);
        let f32_val = ParamValue::F32(3.14f32);
        let error_val = ParamValue::Error(-2147024809);
        let hresult_val = ParamValue::HResult(0x80004005u32 as i32);
        let cls_id_val = ParamValue::ClsId([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 
                                           0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0]);
        
        // Проверяем, что значения созданы правильно
        match i8_val {
            ParamValue::I8(v) => assert_eq!(v, 127),
            _ => panic!("Expected I8"),
        }
        
        match u32_val {
            ParamValue::U32(v) => assert_eq!(v, 4294967295),
            _ => panic!("Expected U32"),
        }
        
        match i64_val {
            ParamValue::I64(v) => assert_eq!(v, 9223372036854775807),
            _ => panic!("Expected I64"),
        }
        
        match f32_val {
            ParamValue::F32(v) => assert!((v - 3.14f32).abs() < 0.001),
            _ => panic!("Expected F32"),
        }
        
        match error_val {
            ParamValue::Error(v) => assert_eq!(v, -2147024809),
            _ => panic!("Expected Error"),
        }
        
        match hresult_val {
            ParamValue::HResult(v) => assert_eq!(v, 0x80004005u32 as i32),
            _ => panic!("Expected HResult"),
        }
        
        match cls_id_val {
            ParamValue::ClsId(v) => assert_eq!(v, [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 
                                                   0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0]),
            _ => panic!("Expected ClsId"),
        }
    }
}
