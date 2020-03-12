use crate::trust::SecTrustRef;
use crate::base::{SecCertificateRef, SecIdentityRef};

use core_foundation_sys::array::CFArrayRef;


// https://developer.apple.com/documentation/security/sec_identity_t?language=objc
pub enum sec_identity { }
pub type sec_identity_t = *mut sec_identity;

pub enum sec_trust { }
pub type sec_trust_t = *mut sec_trust;

pub enum sec_certificate { }
pub type sec_certificate_t = *mut sec_certificate;


extern "C" {
    pub fn sec_trust_create(trust: SecTrustRef) -> sec_trust_t;
    pub fn sec_trust_copy_ref(trust: sec_trust_t) -> SecTrustRef;

    pub fn sec_identity_create(identity: SecIdentityRef) -> sec_identity_t;
    pub fn sec_identity_create_with_certificates(identity: SecIdentityRef,
                                                 certificates: CFArrayRef) -> sec_identity_t;
    pub fn sec_identity_copy_ref(identity: sec_identity_t) -> SecIdentityRef;
    pub fn sec_identity_copy_certificates_ref(identity: sec_identity_t) -> CFArrayRef;
    
    pub fn sec_certificate_create(certificate: SecCertificateRef) -> sec_certificate_t;
    pub fn sec_certificate_copy_ref(certificate: sec_certificate_t) -> SecCertificateRef;
}