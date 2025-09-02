use std::sync::Arc;

use native_api_1c::native_api_1c_core::{
    errors::NativeApiError,
    ffi::connection::Connection,
    interface::{AddInWrapper, ParamValue, ParamValues},
};
use native_api_1c_macro::AddIn;
use rstest::{fixture, rstest};

/// Тест для новых упрощенных атрибутов
#[derive(AddIn)]
struct TestNewAttributes {
    #[connection] // Новый упрощенный атрибут
    connection: Arc<Option<&'static Connection>>,

    #[prop(ty = Int, name = "TestProp", ru = "ТестСвойство", readable, writable)]
    // Новый упрощенный атрибут
    pub test_prop: i32,

    #[func(name = "TestFunction", ru = "ТестФункция")] // Новый упрощенный атрибут
    #[param(ty = Int)] // Новый упрощенный атрибут
    #[param(ty = Int, default = 42)] // Новый упрощенный атрибут
    #[return_type(ty = Int, result)] // Новый упрощенный атрибут
    pub test_function: fn(&Self, i32, i32) -> Result<i32, NativeApiError>,

    #[func(name = "TestProcedure", ru = "ТестПроцедура")] // Новый упрощенный атрибут
    #[param(ty = Int)] // Новый упрощенный атрибут
    pub test_procedure: fn(&mut Self, i32),
}

#[fixture]
fn add_in() -> TestNewAttributes {
    TestNewAttributes {
        connection: Arc::new(None),
        test_prop: 100,
        test_function: |addin, a, b| Ok(a + b + addin.test_prop),
        test_procedure: |addin, value| {
            addin.test_prop = value;
        },
    }
}

#[rstest]
fn test_new_attributes_properties(add_in: TestNewAttributes) {
    // Тестируем свойства
    assert_eq!(add_in.get_n_props(), 1);
    assert_eq!(
        add_in.find_prop(&[0x0054, 0x0065, 0x0073, 0x0074, 0x0050, 0x0072, 0x006F, 0x0070]),
        Some(0)
    ); // "TestProp"
    assert!(add_in.is_prop_readable(0));
    assert!(add_in.is_prop_writable(0));
    assert_eq!(add_in.get_prop_val(0), Ok(ParamValue::I32(100)));
}

#[rstest]
fn test_new_attributes_methods(add_in: TestNewAttributes) {
    // Тестируем методы
    assert_eq!(add_in.get_n_methods(), 2);
    assert_eq!(
        add_in.find_method(&[
            0x0054, 0x0065, 0x0073, 0x0074, 0x0046, 0x0075, 0x006E, 0x0063, 0x0074, 0x0069, 0x006F,
            0x006E
        ]),
        Some(0)
    ); // "TestFunction"
    assert_eq!(
        add_in.find_method(&[
            0x0054, 0x0065, 0x0073, 0x0074, 0x0050, 0x0072, 0x006F, 0x0063, 0x0065, 0x0064, 0x0075,
            0x0072, 0x0065
        ]),
        Some(1)
    ); // "TestProcedure"
}

#[rstest]
fn test_new_attributes_function_call(mut add_in: TestNewAttributes) {
    // Тестируем вызов функции
    let mut params = ParamValues::new(vec![ParamValue::I32(10), ParamValue::I32(20)]);
    let result = add_in.call_as_func(0, &mut params);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ParamValue::I32(130)); // 10 + 20 + 100
}

#[rstest]
fn test_new_attributes_procedure_call(mut add_in: TestNewAttributes) {
    // Тестируем вызов процедуры
    let mut params = ParamValues::new(vec![ParamValue::I32(200)]);
    let result = add_in.call_as_proc(1, &mut params);
    assert!(result.is_ok());

    // Проверяем, что свойство изменилось
    assert_eq!(add_in.get_prop_val(0), Ok(ParamValue::I32(200)));
}

#[rstest]
fn test_new_attributes_connection(mut add_in: TestNewAttributes) {
    // Тестируем соединение
    let connection = unsafe { std::mem::transmute(0x1 as *const Connection) };

    let result = add_in.init(connection);
    assert!(result);
    assert!(add_in.connection.is_some());
}
