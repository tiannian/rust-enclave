use core::ffi::c_void;

use crate::raw::edger8r::SgxStatus;

pub type OCallFuncPtr = extern "C" fn(*mut c_void) -> SgxStatus;

#[repr(C)]
pub struct OCallTable<const N: usize> {
    nr_ocall: usize,
    pub table: [OCallFuncPtr; N],
}

