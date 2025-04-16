use sha3::Sha3_256;
use hmac::{Hmac, Mac};

#[derive(Debug)]
/// HMAC signer to verify authenticity of Eligibility, Slack ID, and Github Username fields
pub struct VerificationSigner {
    secret: Vec<u8>
}

impl VerificationSigner {
    pub fn new(secret: String) -> Self {
        Self {
            secret: secret.into_bytes()
        }
    }
    
    fn mac(&self) -> Hmac<Sha3_256> {
        Hmac::<Sha3_256>::new_from_slice(&self.secret)
            .expect("secret is valid")
    }
    pub fn hash_secret(&self, data: &str) -> String {
        let mut mac = self.mac();
        mac.update(data.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    pub fn verify_secret(&self, expected: &str, data: &str) -> bool {
        let mut mac = self.mac();
        mac.update(data.as_bytes());
        mac.verify_slice(expected.as_bytes()).is_ok()
    }
}
