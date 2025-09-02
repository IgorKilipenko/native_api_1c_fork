use proc_macro2::TokenStream;
use quote::quote;

use crate::derive_addin::collectors_base::{CodeGenerationResult, EnumerableCollector};

/// Генератор кода для AddInWrapper реализации
pub struct AddInCodeGenerator {
    struct_ident: syn::Ident,
    connection_field_name: TokenStream,
    add_in_name_literal: TokenStream,
}

impl AddInCodeGenerator {
    pub fn new(
        struct_ident: syn::Ident,
        connection_field_name: TokenStream,
        add_in_name_literal: TokenStream,
    ) -> Self {
        Self {
            struct_ident,
            connection_field_name,
            add_in_name_literal,
        }
    }

    /// Генерирует полную реализацию AddInWrapper
    pub fn generate_impl_block(
        &self,
        prop_results: Vec<CodeGenerationResult>,
        func_results: Vec<CodeGenerationResult>,
    ) -> TokenStream {
        let struct_ident = &self.struct_ident;
        let connection_field_name = &self.connection_field_name;
        let add_in_name_literal = &self.add_in_name_literal;

        // Фильтруем только нужные результаты
        let prop_definitions: Vec<_> = prop_results
            .into_iter()
            .filter(|r| r.is_needed)
            .map(|r| r.code)
            .collect();

        let func_definitions: Vec<_> = func_results
            .into_iter()
            .filter(|r| r.is_needed)
            .map(|r| r.code)
            .collect();

        quote! {
            impl native_api_1c::native_api_1c_core::interface::AddInWrapper for #struct_ident {
                fn init(&mut self, interface: &'static native_api_1c::native_api_1c_core::ffi::connection::Connection) -> bool {
                    self.#connection_field_name = std::sync::Arc::new(Some(interface));
                    true
                }

                fn get_info(&self) -> u16 {
                    2000
                }

                fn done(&mut self) {}

                fn register_extension_as(&mut self) -> &[u16] {
                    &utf16_lit::utf16_null!(#add_in_name_literal)
                }

                #(#prop_definitions)*
                #(#func_definitions)*

                fn set_locale(&mut self, loc: &[u16]) {
                    // Реализация по умолчанию
                }

                fn set_user_interface_language_code(&mut self, lang: &[u16]) {
                    // Реализация по умолчанию
                }
            }
        }
    }

    /// Генерирует код для свойств
    pub fn generate_props_code<T>(&self, props: &[T]) -> Vec<CodeGenerationResult>
    where
        T: for<'a> EnumerableCollector<'a, T>,
    {
        let mut results = Vec::new();
        
        for (index, _) in props.iter().enumerate() {
            // Здесь будет логика генерации для каждого свойства
            // Пока что возвращаем пустой результат
            results.push(CodeGenerationResult::not_needed("placeholder"));
        }
        
        results
    }

    /// Генерирует код для функций
    pub fn generate_functions_code<T>(&self, functions: &[T]) -> Vec<CodeGenerationResult>
    where
        T: for<'a> EnumerableCollector<'a, T>,
    {
        let mut results = Vec::new();
        
        for (index, _) in functions.iter().enumerate() {
            // Здесь будет логика генерации для каждой функции
            // Пока что возвращаем пустой результат
            results.push(CodeGenerationResult::not_needed("placeholder"));
        }
        
        results
    }
}
