use proc_macro2::TokenStream;

/// Базовый trait для всех коллекторов, которые генерируют код
pub trait CodeGenerator {
    /// Генерирует код для реализации
    fn generate(&self) -> Result<TokenStream, darling::Error>;
    
    /// Возвращает имя генерируемого метода
    fn method_name(&self) -> &'static str;
    
    /// Проверяет, нужен ли этот генератор для данной структуры
    fn is_needed(&self) -> bool;
}

/// Базовый trait для коллекторов, которые работают с перечислением
pub trait EnumerableCollector<'a, T: 'a>: FromIterator<(usize, &'a T)> + Default {
    /// Генерирует код на основе собранных данных
    fn generate_code(&self) -> Result<TokenStream, darling::Error>;
    
    /// Возвращает имя генерируемого метода
    fn method_name(&self) -> &'static str;
    
    /// Проверяет, нужен ли этот коллектор
    fn is_needed(&self) -> bool;
}

/// Результат генерации кода
#[derive(Debug)]
pub struct CodeGenerationResult {
    pub method_name: &'static str,
    pub code: TokenStream,
    pub is_needed: bool,
}

impl CodeGenerationResult {
    pub fn new(method_name: &'static str, code: TokenStream, is_needed: bool) -> Self {
        Self {
            method_name,
            code,
            is_needed,
        }
    }
    
    pub fn needed(method_name: &'static str, code: TokenStream) -> Self {
        Self::new(method_name, code, true)
    }
    
    pub fn not_needed(method_name: &'static str) -> Self {
        Self::new(method_name, TokenStream::new(), false)
    }
}

/// Утилиты для работы с коллекторами
pub mod utils {
    use super::*;
    
    /// Создает ошибку для пустого коллектора
    pub fn empty_collector_error(collector_name: &str) -> darling::Error {
        darling::Error::custom(format!("No data found for collector: {}", collector_name))
    }
    
    /// Проверяет, есть ли данные для генерации
    pub fn has_data<T>(data: &[T]) -> bool {
        !data.is_empty()
    }
}
