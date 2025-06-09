// Generated protocol buffer bindings for our gRPC services
// These modules are automatically generated from .proto files during build

/// Sudoku service protocol buffer definitions
/// Generated from proto/sudoku.proto
pub mod sudoku {
    tonic::include_proto!("sudoku");
}

/// Factorial service protocol buffer definitions  
/// Generated from proto/factorial.proto
pub mod factorial {
    tonic::include_proto!("factorial");
}

/// Protocol buffer file descriptor set for gRPC reflection
/// This allows clients to introspect the server's available services
pub mod proto {
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("descriptor");
}
