
extern crate alloc;

use alloc::boxed::Box;
use rasn::prelude::*;

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum TestChoice {
    Number1(()),
    Number2(bool),
    Number3(Box<TopLevel>),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct TopLevel {
    #[rasn(value("1..=8"))]
    pub test: u8,
    pub choice: TestChoice,
}

impl TopLevel {
    pub fn new(test: u8, choice: TestChoice) -> Self {
        Self { test, choice }
    }
}
