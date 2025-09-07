//! Безопасные обертки для работы с TVariant
//! 
//! Этот модуль предоставляет безопасные обертки для работы с TVariant,
//! которые автоматически управляют памятью и предоставляют удобный API.

use crate::ffi::provided_types::{TVariant, VariantType};
use crate::interface::ParamValue;
use crate::errors::{NativeApiError, TypeConversionError};

/// Безопасная обертка для TVariant
/// 
/// Автоматически управляет памятью и предоставляет безопасный API
/// для работы с VARIANT типами.
pub struct SafeVariant {
    inner: TVariant,
}

impl SafeVariant {
    /// Создает новый пустой SafeVariant
    pub fn new() -> Self {
        Self {
            inner: TVariant::default(),
        }
    }

    /// Создает SafeVariant из TVariant
    pub fn from_tvariant(tvariant: TVariant) -> Self {
        Self { inner: tvariant }
    }

    /// Преобразует в TVariant
    pub fn into_tvariant(self) -> TVariant {
        self.inner
    }

    /// Получает ссылку на внутренний TVariant
    pub fn as_tvariant(&self) -> &TVariant {
        &self.inner
    }

    /// Получает мутабельную ссылку на внутренний TVariant
    pub fn as_tvariant_mut(&mut self) -> &mut TVariant {
        &mut self.inner
    }

    /// Получает тип варианта
    pub fn get_type(&self) -> VariantType {
        self.inner.vt.clone()
    }

    /// Проверяет, является ли вариант пустым
    pub fn is_empty(&self) -> bool {
        matches!(self.inner.vt, VariantType::Empty)
    }

    /// Проверяет, является ли вариант NULL
    pub fn is_null(&self) -> bool {
        matches!(self.inner.vt, VariantType::Null)
    }

    /// Устанавливает значение как NULL
    pub fn set_null(&mut self) {
        self.inner.update_to_null();
    }

    /// Устанавливает значение как пустое
    pub fn set_empty(&mut self) {
        self.inner.vt = VariantType::Empty;
    }

    /// Устанавливает булево значение
    pub fn set_bool(&mut self, value: bool) {
        self.inner.update_to_bool(value);
    }

    /// Получает булево значение
    pub fn get_bool(&self) -> Result<bool, NativeApiError> {
        match self.inner.vt {
            VariantType::Bool => Ok(unsafe { self.inner.value.bool }),
            _ => Err(TypeConversionError::ToString {
                message: format!("Expected Bool, got {:?}", self.inner.vt),
            }.into()),
        }
    }

    /// Устанавливает целое число (i32)
    pub fn set_i32(&mut self, value: i32) {
        self.inner.update_to_i32(value);
    }

    /// Получает целое число (i32)
    pub fn get_i32(&self) -> Result<i32, NativeApiError> {
        match self.inner.vt {
            VariantType::Int32 => Ok(unsafe { self.inner.value.i32 }),
            _ => Err(TypeConversionError::ToNumber {
                message: format!("Expected Int32, got {:?}", self.inner.vt),
            }.into()),
        }
    }

    /// Устанавливает число с плавающей точкой (f64)
    pub fn set_f64(&mut self, value: f64) {
        self.inner.update_to_f64(value);
    }

    /// Получает число с плавающей точкой (f64)
    pub fn get_f64(&self) -> Result<f64, NativeApiError> {
        match self.inner.vt {
            VariantType::Double => Ok(unsafe { self.inner.value.f64 }),
            _ => Err(TypeConversionError::ToNumber {
                message: format!("Expected Double, got {:?}", self.inner.vt),
            }.into()),
        }
    }

    /// Устанавливает строку (WCHAR_T*)
    pub fn set_string(&mut self, value: &str) -> Result<(), NativeApiError> {
        // Здесь нужно будет реализовать безопасное преобразование строки
        // Пока возвращаем ошибку, так как это требует работы с памятью
        Err(TypeConversionError::FromString {
            message: "String conversion not implemented yet".to_string(),
        }.into())
    }

    /// Получает строку (WCHAR_T*)
    pub fn get_string(&self) -> Result<String, NativeApiError> {
        match self.inner.vt {
            VariantType::PStr => {
                // Здесь нужно будет реализовать безопасное преобразование строки
                // Пока возвращаем ошибку, так как это требует работы с памятью
                Err(TypeConversionError::ToString {
                    message: "String conversion not implemented yet".to_string(),
                }.into())
            }
            _ => Err(TypeConversionError::ToString {
                message: format!("Expected PStr, got {:?}", self.inner.vt),
            }.into()),
        }
    }

    /// Устанавливает дату (Windows DATE format)
    pub fn set_date(&mut self, value: f64) {
        self.inner.update_to_date_double(value);
    }

    /// Получает дату (Windows DATE format)
    pub fn get_date(&self) -> Result<f64, NativeApiError> {
        match self.inner.vt {
            VariantType::Date => Ok(unsafe { self.inner.value.date }),
            _ => Err(TypeConversionError::ToNumber {
                message: format!("Expected Date, got {:?}", self.inner.vt),
            }.into()),
        }
    }

    /// Устанавливает код ошибки
    pub fn set_error(&mut self, value: i32) {
        self.inner.update_to_error(value);
    }

    /// Получает код ошибки
    pub fn get_error(&self) -> Result<i32, NativeApiError> {
        match self.inner.vt {
            VariantType::Error => Ok(unsafe { self.inner.value.error }),
            _ => Err(TypeConversionError::ToNumber {
                message: format!("Expected Error, got {:?}", self.inner.vt),
            }.into()),
        }
    }


    /// Устанавливает CLSID (UUID/GUID)
    pub fn set_cls_id(&mut self, value: [u8; 16]) {
        self.inner.update_to_cls_id(value);
    }

    /// Получает CLSID (UUID/GUID)
    pub fn get_cls_id(&self) -> Result<[u8; 16], NativeApiError> {
        match self.inner.vt {
            VariantType::ClsID => Ok(unsafe { self.inner.value.cls_id }),
            _ => Err(TypeConversionError::ToString {
                message: format!("Expected ClsID, got {:?}", self.inner.vt),
            }.into()),
        }
    }

    /// Преобразует в ParamValue
    pub fn to_param_value(&self) -> ParamValue {
        ParamValue::from(&self.inner)
    }

    /// Создает из ParamValue
    pub fn from_param_value(param_value: &ParamValue) -> Self {
        // Пока используем простое преобразование через ParamValue
        // TODO: Реализовать полное преобразование с MemoryManager
        let mut tvariant = TVariant::default();
        // Простое преобразование для базовых типов
        match param_value {
            ParamValue::Empty => tvariant.vt = VariantType::Empty,
            ParamValue::Null => tvariant.update_to_null(),
            ParamValue::Bool(v) => tvariant.update_to_bool(*v),
            ParamValue::I32(v) => tvariant.update_to_i32(*v),
            ParamValue::F64(v) => tvariant.update_to_f64(*v),
            ParamValue::DateDouble(v) => tvariant.update_to_date_double(*v),
            ParamValue::Error(v) => tvariant.update_to_error(*v),
            ParamValue::ClsId(v) => tvariant.update_to_cls_id(*v),
            _ => {
                // Для сложных типов пока устанавливаем Empty
                tvariant.vt = VariantType::Empty;
            }
        }
        Self { inner: tvariant }
    }
}

impl Default for SafeVariant {
    fn default() -> Self {
        Self::new()
    }
}

impl From<TVariant> for SafeVariant {
    fn from(tvariant: TVariant) -> Self {
        Self::from_tvariant(tvariant)
    }
}

impl From<SafeVariant> for TVariant {
    fn from(safe_variant: SafeVariant) -> Self {
        safe_variant.into_tvariant()
    }
}

impl From<&ParamValue> for SafeVariant {
    fn from(param_value: &ParamValue) -> Self {
        Self::from_param_value(param_value)
    }
}

impl From<SafeVariant> for ParamValue {
    fn from(safe_variant: SafeVariant) -> Self {
        safe_variant.to_param_value()
    }
}

/// Безопасная обертка для работы с массивом TVariant
pub struct SafeVariantArray {
    inner: Vec<SafeVariant>,
}

impl SafeVariantArray {
    /// Создает новый пустой массив
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Создает массив заданного размера
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    /// Добавляет элемент в массив
    pub fn push(&mut self, variant: SafeVariant) {
        self.inner.push(variant);
    }

    /// Получает элемент по индексу
    pub fn get(&self, index: usize) -> Option<&SafeVariant> {
        self.inner.get(index)
    }

    /// Получает мутабельный элемент по индексу
    pub fn get_mut(&mut self, index: usize) -> Option<&mut SafeVariant> {
        self.inner.get_mut(index)
    }

    /// Получает длину массива
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Проверяет, пуст ли массив
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Преобразует в Vec<SafeVariant>
    pub fn into_vec(self) -> Vec<SafeVariant> {
        self.inner
    }

    /// Создает из Vec<SafeVariant>
    pub fn from_vec(vec: Vec<SafeVariant>) -> Self {
        Self { inner: vec }
    }
}

impl Default for SafeVariantArray {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<SafeVariant>> for SafeVariantArray {
    fn from(vec: Vec<SafeVariant>) -> Self {
        Self::from_vec(vec)
    }
}

impl From<SafeVariantArray> for Vec<SafeVariant> {
    fn from(array: SafeVariantArray) -> Self {
        array.into_vec()
    }
}
