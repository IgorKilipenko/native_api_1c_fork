#!/bin/bash

# Скрипт для массового обновления тестов для использования NativeApiError

echo "🔄 Обновляем тесты для использования NativeApiError..."

# Обновляем component_creation.rs
echo "📝 Обновляем tests/ffi/component_creation.rs..."
sed -i 's/Result<ParamValue, ()>/Result<ParamValue, NativeApiError>/g' tests/ffi/component_creation.rs
sed -i 's/Result<(), ()>/Result<(), NativeApiError>/g' tests/ffi/component_creation.rs
sed -i 's/Err(())/Err(NativeApiError::operation("Operation failed"))/g' tests/ffi/component_creation.rs

# Обновляем addin_wrapper.rs
echo "📝 Обновляем tests/interface/addin_wrapper.rs..."
sed -i 's/Result<ParamValue, ()>/Result<ParamValue, NativeApiError>/g' tests/interface/addin_wrapper.rs
sed -i 's/Result<(), ()>/Result<(), NativeApiError>/g' tests/interface/addin_wrapper.rs
sed -i 's/Err(())/Err(NativeApiError::operation("Operation failed"))/g' tests/interface/addin_wrapper.rs

# Добавляем импорт NativeApiError в component_creation.rs
echo "📝 Добавляем импорт NativeApiError в component_creation.rs..."
sed -i 's/use native_api_1c_core::{/use native_api_1c_core::{errors::NativeApiError, /' tests/ffi/component_creation.rs

# Добавляем импорт NativeApiError в addin_wrapper.rs
echo "📝 Добавляем импорт NativeApiError в addin_wrapper.rs..."
sed -i 's/use native_api_1c_core::interface::{AddInWrapper, ParamValue, ParamValues};/use native_api_1c_core::{errors::NativeApiError, interface::{AddInWrapper, ParamValue, ParamValues}};/' tests/interface/addin_wrapper.rs

echo "✅ Обновление завершено!"
echo "Теперь запустите: cargo test"
