// Copyright (c) 2025 by NVIDIA.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use crate::admin::error::*;
use crate::admin::AdminBackend;
use actix_web::{http::header::Header, HttpRequest};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use jwt_simple::token::Token;
use jwt_simple::{
    claims::NoCustomClaims,
    common::VerificationOptions,
    prelude::{ECDSAP384PublicKeyLike, ES384PublicKey, Ed25519PublicKey, EdDSAPublicKeyLike},
};
use log::info;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct SimpleAdminConfig {
    pub personas: Vec<SimplePersonaConfig>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SimplePersonaConfig {
    pub id: String,
    pub public_key_path: PathBuf,
    #[serde(default)]
    pub key_type: KeyType,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum KeyType {
    EdDSA,
    ES384,
}

impl Default for KeyType {
    fn default() -> Self {
        KeyType::EdDSA
    }
}

pub enum PersonaKey {
    EdDSA(Ed25519PublicKey),
    ES384(ES384PublicKey),
}

pub struct SimplePersona {
    id: String,
    public_key: PersonaKey,
    key_id: String,
}

pub struct SimpleAdminBackend {
    personas: Vec<SimplePersona>,
}

impl SimpleAdminBackend {
    pub fn new(config: SimpleAdminConfig) -> Result<Self> {
        let mut personas = Vec::new();

        for persona_config in &config.personas {
            let user_public_key_pem = std::fs::read_to_string(&persona_config.public_key_path)?;

            let (public_key, key_id) = match persona_config.key_type {
                KeyType::EdDSA => {
                    let pk = Ed25519PublicKey::from_pem(&user_public_key_pem)?;
                    let mut pub_key = pk.clone();
                    let kid = pub_key.create_key_id();
                    (PersonaKey::EdDSA(pk), kid.to_string())
                }
                KeyType::ES384 => {
                    let pk = ES384PublicKey::from_pem(&user_public_key_pem)?;
                    let mut pub_key = pk.clone();
                    let kid = pub_key.create_key_id();
                    (PersonaKey::ES384(pk), kid.to_string())
                }
            };

            personas.push(SimplePersona {
                id: persona_config.id.clone(),
                public_key,
                key_id,
            });
        }

        Ok(SimpleAdminBackend { personas })
    }
}

impl AdminBackend for SimpleAdminBackend {
    fn validate_admin_token(&self, request: &HttpRequest) -> Result<()> {
        let mut token_validated = false;

        let bearer = Authorization::<Bearer>::parse(request)?.into_scheme();
        let token = bearer.token();

        let header = Token::decode_metadata(token)?;
        let alg = header.algorithm();
        let kid_opt = header.key_id();

        let personas_to_try: Vec<&SimplePersona> = if let Some(kid) = kid_opt {
            let filtered: Vec<_> = self
                .personas
                .iter()
                .filter(|p| p.key_id.as_str() == kid)
                .collect();

            if filtered.is_empty() {
                info!("No persona found for kid={kid}, falling back to algorithm matching");
                self.personas.iter().collect()
            } else {
                filtered
            }
        } else {
            self.personas.iter().collect()
        };

        for persona in personas_to_try {
            let res = match (&persona.public_key, alg) {
                (PersonaKey::EdDSA(pk), "EdDSA") => {
                    pk.verify_token::<NoCustomClaims>(token, Some(VerificationOptions::default()))
                }
                (PersonaKey::ES384(pk), "ES384") => {
                    pk.verify_token::<NoCustomClaims>(token, Some(VerificationOptions::default()))
                }
                _ => continue,
            };

            match res {
                Ok(_claims) => {
                    token_validated = true;
                    info!("Admin access granted for {}", persona.id);
                    break;
                }
                Err(e) => {
                    info!("Access not granted for {} due to:\n{}", persona.id, e);
                }
            }
        }

        if !token_validated {
            Err(Error::AdminAccessDenied)
        } else {
            Ok(())
        }
    }
}
