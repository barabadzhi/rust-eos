use crate::{
    AccountName,
    bitutil,
    BlockTimestamp,
    Checksum256,
    Extension,
    NumBytes,
    ProducerSchedule as ProducerScheduleV1,
    ProducerScheduleV2,
    Read,
    Signature,
    Write,
};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Read, Write, NumBytes, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[eosio_core_root_path = "crate"]
pub struct BlockHeader { // FIXME/TODO Make generic over the producer schedule!
    pub timestamp: BlockTimestamp,
    pub producer: AccountName,
    pub confirmed: u16,
    pub previous: Checksum256,
    pub transaction_mroot: Checksum256,
    pub action_mroot: Checksum256,
    pub schedule_version: u32,
    pub new_producers: Option<ProducerScheduleV2>,
    pub header_extensions: Vec<Extension>,
}

impl core::fmt::Display for BlockHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "block_num: {}\n\
            id: {}\n\
            timestamp: {}\n\
            producer: {}\n\
            confirmed: {}\n\
            previous: {}\n\
            transaction_mroot: {}\n\
            action_mroot: {}\n\
            schedule_version: {}\n\
            new_producers: {:?}\n\
            header_extensions: {:?}",
            self.block_num(),
            self.id().unwrap(),
            self.timestamp,
            self.producer,
            self.confirmed,
            self.previous,
            self.transaction_mroot,
            self.action_mroot,
            self.schedule_version,
            self.new_producers,
            self.header_extensions,
        )
    }
}

impl BlockHeader {
    pub fn new(
        timestamp: BlockTimestamp,
        producer: AccountName,
        confirmed: u16,
        previous: Checksum256,
        transaction_mroot: Checksum256,
        action_mroot: Checksum256,
        schedule_version: u32,
        new_producers: Option<ProducerScheduleV2>,
        header_extensions: Vec<Extension>,
    ) -> Self {
        Self {
            timestamp,
            producer,
            confirmed,
            previous,
            transaction_mroot,
            action_mroot,
            schedule_version,
            new_producers,
            header_extensions,
        }
    }

    pub fn digest(&self) -> crate::Result<Checksum256> {
        Checksum256::hash(self.clone())
    }

    pub fn serialize(&self) -> crate::Result<Vec<u8>> {
        let mut data = vec![0u8; self.num_bytes()];
        self.write(&mut data, &mut 0).map_err(crate::Error::BytesWriteError)?;
        Ok(data)
    }

    pub fn id(&self) -> crate::Result<Checksum256> {
        let mut result = self.digest()?;
        let mut hash0 = result.hash0();
        hash0 &= 0xffffffff00000000;
        hash0 += bitutil::endian_reverse_u32(self.block_num()) as u64;
        result.set_hash0(hash0);
        Ok(result)
    }

    pub fn block_num(&self) -> u32 {
        Self::num_from_id(self.previous) + 1
    }

    pub fn num_from_id(id: Checksum256) -> u32 {
        bitutil::endian_reverse_u32(id.hash0() as u32)
    }
}

#[derive(Debug, Clone, Default, Read, Write, NumBytes, PartialEq)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[eosio_core_root_path = "crate"]
pub struct SignedBlockHeader {
    pub block_header: BlockHeader,
    pub producer_signature: Signature,
}

impl SignedBlockHeader {
    pub fn new(block_header: BlockHeader, producer_signature: Signature) -> Self {
        Self {
            block_header,
            producer_signature,
        }
    }

    pub fn id(&self) -> crate::Result<Checksum256> {
        self.block_header.id()
    }

    pub fn block_num(&self) -> u32 {
        self.block_header.block_num()
    }
}

impl core::fmt::Display for SignedBlockHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}\n\
            producer_signature: {}",
            self.block_header,
            self.producer_signature,
        )
    }
}


#[derive(Debug, Clone, Default, Read, Write, NumBytes, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[eosio_core_root_path = "crate"]
pub struct BlockHeaderV1 { // FIXME SO do I need to version this? Could just c/p for simplicity
    pub timestamp: BlockTimestamp,
    pub producer: AccountName,
    pub confirmed: u16,
    pub previous: Checksum256,
    pub transaction_mroot: Checksum256,
    pub action_mroot: Checksum256,
    pub schedule_version: u32,
    pub new_producers: Option<ProducerScheduleV1>,
    pub header_extensions: Vec<Extension>,
}

impl core::fmt::Display for BlockHeaderV1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "block_num: {}\n\
            id: {}\n\
            timestamp: {}\n\
            producer: {}\n\
            confirmed: {}\n\
            previous: {}\n\
            transaction_mroot: {}\n\
            action_mroot: {}\n\
            schedule_version: {}\n\
            new_producers: {:?}\n\
            header_extensions: {:?}",
            self.block_num(),
            self.id().unwrap(),
            self.timestamp,
            self.producer,
            self.confirmed,
            self.previous,
            self.transaction_mroot,
            self.action_mroot,
            self.schedule_version,
            self.new_producers,
            self.header_extensions,
        )
    }
}

impl BlockHeaderV1 {
    pub fn new(
        timestamp: BlockTimestamp,
        producer: AccountName,
        confirmed: u16,
        previous: Checksum256,
        transaction_mroot: Checksum256,
        action_mroot: Checksum256,
        schedule_version: u32,
        new_producers: Option<ProducerScheduleV1>,
        header_extensions: Vec<Extension>,
    ) -> Self {
        Self {
            timestamp,
            producer,
            confirmed,
            previous,
            transaction_mroot,
            action_mroot,
            schedule_version,
            new_producers,
            header_extensions,
        }
    }

    pub fn digest(&self) -> crate::Result<Checksum256> {
        Checksum256::hash(self.clone())
    }

    pub fn serialize(&self) -> crate::Result<Vec<u8>> {
        let mut data = vec![0u8; self.num_bytes()];
        self.write(&mut data, &mut 0).map_err(crate::Error::BytesWriteError)?;
        Ok(data)
    }

    pub fn id(&self) -> crate::Result<Checksum256> {
        let mut result = self.digest()?;
        let mut hash0 = result.hash0();
        hash0 &= 0xffffffff00000000;
        hash0 += bitutil::endian_reverse_u32(self.block_num()) as u64;
        result.set_hash0(hash0);
        Ok(result)
    }

    pub fn block_num(&self) -> u32 {
        Self::num_from_id(self.previous) + 1
    }

    pub fn num_from_id(id: Checksum256) -> u32 {
        bitutil::endian_reverse_u32(id.hash0() as u32)
    }
}
