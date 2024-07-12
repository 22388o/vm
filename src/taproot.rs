#![allow(dead_code)]

use musig2::secp256k1::{Parity, PublicKey, Scalar, Secp256k1, XOnlyPublicKey};
use sha2::Digest as _;
use sha2::Sha256;
use std::cmp::Ordering;
use std::vec;

const LEAF_VERSION: u8 = 0xc0;

pub enum HashTag {
    TapLeafTag,
    TapBranchTag,
    TapTweakTag,
}

#[derive(Clone)]
pub enum Branch {
    Leaf(TapLeaf),
    Branch(Box<TapBranch>),
}

#[derive(Clone)]
pub struct TapLeaf {
    leaf_version: u8,
    tap_script: Vec<u8>,
}

impl TapLeaf {
    pub fn new(tap_script: Vec<u8>) -> TapLeaf {
        TapLeaf {
            leaf_version: LEAF_VERSION,
            tap_script,
        }
    }

    pub fn new_version(tap_script: Vec<u8>, leaf_version: u8) -> TapLeaf {
        TapLeaf {
            leaf_version,
            tap_script,
        }
    }

    pub fn hash(&self) -> [u8; 32] {
        hash_tap_leaf(&self.tap_script, self.leaf_version)
    }

    pub fn hash_as_vec(&self) -> Vec<u8> {
        self.hash().to_vec()
    }

    pub fn into_branch(&self) -> Branch {
        Branch::Leaf(self.clone())
    }
}

#[derive(Clone)]
pub struct TapBranch {
    left_branch: Branch,
    right_branch: Branch,
}

impl TapBranch {
    pub fn new(first: Branch, second: Branch) -> TapBranch {
        let first_branch_vec: Vec<u8> = match &first {
            Branch::Leaf(leaf) => leaf.hash_as_vec(),
            Branch::Branch(branch) => branch.hash_as_vec(),
        };

        let second_branch_vec: Vec<u8> = match &second {
            Branch::Leaf(leaf) => leaf.hash_as_vec(),
            Branch::Branch(branch) => branch.hash_as_vec(),
        };

        match &first_branch_vec.cmp(&second_branch_vec) {
            Ordering::Less => TapBranch {
                left_branch: first,
                right_branch: second,
            },
            _ => TapBranch {
                left_branch: second,
                right_branch: first,
            },
        }
    }

    pub fn hash(&self) -> [u8; 32] {
        let left_branch_vec: Vec<u8> = match &self.left_branch {
            Branch::Branch(branch) => branch.hash_as_vec(),
            Branch::Leaf(leaf) => leaf.hash_as_vec(),
        };

        let right_branch_vec: Vec<u8> = match &self.right_branch {
            Branch::Branch(branch) => branch.hash_as_vec(),
            Branch::Leaf(leaf) => leaf.hash_as_vec(),
        };

        hash_tap_branch(&left_branch_vec, &right_branch_vec)
    }

    pub fn hash_as_vec(&self) -> Vec<u8> {
        self.hash().to_vec()
    }

    pub fn into_branch(&self) -> Branch {
        Branch::Branch(Box::new(self.clone()))
    }
}

pub struct TapRoot {
    inner_key: XOnlyPublicKey,
    uppermost_branch: Branch,
}

impl TapRoot {
    pub fn new(key: PublicKey, branch: Branch) -> TapRoot {
        let inner_key = match &key.x_only_public_key().1 {
            Parity::Even => key.clone().x_only_public_key().0,
            Parity::Odd => key.clone().negate(&Secp256k1::new()).x_only_public_key().0,
        };

        TapRoot {
            inner_key,
            uppermost_branch: branch,
        }
    }

    pub fn inner_key_full(&self) -> PublicKey {
        self.inner_key.public_key(Parity::Even)
    }

    pub fn tap_tweak(&self) -> [u8; 32] {
        let inner_vec: Vec<u8> = self.inner_key.serialize().to_vec();
        let tweak_vec: Vec<u8> = match &self.uppermost_branch {
            Branch::Leaf(leaf) => leaf.hash_as_vec(),
            Branch::Branch(branch) => branch.hash_as_vec(),
        };

        hash_tap_tweak(&inner_vec, &tweak_vec)
    }

    pub fn tweaked_key(&self) -> PublicKey {
        let scalar: Scalar = Scalar::from_be_bytes(self.tap_tweak()).unwrap();

        self.inner_key_full()
            .add_exp_tweak(&Secp256k1::new(), &scalar)
            .unwrap()
    }

    pub fn tweaked_key_parity(&self) -> Parity {
        let (_, parity) = self.tweaked_key().x_only_public_key();
        parity
    }
    pub fn tweaked_key_x_only(&self) -> XOnlyPublicKey {
        let (x_only, _) = self.tweaked_key().x_only_public_key();
        x_only
    }

    pub fn spk(&self) -> Vec<u8> {
        let mut spk: Vec<u8> = vec![0x51, 0x20];
        spk.extend(
            self.tweaked_key()
                .x_only_public_key()
                .0
                .serialize()
                .to_vec(),
        );
        spk
    }
}

pub fn tagged_hash(data: impl AsRef<[u8]>, tag: HashTag) -> [u8; 32] {
    let tag_digest = match tag {
        HashTag::TapLeafTag => Sha256::digest("TapLeaf"),
        HashTag::TapBranchTag => Sha256::digest("TapBranch"),
        HashTag::TapTweakTag => Sha256::digest("TapTweak"),
    };

    let hash: [u8; 32] = {
        Sha256::new()
            .chain_update(&tag_digest)
            .chain_update(&tag_digest)
            .chain_update(&data)
            .finalize()
            .into()
    };

    hash
}

pub fn hash_tap_leaf(raw_script: &Vec<u8>, version: u8) -> [u8; 32] {
    let mut data: Vec<u8> = Vec::new();

    data.extend_from_slice(&[version]);
    data.extend_from_slice(&[(&raw_script).len() as u8]);
    data.extend_from_slice(raw_script);

    tagged_hash(&data, HashTag::TapLeafTag)
}

pub fn hash_tap_branch(left_branch: &Vec<u8>, right_branch: &Vec<u8>) -> [u8; 32] {
    let mut data: Vec<u8> = Vec::new();

    data.extend_from_slice(left_branch);
    data.extend_from_slice(right_branch);

    tagged_hash(&data, HashTag::TapBranchTag)
}

pub fn hash_tap_tweak(inner_key: &Vec<u8>, tweak: &Vec<u8>) -> [u8; 32] {
    let mut data: Vec<u8> = Vec::new();

    data.extend_from_slice(inner_key);
    data.extend_from_slice(tweak);

    tagged_hash(&data, HashTag::TapTweakTag)
}
