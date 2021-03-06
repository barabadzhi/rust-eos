use std::{fmt, io, str::FromStr};

use bitcoin_hashes::{Hash as HashTrait, sha256};
use byteorder::{ByteOrder, LittleEndian};
use secp256k1::{self, Message, Secp256k1};

use crate::{error, hash};
use crate::base58;
use crate::constant::*;
use crate::secret::SecretKey;
use crate::signature::Signature;

/// A Secp256k1 public key
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PublicKey {
    /// Whether this public key should be serialized as compressed
    pub compressed: bool,
    /// The actual Secp256k1 key
    pub key: secp256k1::PublicKey,
}

impl PublicKey {
    /// Write the public key into a writer
    pub fn write_into<W: io::Write>(&self, mut writer: W) {
        let write_res: io::Result<()> = if self.compressed {
            writer.write_all(&self.key.serialize())
        } else {
            writer.write_all(&self.key.serialize_uncompressed())
        };
        debug_assert!(write_res.is_ok());
    }

    /// Serialize the public key to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.write_into(&mut buf);

        buf
    }

    /// Serialize the public key to Eos format string
    pub fn to_eos_fmt(&self) -> String {
        let h160 = hash::ripemd160(&self.key.serialize());
        let mut public_key = [0u8; PUBLIC_KEY_WITH_CHECKSUM_SIZE];
        public_key[..PUBLIC_KEY_SIZE].copy_from_slice(self.to_bytes().as_ref());
        public_key[PUBLIC_KEY_SIZE..].copy_from_slice(&h160.take()[..PUBLIC_KEY_CHECKSUM_SIZE]);

        format!("EOS{}", base58::encode_slice(&public_key))
    }

    /// Verify a signature on a message with public key.
    pub fn verify(&self, message_slice: &[u8], signature: &Signature) -> crate::Result<()> {
        let msg_hash = sha256::Hash::hash(&message_slice);
        self.verify_hash(&msg_hash, &signature)
    }

    /// Verify a signature on a hash with public key.
    pub fn verify_hash(&self, hash: &[u8], signature: &Signature) -> crate::Result<()> {
        let secp = Secp256k1::verification_only();
        let msg = Message::from_slice(&hash).unwrap();
        secp.verify(&msg, &signature.to_standard(), &self.key)?;

        Ok(())
    }

    /// Deserialize a public key from a slice
    pub fn from_slice(data: &[u8]) -> crate::Result<PublicKey> {
        let compressed: bool = match data.len() {
            PUBLIC_KEY_SIZE => true,
            UNCOMPRESSED_PUBLIC_KEY_SIZE => false,
            len => { return Err(base58::Error::InvalidLength(len).into()); }
        };

        Ok(PublicKey {
            compressed,
            key: secp256k1::PublicKey::from_slice(data)?,
        })
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.compressed {
            write!(f, "{}", self.to_eos_fmt())?;
        } else {
            for ch in &self.key.serialize_uncompressed()[..] {
                write!(f, "{:02x}", ch)?;
            }
        }

        Ok(())
    }
}

impl FromStr for PublicKey {
    type Err = error::Error;
    fn from_str(s: &str) -> crate::Result<PublicKey> {
        if !s.starts_with("EOS") {
            return Err(secp256k1::Error::InvalidPublicKey.into());
        }

        let s_hex = base58::from(&s[3..])?;
        if s_hex.len() != PUBLIC_KEY_WITH_CHECKSUM_SIZE {
            return Err(secp256k1::Error::InvalidPublicKey.into());
        }
        let raw = &s_hex[..PUBLIC_KEY_SIZE];

        // Verify checksum
        let expected = LittleEndian::read_u32(&hash::ripemd160(raw)[..4]);
        let actual = LittleEndian::read_u32(&s_hex[PUBLIC_KEY_SIZE..PUBLIC_KEY_WITH_CHECKSUM_SIZE]);
        if expected != actual {
            return Err(base58::Error::BadChecksum(expected, actual).into());
        }

        let key = secp256k1::PublicKey::from_slice(&raw)?;

        Ok(PublicKey { key, compressed: true })
    }
}

impl<'a> From<&'a SecretKey> for PublicKey {
    /// Derive this public key from its corresponding `SecretKey`.
    fn from(sk: &SecretKey) -> PublicKey {
        let secp = Secp256k1::new();

        PublicKey {
            compressed: true,
            key: secp256k1::PublicKey::from_secret_key(&secp, &sk.key),
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use secp256k1::Error::IncorrectSignature;

    use crate::error;
    use crate::error::Error::Secp256k1;
    use crate::signature::Signature;

    use super::PublicKey;

    #[test]
    fn pk_from_str_should_work() {
        let pk_str = "EOS8FdQ4gt16pFcSiXAYCcHnkHTS2nNLFWGZXW5sioAdvQuMxKhAm";
        let pk = PublicKey::from_str(pk_str);
        assert!(pk.is_ok());
        assert_eq!(pk.unwrap().to_string(), pk_str);
    }

    #[test]
    fn pk_from_str_should_error() {
        let pk_str = "8FdQ4gt16pFcSiXAYCcHnkHTS2nNLFWGZXW5sioAdvQuMxKhAm";
        let pk = PublicKey::from_str(pk_str);
        assert!(pk.is_err());
        assert_eq!(pk.unwrap_err(), error::Error::Secp256k1(secp256k1::Error::InvalidPublicKey));
    }

    #[test]
    fn pk_verify_should_work() {
        let pk_str = "EOS86jwjSu9YkD4JDJ7nGK1Rx2SmvNMQ3XiKrvFndABzLDPwk1ZHx";
        let sig_str = "SIG_K1_KomV6FEHKdtZxGDwhwSubEAcJ7VhtUQpEt5P6iDz33ic936aSXx87B2L56C8JLQkqNpp1W8ZXjrKiLHUEB4LCGeXvbtVuR";

        let pk = PublicKey::from_str(pk_str);
        assert!(pk.is_ok());
        let sig = Signature::from_str(sig_str);
        assert!(sig.is_ok());

        let vfy = pk.unwrap().verify("hello".as_bytes(), &sig.unwrap());
        assert!(vfy.is_ok());
    }

    #[test]
    fn pk_verify_should_error() {
        let pk_str = "EOS86jwjSu9YkD4JDJ7nGK1Rx2SmvNMQ3XiKrvFndABzLDPwk1ZHx";
        let sig_str = "SIG_K1_KomV6FEHKdtZxGDwhwSubEAcJ7VhtUQpEt5P6iDz33ic936aSXx87B2L56C8JLQkqNpp1W8ZXjrKiLHUEB4LCGeXvbtVuR";

        let pk = PublicKey::from_str(pk_str);
        assert!(pk.is_ok());
        let sig = Signature::from_str(sig_str);
        assert!(sig.is_ok());

        let vfy = pk.unwrap().verify("world".as_bytes(), &sig.unwrap());
        assert!(vfy.is_err());
        assert_eq!(vfy, Err(Secp256k1(IncorrectSignature)));
    }
}
