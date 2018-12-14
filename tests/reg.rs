#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate drone_core;

use drone_core::bitfield::Bitfield;
use drone_core::reg;
use drone_core::reg::prelude::*;
use std::mem::size_of;
use test_block::test_reg::Val;
use test_block::TestReg;

reg! {
  /// Test reg doc attribute
  #[doc = "test reg attribute"]
  pub mod TEST_BLOCK TEST_REG;

  0xDEAD_BEEF 0x20 0xBEEF_CACE RReg WReg;

  TEST_BIT { 0 1 RRRegField WWRegField }
  TEST_BITS { 1 3 RRRegField WWRegField }
}

reg::index! {
  /// Test index doc attribute
  #[doc = "test index attribute"]
  pub macro reg_idx;
  super;;

  /// Test block doc attribute
  #[doc = "test block attribute"]
  pub mod TEST_BLOCK {
    TEST_REG;
  }
}

reg_idx! {
  /// Test index doc attribute
  #[doc = "test index attribute"]
  pub struct RegIdx;
}

#[test]
fn reg_val_default() {
  unsafe {
    assert_eq!(Val::default().bits(), 0xBEEF_CACE);
  }
}

#[test]
fn size_of_reg() {
  assert_eq!(size_of::<TestReg<Urt>>(), 0);
  assert_eq!(size_of::<TestReg<Srt>>(), 0);
  assert_eq!(size_of::<TestReg<Crt>>(), 0);
}

#[test]
fn size_of_reg_val() {
  assert_eq!(size_of::<Val>(), 4);
}
