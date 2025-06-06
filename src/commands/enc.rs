use crate::log;
// use crate::err;


pub fn handle_enc(decrypt: bool, file: String, password: String) {
    if decrypt {
        log!("decrypting file: {}, with password: {}", file, password);
    } else {
        log!("encrypting file: {}, with password: {}", file, password);
    }
}
