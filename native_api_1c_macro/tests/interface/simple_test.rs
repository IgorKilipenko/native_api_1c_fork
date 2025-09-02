use std::sync::Arc;

use native_api_1c::native_api_1c_core::ffi::connection::Connection;
use native_api_1c_macro::AddIn;

#[derive(AddIn)]
struct SimpleTest {
    #[add_in_con]
    connection: Arc<Option<&'static Connection>>,

    #[add_in_prop(ty = Int, name = "TestProp", name_ru = "ТестСвойство", readable, writable)]
    pub test_prop: i32,
}

#[test]
fn test_simple_struct() {
    let _test = SimpleTest {
        connection: Arc::new(None),
        test_prop: 42,
    };
    // Если структура создается без ошибок, значит макрос работает
    assert!(true);
}
