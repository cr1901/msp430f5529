#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ta1ctl: TA1CTL,
    ta1cctl0: TA1CCTL0,
    ta1cctl1: TA1CCTL1,
    ta1cctl2: TA1CCTL2,
    _reserved4: [u8; 0x08],
    ta1r: TA1R,
    ta1ccr0: TA1CCR0,
    ta1ccr1: TA1CCR1,
    ta1ccr2: TA1CCR2,
    _reserved8: [u8; 0x08],
    ta1ex0: TA1EX0,
    _reserved9: [u8; 0x0c],
    ta1iv: TA1IV,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer1_A3 Control"]
    #[inline(always)]
    pub const fn ta1ctl(&self) -> &TA1CTL {
        &self.ta1ctl
    }
    #[doc = "0x02 - Timer1_A3 Capture/Compare Control 0"]
    #[inline(always)]
    pub const fn ta1cctl0(&self) -> &TA1CCTL0 {
        &self.ta1cctl0
    }
    #[doc = "0x04 - Timer1_A3 Capture/Compare Control 1"]
    #[inline(always)]
    pub const fn ta1cctl1(&self) -> &TA1CCTL1 {
        &self.ta1cctl1
    }
    #[doc = "0x06 - Timer1_A3 Capture/Compare Control 2"]
    #[inline(always)]
    pub const fn ta1cctl2(&self) -> &TA1CCTL2 {
        &self.ta1cctl2
    }
    #[doc = "0x10 - Timer1_A3"]
    #[inline(always)]
    pub const fn ta1r(&self) -> &TA1R {
        &self.ta1r
    }
    #[doc = "0x12 - Timer1_A3 Capture/Compare 0"]
    #[inline(always)]
    pub const fn ta1ccr0(&self) -> &TA1CCR0 {
        &self.ta1ccr0
    }
    #[doc = "0x14 - Timer1_A3 Capture/Compare 1"]
    #[inline(always)]
    pub const fn ta1ccr1(&self) -> &TA1CCR1 {
        &self.ta1ccr1
    }
    #[doc = "0x16 - Timer1_A3 Capture/Compare 2"]
    #[inline(always)]
    pub const fn ta1ccr2(&self) -> &TA1CCR2 {
        &self.ta1ccr2
    }
    #[doc = "0x20 - Timer1_A3 Expansion Register 0"]
    #[inline(always)]
    pub const fn ta1ex0(&self) -> &TA1EX0 {
        &self.ta1ex0
    }
    #[doc = "0x2e - Timer1_A3 Interrupt Vector Word"]
    #[inline(always)]
    pub const fn ta1iv(&self) -> &TA1IV {
        &self.ta1iv
    }
}
#[doc = "TA1CTL (rw) register accessor: Timer1_A3 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta1ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta1ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1ctl`]
module"]
pub type TA1CTL = crate::Reg<ta1ctl::TA1CTL_SPEC>;
#[doc = "Timer1_A3 Control"]
pub mod ta1ctl;
#[doc = "TA1CCTL0 (rw) register accessor: Timer1_A3 Capture/Compare Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta1cctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta1cctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1cctl0`]
module"]
pub type TA1CCTL0 = crate::Reg<ta1cctl0::TA1CCTL0_SPEC>;
#[doc = "Timer1_A3 Capture/Compare Control 0"]
pub mod ta1cctl0;
#[doc = "TA1CCTL1 (rw) register accessor: Timer1_A3 Capture/Compare Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta1cctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta1cctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1cctl1`]
module"]
pub type TA1CCTL1 = crate::Reg<ta1cctl1::TA1CCTL1_SPEC>;
#[doc = "Timer1_A3 Capture/Compare Control 1"]
pub mod ta1cctl1;
#[doc = "TA1CCTL2 (rw) register accessor: Timer1_A3 Capture/Compare Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta1cctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta1cctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1cctl2`]
module"]
pub type TA1CCTL2 = crate::Reg<ta1cctl2::TA1CCTL2_SPEC>;
#[doc = "Timer1_A3 Capture/Compare Control 2"]
pub mod ta1cctl2;
#[doc = "TA1R (rw) register accessor: Timer1_A3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1r`]
module"]
pub type TA1R = crate::Reg<ta1r::TA1R_SPEC>;
#[doc = "Timer1_A3"]
pub mod ta1r;
#[doc = "TA1CCR0 (rw) register accessor: Timer1_A3 Capture/Compare 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta1ccr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta1ccr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1ccr0`]
module"]
pub type TA1CCR0 = crate::Reg<ta1ccr0::TA1CCR0_SPEC>;
#[doc = "Timer1_A3 Capture/Compare 0"]
pub mod ta1ccr0;
#[doc = "TA1CCR1 (rw) register accessor: Timer1_A3 Capture/Compare 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta1ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta1ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1ccr1`]
module"]
pub type TA1CCR1 = crate::Reg<ta1ccr1::TA1CCR1_SPEC>;
#[doc = "Timer1_A3 Capture/Compare 1"]
pub mod ta1ccr1;
#[doc = "TA1CCR2 (rw) register accessor: Timer1_A3 Capture/Compare 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta1ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta1ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1ccr2`]
module"]
pub type TA1CCR2 = crate::Reg<ta1ccr2::TA1CCR2_SPEC>;
#[doc = "Timer1_A3 Capture/Compare 2"]
pub mod ta1ccr2;
#[doc = "TA1EX0 (rw) register accessor: Timer1_A3 Expansion Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta1ex0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta1ex0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1ex0`]
module"]
pub type TA1EX0 = crate::Reg<ta1ex0::TA1EX0_SPEC>;
#[doc = "Timer1_A3 Expansion Register 0"]
pub mod ta1ex0;
#[doc = "TA1IV (rw) register accessor: Timer1_A3 Interrupt Vector Word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta1iv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta1iv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta1iv`]
module"]
pub type TA1IV = crate::Reg<ta1iv::TA1IV_SPEC>;
#[doc = "Timer1_A3 Interrupt Vector Word"]
pub mod ta1iv;
