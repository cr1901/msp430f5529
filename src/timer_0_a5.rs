#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer0_A5 Control"]
    pub ta0ctl: crate::Reg<ta0ctl::TA0CTL_SPEC>,
    #[doc = "0x02 - Timer0_A5 Capture/Compare Control 0"]
    pub ta0cctl0: crate::Reg<ta0cctl0::TA0CCTL0_SPEC>,
    #[doc = "0x04 - Timer0_A5 Capture/Compare Control 1"]
    pub ta0cctl1: crate::Reg<ta0cctl1::TA0CCTL1_SPEC>,
    #[doc = "0x06 - Timer0_A5 Capture/Compare Control 2"]
    pub ta0cctl2: crate::Reg<ta0cctl2::TA0CCTL2_SPEC>,
    #[doc = "0x08 - Timer0_A5 Capture/Compare Control 3"]
    pub ta0cctl3: crate::Reg<ta0cctl3::TA0CCTL3_SPEC>,
    #[doc = "0x0a - Timer0_A5 Capture/Compare Control 4"]
    pub ta0cctl4: crate::Reg<ta0cctl4::TA0CCTL4_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x10 - Timer0_A5"]
    pub ta0r: crate::Reg<ta0r::TA0R_SPEC>,
    #[doc = "0x12 - Timer0_A5 Capture/Compare 0"]
    pub ta0ccr0: crate::Reg<ta0ccr0::TA0CCR0_SPEC>,
    #[doc = "0x14 - Timer0_A5 Capture/Compare 1"]
    pub ta0ccr1: crate::Reg<ta0ccr1::TA0CCR1_SPEC>,
    #[doc = "0x16 - Timer0_A5 Capture/Compare 2"]
    pub ta0ccr2: crate::Reg<ta0ccr2::TA0CCR2_SPEC>,
    #[doc = "0x18 - Timer0_A5 Capture/Compare 3"]
    pub ta0ccr3: crate::Reg<ta0ccr3::TA0CCR3_SPEC>,
    #[doc = "0x1a - Timer0_A5 Capture/Compare 4"]
    pub ta0ccr4: crate::Reg<ta0ccr4::TA0CCR4_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x20 - Timer0_A5 Expansion Register 0"]
    pub ta0ex0: crate::Reg<ta0ex0::TA0EX0_SPEC>,
    _reserved13: [u8; 0x0c],
    #[doc = "0x2e - Timer0_A5 Interrupt Vector Word"]
    pub ta0iv: crate::Reg<ta0iv::TA0IV_SPEC>,
}
#[doc = "TA0CTL register accessor: an alias for `Reg<TA0CTL_SPEC>`"]
pub type TA0CTL = crate::Reg<ta0ctl::TA0CTL_SPEC>;
#[doc = "Timer0_A5 Control"]
pub mod ta0ctl;
#[doc = "TA0CCTL0 register accessor: an alias for `Reg<TA0CCTL0_SPEC>`"]
pub type TA0CCTL0 = crate::Reg<ta0cctl0::TA0CCTL0_SPEC>;
#[doc = "Timer0_A5 Capture/Compare Control 0"]
pub mod ta0cctl0;
#[doc = "TA0CCTL1 register accessor: an alias for `Reg<TA0CCTL1_SPEC>`"]
pub type TA0CCTL1 = crate::Reg<ta0cctl1::TA0CCTL1_SPEC>;
#[doc = "Timer0_A5 Capture/Compare Control 1"]
pub mod ta0cctl1;
#[doc = "TA0CCTL2 register accessor: an alias for `Reg<TA0CCTL2_SPEC>`"]
pub type TA0CCTL2 = crate::Reg<ta0cctl2::TA0CCTL2_SPEC>;
#[doc = "Timer0_A5 Capture/Compare Control 2"]
pub mod ta0cctl2;
#[doc = "TA0CCTL3 register accessor: an alias for `Reg<TA0CCTL3_SPEC>`"]
pub type TA0CCTL3 = crate::Reg<ta0cctl3::TA0CCTL3_SPEC>;
#[doc = "Timer0_A5 Capture/Compare Control 3"]
pub mod ta0cctl3;
#[doc = "TA0CCTL4 register accessor: an alias for `Reg<TA0CCTL4_SPEC>`"]
pub type TA0CCTL4 = crate::Reg<ta0cctl4::TA0CCTL4_SPEC>;
#[doc = "Timer0_A5 Capture/Compare Control 4"]
pub mod ta0cctl4;
#[doc = "TA0R register accessor: an alias for `Reg<TA0R_SPEC>`"]
pub type TA0R = crate::Reg<ta0r::TA0R_SPEC>;
#[doc = "Timer0_A5"]
pub mod ta0r;
#[doc = "TA0CCR0 register accessor: an alias for `Reg<TA0CCR0_SPEC>`"]
pub type TA0CCR0 = crate::Reg<ta0ccr0::TA0CCR0_SPEC>;
#[doc = "Timer0_A5 Capture/Compare 0"]
pub mod ta0ccr0;
#[doc = "TA0CCR1 register accessor: an alias for `Reg<TA0CCR1_SPEC>`"]
pub type TA0CCR1 = crate::Reg<ta0ccr1::TA0CCR1_SPEC>;
#[doc = "Timer0_A5 Capture/Compare 1"]
pub mod ta0ccr1;
#[doc = "TA0CCR2 register accessor: an alias for `Reg<TA0CCR2_SPEC>`"]
pub type TA0CCR2 = crate::Reg<ta0ccr2::TA0CCR2_SPEC>;
#[doc = "Timer0_A5 Capture/Compare 2"]
pub mod ta0ccr2;
#[doc = "TA0CCR3 register accessor: an alias for `Reg<TA0CCR3_SPEC>`"]
pub type TA0CCR3 = crate::Reg<ta0ccr3::TA0CCR3_SPEC>;
#[doc = "Timer0_A5 Capture/Compare 3"]
pub mod ta0ccr3;
#[doc = "TA0CCR4 register accessor: an alias for `Reg<TA0CCR4_SPEC>`"]
pub type TA0CCR4 = crate::Reg<ta0ccr4::TA0CCR4_SPEC>;
#[doc = "Timer0_A5 Capture/Compare 4"]
pub mod ta0ccr4;
#[doc = "TA0EX0 register accessor: an alias for `Reg<TA0EX0_SPEC>`"]
pub type TA0EX0 = crate::Reg<ta0ex0::TA0EX0_SPEC>;
#[doc = "Timer0_A5 Expansion Register 0"]
pub mod ta0ex0;
#[doc = "TA0IV register accessor: an alias for `Reg<TA0IV_SPEC>`"]
pub type TA0IV = crate::Reg<ta0iv::TA0IV_SPEC>;
#[doc = "Timer0_A5 Interrupt Vector Word"]
pub mod ta0iv;
