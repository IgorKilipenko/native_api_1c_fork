use std::{
    ffi::{c_int, c_void},
    ptr,
    slice::from_raw_parts,
};

use chrono::{Datelike, Timelike};

use crate::interface::ParamValue;

use super::memory_manager::{AllocationError, MemoryManager};

/// Type representing 1C date and time values
/// # Fields
/// * `sec` - seconds after the minute - [0, 60] including leap second
/// * `min` - minutes after the hour - [0, 59]
/// * `hour` - hours since midnight - [0, 23]
/// * `mday` - day of the month - [1, 31]
/// * `mon` - month of the year - [0, 11]
/// * `year` - years since 1900
/// * `wday` - days since Sunday - [0, 6]
/// * `yday` - days since January 1 - [0, 365]
/// * `isdst` - daylight savings time flag
/// * `gmtoff` - seconds east of UTC (unix only)
/// * `zone` - timezone abbreviation (unix only)
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Tm {
    pub sec: c_int,
    pub min: c_int,
    pub hour: c_int,
    pub mday: c_int,
    pub mon: c_int,
    pub year: c_int,
    pub wday: c_int,
    pub yday: c_int,
    pub isdst: c_int,

    #[cfg(target_family = "unix")]
    pub gmtoff: std::ffi::c_long,
    #[cfg(target_family = "unix")]
    pub zone: std::ffi::c_char,
}

impl From<&Tm> for chrono::NaiveDateTime {
    fn from(value: &Tm) -> Self {
        let default_time = chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let from_result = chrono::NaiveDate::from_ymd_opt(
            1900 + value.year,
            (1 + value.mon) as u32,
            (value.mday) as u32,
        )
        .ok_or(());
        if from_result.is_err() {
            return default_time;
        }

        let from_result = from_result.unwrap().and_hms_opt(
            value.hour as u32,
            value.min as u32,
            value.sec as u32,
        );
        if from_result.is_none() {
            return default_time;
        }

        from_result.unwrap()
    }
}

impl From<Tm> for chrono::NaiveDateTime {
    fn from(value: Tm) -> Self {
        Self::from(&value)
    }
}

impl From<&chrono::NaiveDateTime> for Tm {
    fn from(value: &chrono::NaiveDateTime) -> Self {
        Self {
            sec: value.time().second() as c_int,
            min: value.time().minute() as c_int,
            hour: value.time().hour() as c_int,
            mday: value.date().day() as c_int,
            mon: value.date().month0() as c_int,
            year: value.date().year() as c_int - 1900,
            wday: value.date().weekday().num_days_from_sunday() as c_int,
            yday: value.ordinal0() as c_int,
            isdst: 0,
            #[cfg(target_family = "unix")]
            gmtoff: 0,
            #[cfg(target_family = "unix")]
            zone: 0,
        }
    }
}

impl From<chrono::NaiveDateTime> for Tm {
    fn from(value: chrono::NaiveDateTime) -> Self {
        Self::from(&value)
    }
}

#[cfg(target_family = "unix")]
impl PartialEq for Tm {
    fn eq(&self, other: &Self) -> bool {
        self.sec == other.sec
            && self.min == other.min
            && self.hour == other.hour
            && self.mday == other.mday
            && self.mon == other.mon
            && self.year == other.year
            && self.wday == other.wday
            && self.yday == other.yday
            && self.isdst == other.isdst
            && self.gmtoff == other.gmtoff
            && self.zone == other.zone
    }
}

#[cfg(target_family = "windows")]
impl PartialEq for Tm {
    fn eq(&self, other: &Self) -> bool {
        self.sec == other.sec
            && self.min == other.min
            && self.hour == other.hour
            && self.mday == other.mday
            && self.mon == other.mon
            && self.year == other.year
            && self.wday == other.wday
            && self.yday == other.yday
            && self.isdst == other.isdst
    }
}

/// Type representing 1C variant values
/// # Fields
/// `mem` - pointer to the MemoryManager object
/// `variant` - pointer to the TVariant object
/// `result` - pointer to the result of the operation
pub struct ReturnValue<'a> {
    pub mem: &'a MemoryManager,
    pub variant: &'a mut TVariant,
    pub result: &'a mut bool,
}

#[allow(dead_code)]
impl<'a> ReturnValue<'a> {
    /// Creates a new ReturnValue object
    pub fn new(
        mem: &'a MemoryManager,
        variant: &'a mut TVariant,
        result: &'a mut bool,
    ) -> Self {
        Self {
            mem,
            variant,
            result,
        }
    }

    /// Sets the value of the ReturnValue object to empty
    pub fn set_empty(self) {
        self.variant.vt = VariantType::Empty;
    }

    /// Sets the value of the ReturnValue object to integer `i32`
    pub fn set_i32(self, val: i32) {
        self.variant.vt = VariantType::Int32;
        self.variant.value.i32 = val;
    }

    /// Sets the value of the ReturnValue object to bool `bool`
    pub fn set_bool(self, val: bool) {
        self.variant.vt = VariantType::Bool;
        self.variant.value.bool = val;
    }

    /// Sets the value of the ReturnValue object to float `f64`
    pub fn set_f64(self, val: f64) {
        self.variant.vt = VariantType::Double;
        self.variant.value.f64 = val;
    }

    /// Sets the value of the ReturnValue object to date-time `Tm`
    pub fn set_date(self, val: Tm) {
        self.variant.vt = VariantType::Time;
        self.variant.value.tm = val;
    }

    /// Sets the value of the ReturnValue object to UTF-16 `&[u16]`
    pub fn set_str(self, val: &[u16]) {
        let Ok(ptr) = self.mem.alloc_str(val.len()) else {
            *self.result = false;
            return;
        };

        unsafe {
            ptr::copy_nonoverlapping(val.as_ptr(), ptr.as_ptr(), val.len())
        };

        self.variant.vt = VariantType::WStr;
        self.variant.value.data_str.ptr = ptr.as_ptr();
        self.variant.value.data_str.len = val.len() as u32;
    }

    /// Sets the value of the ReturnValue object to blob `&[u8]`
    pub fn set_blob(self, val: &[u8]) {
        let Ok(ptr) = self.mem.alloc_blob(val.len()) else {
            *self.result = false;
            return;
        };

        unsafe {
            ptr::copy_nonoverlapping(val.as_ptr(), ptr.as_ptr(), val.len())
        };

        self.variant.vt = VariantType::Blob;
        self.variant.value.data_blob.ptr = ptr.as_ptr();
        self.variant.value.data_blob.len = val.len() as u32;
    }
}

impl<'a> From<&'a TVariant> for ParamValue {
    fn from(param: &'a TVariant) -> ParamValue {
        unsafe {
            match param.vt {
                VariantType::Empty => ParamValue::Empty,
                VariantType::Null => ParamValue::Null,
                VariantType::Bool => ParamValue::Bool(param.value.bool),
                VariantType::Int8 => ParamValue::I8(param.value.i8),
                VariantType::Int16 => ParamValue::I16(param.value.i16),
                VariantType::Int32 => ParamValue::I32(param.value.i32),
                VariantType::Int64 => ParamValue::I64(param.value.i64),
                VariantType::UInt8 => ParamValue::U8(param.value.u8),
                VariantType::UInt16 => ParamValue::U16(param.value.u16),
                VariantType::UInt32 => ParamValue::U32(param.value.u32),
                VariantType::UInt64 => ParamValue::U64(param.value.u64),
                VariantType::Float => ParamValue::F32(param.value.f32),
                VariantType::Double => ParamValue::F64(param.value.f64),
                VariantType::Date => ParamValue::DateDouble(param.value.date),
                VariantType::Time => ParamValue::Date(param.value.tm),
                VariantType::PStr => ParamValue::AnsiString(
                    from_raw_parts(
                        param.value.data_str.ptr as *const u8,
                        param.value.data_str.len as usize,
                    )
                    .into(),
                ),
                VariantType::WStr => ParamValue::String(
                    from_raw_parts(
                        param.value.data_str.ptr,
                        param.value.data_str.len as usize,
                    )
                    .into(),
                ),
                VariantType::Blob => ParamValue::Blob(
                    from_raw_parts(
                        param.value.data_blob.ptr,
                        param.value.data_blob.len as usize,
                    )
                    .into(),
                ),
                VariantType::Error => ParamValue::Error(param.value.error),
                VariantType::HResult => ParamValue::HResult(param.value.hresult),
                VariantType::ClsID => ParamValue::ClsId(param.value.cls_id),
                _ => ParamValue::Empty,
            }
        }
    }
}

#[repr(u16)]
#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum VariantType {
    Empty = 0,
    Null,
    Int16,     //int16_t
    Int32,     //int32_t
    Float,     //float
    Double,    //double
    Date,      //DATE (double)
    Time,      //struct tm
    PStr,      //struct str    string
    Interface, //struct iface
    Error,     //int32_t errCode
    Bool,      //bool
    Int8,      //int8_t
    UInt8,     //uint8_t
    UInt16,    //uint16_t
    UInt32,    //uint32_t
    Int64,     //int64_t
    UInt64,    //uint64_t
    Int,       //int   Depends on architecture
    UInt,      //unsigned int  Depends on architecture
    HResult,   //long hRes
    WStr,      //struct wstr
    Blob,      //means in struct str binary data contain
    ClsID,     //UUID

    Undefined = 0xFFFF,
}

/// Type representing stored String data
/// # Fields
/// * `ptr` - pointer to the data
/// * `len` - length of the data
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DataStr {
    pub ptr: *mut u16,
    pub len: u32,
}

/// Type representing stored Blob data
/// # Fields
/// * `ptr` - pointer to the data
/// * `len` - length of the data
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DataBlob {
    pub ptr: *mut u8,
    pub len: u32,
}

/// Type encapsulating 1C variant values
/// # Fields
/// * `bool` - boolean value
/// * `i8` - 8-bit signed integer
/// * `i16` - 16-bit signed integer
/// * `i32` - 32-bit signed integer
/// * `i64` - 64-bit signed integer
/// * `u8` - 8-bit unsigned integer
/// * `u16` - 16-bit unsigned integer
/// * `u32` - 32-bit unsigned integer
/// * `u64` - 64-bit unsigned integer
/// * `f32` - 32-bit float value
/// * `f64` - 64-bit float value
/// * `date` - Windows DATE format (f64)
/// * `tm` - date-time value (struct tm)
/// * `error` - error code (i32)
/// * `hresult` - HRESULT code (i32)
/// * `data_str` - UTF-16 string value
/// * `data_blob` - blob value
/// * `cls_id` - UUID/GUID (16 bytes)
#[repr(C)]
pub union VariantValue {
    pub bool: bool,
    pub i8: i8,
    pub i16: i16,
    pub i32: i32,
    pub i64: i64,
    pub u8: u8,
    pub u16: u16,
    pub u32: u32,
    pub u64: u64,
    pub f32: f32,
    pub f64: f64,
    pub date: f64,  // Windows DATE format
    pub tm: Tm,
    pub error: i32,
    pub hresult: i32,
    pub data_str: DataStr,
    pub data_blob: DataBlob,
    pub cls_id: [u8; 16], // UUID/GUID
}

/// Type encapsulating 1C variant values for internal use
#[repr(C)]
pub struct TVariant {
    pub value: VariantValue,
    pub elements: u32, //Dimension for an one-dimensional array in pvarVal
    pub vt: VariantType,
}

impl Default for TVariant {
    fn default() -> Self {
        Self {
            value: VariantValue { bool: false },
            elements: 0,
            vt: VariantType::Empty,
        }
    }
}

impl TVariant {
    /// # Safety
    /// This function is unsafe because it manipulates pointers, provided by the 1C platform.
    /// Function is safe as long as 1C platform provides valid pointers.
    pub unsafe fn update_to_str(
        &mut self,
        mem_mngr: &MemoryManager,
        v: &[u16],
    ) -> Result<u32, AllocationError> {
        let old_pointer = self.value.data_str.ptr;

        let ptr = mem_mngr.alloc_str(v.len())?;
        ptr::copy_nonoverlapping(v.as_ptr(), ptr.as_ptr(), v.len());

        self.value.data_str.ptr = ptr.as_ptr();
        self.value.data_str.len = v.len() as u32;

        mem_mngr.free_memory(&mut old_pointer.cast::<c_void>());

        self.vt = VariantType::WStr;

        Ok(self.value.data_str.len)
    }

    /// # Safety
    /// This function is unsafe because it manipulates pointers, provided by the 1C platform.
    /// Function is safe as long as 1C platform provides valid pointers.
    pub unsafe fn update_to_blob(
        &mut self,
        mem_mngr: &MemoryManager,
        v: &[u8],
    ) -> Result<u32, AllocationError> {
        let old_pointer = self.value.data_blob.ptr;

        let ptr = mem_mngr.alloc_blob(v.len())?;
        ptr::copy_nonoverlapping(v.as_ptr(), ptr.as_ptr(), v.len());

        self.value.data_blob.ptr = ptr.as_ptr();
        self.value.data_blob.len = v.len() as u32;

        mem_mngr.free_memory(&mut old_pointer.cast::<c_void>());

        self.vt = VariantType::Blob;

        Ok(self.value.data_blob.len)
    }

    pub fn update_to_bool(&mut self, v: bool) {
        self.value.bool = v;
        self.vt = VariantType::Bool;
    }

    pub fn update_to_i32(&mut self, v: i32) {
        self.value.i32 = v;
        self.vt = VariantType::Int32;
    }

    pub fn update_to_f64(&mut self, v: f64) {
        self.value.f64 = v;
        self.vt = VariantType::Double;
    }

    pub fn update_to_date(&mut self, v: Tm) {
        self.value.tm = v;
        self.vt = VariantType::Time;
    }

    // Новые методы для поддержки дополнительных типов VARIANT

    pub fn update_to_null(&mut self) {
        self.vt = VariantType::Null;
    }

    pub fn update_to_i8(&mut self, v: i8) {
        self.value.i8 = v;
        self.vt = VariantType::Int8;
    }

    pub fn update_to_i16(&mut self, v: i16) {
        self.value.i16 = v;
        self.vt = VariantType::Int16;
    }

    pub fn update_to_i64(&mut self, v: i64) {
        self.value.i64 = v;
        self.vt = VariantType::Int64;
    }

    pub fn update_to_u8(&mut self, v: u8) {
        self.value.u8 = v;
        self.vt = VariantType::UInt8;
    }

    pub fn update_to_u16(&mut self, v: u16) {
        self.value.u16 = v;
        self.vt = VariantType::UInt16;
    }

    pub fn update_to_u32(&mut self, v: u32) {
        self.value.u32 = v;
        self.vt = VariantType::UInt32;
    }

    pub fn update_to_u64(&mut self, v: u64) {
        self.value.u64 = v;
        self.vt = VariantType::UInt64;
    }

    pub fn update_to_f32(&mut self, v: f32) {
        self.value.f32 = v;
        self.vt = VariantType::Float;
    }

    pub fn update_to_date_double(&mut self, v: f64) {
        self.value.date = v;
        self.vt = VariantType::Date;
    }

    pub fn update_to_error(&mut self, v: i32) {
        self.value.error = v;
        self.vt = VariantType::Error;
    }

    pub fn update_to_hresult(&mut self, v: i32) {
        self.value.hresult = v;
        self.vt = VariantType::HResult;
    }

    pub fn update_to_cls_id(&mut self, v: [u8; 16]) {
        self.value.cls_id = v;
        self.vt = VariantType::ClsID;
    }

    pub fn update_from_return(
        &mut self,
        mem_mngr: &MemoryManager,
        value: &ParamValue,
    ) {
        match value {
            ParamValue::Empty => self.vt = VariantType::Empty,
            ParamValue::Null => self.update_to_null(),
            ParamValue::Bool(v) => self.update_to_bool(*v),
            ParamValue::I8(v) => self.update_to_i8(*v),
            ParamValue::I16(v) => self.update_to_i16(*v),
            ParamValue::I32(v) => self.update_to_i32(*v),
            ParamValue::I64(v) => self.update_to_i64(*v),
            ParamValue::U8(v) => self.update_to_u8(*v),
            ParamValue::U16(v) => self.update_to_u16(*v),
            ParamValue::U32(v) => self.update_to_u32(*v),
            ParamValue::U64(v) => self.update_to_u64(*v),
            ParamValue::F32(v) => self.update_to_f32(*v),
            ParamValue::F64(v) => self.update_to_f64(*v),
            ParamValue::Date(v) => self.update_to_date(*v),
            ParamValue::DateDouble(v) => self.update_to_date_double(*v),
            ParamValue::String(v) => {
                let _ = unsafe { self.update_to_str(mem_mngr, v.as_slice()) };
            }
            ParamValue::AnsiString(v) => {
                // Для ANSI строк нужна специальная обработка
                // Пока используем WStr, позже добавим полную поддержку PStr
                let _ = unsafe { self.update_to_str(mem_mngr, &[]) };
            }
            ParamValue::Blob(v) => {
                let _ = unsafe { self.update_to_blob(mem_mngr, v.as_slice()) };
            }
            ParamValue::Error(v) => self.update_to_error(*v),
            ParamValue::HResult(v) => self.update_to_hresult(*v),
            ParamValue::ClsId(v) => self.update_to_cls_id(*v),
        }
    }
}
