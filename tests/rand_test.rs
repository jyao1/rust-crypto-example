// Copyright (c) 2021 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent
//
// plese use `cargo test -- --nocapture --test-threads=1` to see result.

use rand::RngCore;

#[test]
fn test_rust_rand() {
  println!("test_rust_rand");
  let mut buffer = [0u64; 4];

  for _i in 0..4 {
    for b in buffer.iter_mut() {
      *b = rand::random();
    }
    println!("random - {:0x?}", buffer);
  }
}

#[test]
fn test_rdrand() {
  println!("test_rdrand");
  let mut r = rdrand::RdRand::new().unwrap();
  let mut buffer = [0u8; 40];
 
  for _i in 0..4 {
    r.fill_bytes(&mut buffer);
    println!("rdrand - {:0x?}", buffer);
  }

  let mut r = rdrand::RdSeed::new().unwrap();
  for _i in 0..4 {
    r.fill_bytes(&mut buffer);
    println!("rdseed - {:0x?}", buffer);
  }

}

#[test]
fn test_ring_rand() {
  println!("test_ring_rand");
  let rng = ring::rand::SystemRandom::new();
  for _i in 0..4 {
    let r: [u8; 32] = ring::rand::generate(&rng).unwrap().expose();
    println!("rand - {:0x?}", r);
  }
}
