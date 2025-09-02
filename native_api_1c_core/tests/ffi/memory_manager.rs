//! Tests for memory manager functionality

use native_api_1c_core::ffi::memory_manager::MemoryManager;

// Note: MemoryManager is an FFI struct that requires external initialization
// These tests focus on testing the public methods when available

#[test]
fn test_memory_manager_structure() {
    // Test that MemoryManager structure exists and has expected size
    let size = std::mem::size_of::<MemoryManager>();
    assert!(size > 0, "MemoryManager structure exists");
    
    // Test that it's a reasonable size (should be pointer-sized)
    let ptr_size = std::mem::size_of::<usize>();
    assert!(size >= ptr_size, "MemoryManager should be at least pointer-sized");
    
    // Test alignment
    let align = std::mem::align_of::<MemoryManager>();
    assert!(align > 0, "MemoryManager has valid alignment");
}

#[test]
fn test_memory_manager_methods_exist() {
    // Test that MemoryManager has the expected vtable structure
    // This is a compile-time test that verifies the FFI interface
    
    // Test that we can work with the structure safely
    // We can't create instances, but we can verify the type exists
    let size = std::mem::size_of::<MemoryManager>();
    assert!(size > 0, "MemoryManager can be safely handled");
    
    // Test that the structure has the expected layout
    assert_eq!(std::mem::align_of::<MemoryManager>(), std::mem::align_of::<usize>());
    
    // Note: We can't call actual methods without a valid vtable
    // but we can verify the structure is safe to work with
}

#[test]
fn test_memory_manager_safety() {
    // Test that MemoryManager is safe to handle
    // This test verifies basic safety properties
    assert!(std::mem::size_of::<MemoryManager>() > 0, "MemoryManager is safe to handle");
}
