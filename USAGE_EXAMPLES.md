# 📖 Примеры использования

> Гайд по использованию на русском языке можно посмотреть 
> [здесь](https://infostart.ru/1c/articles/1920565/) и задать вопросы по использованию, но не 
> оставляйте там комментарии об ошибках, т.к. там сложно это обсуждать. Лучше создайте issue в этом 
> репозитории.

## 🆕 Новые упрощенные атрибуты (Рекомендуется)

Библиотека теперь поддерживает упрощенный синтаксис атрибутов для лучшего опыта разработки:

### Свойства
```rust
#[prop(ty = Int, name = "MyProp", ru = "МоеСвойство", readable, writable)]
pub my_property: i32,
```

### Функции
```rust
#[func(name = "MyFunction", ru = "МояФункция")]
#[param(ty = Int)]
#[param(ty = Int, default = 12)]
#[return_type(ty = Int, result)]
pub my_function: fn(&Self, i32, i32) -> Result<i32, NativeApiError>,
```

### Соединение
```rust
#[connection]
connection: Arc<Option<&'static Connection>>,
```

## Устаревшие атрибуты (все еще поддерживаются)

### Атрибуты `#[add_in_prop(...)]`
- `name` - имя свойства в 1C
- `name_ru` - имя свойства в 1C на русском языке
- `readable` - свойство доступно для чтения из 1C
- `writable` - свойство доступно для записи из 1C

Доступные типы свойств: `i32`, `f64`, `bool`, `String`

## Функции или процедуры `#[add_in_func(...)]`
- `name` - имя функции в 1C
- `name_ru` - имя функции в 1C на русском языке

### Входные аргументы, `#[arg(ty = ...)]`, для каждого типа аргумента должно быть указано, одно из:

| Определение типа | Тип Rust               | Тип 1C                 |
|-----------------|------------------------|------------------------|
| `Int`           | `i32`                   | `Number` (Int)          |
| `Float`         | `f64`                   | `Number` (Float or Int) |
| `Bool`          | `bool`                  | `Boolean`               |
| `Str`           | `String`                | `String`                |
| `Date`          | `chrono::NaiveDateTime` | `Date`                  |
| `Blob`          | `Vec<u8>`               | `BinaryData`            |

### Возвращаемые значения, `#[returns(ty = ...)]`, тип должен быть указан, один из:

| Определение типа | Тип Rust               | Тип 1C      |
|-----------------|------------------------|-------------|
| `Int`           | `i32`                   | `Number`     |
| `Float`         | `f64`                   | `Number`     |
| `Bool`          | `bool`                  | `Boolean`    |
| `Str`           | `String`                | `String`     |
| `Date`          | `chrono::NaiveDateTime` | `Date`       |
| `Blob`          | `Vec<u8>`               | `BinaryData` |
| `None`          | `()`                    | `Undefined`  |

Дополнительно можно использовать `Result<T, NativeApiError>`, где `T` - один из вышеперечисленных. В этом случае в атрибуте `#[returns(...)]` должно быть указано `result`: `#[returns(Int, result)]` для `Result<i32, NativeApiError>`

## 🆕 Улучшенная обработка ошибок

Библиотека теперь предоставляет комплексную обработку ошибок с конкретными типами ошибок:

```rust
use native_api_1c_core::errors::NativeApiError;

// Доступные конкретные типы ошибок:
// - PropertyError: Ошибки, связанные со свойствами (не найдено, не читается, не записывается)
// - MethodError: Ошибки, связанные с методами (не найден, ошибка выполнения, неверные параметры)
// - MemoryError: Ошибки управления памятью
// - InitializationError: Ошибки инициализации компонента
// - ParameterError: Ошибки валидации параметров
// - TypeConversionError: Ошибки преобразования типов

fn my_function(&self, arg: i32) -> Result<i32, NativeApiError> {
    if arg < 0 {
        return Err(ParameterError::invalid_value("Аргумент должен быть положительным").into());
    }
    Ok(arg * 2)
}
```

## Пример

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
# Рекомендуемый способ - через наш форк с улучшениями
native_api_1c = { git = "https://github.com/IgorKilipenko/native_api_1c_fork.git", branch = "main" }
# Альтернативно - оригинальная версия
# native_api_1c = "0.10.5"
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
    /// соединение с 1C, используется для вызова событий
    /// Arc используется для обеспечения доступа из нескольких потоков
    #[connection] // Новый упрощенный атрибут
    connection: Arc<Option<&'static Connection>>,

    /// Свойство, доступное для чтения и записи из 1C
    #[prop(ty = Int, name = "MyProp", ru = "МоеСвойство", readable, writable)] // Новый упрощенный атрибут
    pub some_prop: i32,

    /// Свойство, доступное для чтения из 1C, но не для записи
    #[prop(ty = Int, name = "ProtectedProp", ru = "ЗащищенноеСвойство", readable)] // Новый упрощенный атрибут
    pub protected_prop: i32,

    /// Функция, принимающая один или два аргумента и возвращающая результат
    /// В 1C она может быть вызвана как:
    /// ```bsl
    ///  CompObj.MyFunction(10, 15); // 2-й аргумент = 15
    ///  CompObj.MyFunction(10);     // 2-й аргумент = 12 (значение по умолчанию)
    /// ```
    /// Если функция возвращает ошибку, но не паникует, то 1C выбросит исключение
    #[func(name = "MyFunction", ru = "МояФункция")] // Новый упрощенный атрибут
    #[param(ty = Int)] // Новый упрощенный атрибут
    #[param(ty = Int, default = 12)] // значение по умолчанию для второго аргумента
    #[return_type(ty = Int, result)] // Новый упрощенный атрибут
    pub my_function: fn(&Self, i32, i32) -> Result<i32, NativeApiError>, // Улучшенная обработка ошибок

    /// Функция, не принимающая аргументов и возвращающая строку
    #[func(name = "GetString", ru = "ПолучитьСтроку")] // Новый упрощенный атрибут
    #[return_type(ty = Str)] // Новый упрощенный атрибут
    pub get_string: fn(&mut Self) -> String,

    /// Процедура, не принимающая аргументов и ничего не возвращающая
    #[func(name = "MyProcedure", ru = "МояПроцедура")] // Новый упрощенный атрибут
    pub my_procedure: fn(&mut Self),

    /// Приватное поле, не видимое из 1C
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
        // Пример улучшенной обработки ошибок
        if arg < 0 {
            return Err(NativeApiError::operation("Первый аргумент должен быть неотрицательным"));
        }
        
        Ok(self.protected_prop
            + self.some_prop
            + arg
            + self.private_field
            + arg_maybe_default)
    }

    fn get_string_inner(&mut self) -> String {
        self.protected_prop += 10;
        "Какая-то строка из rust".to_string()
    }

    fn my_procedure_inner(&mut self) {
        self.protected_prop += 5;
    }
}

extern_functions! {
    SampleAddIn::default(),
}
```

### Добавление дополнительных объектов

Метод `extern_functions!` может принимать несколько объектов, например:

```rust
extern_functions! {
    SampleAddIn::default(),
    AnotherAddIn::default(),
    YetAnotherAddIn::default(),
}
```

Эти объекты должны иметь реализованный трейт `AddIn`. Это можно сделать либо с помощью `#[derive(AddIn)]`, либо вручную. Последнее полезно, когда вам нужно необычное поведение, которое нельзя вывести автоматически.

## 🆕 Последние улучшения

### Улучшенная обработка ошибок
- **Комплексные типы ошибок** с конкретными вариантами ошибок для различных сценариев
- **Лучшее распространение ошибок** по всему коду
- **Информативные сообщения об ошибках** для упрощения отладки

### Упрощенные атрибуты
- **Новые короткие имена атрибутов** для лучшего опыта разработки
- **Обратная совместимость** с существующими атрибутами `#[add_in_*]`
- **Постепенный путь миграции** к новому синтаксису

### Комплексное тестирование
- **73 новых теста** покрывающих все основные компоненты
- **Безопасные mock-реализации** для избежания небезопасных FFI вызовов во время тестирования
- **100% покрытие тестами** основной функциональности

### Рефакторинг макросов
- **Улучшенная генерация кода** с лучшей архитектурой
- **Базовые трейты** для унифицированного интерфейса коллекторов
- **Повышенная производительность** и поддерживаемость

## Руководство по миграции

### От устаревших атрибутов к новым атрибутам

| Устаревший | Новый | Примечания |
|------------|-------|------------|
| `#[add_in_prop]` | `#[prop]` | Более короткий синтаксис |
| `#[add_in_func]` | `#[func]` | Более короткий синтаксис |
| `#[add_in_con]` | `#[connection]` | Более описательный |
| `#[arg]` | `#[param]` | Более описательный |
| `#[returns]` | `#[return_type]` | Более описательный |
| `name_ru` | `ru` | Более короткое имя параметра |

### От общих ошибок к конкретным ошибкам

```rust
// Старый способ
fn my_function(&self, arg: i32) -> Result<i32, ()> {
    if arg < 0 {
        return Err(());
    }
    Ok(arg * 2)
}

// Новый способ
use native_api_1c_core::errors::NativeApiError;

fn my_function(&self, arg: i32) -> Result<i32, NativeApiError> {
    if arg < 0 {
        return Err(NativeApiError::operation("Аргумент должен быть неотрицательным"));
    }
    Ok(arg * 2)
}
```
