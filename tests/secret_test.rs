// Copyright (c) 2021 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent
//
// plese use `cargo test -- --nocapture --test-threads=1` to see result.

use secrecy::{ExposeSecret, SecretBox, SecretVec, SecretString};

use rust_crypto_example::secret::{PrivateKey, Seed, SecretSeed};

/*
  NOTE: This is demo to show the secret in stack is NOT zeroized,
  because the drop() moves the object.
*/
#[test]
fn test_zeroize_stack() {
    let mut private_key = PrivateKey::default();
    println!("Test private key on stack: {:p}", &private_key);

    let priv_key_sk_seed_ptr = private_key.sk_seed.expose_secret().s.as_ptr();
    println!("Pointer to PrivateKey.sk_seed (stack): {:p}", priv_key_sk_seed_ptr);
    let priv_key_sk_data_ptr = private_key.sk_data.expose_secret().d.as_ptr();
    println!("Pointer to PrivateKey.sk_data (stack): {:p}", priv_key_sk_data_ptr);
    let priv_key_pk_seed_ptr = private_key.pk_seed.s.as_ptr();
    println!("Pointer to PrivateKey.pk_seed (stack): {:p}", priv_key_pk_seed_ptr);
    let priv_key_pk_data_ptr = private_key.pk_data.d.as_ptr();
    println!("Pointer to PrivateKey.pk_data (stack): {:p}", priv_key_pk_data_ptr);

    let sk_seed = SecretSeed::new(Seed::new());
    println!("Pointer to sk_seed (stack): {:p}", sk_seed.expose_secret().s.as_ptr());
    let pk_seed = Seed::new();
    println!("Pointer to pk_seed (stack): {:p}", pk_seed.s.as_ptr());
    private_key.private_keygen(&sk_seed, &pk_seed);
    drop(sk_seed);

    println!("private_key: {:02x?}", private_key);
    println!("private_key.sk_seed: {:02x?}", private_key.sk_seed.expose_secret());
    println!("private_key.sk_data: {:02x?}", private_key.sk_data.expose_secret());
    println!("drop private_key - start ...");
    drop(private_key);
    println!("drop private_key - end.");

    println!("PrivateKey.sk_seed: {:0x?}", unsafe {
        core::slice::from_raw_parts(priv_key_sk_seed_ptr, 16)
    });
    println!("PrivateKey.sk_data: {:0x?}", unsafe {
      core::slice::from_raw_parts(priv_key_sk_data_ptr, 16)
    });
    println!("PrivateKey.pk_seed: {:0x?}", unsafe {
      core::slice::from_raw_parts(priv_key_pk_seed_ptr, 16)
    });
    println!("PrivateKey.pk_data: {:0x?}", unsafe {
      core::slice::from_raw_parts(priv_key_pk_data_ptr, 16)
    });
}

/*
  NOTE: This is demo to show the secret in heap is zeroized.
*/
#[test]
fn test_zeroize_heap() {

    let mut private_key = Box::new(PrivateKey::default());
    println!("Test private key on heap: {:p}", private_key.as_ref());

    let priv_key_sk_seed_ptr = private_key.sk_seed.expose_secret().s.as_ptr();
    println!("Pointer to PrivateKey.sk_seed (heap): {:p}", priv_key_sk_seed_ptr);
    let priv_key_sk_data_ptr = private_key.sk_data.expose_secret().d.as_ptr();
    println!("Pointer to PrivateKey.sk_data (heap): {:p}", priv_key_sk_data_ptr);
    let priv_key_pk_seed_ptr = private_key.pk_seed.s.as_ptr();
    println!("Pointer to PrivateKey.pk_seed (heap): {:p}", priv_key_pk_seed_ptr);
    let priv_key_pk_data_ptr = private_key.pk_data.d.as_ptr();
    println!("Pointer to PrivateKey.pk_data (heap): {:p}", priv_key_pk_data_ptr);

    let sk_seed = Box::new(SecretSeed::new(Seed::new()));
    println!("Pointer to sk_seed (heap): {:p}", sk_seed.expose_secret().s.as_ptr());
    let pk_seed = Seed::new();
    println!("Pointer to pk_seed (stack): {:p}", pk_seed.s.as_ptr());
    private_key.private_keygen(&sk_seed, &pk_seed);
    drop(sk_seed);

    println!("private_key: {:02x?}", private_key);
    println!("private_key.sk_seed: {:02x?}", private_key.sk_seed.expose_secret());
    println!("private_key.sk_data: {:02x?}", private_key.sk_data.expose_secret());
    println!("drop private_key - start ...");
    drop(private_key);
    println!("drop private_key - end.");

    println!("PrivateKey.sk_seed: {:0x?}", unsafe {
        core::slice::from_raw_parts(priv_key_sk_seed_ptr, 16)
    });
    println!("PrivateKey.sk_data: {:0x?}", unsafe {
      core::slice::from_raw_parts(priv_key_sk_data_ptr, 16)
    });
    println!("PrivateKey.pk_seed: {:0x?}", unsafe {
      core::slice::from_raw_parts(priv_key_pk_seed_ptr, 16)
    });
    println!("PrivateKey.pk_data: {:0x?}", unsafe {
      core::slice::from_raw_parts(priv_key_pk_data_ptr, 16)
    });
}


#[test]
fn test_secret_vec() {
  struct SecretVecKey(SecretVec<u8>);

  let secret_data = SecretVecKey(SecretVec::new(vec![0x5a, 0x5a, 0x5a, 0x5a]));
  let ptr = secret_data.0.expose_secret().as_ptr();
  println!("Test SecretVec: {:p}", &secret_data);
  println!("Vec data: {:0x?}", unsafe {
    core::slice::from_raw_parts(ptr, 4)
  });
  drop(secret_data);

  println!("Vec data after drop: {:0x?}", unsafe {
    core::slice::from_raw_parts(ptr, 4)
  });
}


#[test]
fn test_secret_box() {
  struct SecretBoxKey(SecretBox<[u32]>);

  let secret_data = SecretBoxKey(SecretBox::new(Box::new([0x5a5a5a5a])));
  let ptr = secret_data.0.expose_secret().as_ptr();
  println!("Test SecretBox: {:p}", &secret_data);
  println!("Box data: {:0x?}", unsafe {
    core::slice::from_raw_parts(ptr, 1)
  });
  drop(secret_data);

  println!("Box data after drop: {:0x?}", unsafe {
    core::slice::from_raw_parts(ptr, 1)
  });
}

#[test]
fn test_secret_string() {
  struct SecretStringKey(SecretString);

  let secret_data = SecretStringKey(SecretString::new("ZZZZ".to_string()));
  let ptr = secret_data.0.expose_secret().as_ptr();
  println!("Test SecretString: {:p}", &secret_data);
  println!("String data: {:0x?}", unsafe {
    core::slice::from_raw_parts(ptr, 4)
  });
  drop(secret_data);

  println!("String data after drop: {:0x?}", unsafe {
    core::slice::from_raw_parts(ptr, 4)
  });
}
