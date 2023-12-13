#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tb0ctl: TB0CTL,
    tb0cctl0: TB0CCTL0,
    tb0cctl1: TB0CCTL1,
    tb0cctl2: TB0CCTL2,
    tb0cctl3: TB0CCTL3,
    tb0cctl4: TB0CCTL4,
    tb0cctl5: TB0CCTL5,
    tb0cctl6: TB0CCTL6,
    tb0r: TB0R,
    tb0ccr0: TB0CCR0,
    tb0ccr1: TB0CCR1,
    tb0ccr2: TB0CCR2,
    tb0ccr3: TB0CCR3,
    tb0ccr4: TB0CCR4,
    tb0ccr5: TB0CCR5,
    tb0ccr6: TB0CCR6,
    tb0ex0: TB0EX0,
    _reserved17: [u8; 0x0c],
    tb0iv: TB0IV,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer0_B7 Control"]
    #[inline(always)]
    pub const fn tb0ctl(&self) -> &TB0CTL {
        &self.tb0ctl
    }
    #[doc = "0x02 - Timer0_B7 Capture/Compare Control 0"]
    #[inline(always)]
    pub const fn tb0cctl0(&self) -> &TB0CCTL0 {
        &self.tb0cctl0
    }
    #[doc = "0x04 - Timer0_B7 Capture/Compare Control 1"]
    #[inline(always)]
    pub const fn tb0cctl1(&self) -> &TB0CCTL1 {
        &self.tb0cctl1
    }
    #[doc = "0x06 - Timer0_B7 Capture/Compare Control 2"]
    #[inline(always)]
    pub const fn tb0cctl2(&self) -> &TB0CCTL2 {
        &self.tb0cctl2
    }
    #[doc = "0x08 - Timer0_B7 Capture/Compare Control 3"]
    #[inline(always)]
    pub const fn tb0cctl3(&self) -> &TB0CCTL3 {
        &self.tb0cctl3
    }
    #[doc = "0x0a - Timer0_B7 Capture/Compare Control 4"]
    #[inline(always)]
    pub const fn tb0cctl4(&self) -> &TB0CCTL4 {
        &self.tb0cctl4
    }
    #[doc = "0x0c - Timer0_B7 Capture/Compare Control 5"]
    #[inline(always)]
    pub const fn tb0cctl5(&self) -> &TB0CCTL5 {
        &self.tb0cctl5
    }
    #[doc = "0x0e - Timer0_B7 Capture/Compare Control 6"]
    #[inline(always)]
    pub const fn tb0cctl6(&self) -> &TB0CCTL6 {
        &self.tb0cctl6
    }
    #[doc = "0x10 - Timer0_B7"]
    #[inline(always)]
    pub const fn tb0r(&self) -> &TB0R {
        &self.tb0r
    }
    #[doc = "0x12 - Timer0_B7 Capture/Compare 0"]
    #[inline(always)]
    pub const fn tb0ccr0(&self) -> &TB0CCR0 {
        &self.tb0ccr0
    }
    #[doc = "0x14 - Timer0_B7 Capture/Compare 1"]
    #[inline(always)]
    pub const fn tb0ccr1(&self) -> &TB0CCR1 {
        &self.tb0ccr1
    }
    #[doc = "0x16 - Timer0_B7 Capture/Compare 2"]
    #[inline(always)]
    pub const fn tb0ccr2(&self) -> &TB0CCR2 {
        &self.tb0ccr2
    }
    #[doc = "0x18 - Timer0_B7 Capture/Compare 3"]
    #[inline(always)]
    pub const fn tb0ccr3(&self) -> &TB0CCR3 {
        &self.tb0ccr3
    }
    #[doc = "0x1a - Timer0_B7 Capture/Compare 4"]
    #[inline(always)]
    pub const fn tb0ccr4(&self) -> &TB0CCR4 {
        &self.tb0ccr4
    }
    #[doc = "0x1c - Timer0_B7 Capture/Compare 5"]
    #[inline(always)]
    pub const fn tb0ccr5(&self) -> &TB0CCR5 {
        &self.tb0ccr5
    }
    #[doc = "0x1e - Timer0_B7 Capture/Compare 6"]
    #[inline(always)]
    pub const fn tb0ccr6(&self) -> &TB0CCR6 {
        &self.tb0ccr6
    }
    #[doc = "0x20 - Timer0_B7 Expansion Register 0"]
    #[inline(always)]
    pub const fn tb0ex0(&self) -> &TB0EX0 {
        &self.tb0ex0
    }
    #[doc = "0x2e - Timer0_B7 Interrupt Vector Word"]
    #[inline(always)]
    pub const fn tb0iv(&self) -> &TB0IV {
        &self.tb0iv
    }
}
#[doc = "TB0CTL (rw) register accessor: Timer0_B7 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0ctl`]
module"]
pub type TB0CTL = crate::Reg<tb0ctl::TB0CTL_SPEC>;
#[doc = "Timer0_B7 Control"]
pub mod tb0ctl;
#[doc = "TB0CCTL0 (rw) register accessor: Timer0_B7 Capture/Compare Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0cctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0cctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0cctl0`]
module"]
pub type TB0CCTL0 = crate::Reg<tb0cctl0::TB0CCTL0_SPEC>;
#[doc = "Timer0_B7 Capture/Compare Control 0"]
pub mod tb0cctl0;
#[doc = "TB0CCTL1 (rw) register accessor: Timer0_B7 Capture/Compare Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0cctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0cctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0cctl1`]
module"]
pub type TB0CCTL1 = crate::Reg<tb0cctl1::TB0CCTL1_SPEC>;
#[doc = "Timer0_B7 Capture/Compare Control 1"]
pub mod tb0cctl1;
#[doc = "TB0CCTL2 (rw) register accessor: Timer0_B7 Capture/Compare Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0cctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0cctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0cctl2`]
module"]
pub type TB0CCTL2 = crate::Reg<tb0cctl2::TB0CCTL2_SPEC>;
#[doc = "Timer0_B7 Capture/Compare Control 2"]
pub mod tb0cctl2;
#[doc = "TB0CCTL3 (rw) register accessor: Timer0_B7 Capture/Compare Control 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0cctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0cctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0cctl3`]
module"]
pub type TB0CCTL3 = crate::Reg<tb0cctl3::TB0CCTL3_SPEC>;
#[doc = "Timer0_B7 Capture/Compare Control 3"]
pub mod tb0cctl3;
#[doc = "TB0CCTL4 (rw) register accessor: Timer0_B7 Capture/Compare Control 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0cctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0cctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0cctl4`]
module"]
pub type TB0CCTL4 = crate::Reg<tb0cctl4::TB0CCTL4_SPEC>;
#[doc = "Timer0_B7 Capture/Compare Control 4"]
pub mod tb0cctl4;
#[doc = "TB0CCTL5 (rw) register accessor: Timer0_B7 Capture/Compare Control 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0cctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0cctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0cctl5`]
module"]
pub type TB0CCTL5 = crate::Reg<tb0cctl5::TB0CCTL5_SPEC>;
#[doc = "Timer0_B7 Capture/Compare Control 5"]
pub mod tb0cctl5;
#[doc = "TB0CCTL6 (rw) register accessor: Timer0_B7 Capture/Compare Control 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0cctl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0cctl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0cctl6`]
module"]
pub type TB0CCTL6 = crate::Reg<tb0cctl6::TB0CCTL6_SPEC>;
#[doc = "Timer0_B7 Capture/Compare Control 6"]
pub mod tb0cctl6;
#[doc = "TB0R (rw) register accessor: Timer0_B7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0r`]
module"]
pub type TB0R = crate::Reg<tb0r::TB0R_SPEC>;
#[doc = "Timer0_B7"]
pub mod tb0r;
#[doc = "TB0CCR0 (rw) register accessor: Timer0_B7 Capture/Compare 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0ccr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0ccr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0ccr0`]
module"]
pub type TB0CCR0 = crate::Reg<tb0ccr0::TB0CCR0_SPEC>;
#[doc = "Timer0_B7 Capture/Compare 0"]
pub mod tb0ccr0;
#[doc = "TB0CCR1 (rw) register accessor: Timer0_B7 Capture/Compare 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0ccr1`]
module"]
pub type TB0CCR1 = crate::Reg<tb0ccr1::TB0CCR1_SPEC>;
#[doc = "Timer0_B7 Capture/Compare 1"]
pub mod tb0ccr1;
#[doc = "TB0CCR2 (rw) register accessor: Timer0_B7 Capture/Compare 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0ccr2`]
module"]
pub type TB0CCR2 = crate::Reg<tb0ccr2::TB0CCR2_SPEC>;
#[doc = "Timer0_B7 Capture/Compare 2"]
pub mod tb0ccr2;
#[doc = "TB0CCR3 (rw) register accessor: Timer0_B7 Capture/Compare 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0ccr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0ccr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0ccr3`]
module"]
pub type TB0CCR3 = crate::Reg<tb0ccr3::TB0CCR3_SPEC>;
#[doc = "Timer0_B7 Capture/Compare 3"]
pub mod tb0ccr3;
#[doc = "TB0CCR4 (rw) register accessor: Timer0_B7 Capture/Compare 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0ccr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0ccr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0ccr4`]
module"]
pub type TB0CCR4 = crate::Reg<tb0ccr4::TB0CCR4_SPEC>;
#[doc = "Timer0_B7 Capture/Compare 4"]
pub mod tb0ccr4;
#[doc = "TB0CCR5 (rw) register accessor: Timer0_B7 Capture/Compare 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0ccr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0ccr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0ccr5`]
module"]
pub type TB0CCR5 = crate::Reg<tb0ccr5::TB0CCR5_SPEC>;
#[doc = "Timer0_B7 Capture/Compare 5"]
pub mod tb0ccr5;
#[doc = "TB0CCR6 (rw) register accessor: Timer0_B7 Capture/Compare 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0ccr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0ccr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0ccr6`]
module"]
pub type TB0CCR6 = crate::Reg<tb0ccr6::TB0CCR6_SPEC>;
#[doc = "Timer0_B7 Capture/Compare 6"]
pub mod tb0ccr6;
#[doc = "TB0EX0 (rw) register accessor: Timer0_B7 Expansion Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0ex0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0ex0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0ex0`]
module"]
pub type TB0EX0 = crate::Reg<tb0ex0::TB0EX0_SPEC>;
#[doc = "Timer0_B7 Expansion Register 0"]
pub mod tb0ex0;
#[doc = "TB0IV (rw) register accessor: Timer0_B7 Interrupt Vector Word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0iv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0iv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0iv`]
module"]
pub type TB0IV = crate::Reg<tb0iv::TB0IV_SPEC>;
#[doc = "Timer0_B7 Interrupt Vector Word"]
pub mod tb0iv;
