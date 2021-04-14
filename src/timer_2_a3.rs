#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer2_A3 Control"]
    pub ta2ctl: crate::Reg<ta2ctl::TA2CTL_SPEC>,
    #[doc = "0x02 - Timer2_A3 Capture/Compare Control 0"]
    pub ta2cctl0: crate::Reg<ta2cctl0::TA2CCTL0_SPEC>,
    #[doc = "0x04 - Timer2_A3 Capture/Compare Control 1"]
    pub ta2cctl1: crate::Reg<ta2cctl1::TA2CCTL1_SPEC>,
    #[doc = "0x06 - Timer2_A3 Capture/Compare Control 2"]
    pub ta2cctl2: crate::Reg<ta2cctl2::TA2CCTL2_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x10 - Timer2_A3"]
    pub ta2r: crate::Reg<ta2r::TA2R_SPEC>,
    #[doc = "0x12 - Timer2_A3 Capture/Compare 0"]
    pub ta2ccr0: crate::Reg<ta2ccr0::TA2CCR0_SPEC>,
    #[doc = "0x14 - Timer2_A3 Capture/Compare 1"]
    pub ta2ccr1: crate::Reg<ta2ccr1::TA2CCR1_SPEC>,
    #[doc = "0x16 - Timer2_A3 Capture/Compare 2"]
    pub ta2ccr2: crate::Reg<ta2ccr2::TA2CCR2_SPEC>,
    _reserved8: [u8; 0x08],
    #[doc = "0x20 - Timer2_A3 Expansion Register 0"]
    pub ta2ex0: crate::Reg<ta2ex0::TA2EX0_SPEC>,
    _reserved9: [u8; 0x0c],
    #[doc = "0x2e - Timer2_A3 Interrupt Vector Word"]
    pub ta2iv: crate::Reg<ta2iv::TA2IV_SPEC>,
}
#[doc = "TA2CTL register accessor: an alias for `Reg<TA2CTL_SPEC>`"]
pub type TA2CTL = crate::Reg<ta2ctl::TA2CTL_SPEC>;
#[doc = "Timer2_A3 Control"]
pub mod ta2ctl;
#[doc = "TA2CCTL0 register accessor: an alias for `Reg<TA2CCTL0_SPEC>`"]
pub type TA2CCTL0 = crate::Reg<ta2cctl0::TA2CCTL0_SPEC>;
#[doc = "Timer2_A3 Capture/Compare Control 0"]
pub mod ta2cctl0;
#[doc = "TA2CCTL1 register accessor: an alias for `Reg<TA2CCTL1_SPEC>`"]
pub type TA2CCTL1 = crate::Reg<ta2cctl1::TA2CCTL1_SPEC>;
#[doc = "Timer2_A3 Capture/Compare Control 1"]
pub mod ta2cctl1;
#[doc = "TA2CCTL2 register accessor: an alias for `Reg<TA2CCTL2_SPEC>`"]
pub type TA2CCTL2 = crate::Reg<ta2cctl2::TA2CCTL2_SPEC>;
#[doc = "Timer2_A3 Capture/Compare Control 2"]
pub mod ta2cctl2;
#[doc = "TA2R register accessor: an alias for `Reg<TA2R_SPEC>`"]
pub type TA2R = crate::Reg<ta2r::TA2R_SPEC>;
#[doc = "Timer2_A3"]
pub mod ta2r;
#[doc = "TA2CCR0 register accessor: an alias for `Reg<TA2CCR0_SPEC>`"]
pub type TA2CCR0 = crate::Reg<ta2ccr0::TA2CCR0_SPEC>;
#[doc = "Timer2_A3 Capture/Compare 0"]
pub mod ta2ccr0;
#[doc = "TA2CCR1 register accessor: an alias for `Reg<TA2CCR1_SPEC>`"]
pub type TA2CCR1 = crate::Reg<ta2ccr1::TA2CCR1_SPEC>;
#[doc = "Timer2_A3 Capture/Compare 1"]
pub mod ta2ccr1;
#[doc = "TA2CCR2 register accessor: an alias for `Reg<TA2CCR2_SPEC>`"]
pub type TA2CCR2 = crate::Reg<ta2ccr2::TA2CCR2_SPEC>;
#[doc = "Timer2_A3 Capture/Compare 2"]
pub mod ta2ccr2;
#[doc = "TA2EX0 register accessor: an alias for `Reg<TA2EX0_SPEC>`"]
pub type TA2EX0 = crate::Reg<ta2ex0::TA2EX0_SPEC>;
#[doc = "Timer2_A3 Expansion Register 0"]
pub mod ta2ex0;
#[doc = "TA2IV register accessor: an alias for `Reg<TA2IV_SPEC>`"]
pub type TA2IV = crate::Reg<ta2iv::TA2IV_SPEC>;
#[doc = "Timer2_A3 Interrupt Vector Word"]
pub mod ta2iv;
