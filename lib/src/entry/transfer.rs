#![allow(dead_code)]

use bit_vec::BitVec;
use musig2::secp256k1::XOnlyPublicKey;

type Bytes = Vec<u8>;
type Key = XOnlyPublicKey;

use crate::{
    hash::{tagged_hash, HashTag},
    serialize::{
        cpe::CompactPayloadEncoding,
        serialization::{Serialization, SerializationError},
        sighash::Sighash,
    },
    valtype::{account::Account, maybe_common::MaybeCommon, value::ShortVal},
};

#[derive(Clone, Copy)]
pub struct Transfer {
    from: Account,
    to: MaybeCommon<Account>,
    amount: MaybeCommon<ShortVal>,
}

impl Transfer {
    pub fn new(from: Account, to: MaybeCommon<Account>, amount: MaybeCommon<ShortVal>) -> Transfer {
        Transfer { from, to, amount }
    }

    pub fn new_uncommon(from: Account, to: Account, amount: ShortVal) -> Transfer {
        Transfer {
            from,
            to: MaybeCommon::Uncommon(to),
            amount: MaybeCommon::Uncommon(amount),
        }
    }

    pub fn set_from_account_index(&mut self, account_index: u32) {
        self.from.set_account_index(account_index);
    }

    pub fn set_to_account_index(&mut self, account_index: u32) {
        match self.to {
            MaybeCommon::Uncommon(mut to) => {
                to.set_account_index(account_index);
            }
            MaybeCommon::Common(mut to, _) => {
                to.set_account_index(account_index);
            }
        }
    }

    pub fn set_to_common_index(&mut self, common_index: u8) {
        let to = match self.to {
            MaybeCommon::Uncommon(to) => to,
            MaybeCommon::Common(to, _) => to,
        };

        self.to = MaybeCommon::Common(to, common_index);
    }

    pub fn set_amount_common_index(&mut self, common_index: u8) {
        let amount = match self.amount {
            MaybeCommon::Uncommon(amount) => amount,
            MaybeCommon::Common(amount, _) => amount,
        };

        self.amount = MaybeCommon::Common(amount, common_index);
    }
}

impl CompactPayloadEncoding for Transfer {
    fn to_cpe(&self) -> BitVec {
        let mut bit_vec = BitVec::new();

        // Transfer or call
        bit_vec.push(false);

        // Transfer
        bit_vec.push(false);

        // From
        bit_vec.extend(self.from.to_cpe());

        // To
        bit_vec.extend(self.to.to_cpe());

        // Amount
        bit_vec.extend(self.amount.to_cpe());

        bit_vec
    }
}

impl Serialization for Transfer {
    fn serialize(&self) -> Bytes {
        let mut bytes = Vec::<u8>::new();

        // From
        let from = self.from.key().serialize();
        bytes.extend(from);

        // To
        let to = match self.to {
            MaybeCommon::Uncommon(to) => to.key().serialize(),
            MaybeCommon::Common(to, _) => to.key().serialize(),
        };
        bytes.extend(to);

        // Amount
        let amount = match self.amount {
            MaybeCommon::Uncommon(amount) => amount.value(),
            MaybeCommon::Common(amount, _) => amount.value(),
        };
        bytes.extend(amount.to_le_bytes());

        bytes
    }

    fn from_bytes(bytes: Bytes) -> Result<Transfer, SerializationError> {
        // From
        let from = &bytes[0..32];
        let from_key = Key::from_slice(from).map_err(|_| SerializationError::KeyParseError)?;
        let from_account = Account::new(from_key);

        // To
        let to = &bytes[32..64];
        let to_key = Key::from_slice(to).map_err(|_| SerializationError::KeyParseError)?;
        let to_account = Account::new(to_key);

        // Amount
        let amount: &[u8] = &bytes[64..68];
        let amount_u32 = u32::from_le_bytes([amount[0], amount[1], amount[2], amount[3]]);
        let amount_short_val = ShortVal::new(amount_u32);

        Ok(Transfer::new_uncommon(
            from_account,
            to_account,
            amount_short_val,
        ))
    }
}

impl Sighash for Transfer {
    fn sighash(&self, prev_state_hash: [u8; 32]) -> [u8; 32] {
        let mut sighash_preimage = Vec::<u8>::new();

        sighash_preimage.extend(prev_state_hash);
        sighash_preimage.extend(self.serialize());

        tagged_hash(sighash_preimage, HashTag::SighashTransfer)
    }
}
