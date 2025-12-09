use anyhow::{bail, Result};
use jwt_simple::prelude::{
    Claims, Duration, ECDSAP384KeyPairLike, ECDSAP384PublicKeyLike, ES384KeyPair, Ed25519KeyPair,
    EdDSAKeyPairLike, EdDSAPublicKeyLike,
};

enum Signer {
    Ed25519(Ed25519KeyPair),
    P384(ES384KeyPair),
}

impl Signer {
    pub fn sign_token(&self) -> Result<String> {
        let claims = Claims::create(Duration::from_hours(2));

        match self {
            Signer::Ed25519(key_pair) => Ok(key_pair.sign(claims)?),
            Signer::P384(key_pair) => Ok(key_pair.sign(claims)?),
        }
    }
}

pub fn build_token(auth_key: &str) -> Result<String> {
    let signer = load_private_key_auto(auth_key)?;
    let token = signer.sign_token()?;
    Ok(token)
}

fn load_private_key_auto(auth_key: &str) -> Result<Signer> {
    if let Ok(key) = Ed25519KeyPair::from_pem(auth_key) {
        let mut pub_key = key.public_key();
        let key_id = pub_key.create_key_id();
        let mut key_with_id = key;
        key_with_id = key_with_id.with_key_id(key_id);
        return Ok(Signer::Ed25519(key_with_id));
    }

    if let Ok(key) = ES384KeyPair::from_pem(auth_key) {
        let mut pub_key = key.public_key();
        let key_id = pub_key.create_key_id();
        let mut key_with_id = key;
        key_with_id = key_with_id.with_key_id(key_id);
        return Ok(Signer::P384(key_with_id));
    }

    bail!("Unsupported private key format: neither Ed25519 nor P-384");
}
