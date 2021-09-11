use super::edger8r::{SgxEnclaveId, SgxStatus};

extern "C" {
    pub fn sgx_create_enclave(
        filename: *const u8,
        debug: i32,
        token: *mut u8,
        updated: i32,
        eid: SgxEnclaveId,
    ) -> SgxStatus;
}
