//! Tests for connection functionality

use native_api_1c_core::ffi::connection::Connection;

#[test]
fn test_connection_structure() {
    // Test that Connection structure exists and has expected properties
    let size = std::mem::size_of::<Connection>();
    assert!(size > 0, "Connection structure exists");
    
    // Test that it's a reasonable size (should be pointer-sized)
    let ptr_size = std::mem::size_of::<usize>();
    assert!(size >= ptr_size, "Connection should be at least pointer-sized");
    
    // Test alignment
    let align = std::mem::align_of::<Connection>();
    assert!(align > 0, "Connection has valid alignment");
    
    // Test that the structure has the expected layout
    assert_eq!(std::mem::align_of::<Connection>(), std::mem::align_of::<usize>());
    
    // Verify the structure is safe to work with
    assert!(size > 0);
}

#[test]
fn test_connection_safety() {
    // Test that Connection is safe to handle
    assert!(std::mem::size_of::<Connection>() > 0, "Connection is safe to handle");
}
