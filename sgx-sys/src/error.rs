use crate::raw::edger8r::SgxStatus;

pub enum Error {
    /// Unexpected error
    Unexpected,
    /// The parameter is incorrect
    InvalidParameter,
    /// Not enough memory is available to complete this operation
    OutOfMemory,
    /// Enclave lost after power transition or used in child process created by linux
    EnclaveLost,
    /// SGX API is invoked in incorrect order or state
    InvalidState,
    /// Feature is not supported on this platform
    FeatureNotSupported,
    /// Enclave is exited with pthread
    PthreadExit,
    /// Failed to reserve memory for the enclave
    MemoryMapFailure,
    /// The ecall
    InvalidFunction,
    /// The enclave is out of TCS
    OutOfTcs,
    /// The enclave is crashed
    EnclaveCrashed,
    /// The ECALL is not allowed at this time
    EcallNotAllowed,
    /// The OCALL is not allowed at this time
    OcallNotAllowed,
    /// The enclave is running out of stack
    StackOverrun,
    /// The enclave image has undefined symbol
    UndefinedSymbol,
    /// The enclave image is not correct
    InvalidEnclave,
    /// The enclave id is invalid
    InvalidEnclaveId,
    /// The signature is invalid
    InvalidSignature,
    /// The enclave is signed as product enclave
    NdebugEnclave,
    /// Not enough EPC is available to load the enclave
    OutOfEpc,
    /// Can
    NoDevice,
    /// Page mapping failed in driver
    MemoryMapConflict,
    /// The metadata is incorrect
    InvalidMetadata,
    /// Device is busy
    DeviceBusy,
    /// Metadata version is inconsistent between uRTS and sgx
    InvalidVersion,
    /// The target enclave
    ModeIncompatible,
    /// Can
    EnclaveFileAccess,
    /// The MiscSelct
    InvalidMisc,
    /// The launch token is not correct
    InvalidLaunchToken,
    /// Indicates verification error for reports
    MacMismatch,
    /// The enclave is not authorized
    InvalidAttribute,
    /// The cpu svn is beyond platform
    InvalidCpusvn,
    /// The isv svn is greater than the enclave
    InvalidIsvsvn,
    /// The key name is an unsupported value
    InvalidKeyname,
    /// Indicates aesm didn
    ServiceUnavailable,
    /// The request to aesm timed out
    ServiceTimeout,
    /// Indicates epid blob verification error
    AeInvalidEpidblob,
    /// Enclave not authorized to run
    ServiceInvalidPrivilege,
    /// The EPID group membership is revoked
    EpidMemberRevoked,
    /// SGX needs to be updated
    UpdateNeeded,
    /// Network connecting or proxy setting issue is encountered
    NetworkFailure,
    /// Session is invalid or ended by server
    AeSessionInvalid,
    /// The requested service is temporarily not available
    Busy,
    /// The Monotonic Counter doesn
    McNotFound,
    /// Caller doesn
    McNoAccessRight,
    /// Monotonic counters are used out
    McUsedUp,
    /// Monotonic counters exceeds quota limitation
    McOverQuota,
    /// Key derivation function doesn
    KdfMismatch,
    /// EPID Provisioning failed due to platform not recognized by backend server
    UnrecognizedPlatform,
    /// The config for trigging EPID Provisiong or PSE Provisiong
    UnsupportedConfig,
    /// Not enough privilege to perform the operation
    NoPrivilege,
    /// trying to encrypt an already encrypted enclave
    PclEncrypted,
    /// trying to load a plain enclave using sgx
    PclNotEncrypted,
    /// section mac result does not match build time mac
    PclMacMismatch,
    /// Unsealed key MAC does not match MAC of key hardcoded in enclave binary
    PclShaMismatch,
    /// GUID in sealed blob does not match GUID hardcoded in enclave binary
    PclGuidMismatch,
    /// The file is in bad status
    FileBadStatus,
    /// The Key ID field is all zeros
    FileNoKeyId,
    /// The current file name is different then the original file name
    FileNameMismatch,
    /// The file is not an SGX file
    FileNotSgxFile,
    /// A recovery file can
    FileCantOpenRecoveryFile,
    /// A recovery file can
    FileCantWriteRecoveryFile,
    /// When openeing the file
    FileRecoveryNeeded,
    /// fflush operation
    FileFlushFailed,
    /// fclose operation
    FileCloseFailed,
    /// platform quoting infrastructure does not support the key
    UnsupportedAttKeyId,
    /// Failed to generate and certify the attestation key
    AttKeyCertificationFailure,
    /// The platform quoting infrastructure does not have the attestation key available to generate quote
    AttKeyUninitialized,
    /// TThe data returned by the platform library
    InvalidAttKeyCertData,
    /// The PCK Cert for the platform is not available
    PlatformCertUnavailable,
    /// The ioctl for enclave
    EnclaveCreateInterrupted,
    /// Unknown error
    Unknown,
}

impl From<SgxStatus> for Error {
    fn from(e: SgxStatus) -> Self {
        match e {
            0x0001 => Self::Unexpected,
            0x0002 => Self::InvalidParameter,
            0x0003 => Self::OutOfMemory,
            0x0004 => Self::EnclaveLost,
            0x0005 => Self::InvalidState,
            0x0008 => Self::FeatureNotSupported,
            0x0009 => Self::PthreadExit,
            0x000a => Self::MemoryMapFailure,
            0x1001 => Self::InvalidFunction,
            0x1003 => Self::OutOfTcs,
            0x1006 => Self::EnclaveCrashed,
            0x1007 => Self::EcallNotAllowed,
            0x1008 => Self::OcallNotAllowed,
            0x1009 => Self::StackOverrun,
            0x2000 => Self::UndefinedSymbol,
            0x2001 => Self::InvalidEnclave,
            0x2002 => Self::InvalidEnclaveId,
            0x2003 => Self::InvalidSignature,
            0x2004 => Self::NdebugEnclave,
            0x2005 => Self::OutOfEpc,
            0x2006 => Self::NoDevice,
            0x2007 => Self::MemoryMapConflict,
            0x2009 => Self::InvalidMetadata,
            0x200c => Self::DeviceBusy,
            0x200d => Self::InvalidVersion,
            0x200e => Self::ModeIncompatible,
            0x200f => Self::EnclaveFileAccess,
            0x2010 => Self::InvalidMisc,
            0x2011 => Self::InvalidLaunchToken,
            0x3001 => Self::MacMismatch,
            0x3002 => Self::InvalidAttribute,
            0x3003 => Self::InvalidCpusvn,
            0x3004 => Self::InvalidIsvsvn,
            0x3005 => Self::InvalidKeyname,
            0x4001 => Self::ServiceUnavailable,
            0x4002 => Self::ServiceTimeout,
            0x4003 => Self::AeInvalidEpidblob,
            0x4004 => Self::ServiceInvalidPrivilege,
            0x4005 => Self::EpidMemberRevoked,
            0x4006 => Self::UpdateNeeded,
            0x4007 => Self::NetworkFailure,
            0x4008 => Self::AeSessionInvalid,
            0x400a => Self::Busy,
            0x400c => Self::McNotFound,
            0x400d => Self::McNoAccessRight,
            0x400e => Self::McUsedUp,
            0x400f => Self::McOverQuota,
            0x4011 => Self::KdfMismatch,
            0x4012 => Self::UnrecognizedPlatform,
            0x4013 => Self::UnsupportedConfig,
            0x5002 => Self::NoPrivilege,
            0x6001 => Self::PclEncrypted,
            0x6002 => Self::PclNotEncrypted,
            0x6003 => Self::PclMacMismatch,
            0x6004 => Self::PclShaMismatch,
            0x6005 => Self::PclGuidMismatch,
            0x7001 => Self::FileBadStatus,
            0x7002 => Self::FileNoKeyId,
            0x7003 => Self::FileNameMismatch,
            0x7004 => Self::FileNotSgxFile,
            0x7005 => Self::FileCantOpenRecoveryFile,
            0x7006 => Self::FileCantWriteRecoveryFile,
            0x7007 => Self::FileRecoveryNeeded,
            0x7008 => Self::FileFlushFailed,
            0x7009 => Self::FileCloseFailed,
            0x8001 => Self::UnsupportedAttKeyId,
            0x8002 => Self::AttKeyCertificationFailure,
            0x8003 => Self::AttKeyUninitialized,
            0x8004 => Self::InvalidAttKeyCertData,
            0x8005 => Self::PlatformCertUnavailable,
            0xF001 => Self::EnclaveCreateInterrupted,
            _ => Self::Unknown,
        }
    }
}
