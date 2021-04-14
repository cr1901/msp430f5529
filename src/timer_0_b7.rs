#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer0_B7 Control"]
    pub tb0ctl: crate::Reg<tb0ctl::TB0CTL_SPEC>,
    #[doc = "0x02 - Timer0_B7 Capture/Compare Control 0"]
    pub tb0cctl0: crate::Reg<tb0cctl0::TB0CCTL0_SPEC>,
    #[doc = "0x04 - Timer0_B7 Capture/Compare Control 1"]
    pub tb0cctl1: crate::Reg<tb0cctl1::TB0CCTL1_SPEC>,
    #[doc = "0x06 - Timer0_B7 Capture/Compare Control 2"]
    pub tb0cctl2: crate::Reg<tb0cctl2::TB0CCTL2_SPEC>,
    #[doc = "0x08 - Timer0_B7 Capture/Compare Control 3"]
    pub tb0cctl3: crate::Reg<tb0cctl3::TB0CCTL3_SPEC>,
    #[doc = "0x0a - Timer0_B7 Capture/Compare Control 4"]
    pub tb0cctl4: crate::Reg<tb0cctl4::TB0CCTL4_SPEC>,
    #[doc = "0x0c - Timer0_B7 Capture/Compare Control 5"]
    pub tb0cctl5: crate::Reg<tb0cctl5::TB0CCTL5_SPEC>,
    #[doc = "0x0e - Timer0_B7 Capture/Compare Control 6"]
    pub tb0cctl6: crate::Reg<tb0cctl6::TB0CCTL6_SPEC>,
    #[doc = "0x10 - Timer0_B7"]
    pub tb0r: crate::Reg<tb0r::TB0R_SPEC>,
    #[doc = "0x12 - Timer0_B7 Capture/Compare 0"]
    pub tb0ccr0: crate::Reg<tb0ccr0::TB0CCR0_SPEC>,
    #[doc = "0x14 - Timer0_B7 Capture/Compare 1"]
    pub tb0ccr1: crate::Reg<tb0ccr1::TB0CCR1_SPEC>,
    #[doc = "0x16 - Timer0_B7 Capture/Compare 2"]
    pub tb0ccr2: crate::Reg<tb0ccr2::TB0CCR2_SPEC>,
    #[doc = "0x18 - Timer0_B7 Capture/Compare 3"]
    pub tb0ccr3: crate::Reg<tb0ccr3::TB0CCR3_SPEC>,
    #[doc = "0x1a - Timer0_B7 Capture/Compare 4"]
    pub tb0ccr4: crate::Reg<tb0ccr4::TB0CCR4_SPEC>,
    #[doc = "0x1c - Timer0_B7 Capture/Compare 5"]
    pub tb0ccr5: crate::Reg<tb0ccr5::TB0CCR5_SPEC>,
    #[doc = "0x1e - Timer0_B7 Capture/Compare 6"]
    pub tb0ccr6: crate::Reg<tb0ccr6::TB0CCR6_SPEC>,
    #[doc = "0x20 - Timer0_B7 Expansion Register 0"]
    pub tb0ex0: crate::Reg<tb0ex0::TB0EX0_SPEC>,
    _reserved17: [u8; 0x0c],
    #[doc = "0x2e - Timer0_B7 Interrupt Vector Word"]
    pub tb0iv: crate::Reg<tb0iv::TB0IV_SPEC>,
}
#[doc = "TB0CTL register accessor: an alias for `Reg<TB0CTL_SPEC>`"]
pub type TB0CTL = crate::Reg<tb0ctl::TB0CTL_SPEC>;
#[doc = "Timer0_B7 Control"]
pub mod tb0ctl;
#[doc = "TB0CCTL0 register accessor: an alias for `Reg<TB0CCTL0_SPEC>`"]
pub type TB0CCTL0 = crate::Reg<tb0cctl0::TB0CCTL0_SPEC>;
#[doc = "Timer0_B7 Capture/Compare Control 0"]
pub mod tb0cctl0;
#[doc = "TB0CCTL1 register accessor: an alias for `Reg<TB0CCTL1_SPEC>`"]
pub type TB0CCTL1 = crate::Reg<tb0cctl1::TB0CCTL1_SPEC>;
#[doc = "Timer0_B7 Capture/Compare Control 1"]
pub mod tb0cctl1;
#[doc = "TB0CCTL2 register accessor: an alias for `Reg<TB0CCTL2_SPEC>`"]
pub type TB0CCTL2 = crate::Reg<tb0cctl2::TB0CCTL2_SPEC>;
#[doc = "Timer0_B7 Capture/Compare Control 2"]
pub mod tb0cctl2;
#[doc = "TB0CCTL3 register accessor: an alias for `Reg<TB0CCTL3_SPEC>`"]
pub type TB0CCTL3 = crate::Reg<tb0cctl3::TB0CCTL3_SPEC>;
#[doc = "Timer0_B7 Capture/Compare Control 3"]
pub mod tb0cctl3;
#[doc = "TB0CCTL4 register accessor: an alias for `Reg<TB0CCTL4_SPEC>`"]
pub type TB0CCTL4 = crate::Reg<tb0cctl4::TB0CCTL4_SPEC>;
#[doc = "Timer0_B7 Capture/Compare Control 4"]
pub mod tb0cctl4;
#[doc = "TB0CCTL5 register accessor: an alias for `Reg<TB0CCTL5_SPEC>`"]
pub type TB0CCTL5 = crate::Reg<tb0cctl5::TB0CCTL5_SPEC>;
#[doc = "Timer0_B7 Capture/Compare Control 5"]
pub mod tb0cctl5;
#[doc = "TB0CCTL6 register accessor: an alias for `Reg<TB0CCTL6_SPEC>`"]
pub type TB0CCTL6 = crate::Reg<tb0cctl6::TB0CCTL6_SPEC>;
#[doc = "Timer0_B7 Capture/Compare Control 6"]
pub mod tb0cctl6;
#[doc = "TB0R register accessor: an alias for `Reg<TB0R_SPEC>`"]
pub type TB0R = crate::Reg<tb0r::TB0R_SPEC>;
#[doc = "Timer0_B7"]
pub mod tb0r;
#[doc = "TB0CCR0 register accessor: an alias for `Reg<TB0CCR0_SPEC>`"]
pub type TB0CCR0 = crate::Reg<tb0ccr0::TB0CCR0_SPEC>;
#[doc = "Timer0_B7 Capture/Compare 0"]
pub mod tb0ccr0;
#[doc = "TB0CCR1 register accessor: an alias for `Reg<TB0CCR1_SPEC>`"]
pub type TB0CCR1 = crate::Reg<tb0ccr1::TB0CCR1_SPEC>;
#[doc = "Timer0_B7 Capture/Compare 1"]
pub mod tb0ccr1;
#[doc = "TB0CCR2 register accessor: an alias for `Reg<TB0CCR2_SPEC>`"]
pub type TB0CCR2 = crate::Reg<tb0ccr2::TB0CCR2_SPEC>;
#[doc = "Timer0_B7 Capture/Compare 2"]
pub mod tb0ccr2;
#[doc = "TB0CCR3 register accessor: an alias for `Reg<TB0CCR3_SPEC>`"]
pub type TB0CCR3 = crate::Reg<tb0ccr3::TB0CCR3_SPEC>;
#[doc = "Timer0_B7 Capture/Compare 3"]
pub mod tb0ccr3;
#[doc = "TB0CCR4 register accessor: an alias for `Reg<TB0CCR4_SPEC>`"]
pub type TB0CCR4 = crate::Reg<tb0ccr4::TB0CCR4_SPEC>;
#[doc = "Timer0_B7 Capture/Compare 4"]
pub mod tb0ccr4;
#[doc = "TB0CCR5 register accessor: an alias for `Reg<TB0CCR5_SPEC>`"]
pub type TB0CCR5 = crate::Reg<tb0ccr5::TB0CCR5_SPEC>;
#[doc = "Timer0_B7 Capture/Compare 5"]
pub mod tb0ccr5;
#[doc = "TB0CCR6 register accessor: an alias for `Reg<TB0CCR6_SPEC>`"]
pub type TB0CCR6 = crate::Reg<tb0ccr6::TB0CCR6_SPEC>;
#[doc = "Timer0_B7 Capture/Compare 6"]
pub mod tb0ccr6;
#[doc = "TB0EX0 register accessor: an alias for `Reg<TB0EX0_SPEC>`"]
pub type TB0EX0 = crate::Reg<tb0ex0::TB0EX0_SPEC>;
#[doc = "Timer0_B7 Expansion Register 0"]
pub mod tb0ex0;
#[doc = "TB0IV register accessor: an alias for `Reg<TB0IV_SPEC>`"]
pub type TB0IV = crate::Reg<tb0iv::TB0IV_SPEC>;
#[doc = "Timer0_B7 Interrupt Vector Word"]
pub mod tb0iv;
