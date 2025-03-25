// table_mt.rs

//! Transposition table for multi-threaded execution.

use std::sync::atomic::{AtomicU64, Ordering};

use laura_core::Zobrist;

const DATA_FIELD_SIZE: usize = size_of::<PackedData>();
const NODES_OFFSET: u64 = 8;
const NODES_MASK: u64 = 0xFFFF_FFFF_FFFF_FF00;
const DEPTH_MASK: u64 = 0x0000_0000_0000_00FF;

#[derive(Clone, Copy, Debug, Default)]
pub struct HashEntry {
    zobrist: u64,
    nodes: u64,
    depth: u8,
}

impl HashEntry {
    #[inline(always)]
    pub fn get_depth(self) -> usize {
        self.depth as usize
    }

    #[inline(always)]
    pub fn get_nodes(self) -> usize {
        self.nodes as usize
    }

    #[inline(always)]
    pub fn get_zobrist(self) -> usize {
        self.zobrist as usize
    }
}

impl From<HashEntry> for (u64, u64) {
    #[inline(always)]
    fn from(value: HashEntry) -> Self {
        let data: u64 = value.depth as u64 | (value.nodes << NODES_OFFSET);

        (value.zobrist ^ data, data)
    }
}

impl From<(u64, u64)> for HashEntry {
    #[inline(always)]
    fn from((zobrist, data): (u64, u64)) -> Self {
        Self {
            zobrist: zobrist ^ data,
            nodes: (data & NODES_MASK) >> NODES_OFFSET,
            depth: (data & DEPTH_MASK) as u8,
        }
    }
}

#[derive(Debug, Default)]
pub struct PackedData {
    pub zobrist: AtomicU64,
    pub data: AtomicU64,
}

impl PackedData {
    #[inline(always)]
    fn read(&self, hash: Zobrist) -> Option<HashEntry> {
        let checksum: u64 = hash.0;

        let zobrist: u64 = self.zobrist.load(Ordering::Relaxed);
        let data: u64 = self.data.load(Ordering::Acquire);

        if zobrist ^ checksum == data {
            Some(HashEntry::from((zobrist, data)))
        } else {
            None
        }
    }

    #[inline(always)]
    fn read_unchecked(&self) -> HashEntry {
        let zobrist: u64 = self.zobrist.load(Ordering::Relaxed);
        let data: u64 = self.data.load(Ordering::Acquire);

        HashEntry::from((zobrist, data))
    }

    #[inline(always)]
    fn write(&self, entry: HashEntry) {
        let (zobrist, data) = entry.into();

        self.data.store(data, Ordering::Release);
        self.zobrist.store(zobrist, Ordering::Relaxed);
    }
}

#[derive(Debug)]
pub struct HashTable {
    pub table: Vec<PackedData>,
}

impl Default for HashTable {
    fn default() -> Self {
        let mut hash_table: HashTable = Self { table: Vec::new() };
        hash_table.resize(Self::DEFAULT_SIZE);
        hash_table
    }
}

impl HashTable {
    const DEFAULT_SIZE: usize = 64;

    #[inline(always)]
    pub fn resize(&mut self, mb_size: usize) {
        let new_len: usize = (mb_size << 20) / DATA_FIELD_SIZE;
        self.table.resize_with(new_len, PackedData::default);
    }

    #[inline(always)]
    fn get_index(&self, hash: Zobrist) -> usize {
        let hash: u128 = hash.0 as u128;
        let len: u128 = self.table.len() as u128;

        ((hash * len) >> 64) as usize
    }

    #[inline(always)]
    pub fn probe(&self, hash: Zobrist) -> Option<HashEntry> {
        unsafe { self.table.get_unchecked(self.get_index(hash)).read(hash) }
    }

    #[inline(always)]
    pub fn add(&self, zobrist: Zobrist, nodes: u64, depth: u8) {
        let old_slot: &PackedData = unsafe { self.table.get_unchecked(self.get_index(zobrist)) };
        let old: HashEntry = old_slot.read_unchecked();

        let same: bool = zobrist.0 == old.zobrist;
        let deeper: bool = depth > old.depth;

        if !same || deeper {
            old_slot.write(HashEntry {
                zobrist: zobrist.0,
                nodes,
                depth,
            });
        }
    }
}
