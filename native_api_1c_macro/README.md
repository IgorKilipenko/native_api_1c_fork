```rust
use std::sync::Arc;

use native_api_1c::{
    native_api_1c_core::{
        errors::NativeApiError,
        ffi::connection::Connection,
    },
    native_api_1c_macro::{extern_functions, AddIn},
};

#[derive(AddIn)]
pub struct MyAddIn {
    /// connection with 1C, used for calling events
    /// Arc is used to allow multiple threads to access the connection
    #[connection] // New simplified attribute
    connection: Arc<Option<&'static Connection>>,

    /// Property, readable and writable from 1C
    #[prop(ty = Int, name = "MyProp", ru = "МоеСвойство", readable, writable)] // New simplified attribute
    pub some_prop: i32,

    /// Property, readable from 1C but not writable
    #[prop(ty = Int, name = "ProtectedProp", ru = "ЗащищенноеСвойство", readable)] // New simplified attribute
    pub protected_prop: i32,

    /// Function, taking one or two arguments and returning a result
    /// In 1C it can be called as:
    /// ```bsl
    ///  ComponentObject.MyFunction(10, 15); // 2nd argument = 15
    ///  ComponentObject.MyFunction(10);     // 2nd argument = 12 (default value)
    /// ```
    /// If function returns an error, but does not panic, then 1C will throw an exception
    #[func(name = "MyFunction", ru = "МояФункция")] // New simplified attribute
    #[param(ty = Int)] // New simplified attribute
    #[param(ty = Int, default = 12)] // New simplified attribute
    #[return_type(ty = Int, result)] // New simplified attribute
    pub my_function: fn(&Self, i32, i32) -> Result<i32, NativeApiError>, // Enhanced error handling

    /// Function, taking no arguments and returning a string
    #[func(name = "GetString", ru = "ПолучитьСтроку")] // New simplified attribute
    #[return_type(ty = Str)] // New simplified attribute
    pub get_string: fn(&mut Self) -> String,

    /// Procedure, taking no arguments and returning nothing
    #[func(name = "MyProcedure", ru = "МояПроцедура")] // New simplified attribute
    pub my_procedure: fn(&mut Self),

    /// Private field, not visible from 1C
    private_field: i32,
}

impl Default for MyAddIn {
    fn default() -> Self {
        Self {
            connection: Arc::new(None),
            some_prop: 0,
            protected_prop: 50,
            my_function: Self::my_function_inner,
            get_string: Self::get_string_inner,
            my_procedure: Self::my_procedure_inner,
            private_field: 100,
        }
    }
}

impl MyAddIn {
    fn my_function_inner(&self, arg: i32, arg_maybe_default: i32) -> Result<i32, NativeApiError> {
        // Example of enhanced error handling
        if arg < 0 {
            return Err(NativeApiError::operation("First argument must be non-negative"));
        }
        
        Ok(self.protected_prop
            + self.some_prop
            + arg
            + self.private_field
            + arg_maybe_default)
    }

    fn get_string_inner(&mut self) -> String {
        self.protected_prop += 1;
        "Some string from rust".to_string()
    }

    fn my_procedure_inner(&mut self) {
        self.protected_prop += 5;
    }
}

extern_functions! {
    MyAddIn::default(),
}
```