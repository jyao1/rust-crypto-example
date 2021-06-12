// Copyright (c) 2021 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent
//
// plese use `cargo test -- --nocapture --test-threads=1` to see result.

use constant_time_eq::constant_time_eq;

#[test]
fn test_const_time() {
  println!("test_const_time");

  assert!(constant_time_eq(b"foo", b"foo"));
  assert!(!constant_time_eq(b"foo", b"bar"));
  assert!(!constant_time_eq(b"bar", b"baz"));
  
  assert!(!constant_time_eq(b"foo", b""));
  assert!(!constant_time_eq(b"foo", b"quux"));
}
