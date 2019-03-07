use crate::Msbt;
use super::Section;

use std::ptr::NonNull;

#[derive(Debug)]
pub struct Tsy1 {
  pub(crate) msbt: NonNull<Msbt>,
  pub(crate) section: Section,
  pub(crate) _unknown: Vec<u8>, // tons of unknown data
}

impl Tsy1 {
  pub fn msbt(&self) -> &Msbt {
    unsafe { self.msbt.as_ref() }
  }

  pub fn section(&self) -> &Section {
    &self.section
  }

  pub fn unknown_bytes(&self) -> &[u8] {
    &self._unknown
  }
}