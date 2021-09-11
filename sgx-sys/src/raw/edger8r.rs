use core::ffi::c_void;

pub type SgxEnclaveId = u64;
pub type SgxStatus = u32;

extern "C" {
    pub fn sgx_ocalloc(size: usize) -> *mut c_void;

    pub fn sgx_ocfree();

    pub fn sgx_ecall(
        eid: SgxEnclaveId,
        index: isize,
        ocall_table: *const c_void,
        ms: *mut c_void,
    ) -> SgxStatus;

    pub fn sgx_ocall(index: usize, ms: *mut c_void) -> SgxStatus;
}
