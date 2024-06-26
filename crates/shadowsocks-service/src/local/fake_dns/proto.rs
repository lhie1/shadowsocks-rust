//! Data representation in Database

use std::io;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct StorageMeta {
    pub ipv4_network: String,
    pub ipv6_network: String,
    pub version: u32,
}

impl StorageMeta {
    pub fn decode(v: &[u8]) -> io::Result<StorageMeta> {
        bson::from_slice(v).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn encode_to_vec(&self) -> io::Result<Vec<u8>> {
        bson::to_vec(self).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct IpAddrMapping {
    pub domain_name: String,
    pub expire_time: i64,
}

impl IpAddrMapping {
    pub fn decode(v: &[u8]) -> io::Result<IpAddrMapping> {
        bson::from_slice(v).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn encode_to_vec(&self) -> io::Result<Vec<u8>> {
        bson::to_vec(self).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct DomainNameMapping {
    pub ipv4_addr: String,
    pub ipv6_addr: String,
    pub expire_time: i64,
}

impl DomainNameMapping {
    pub fn decode(v: &[u8]) -> io::Result<DomainNameMapping> {
        bson::from_slice(v).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn encode_to_vec(&self) -> io::Result<Vec<u8>> {
        bson::to_vec(self).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}
