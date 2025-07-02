//###########################################################################
// vk_args_parser.rs
// The specific implementation of functions related to args parser
//
// $Copyright: Copyright (C) village
//###########################################################################
use core::ptr;
use alloc::ffi::CString;
use alloc::vec::Vec;
use alloc::string::String;

// Convert a Vec of Rust strings to C-compatible command line arguments (argc/argv)
pub fn vec_to_c_args(args: &[&str]) -> (usize, *mut *mut u8, Vec<CString>) {
    // First create a vector of CStrings to hold copies of each input string
    let c_strings: Vec<_> = args
        .iter()
        .map(|s| CString::new(*s).expect("Failed to convert string to CString"))
        .collect();
    
    // Create a vector of pointers to the internal representation of each CString
    let mut c_ptrs: Vec<_> = c_strings
        .iter()
        .map(|cs| cs.as_ptr() as *mut u8)
        .collect();
    
    // Append a NULL pointer to the end of the pointer vector to mark the end of arguments
    c_ptrs.push(ptr::null_mut());
    
    // Convert the pointer vector into a form that can be passed to C functions
    let argc = c_ptrs.len() - 1;
    let argv = c_ptrs.as_mut_ptr();
    
    // Return the arguments count, arguments vector, and the CString vector
    // The caller must ensure that c_strings is not dropped while the C function uses argv
    (argc, argv, c_strings)
}

// Convert C-compatible command line arguments (argc/argv) to a Vec of Rust strings
pub fn c_args_to_vec(argc: usize, argv: *mut *mut u8) -> Vec<String> {
    let mut result = Vec::with_capacity(argc);
    
    // Iterate over each argument up to argc
    for i in 0..argc {
        let arg_ptr = unsafe { *argv.offset(i as isize) };
        
        if !arg_ptr.is_null() {
            // Get the pointer to the i-th argument
            let clone_ptr = arg_ptr as *mut i8;
            
            // Convert the arg raw to a C string
            let c_string = unsafe { CString::from_raw(clone_ptr) };
            
            // Convert the C string to a Rust String
            match c_string.into_string() {
                Ok(s) => result.push(s),
                Err(_) => {}
            }
        } else {
            break;
        }
    }
    
    result
}
