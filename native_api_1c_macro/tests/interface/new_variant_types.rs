//! Тесты для новых VARIANT типов в макросах

use native_api_1c::native_api_1c_core::interface::ParamValue;
use native_api_1c::native_api_1c_core::errors::NativeApiError;
use native_api_1c::AddIn;

#[derive(AddIn)]
struct TestNewVariantTypes {
    // Тест новых типов в свойствах
    #[prop(ty = I8, name = "TestI8")]
    pub test_i8: i8,
    
    #[prop(ty = U32, name = "TestU32")]
    pub test_u32: u32,
    
    #[prop(ty = I64, name = "TestI64")]
    pub test_i64: i64,
    
    #[prop(ty = F32, name = "TestF32")]
    pub test_f32: f32,
    
    
    #[prop(ty = Error, name = "TestError")]
    pub test_error: i32,
    
    #[prop(ty = HResult, name = "TestHResult")]
    pub test_hresult: i32,
    
    #[prop(ty = ClsId, name = "TestClsId")]
    pub test_cls_id: [u8; 16],
}

impl TestNewVariantTypes {
    // Тест новых типов в функциях
    #[func(name = "GetI8", return_type = I8)]
    pub fn get_i8(&self) -> Result<i8, NativeApiError> {
        Ok(self.test_i8)
    }
    
    #[func(name = "GetU32", return_type = U32)]
    pub fn get_u32(&self) -> Result<u32, NativeApiError> {
        Ok(self.test_u32)
    }
    
    #[func(name = "GetI64", return_type = I64)]
    pub fn get_i64(&self) -> Result<i64, NativeApiError> {
        Ok(self.test_i64)
    }
    
    #[func(name = "GetF32", return_type = F32)]
    pub fn get_f32(&self) -> Result<f32, NativeApiError> {
        Ok(self.test_f32)
    }
    
    
    #[func(name = "GetError", return_type = Error)]
    pub fn get_error(&self) -> Result<i32, NativeApiError> {
        Ok(self.test_error)
    }
    
    #[func(name = "GetHResult", return_type = HResult)]
    pub fn get_hresult(&self) -> Result<i32, NativeApiError> {
        Ok(self.test_hresult)
    }
    
    #[func(name = "GetClsId", return_type = ClsId)]
    pub fn get_cls_id(&self) -> Result<[u8; 16], NativeApiError> {
        Ok(self.test_cls_id)
    }
    
    // Тест функций с параметрами новых типов
    #[func(name = "SetI8")]
    #[param(ty = I8, name = "value")]
    pub fn set_i8(&mut self, value: i8) -> Result<(), NativeApiError> {
        self.test_i8 = value;
        Ok(())
    }
    
    #[func(name = "SetU32")]
    #[param(ty = U32, name = "value")]
    pub fn set_u32(&mut self, value: u32) -> Result<(), NativeApiError> {
        self.test_u32 = value;
        Ok(())
    }
    
    #[func(name = "SetI64")]
    #[param(ty = I64, name = "value")]
    pub fn set_i64(&mut self, value: i64) -> Result<(), NativeApiError> {
        self.test_i64 = value;
        Ok(())
    }
    
    #[func(name = "SetF32")]
    #[param(ty = F32, name = "value")]
    pub fn set_f32(&mut self, value: f32) -> Result<(), NativeApiError> {
        self.test_f32 = value;
        Ok(())
    }
    
    
    #[func(name = "SetError")]
    #[param(ty = Error, name = "value")]
    pub fn set_error(&mut self, value: i32) -> Result<(), NativeApiError> {
        self.test_error = value;
        Ok(())
    }
    
    #[func(name = "SetHResult")]
    #[param(ty = HResult, name = "value")]
    pub fn set_hresult(&mut self, value: i32) -> Result<(), NativeApiError> {
        self.test_hresult = value;
        Ok(())
    }
    
    #[func(name = "SetClsId")]
    #[param(ty = ClsId, name = "value")]
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
    fn test_addin_wrapper_implementation() {
        let addin = TestNewVariantTypes::default();
        
        // Проверяем, что AddInWrapper реализован
        assert_eq!(addin.get_n_props(), 7); // 7 новых свойств
        assert_eq!(addin.get_n_methods(), 14); // 7 getter + 7 setter функций
        
        // Проверяем имена свойств
        assert_eq!(addin.get_prop_name(0, 0).unwrap(), "TestI8");
        assert_eq!(addin.get_prop_name(1, 0).unwrap(), "TestU32");
        assert_eq!(addin.get_prop_name(2, 0).unwrap(), "TestI64");
        assert_eq!(addin.get_prop_name(3, 0).unwrap(), "TestF32");
        assert_eq!(addin.get_prop_name(4, 0).unwrap(), "TestError");
        assert_eq!(addin.get_prop_name(5, 0).unwrap(), "TestHResult");
        assert_eq!(addin.get_prop_name(6, 0).unwrap(), "TestClsId");
        
        // Проверяем имена методов
        assert_eq!(addin.get_method_name(0, 0).unwrap(), "GetI8");
        assert_eq!(addin.get_method_name(1, 0).unwrap(), "GetU32");
        assert_eq!(addin.get_method_name(2, 0).unwrap(), "GetI64");
        assert_eq!(addin.get_method_name(3, 0).unwrap(), "GetF32");
        assert_eq!(addin.get_method_name(4, 0).unwrap(), "GetError");
        assert_eq!(addin.get_method_name(5, 0).unwrap(), "GetHResult");
        assert_eq!(addin.get_method_name(6, 0).unwrap(), "GetClsId");
        assert_eq!(addin.get_method_name(7, 0).unwrap(), "SetI8");
        assert_eq!(addin.get_method_name(8, 0).unwrap(), "SetU32");
        assert_eq!(addin.get_method_name(9, 0).unwrap(), "SetI64");
        assert_eq!(addin.get_method_name(10, 0).unwrap(), "SetF32");
        assert_eq!(addin.get_method_name(11, 0).unwrap(), "SetError");
        assert_eq!(addin.get_method_name(12, 0).unwrap(), "SetHResult");
        assert_eq!(addin.get_method_name(13, 0).unwrap(), "SetClsId");
    }
}
