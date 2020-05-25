// Modules are other .rs source files
mod bootstrap;

// Export functions called by R
pub use bootstrap::bootstrap_rs;
pub use bootstrap::sample_with_replacement;
pub use bootstrap::mean;
pub use bootstrap::variance;
pub use bootstrap::stdev;
pub use bootstrap::sterr;
