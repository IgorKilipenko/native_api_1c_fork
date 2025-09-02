//! Error types for the native_api_1c_core crate

use thiserror::Error;

/// Main error type for the native_api_1c_core crate
#[derive(Error, Debug, Clone, PartialEq)]
pub enum NativeApiError {
    /// Property-related errors
    #[error("Property error: {0}")]
    Property(#[from] PropertyError),
    
    /// Method-related errors
    #[error("Method error: {0}")]
    Method(#[from] MethodError),
    
    /// Memory allocation errors
    #[error("Memory error: {0}")]
    Memory(#[from] MemoryError),
    
    /// Initialization errors
    #[error("Initialization error: {0}")]
    Initialization(#[from] InitializationError),
    
    /// Parameter validation errors
    #[error("Parameter error: {0}")]
    Parameter(#[from] ParameterError),
    
    /// Type conversion errors
    #[error("Type conversion error: {0}")]
    TypeConversion(#[from] TypeConversionError),
    
    /// Generic operation errors
    #[error("Operation failed: {message}")]
    Operation { message: String },
}

/// Property-related errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum PropertyError {
    /// Property not found
    #[error("Property not found: {name}")]
    NotFound { name: String },
    
    /// Property index out of bounds
    #[error("Property index out of bounds: {index}")]
    IndexOutOfBounds { index: usize },
    
    /// Property is not readable
    #[error("Property is not readable: {index}")]
    NotReadable { index: usize },
    
    /// Property is not writable
    #[error("Property is not writable: {index}")]
    NotWritable { index: usize },
    
    /// Property value is invalid
    #[error("Invalid property value: {message}")]
    InvalidValue { message: String },
    
    /// Property alias not found
    #[error("Property alias not found: {alias}")]
    AliasNotFound { alias: usize },
}

/// Method-related errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum MethodError {
    /// Method not found
    #[error("Method not found: {name}")]
    NotFound { name: String },
    
    /// Method index out of bounds
    #[error("Method index out of bounds: {index}")]
    IndexOutOfBounds { index: usize },
    
    /// Method has no return value
    #[error("Method has no return value: {index}")]
    NoReturnValue { index: usize },
    
    /// Method parameter error
    #[error("Method parameter error: {message}")]
    ParameterError { message: String },
    
    /// Method execution failed
    #[error("Method execution failed: {message}")]
    ExecutionFailed { message: String },
    
    /// Method alias not found
    #[error("Method alias not found: {alias}")]
    AliasNotFound { alias: usize },
}

/// Memory allocation errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum MemoryError {
    /// Failed to allocate memory
    #[error("Failed to allocate {size} bytes")]
    AllocationFailed { size: usize },
    
    /// Failed to free memory
    #[error("Failed to free memory at address {address:?}")]
    FreeFailed { address: Option<usize> },
    
    /// Invalid memory address
    #[error("Invalid memory address: {address:?}")]
    InvalidAddress { address: Option<usize> },
    
    /// Memory corruption detected
    #[error("Memory corruption detected")]
    Corruption,
}

/// Initialization errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum InitializationError {
    /// Connection failed
    #[error("Failed to establish connection: {reason}")]
    ConnectionFailed { reason: String },
    
    /// Component already initialized
    #[error("Component already initialized")]
    AlreadyInitialized,
    
    /// Component not initialized
    #[error("Component not initialized")]
    NotInitialized,
    
    /// Invalid configuration
    #[error("Invalid configuration: {message}")]
    InvalidConfig { message: String },
}

/// Parameter validation errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParameterError {
    /// Parameter index out of bounds
    #[error("Parameter index out of bounds: {index}")]
    IndexOutOfBounds { index: usize },
    
    /// Parameter type mismatch
    #[error("Parameter type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },
    
    /// Parameter value is invalid
    #[error("Invalid parameter value: {message}")]
    InvalidValue { message: String },
    
    /// Missing required parameter
    #[error("Missing required parameter: {name}")]
    MissingRequired { name: String },
    
    /// Too many parameters
    #[error("Too many parameters: expected {expected}, got {actual}")]
    TooMany { expected: usize, actual: usize },
}

/// Type conversion errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum TypeConversionError {
    /// Failed to convert to string
    #[error("Failed to convert to string: {message}")]
    ToString { message: String },
    
    /// Failed to convert from string
    #[error("Failed to convert from string: {message}")]
    FromString { message: String },
    
    /// Failed to convert to number
    #[error("Failed to convert to number: {message}")]
    ToNumber { message: String },
    
    /// Failed to convert from number
    #[error("Failed to convert from number: {message}")]
    FromNumber { message: String },
    
    /// Failed to convert date/time
    #[error("Failed to convert date/time: {message}")]
    DateTime { message: String },
    
    /// Unsupported type conversion
    #[error("Unsupported type conversion: {from} -> {to}")]
    Unsupported { from: String, to: String },
}

// Convenience type aliases
pub type NativeApiResult<T> = Result<T, NativeApiError>;
pub type PropertyResult<T> = Result<T, PropertyError>;
pub type MethodResult<T> = Result<T, MethodError>;
pub type MemoryResult<T> = Result<T, MemoryError>;
pub type InitializationResult<T> = Result<T, InitializationError>;
pub type ParameterResult<T> = Result<T, ParameterError>;
pub type TypeConversionResult<T> = Result<T, TypeConversionError>;

// Conversion implementations
impl From<String> for NativeApiError {
    fn from(message: String) -> Self {
        NativeApiError::Operation { message }
    }
}

impl From<&str> for NativeApiError {
    fn from(message: &str) -> Self {
        NativeApiError::Operation { message: message.to_string() }
    }
}

impl From<std::io::Error> for NativeApiError {
    fn from(err: std::io::Error) -> Self {
        NativeApiError::Operation { message: err.to_string() }
    }
}

// Helper functions for common error patterns
impl NativeApiError {
    /// Create a generic operation error
    pub fn operation<T: Into<String>>(message: T) -> Self {
        NativeApiError::Operation { message: message.into() }
    }
    
    /// Create a property not found error
    pub fn property_not_found<T: Into<String>>(name: T) -> Self {
        NativeApiError::Property(PropertyError::NotFound { name: name.into() })
    }
    
    /// Create a method not found error
    pub fn method_not_found<T: Into<String>>(name: T) -> Self {
        NativeApiError::Method(MethodError::NotFound { name: name.into() })
    }
    
    /// Create an index out of bounds error
    pub fn index_out_of_bounds(index: usize) -> Self {
        NativeApiError::Property(PropertyError::IndexOutOfBounds { index })
    }
    
    /// Create a type conversion error
    pub fn type_conversion<T: Into<String>>(message: T) -> Self {
        NativeApiError::TypeConversion(TypeConversionError::ToString { message: message.into() })
    }
}

impl PropertyError {
    /// Create a property not found error
    pub fn not_found<T: Into<String>>(name: T) -> Self {
        PropertyError::NotFound { name: name.into() }
    }
    
    /// Create an index out of bounds error
    pub fn index_out_of_bounds(index: usize) -> Self {
        PropertyError::IndexOutOfBounds { index }
    }
    
    /// Create a not readable error
    pub fn not_readable(index: usize) -> Self {
        PropertyError::NotReadable { index }
    }
    
    /// Create a not writable error
    pub fn not_writable(index: usize) -> Self {
        PropertyError::NotWritable { index }
    }
}

impl MethodError {
    /// Create a method not found error
    pub fn not_found<T: Into<String>>(name: T) -> Self {
        MethodError::NotFound { name: name.into() }
    }
    
    /// Create an index out of bounds error
    pub fn index_out_of_bounds(index: usize) -> Self {
        MethodError::IndexOutOfBounds { index }
    }
    
    /// Create a no return value error
    pub fn no_return_value(index: usize) -> Self {
        MethodError::NoReturnValue { index }
    }
}

impl MemoryError {
    /// Create an allocation failed error
    pub fn allocation_failed(size: usize) -> Self {
        MemoryError::AllocationFailed { size }
    }
    
    /// Create a free failed error
    pub fn free_failed(address: Option<usize>) -> Self {
        MemoryError::FreeFailed { address }
    }
}

impl InitializationError {
    /// Create a connection failed error
    pub fn connection_failed<T: Into<String>>(reason: T) -> Self {
        InitializationError::ConnectionFailed { reason: reason.into() }
    }
    
    /// Create an invalid config error
    pub fn invalid_config<T: Into<String>>(message: T) -> Self {
        InitializationError::InvalidConfig { message: message.into() }
    }
}

impl ParameterError {
    /// Create an index out of bounds error
    pub fn index_out_of_bounds(index: usize) -> Self {
        ParameterError::IndexOutOfBounds { index }
    }
    
    /// Create a type mismatch error
    pub fn type_mismatch<T: Into<String>, U: Into<String>>(expected: T, actual: U) -> Self {
        ParameterError::TypeMismatch { expected: expected.into(), actual: actual.into() }
    }
    
    /// Create an invalid value error
    pub fn invalid_value<T: Into<String>>(message: T) -> Self {
        ParameterError::InvalidValue { message: message.into() }
    }
}

impl TypeConversionError {
    /// Create a to string error
    pub fn to_string<T: Into<String>>(message: T) -> Self {
        TypeConversionError::ToString { message: message.into() }
    }
    
    /// Create a from string error
    pub fn from_string<T: Into<String>>(message: T) -> Self {
        TypeConversionError::FromString { message: message.into() }
    }
    
    /// Create an unsupported conversion error
    pub fn unsupported<T: Into<String>, U: Into<String>>(from: T, to: U) -> Self {
        TypeConversionError::Unsupported { from: from.into(), to: to.into() }
    }
}
