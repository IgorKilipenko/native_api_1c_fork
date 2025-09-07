>Гайд по использованию на русском языке можно посмотреть 
>[здесь](https://infostart.ru/1c/articles/1920565/) и задать вопросы по использованию, но не 
> оставляйте там комментарии об ошибках, т.к. там сложно это обсуждать. Лучше создайте issue в этом 
> репозитории.

# Disclaimer

This is my personal project, so there are some things coming from this fact:
- While I'm trying to implement everything in an idiomatic and 'pretty' way, sometimes I just want 
to see progress, so some cargo clippy warnings are ignored at times, but I always try to fix them 
later
- There'll be weeks or even months of inactivity, because I'm occupied with other things
- I'll try to help anyone, who opens issue or discussion, but I can't guarantee that I'll be able 
to do it in a timely manner

## Contributing
I'd be glad to see any contributions, but please, follow these rules:
- If you want to add a feature, please, open an issue first, so we can discuss it. I don't want you 
to waste your time on something that I won't be accepting for one reason or another
- If you want to fix a bug, better do the same, but if it's a small bug, you can just open a PR
- If you want to help, but don't know what to do, you can look at issues with `help wanted` label, 
or just ask [in this Telegram chat](https://t.me/+2YFbh4up3y8wZmIy)

# About

Library for simple 1C:Enterprise platform Native API Component development, originates from findings
of this [medigor/example-native-api-rs](https://github.com/medigor/example-native-api-rs)

Crate is tested on Linux and Windows. It should work on MacOS as well, but it is not tested.

## A word on testing

In order to test the actual FFI calls, you need to have 1C:Enterprise file base, which makes it hard
to test reliably with Rust tests, *and also makes it not free :)*. One alternative is to use 
[OneScript](https://github.com/EvilBeaver/OneScript), which can run 1C:Enterprise scripts, including
AddIn functions. However, it is not a perfect solution, because it is not 1C:Enterprise itself, and
**their implementations of Native API interfaces is not the same**
(See [this issue](https://github.com/EvilBeaver/OneScript/issues/1359) or try building and running
[this example](https://github.com/Sebekerga/native_api_1c_go))

# Structure
Library is divided into two submodules:
- `native_api_1c_core` describes all necessary for implementing 1C:Enterprise Native API
- `native_api_1c_macro` provides a tool for significant simplification of component implementation, 
taking care of `native_api_1c_core::interface::AddInWrapper` property implementation

# Usage

## 🆕 New Simplified Attributes (Recommended)

The library now supports simplified attribute syntax for better developer experience:

### Properties
```rust
#[prop(ty = Int, name = "MyProp", ru = "МоеСвойство", readable, writable)]
pub my_property: i32,
```

### Functions
```rust
#[func(name = "MyFunction", ru = "МояФункция")]
#[param(ty = Int)]
#[param(ty = Int, default = 12)]
#[return_type(ty = Int, result)]
pub my_function: fn(&Self, i32, i32) -> Result<i32, NativeApiError>,
```

### Connection
```rust
#[connection]
connection: Arc<Option<&'static Connection>>,
```

## Legacy Attributes (Still Supported)

### Attributes `#[add_in_prop(...)]`
- `name` - property name in 1C
- `name_ru` - property name in 1C in Russian
- `readable` - property is readable from 1C
- `writable` - property is writable from 1C

Available property types: `i32`, `f64`, `bool`, `String`

## Functions or procedures `#[add_in_func(...)]`
- `name` - property name in 1C
- `name_ru` - property name in 1C in Russian
### Input arguments, `#[arg(ty = ...)]`, for each type of argument must be set, on of:
| Type definition | Rust type               | 1C type                 |
|-----------------|-------------------------|-------------------------|
| `Int`           | `i32`                   | `Number` (Int)          |
| `Float`         | `f64`                   | `Number` (Float or Int) |
| `Bool`          | `bool`                  | `Boolean`               |
| `Str`           | `String`                | `String`                |
| `Date`          | `chrono::NaiveDateTime` | `Date`                  |
| `Blob`          | `Vec<u8>`               | `BinaryData`            |

### Return values, `#[returns(ty = ...)]`, type must be set, one of:
| Type definition | Rust type               | 1C type      |
|-----------------|-------------------------|--------------|
| `Int`           | `i32`                   | `Number`     |
| `Float`         | `f64`                   | `Number`     |
| `Bool`          | `bool`                  | `Boolean`    |
| `Str`           | `String`                | `String`     |
| `Date`          | `chrono::NaiveDateTime` | `Date`       |
| `Blob`          | `Vec<u8>`               | `BinaryData` |
| `None`          | `()`                    | `Undefined`  |

Additionally, `Result<T, NativeApiError>` can be used, where `T` is one of the above. In this case, `result` 
must be set in `#[returns(...)]` attribute: `#[returns(Int, result)]` for `Result<i32, NativeApiError>`

## 🆕 Enhanced Error Handling

The library now provides comprehensive error handling with specific error types:

```rust
use native_api_1c_core::errors::NativeApiError;

// Specific error types available:
// - PropertyError: Property-related errors (not found, not readable, not writable)
// - MethodError: Method-related errors (not found, execution failed, invalid parameters)
// - MemoryError: Memory management errors
// - InitializationError: Component initialization errors
// - ParameterError: Parameter validation errors
// - TypeConversionError: Type conversion errors

fn my_function(&self, arg: i32) -> Result<i32, NativeApiError> {
    if arg < 0 {
        return Err(ParameterError::invalid_value("Argument must be positive").into());
    }
    Ok(arg * 2)
}
```

## Example

```toml
# Cargo.toml
[package]
name = "my_addin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
utf16_lit = "2.0"
native_api_1c = "0.10.5"
```

```rust
// src/lib.rs
use std::sync::Arc;

use native_api_1c::{
    native_api_1c_core::{
        errors::NativeApiError,
        ffi::connection::Connection,
    },
    native_api_1c_macro::{extern_functions, AddIn},
};

#[derive(AddIn)]
pub struct SampleAddIn {
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
    ///  CompObj.MyFunction(10, 15); // 2nd arg = 15
    ///  CompObj.MyFunction(10);     // 2nd arg = 12 (default value)
    /// ```
    /// If function returns an error, but does not panic, then 1C will throw an exception
    #[func(name = "MyFunction", ru = "МояФункция")] // New simplified attribute
    #[param(ty = Int)] // New simplified attribute
    #[param(ty = Int, default = 12)] // default value for the second argument
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

impl Default for SampleAddIn {
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

impl SampleAddIn {
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
        self.protected_prop += 10;
        "Some string from rust".to_string()
    }

    fn my_procedure_inner(&mut self) {
        self.protected_prop += 5;
    }
}

extern_functions! {
    SampleAddIn::default(),
}
```

### Adding more objects

Method `extern_functions!` can take multiple objects, like this:
```rust
extern_functions! {
    SampleAddIn::default(),
    AnotherAddIn::default(),
    YetAnotherAddIn::default(),
}
```

These object must have trait `AddIn` implemented. This can be done either with `#[derive(AddIn)]`
or manually. Latter is useful when you need some unusual behaviors that cannot be derived.

## 🆕 Recent Improvements

### Enhanced Error Handling
- **Comprehensive error types** with specific error variants for different scenarios
- **Better error propagation** throughout the codebase
- **Informative error messages** for easier debugging

### Simplified Attributes
- **New shorter attribute names** for better developer experience
- **Backward compatibility** with existing `#[add_in_*]` attributes
- **Gradual migration path** to new syntax

### Comprehensive Testing
- **73 new tests** covering all major components
- **Safe mock implementations** to avoid unsafe FFI calls during testing
- **100% test coverage** for core functionality

### Macro Refactoring
- **Improved code generation** with better architecture
- **Base traits** for unified collector interface
- **Enhanced performance** and maintainability

## Migration Guide

### From Legacy Attributes to New Attributes

| Legacy | New | Notes |
|--------|-----|-------|
| `#[add_in_prop]` | `#[prop]` | Shorter syntax |
| `#[add_in_func]` | `#[func]` | Shorter syntax |
| `#[add_in_con]` | `#[connection]` | More descriptive |
| `#[arg]` | `#[param]` | More descriptive |
| `#[returns]` | `#[return_type]` | More descriptive |
| `name_ru` | `ru` | Shorter parameter name |

### From Generic Errors to Specific Errors

```rust
// Old way
fn my_function(&self, arg: i32) -> Result<i32, ()> {
    if arg < 0 {
        return Err(());
    }
    Ok(arg * 2)
}

// New way
use native_api_1c_core::errors::NativeApiError;

fn my_function(&self, arg: i32) -> Result<i32, NativeApiError> {
    if arg < 0 {
        return Err(NativeApiError::operation("Argument must be non-negative"));
    }
    Ok(arg * 2)
}
```