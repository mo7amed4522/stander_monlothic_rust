//! Protocol buffer definitions and generated code
//!
//! This module will contain the generated gRPC code from .proto files
//! To use this, you need to:
//! 1. Create a `proto/` directory in your project root
//! 2. Add your .proto files there
//! 3. Configure build.rs to generate Rust code from proto files




pub mod user_services {
    include!("user_services.rs");
}
