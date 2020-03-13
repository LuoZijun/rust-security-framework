use crate::secure_transport::SSLProtocol;
use crate::cipher_suite::SSLCipherSuite;
use crate::protocol_options::dispatch_data_t;
use crate::protocol_types::sec_certificate_t;

use libc::c_char;
use libc::c_void;


pub enum sec_protocol_metadata { }
pub type sec_protocol_metadata_t = *mut sec_protocol_metadata;


extern "C" {
    pub fn sec_protocol_metadata_get_negotiated_protocol(metadata: sec_protocol_metadata_t) -> *const c_char;
    pub fn sec_protocol_metadata_copy_peer_public_key(metadata: sec_protocol_metadata_t) -> dispatch_data_t;
    pub fn sec_protocol_metadata_get_negotiated_protocol_version(metadata: sec_protocol_metadata_t) -> SSLProtocol;
    pub fn sec_protocol_metadata_get_negotiated_ciphersuite(metadata: sec_protocol_metadata_t) -> SSLCipherSuite;
    pub fn sec_protocol_metadata_get_early_data_accepted(metadata: sec_protocol_metadata_t) -> bool;
    
    pub fn sec_protocol_metadata_access_peer_certificate_chain(metadata: sec_protocol_metadata_t,
        // void (^handler)(sec_certificate_t certificate)
        handler: extern "C" fn(certificate: sec_certificate_t) -> c_void
    ) -> bool;

    pub fn sec_protocol_metadata_access_ocsp_response(metadata: sec_protocol_metadata_t,
        // void (^handler)(dispatch_data_t ocsp_data)
        handler: extern "C" fn(ocsp_data: dispatch_data_t) -> c_void
    ) -> bool;
    
    pub fn sec_protocol_metadata_access_supported_signature_algorithms(metadata: sec_protocol_metadata_t,
        // void (^handler)(uint16_t signature_algorithm)
        handler: extern "C" fn(signature_algorithm: u16) -> c_void
    ) -> bool;
}