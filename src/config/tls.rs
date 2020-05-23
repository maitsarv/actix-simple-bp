use std::fs::File;
use std::io::BufReader;
use super::CONFIG;

use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig};

pub fn load_ssl_keys() -> ServerConfig{
    // load ssl keys
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open(&CONFIG.ssl_cert_file).unwrap());
    let key_file = &mut BufReader::new(File::open(&CONFIG.ssl_key_file).unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();
    return config;
}