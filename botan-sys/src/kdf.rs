use std::os::raw::{c_int, c_char};

extern "C" {

    pub fn botan_pbkdf(pbkdf_algo: *const c_char,
                       out: *mut u8,
                       out_len: usize,
                       passphrase: *const c_char,
                       salt: *const u8,
                       salt_len: usize,
                       iterations: usize) -> c_int;

    pub fn botan_pbkdf_timed(pbkdf_algo: *const c_char,
                             out: *mut u8,
                             out_len: usize,
                             passphrase: *const c_char,
                             salt: *const u8,
                             salt_len: usize,
                             milliseconds_to_run: usize,
                             out_iterations_used: *mut usize) -> c_int;

    pub fn botan_kdf(kdf_algo: *const c_char,
                     out: *mut u8,
                     out_len: usize,
                     secret: *const u8,
                     secret_len: usize,
                     salt: *const u8,
                     salt_len: usize,
                     label: *const u8,
                     label_len: usize) -> c_int;

}
