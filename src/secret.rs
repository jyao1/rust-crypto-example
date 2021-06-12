// Copyright (c) 2021 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent
//
// This example is just to demonstrate how to use secrecy and zeroize.
// The data structure and algorithm are not secure, from cryptography perspective.
// Please don't use it directly.

use secrecy::{CloneableSecret, DebugSecret, ExposeSecret, Secret};
use zeroize::Zeroize;

pub const SEED_SIZE: usize = 32;
pub const KEY_DATA_SIZE: usize = 32;

#[derive(Debug, Clone)]
pub struct Seed {
    pub s: [u8; SEED_SIZE],
}
impl Default for Seed {
    fn default() -> Seed {
        Seed { s: [0; SEED_SIZE] }
    }
}
impl Seed {
    pub fn new() -> Self {
        let s: [u8; SEED_SIZE] = [0x5A; SEED_SIZE];
        Self { s }
    }
}
impl Zeroize for Seed {
    fn zeroize(&mut self) {
        let ptr = self.s.as_ptr();
        println!("Pointer to seed when zeroize: {:p}", ptr);
        self.s.zeroize();
    }
}
impl CloneableSecret for Seed {}
impl DebugSecret for Seed {}
pub type SecretSeed = Secret<Seed>;
impl ExposeSecret<Seed> for Seed {
    fn expose_secret(&self) -> &Seed {
        &self
    }
}

#[derive(Debug, Clone)]
pub struct KeyData {
    pub d: [u8; KEY_DATA_SIZE],
}
impl Default for KeyData {
    fn default() -> KeyData {
        KeyData {
            d: [0; KEY_DATA_SIZE],
        }
    }
}
impl Zeroize for KeyData {
    fn zeroize(&mut self) {
        let ptr = self.d.as_ptr();
        println!("Pointer to key data when zeroize: {:p}", ptr);
        self.d.zeroize();
    }
}
impl CloneableSecret for KeyData {}
impl DebugSecret for KeyData {}
pub type SecretKeyData = Secret<KeyData>;
impl ExposeSecret<KeyData> for KeyData {
    fn expose_secret(&self) -> &KeyData {
        &self
    }
}

#[derive(Debug, Clone)]
pub struct PrivateKey {
    pub oid: u32,
    pub sk_seed: SecretSeed,
    pub sk_data: SecretKeyData,
    pub pk_seed: Seed,
    pub pk_data: KeyData,
}
impl Default for PrivateKey {
    fn default() -> PrivateKey {
        PrivateKey {
            oid: 0,
            sk_seed: SecretSeed::new(Seed::default()),
            sk_data: SecretKeyData::new(KeyData::default()),
            pk_seed: Seed::default(),
            pk_data: KeyData::default(),
        }
    }
}
impl Drop for PrivateKey {
    fn drop(&mut self) {
        let ptr = self.sk_seed.expose_secret().s.as_ptr();
        println!("Pointer to PrivateKey.sk_seed when drop: {:p}", ptr);
    }
}
impl PrivateKey {
    pub fn export(&self, data: &mut [u8]) -> bool {
        if data.len() != 4 + KEY_DATA_SIZE * 2 {
            return false;
        }
        convert_uint32_to_big_endian_array(&mut data[0..4], self.oid);
        data[4..4 + KEY_DATA_SIZE].copy_from_slice(&self.sk_data.expose_secret().d);
        data[4 + KEY_DATA_SIZE..4 + 2 * KEY_DATA_SIZE].copy_from_slice(&self.pk_data.d);

        true
    }
    pub fn import(&mut self, data: &[u8]) -> bool {
        if data.len() != 4 + KEY_DATA_SIZE * 2 {
            return false;
        }
        self.oid = convert_big_endian_array_to_uint32(&data[0..4]);
        let mut temp: KeyData = KeyData::default();
        temp.d.copy_from_slice(&data[4..4 + KEY_DATA_SIZE]);
        self.sk_data = SecretKeyData::new(temp);
        self.pk_data
            .d
            .copy_from_slice(&data[4 + KEY_DATA_SIZE..4 + 2 * KEY_DATA_SIZE]);

        true
    }

    /*
      This algorithm is just example to show the concept.
      It is not cryptography secure.
      Please don't use it in your code.
    */
    pub fn private_keygen(&mut self, sk_seed: &SecretSeed, pk_seed: &Seed) {
        self.sk_seed = sk_seed.clone();
        self.pk_seed = pk_seed.clone();

        let mut sk_data = Box::new(KeyData::default());
        println!("Pointer to temp sk_data (heap): {:p}", sk_data.d.as_ptr());
        for (i, d) in self.sk_seed.expose_secret().s.iter().enumerate() {
            sk_data.d[i] = 0xff_u8 - *d;
        }
        self.sk_data = SecretKeyData::new(*sk_data.clone());
        sk_data.zeroize();

        let mut pk_data: KeyData = KeyData::default();
        println!("Pointer to temp pk_seed (stack): {:p}", pk_data.d.as_ptr());
        for (i, d) in self.pk_seed.s.iter().enumerate() {
            pk_data.d[i] = *d + 1;
        }
        self.pk_data = pk_data;
    }
}

pub fn convert_uint32_to_big_endian_array(arr: &mut [u8], val: u32) {
    assert_eq!(arr.len(), 4);
    arr[0] = ((val >> 24) & 0xff) as u8;
    arr[1] = ((val >> 16) & 0xff) as u8;
    arr[2] = ((val >> 8) & 0xff) as u8;
    arr[3] = (val & 0xff) as u8;
}

pub fn convert_big_endian_array_to_uint32(arr: &[u8]) -> u32 {
    assert_eq!(arr.len(), 4);
    ((((arr[0] as u32) << 24) & 0xFF000000)
        | (((arr[1] as u32) << 16) & 0x00FF0000)
        | (((arr[2] as u32) << 8) & 0x0000FF00)
        | ((arr[3] as u32) & 0x000000FF)) as u32
}

