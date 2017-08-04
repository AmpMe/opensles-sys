#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod bindings;

// From https://www.khronos.org/registry/OpenSL-ES/api/1.0/OpenSLES.h
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SLResult {
    Success,
    PreconditionsViolated,
    ParameterInvalid,
    MemoryFailure,
    ResourceError,
    ResourceLost,
    IoError,
    BufferInsufficient,
    ContentCorrupted,
    ContentUnsupported,
    ContentNotFound,
    PermissionDenied,
    FeatureUnsupported,
    InternalError,
    UnknownError,
    OperationAborted,
    ControlLost
}

impl From<bindings::SLresult> for SLResult {
    fn from(res: bindings::SLresult) -> Self {
        match res {
            0 => SLResult::Success,
            1 => SLResult::PreconditionsViolated,
            2 => SLResult::ParameterInvalid,
            3 => SLResult::MemoryFailure,
            4 => SLResult::ResourceError,
            5 => SLResult::ResourceLost,
            6 => SLResult::IoError,
            7 => SLResult::BufferInsufficient,
            8 => SLResult::ContentCorrupted,
            9 => SLResult::ContentUnsupported,
            10 => SLResult::ContentNotFound,
            11 => SLResult::PermissionDenied,
            12 => SLResult::FeatureUnsupported,
            13 => SLResult::InternalError,
            14 => SLResult::UnknownError,
            15 => SLResult::OperationAborted,
            16 => SLResult::ControlLost,
            _ => SLResult::UnknownError
        }
    }
}
