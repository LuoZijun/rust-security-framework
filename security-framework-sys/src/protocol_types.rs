// SEC_OBJECT_DECL

// https://developer.apple.com/documentation/security/sec_identity_t?language=objc
pub enum sec_identity { }
pub type sec_identity_t = *mut sec_identity;

pub enum sec_trust { }
pub type sec_trust_t = *mut sec_trust;

pub enum sec_certificate { }
pub type sec_certificate_t = *mut sec_certificate;