use crate::protocol_types::sec_identity_t;
use crate::secure_transport::SSLProtocol;
use crate::cipher_suite::SSLCipherSuite;
use crate::secure_transport::SSLCiphersuiteGroup;

use libc::c_char;

//  ALPN in SecureTransport 
// https://forums.developer.apple.com/thread/116193
// 
// SecIdentityRef identity = ... load the certificate...;
// sec_protocol_options_set_local_identity(sec_options, sec_identity_create(identity));
// sec_protocol_options_add_tls_application_protocol(sec_options, "h2");
// 

// SEC_OBJECT_DECL!(sec_protocol_options);
// https://developer.apple.com/documentation/security/sec_protocol_options_t?language=objc
// typedef NSObject<OS_sec_protocol_options> *sec_protocol_options_t;
pub enum sec_protocol_options { }
pub type sec_protocol_options_t = *mut sec_protocol_options;

// #include <dispatch/data.h>
// https://developer.apple.com/documentation/dispatch/dispatch_data_t#discussion
// typedef NSObject<OS_dispatch_data> *dispatch_data_t;
pub enum dispatch_data { }
pub type dispatch_data_t = *mut dispatch_data;

extern "C" {
    // API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
    pub fn sec_protocol_options_set_local_identity(options: sec_protocol_options_t, identity: sec_identity_t);

    // API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
    pub fn sec_protocol_options_add_tls_ciphersuite(options: sec_protocol_options_t, ciphersuite: SSLCipherSuite);

    // API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
    pub fn sec_protocol_options_add_tls_ciphersuite_group(options: sec_protocol_options_t, group: SSLCiphersuiteGroup);

    // API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
    pub fn sec_protocol_options_set_tls_min_version(options: sec_protocol_options_t, version: SSLProtocol);

    // API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
    pub fn sec_protocol_options_set_tls_max_version(options: sec_protocol_options_t, version: SSLProtocol);

    // API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
    pub fn sec_protocol_options_add_tls_application_protocol(options: sec_protocol_options_t, application_protocol: *const c_char);

    // API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
    pub fn sec_protocol_options_set_tls_server_name(options: sec_protocol_options_t, server_name: *const c_char);

    // API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
    pub fn sec_protocol_options_set_tls_diffie_hellman_parameters(options: sec_protocol_options_t, params: dispatch_data_t);

    // API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
    pub fn sec_protocol_options_add_pre_shared_key(options: sec_protocol_options_t, psk: dispatch_data_t, psk_identity: dispatch_data_t);

    // API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
    pub fn sec_protocol_options_set_tls_tickets_enabled(options: sec_protocol_options_t, tickets_enabled: bool);

    // API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
    pub fn sec_protocol_options_set_tls_is_fallback_attempt(options: sec_protocol_options_t, is_fallback_attempt: bool);

    // API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
    pub fn sec_protocol_options_set_tls_resumption_enabled(options: sec_protocol_options_t, resumption_enabled: bool);

    // API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
    pub fn sec_protocol_options_set_tls_false_start_enabled(options: sec_protocol_options_t, false_start_enabled: bool);

    // API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
    pub fn sec_protocol_options_set_tls_ocsp_enabled(options: sec_protocol_options_t, ocsp_enabled: bool);

    // API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
    pub fn sec_protocol_options_set_tls_sct_enabled(options: sec_protocol_options_t, sct_enabled: bool);

    // API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
    pub fn sec_protocol_options_set_tls_renegotiation_enabled(options: sec_protocol_options_t, renegotiation_enabled: bool);

    // API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
    pub fn sec_protocol_options_set_peer_authentication_required(options: sec_protocol_options_t, peer_authentication_required: bool);
}
