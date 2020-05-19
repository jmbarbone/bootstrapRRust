// Import dependencies
extern crate libc;
extern crate rand;
extern crate stats;

// Modules are other .rs source files
mod bootstrap;

// Export functions called by R
pub use bootstrap::bootstrap;
