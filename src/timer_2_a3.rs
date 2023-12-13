#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ta2ctl: TA2CTL,
    ta2cctl0: TA2CCTL0,
    ta2cctl1: TA2CCTL1,
    ta2cctl2: TA2CCTL2,
    _reserved4: [u8; 0x08],
    ta2r: TA2R,
    ta2ccr0: TA2CCR0,
    ta2ccr1: TA2CCR1,
    ta2ccr2: TA2CCR2,
    _reserved8: [u8; 0x08],
    ta2ex0: TA2EX0,
    _reserved9: [u8; 0x0c],
    ta2iv: TA2IV,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer2_A3 Control"]
    #[inline(always)]
    pub const fn ta2ctl(&self) -> &TA2CTL {
        &self.ta2ctl
    }
    #[doc = "0x02 - Timer2_A3 Capture/Compare Control 0"]
    #[inline(always)]
    pub const fn ta2cctl0(&self) -> &TA2CCTL0 {
        &self.ta2cctl0
    }
    #[doc = "0x04 - Timer2_A3 Capture/Compare Control 1"]
    #[inline(always)]
    pub const fn ta2cctl1(&self) -> &TA2CCTL1 {
        &self.ta2cctl1
    }
    #[doc = "0x06 - Timer2_A3 Capture/Compare Control 2"]
    #[inline(always)]
    pub const fn ta2cctl2(&self) -> &TA2CCTL2 {
        &self.ta2cctl2
    }
    #[doc = "0x10 - Timer2_A3"]
    #[inline(always)]
    pub const fn ta2r(&self) -> &TA2R {
        &self.ta2r
    }
    #[doc = "0x12 - Timer2_A3 Capture/Compare 0"]
    #[inline(always)]
    pub const fn ta2ccr0(&self) -> &TA2CCR0 {
        &self.ta2ccr0
    }
    #[doc = "0x14 - Timer2_A3 Capture/Compare 1"]
    #[inline(always)]
    pub const fn ta2ccr1(&self) -> &TA2CCR1 {
        &self.ta2ccr1
    }
    #[doc = "0x16 - Timer2_A3 Capture/Compare 2"]
    #[inline(always)]
    pub const fn ta2ccr2(&self) -> &TA2CCR2 {
        &self.ta2ccr2
    }
    #[doc = "0x20 - Timer2_A3 Expansion Register 0"]
    #[inline(always)]
    pub const fn ta2ex0(&self) -> &TA2EX0 {
        &self.ta2ex0
    }
    #[doc = "0x2e - Timer2_A3 Interrupt Vector Word"]
    #[inline(always)]
    pub const fn ta2iv(&self) -> &TA2IV {
        &self.ta2iv
    }
}
#[doc = "TA2CTL (rw) register accessor: Timer2_A3 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta2ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta2ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta2ctl`]
module"]
pub type TA2CTL = crate::Reg<ta2ctl::TA2CTL_SPEC>;
#[doc = "Timer2_A3 Control"]
pub mod ta2ctl;
#[doc = "TA2CCTL0 (rw) register accessor: Timer2_A3 Capture/Compare Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta2cctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta2cctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta2cctl0`]
module"]
pub type TA2CCTL0 = crate::Reg<ta2cctl0::TA2CCTL0_SPEC>;
#[doc = "Timer2_A3 Capture/Compare Control 0"]
pub mod ta2cctl0;
#[doc = "TA2CCTL1 (rw) register accessor: Timer2_A3 Capture/Compare Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta2cctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta2cctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta2cctl1`]
module"]
pub type TA2CCTL1 = crate::Reg<ta2cctl1::TA2CCTL1_SPEC>;
#[doc = "Timer2_A3 Capture/Compare Control 1"]
pub mod ta2cctl1;
#[doc = "TA2CCTL2 (rw) register accessor: Timer2_A3 Capture/Compare Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta2cctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta2cctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta2cctl2`]
module"]
pub type TA2CCTL2 = crate::Reg<ta2cctl2::TA2CCTL2_SPEC>;
#[doc = "Timer2_A3 Capture/Compare Control 2"]
pub mod ta2cctl2;
#[doc = "TA2R (rw) register accessor: Timer2_A3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta2r`]
module"]
pub type TA2R = crate::Reg<ta2r::TA2R_SPEC>;
#[doc = "Timer2_A3"]
pub mod ta2r;
#[doc = "TA2CCR0 (rw) register accessor: Timer2_A3 Capture/Compare 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta2ccr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta2ccr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta2ccr0`]
module"]
pub type TA2CCR0 = crate::Reg<ta2ccr0::TA2CCR0_SPEC>;
#[doc = "Timer2_A3 Capture/Compare 0"]
pub mod ta2ccr0;
#[doc = "TA2CCR1 (rw) register accessor: Timer2_A3 Capture/Compare 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta2ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta2ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta2ccr1`]
module"]
pub type TA2CCR1 = crate::Reg<ta2ccr1::TA2CCR1_SPEC>;
#[doc = "Timer2_A3 Capture/Compare 1"]
pub mod ta2ccr1;
#[doc = "TA2CCR2 (rw) register accessor: Timer2_A3 Capture/Compare 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta2ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta2ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta2ccr2`]
module"]
pub type TA2CCR2 = crate::Reg<ta2ccr2::TA2CCR2_SPEC>;
#[doc = "Timer2_A3 Capture/Compare 2"]
pub mod ta2ccr2;
#[doc = "TA2EX0 (rw) register accessor: Timer2_A3 Expansion Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta2ex0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta2ex0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta2ex0`]
module"]
pub type TA2EX0 = crate::Reg<ta2ex0::TA2EX0_SPEC>;
#[doc = "Timer2_A3 Expansion Register 0"]
pub mod ta2ex0;
#[doc = "TA2IV (rw) register accessor: Timer2_A3 Interrupt Vector Word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta2iv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta2iv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta2iv`]
module"]
pub type TA2IV = crate::Reg<ta2iv::TA2IV_SPEC>;
#[doc = "Timer2_A3 Interrupt Vector Word"]
pub mod ta2iv;
