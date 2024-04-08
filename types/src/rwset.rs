use core::fmt;
use std::{
    collections::{btree_map, BTreeMap},
    ops::{Deref, DerefMut},
};

use crate::address::AccountAddress;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum WriteSet {
    V0(WriteSetV0),
}

impl Default for WriteSet {
    fn default() -> Self {
        Self::V0(WriteSetV0::default())
    }
}

impl WriteSet {
    pub fn into_mut(self) -> WriteSetMut {
        match self {
            Self::V0(write_set) => write_set.0,
        }
    }
}

impl Deref for WriteSet {
    type Target = WriteSetV0;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::V0(write_set) => write_set,
        }
    }
}

impl DerefMut for WriteSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::V0(write_set) => write_set,
        }
    }
}

impl WriteSet {
    pub fn insert(&mut self, item: (StateKey, WriteOp)) {
        self.0.insert(item)
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct WriteSetV0(WriteSetMut);

impl WriteSetV0 {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> btree_map::Iter<'_, StateKey, WriteOp> {
        self.0.write_set.iter()
    }

    pub fn get(&self, key: &StateKey) -> Option<&WriteOp> {
        self.0.get(key)
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct WriteSetMut {
    write_set: BTreeMap<StateKey, WriteOp>,
}

impl WriteSetMut {
    pub fn new(write_ops: impl IntoIterator<Item = (StateKey, WriteOp)>) -> Self {
        Self {
            write_set: write_ops.into_iter().collect(),
        }
    }

    pub fn insert(&mut self, item: (StateKey, WriteOp)) {
        self.write_set.insert(item.0, item.1);
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.write_set.is_empty()
    }

    pub fn len(&self) -> usize {
        self.write_set.len()
    }

    pub fn freeze(self) -> Result<WriteSet> {
        // TODO: add structural validation
        Ok(WriteSet::V0(WriteSetV0(self)))
    }

    pub fn get(&self, key: &StateKey) -> Option<&WriteOp> {
        self.write_set.get(key)
    }

    pub fn as_inner_mut(&mut self) -> &mut BTreeMap<StateKey, WriteOp> {
        &mut self.write_set
    }

    pub fn squash(mut self, other: Self) -> Result<Self> {
        use btree_map::Entry::*;
        use WriteOp::*;

        for (key, op) in other.write_set.into_iter() {
            match self.write_set.entry(key) {
                Occupied(mut entry) => {
                    let r = entry.get_mut();
                    match (&r, op) {
                        (Modification(_) | Creation(_), Creation(_))
                        | (Deletion, Deletion | Modification(_)) => {
                            bail!("The given change sets cannot be squashed")
                        }
                        (Modification(_), Modification(data)) => *r = Modification(data),
                        (Creation(_), Modification(data)) => *r = Creation(data),
                        (Modification(_), Deletion) => *r = Deletion,
                        (Deletion, Creation(data)) => *r = Modification(data),
                        (Creation(_), Deletion) => {
                            entry.remove();
                        }
                    }
                }
                Vacant(entry) => {
                    entry.insert(op);
                }
            }
        }

        Ok(self)
    }
}

impl ::std::iter::FromIterator<(StateKey, WriteOp)> for WriteSetMut {
    fn from_iter<I: IntoIterator<Item = (StateKey, WriteOp)>>(iter: I) -> Self {
        let mut ws = WriteSetMut::default();
        for write in iter {
            ws.insert((write.0, write.1));
        }
        ws
    }
}

impl<'a> IntoIterator for &'a WriteSet {
    type Item = (&'a StateKey, &'a WriteOp);
    type IntoIter = btree_map::Iter<'a, StateKey, WriteOp>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            WriteSet::V0(write_set) => write_set.0.write_set.iter(),
        }
    }
}

impl ::std::iter::IntoIterator for WriteSet {
    type Item = (StateKey, WriteOp);
    type IntoIter = btree_map::IntoIter<StateKey, WriteOp>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::V0(write_set) => write_set.0.write_set.into_iter(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub struct AccessPath {
    pub address: AccountAddress,
    #[serde(with = "serde_bytes")]
    pub path: Vec<u8>,
}

impl AccessPath {
    pub fn new(address: AccountAddress, path: Vec<u8>) -> Self {
        AccessPath { address, path }
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TableHandle(pub AccountAddress);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd, Hash)]
pub enum StateKey {
    AccessPath(AccessPath),
    TableItem {
        handle: TableHandle,
        #[serde(with = "serde_bytes")]
        key: Vec<u8>,
    },
    // Only used for testing
    #[serde(with = "serde_bytes")]
    Raw(Vec<u8>),
}

impl fmt::Debug for AccessPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AccessPath {{ address: {:x}, path: {} }}",
            self.address,
            hex::encode(&self.path)
        )
    }
}

#[derive(Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum WriteOp {
    Creation(#[serde(with = "serde_bytes")] Vec<u8>),
    Modification(#[serde(with = "serde_bytes")] Vec<u8>),
    Deletion,
}

impl WriteOp {
    #[inline]
    pub fn is_deletion(&self) -> bool {
        match self {
            WriteOp::Deletion => true,
            WriteOp::Modification(_) | WriteOp::Creation(_) => false,
        }
    }

    pub fn is_creation(&self) -> bool {
        match self {
            WriteOp::Creation(_) => true,
            WriteOp::Modification(_) | WriteOp::Deletion => false,
        }
    }

    pub fn is_modification(&self) -> bool {
        match self {
            WriteOp::Modification(_) => true,
            WriteOp::Creation(_) | WriteOp::Deletion => false,
        }
    }
}

pub trait TransactionWrite {
    fn extract_raw_bytes(&self) -> Option<Vec<u8>>;
}

impl TransactionWrite for WriteOp {
    fn extract_raw_bytes(&self) -> Option<Vec<u8>> {
        match self {
            WriteOp::Creation(v) | WriteOp::Modification(v) => Some(v.clone()),
            WriteOp::Deletion => None,
        }
    }
}

impl std::fmt::Debug for WriteOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteOp::Modification(value) => write!(
                f,
                "Modification({})",
                value
                    .iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect::<String>()
            ),
            WriteOp::Creation(value) => write!(
                f,
                "Creation({})",
                value
                    .iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect::<String>()
            ),
            WriteOp::Deletion => write!(f, "Deletion"),
        }
    }
}
