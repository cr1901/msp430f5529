#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tactl: TACTL,
    tacctl0: TACCTL0,
    tacctl1: TACCTL1,
    tacctl2: TACCTL2,
    tacctl3: TACCTL3,
    tacctl4: TACCTL4,
    _reserved6: [u8; 0x04],
    tar: TAR,
    taccr0: TACCR0,
    taccr1: TACCR1,
    taccr2: TACCR2,
    taccr3: TACCR3,
    taccr4: TACCR4,
    _reserved12: [u8; 0x04],
    ta0ex0: TA0EX0,
    _reserved13: [u8; 0x0c],
    taiv: TAIV,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer0_A5 Control"]
    #[inline(always)]
    pub const fn tactl(&self) -> &TACTL {
        &self.tactl
    }
    #[doc = "0x02 - Timer0_A5 Capture/Compare Control 0"]
    #[inline(always)]
    pub const fn tacctl0(&self) -> &TACCTL0 {
        &self.tacctl0
    }
    #[doc = "0x04 - Timer0_A5 Capture/Compare Control 1"]
    #[inline(always)]
    pub const fn tacctl1(&self) -> &TACCTL1 {
        &self.tacctl1
    }
    #[doc = "0x06 - Timer0_A5 Capture/Compare Control 2"]
    #[inline(always)]
    pub const fn tacctl2(&self) -> &TACCTL2 {
        &self.tacctl2
    }
    #[doc = "0x08 - Timer0_A5 Capture/Compare Control 3"]
    #[inline(always)]
    pub const fn tacctl3(&self) -> &TACCTL3 {
        &self.tacctl3
    }
    #[doc = "0x0a - Timer0_A5 Capture/Compare Control 4"]
    #[inline(always)]
    pub const fn tacctl4(&self) -> &TACCTL4 {
        &self.tacctl4
    }
    #[doc = "0x10 - Timer0_A5"]
    #[inline(always)]
    pub const fn tar(&self) -> &TAR {
        &self.tar
    }
    #[doc = "0x12 - Timer0_A5 Capture/Compare 0"]
    #[inline(always)]
    pub const fn taccr0(&self) -> &TACCR0 {
        &self.taccr0
    }
    #[doc = "0x14 - Timer0_A5 Capture/Compare 1"]
    #[inline(always)]
    pub const fn taccr1(&self) -> &TACCR1 {
        &self.taccr1
    }
    #[doc = "0x16 - Timer0_A5 Capture/Compare 2"]
    #[inline(always)]
    pub const fn taccr2(&self) -> &TACCR2 {
        &self.taccr2
    }
    #[doc = "0x18 - Timer0_A5 Capture/Compare 3"]
    #[inline(always)]
    pub const fn taccr3(&self) -> &TACCR3 {
        &self.taccr3
    }
    #[doc = "0x1a - Timer0_A5 Capture/Compare 4"]
    #[inline(always)]
    pub const fn taccr4(&self) -> &TACCR4 {
        &self.taccr4
    }
    #[doc = "0x20 - Timer0_A5 Expansion Register 0"]
    #[inline(always)]
    pub const fn ta0ex0(&self) -> &TA0EX0 {
        &self.ta0ex0
    }
    #[doc = "0x2e - Timer0_A5 Interrupt Vector Word"]
    #[inline(always)]
    pub const fn taiv(&self) -> &TAIV {
        &self.taiv
    }
}
#[doc = "TACTL (rw) register accessor: Timer0_A5 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tactl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tactl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tactl`]
module"]
pub type TACTL = crate::Reg<tactl::TACTL_SPEC>;
#[doc = "Timer0_A5 Control"]
pub mod tactl;
#[doc = "TACCTL0 (rw) register accessor: Timer0_A5 Capture/Compare Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tacctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tacctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tacctl0`]
module"]
pub type TACCTL0 = crate::Reg<tacctl0::TACCTL0_SPEC>;
#[doc = "Timer0_A5 Capture/Compare Control 0"]
pub mod tacctl0;
#[doc = "TACCTL1 (rw) register accessor: Timer0_A5 Capture/Compare Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tacctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tacctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tacctl1`]
module"]
pub type TACCTL1 = crate::Reg<tacctl1::TACCTL1_SPEC>;
#[doc = "Timer0_A5 Capture/Compare Control 1"]
pub mod tacctl1;
#[doc = "TACCTL2 (rw) register accessor: Timer0_A5 Capture/Compare Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tacctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tacctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tacctl2`]
module"]
pub type TACCTL2 = crate::Reg<tacctl2::TACCTL2_SPEC>;
#[doc = "Timer0_A5 Capture/Compare Control 2"]
pub mod tacctl2;
#[doc = "TACCTL3 (rw) register accessor: Timer0_A5 Capture/Compare Control 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tacctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tacctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tacctl3`]
module"]
pub type TACCTL3 = crate::Reg<tacctl3::TACCTL3_SPEC>;
#[doc = "Timer0_A5 Capture/Compare Control 3"]
pub mod tacctl3;
#[doc = "TACCTL4 (rw) register accessor: Timer0_A5 Capture/Compare Control 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tacctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tacctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tacctl4`]
module"]
pub type TACCTL4 = crate::Reg<tacctl4::TACCTL4_SPEC>;
#[doc = "Timer0_A5 Capture/Compare Control 4"]
pub mod tacctl4;
#[doc = "TAR (rw) register accessor: Timer0_A5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tar`]
module"]
pub type TAR = crate::Reg<tar::TAR_SPEC>;
#[doc = "Timer0_A5"]
pub mod tar;
#[doc = "TACCR0 (rw) register accessor: Timer0_A5 Capture/Compare 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`taccr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`taccr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@taccr0`]
module"]
pub type TACCR0 = crate::Reg<taccr0::TACCR0_SPEC>;
#[doc = "Timer0_A5 Capture/Compare 0"]
pub mod taccr0;
#[doc = "TACCR1 (rw) register accessor: Timer0_A5 Capture/Compare 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`taccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`taccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@taccr1`]
module"]
pub type TACCR1 = crate::Reg<taccr1::TACCR1_SPEC>;
#[doc = "Timer0_A5 Capture/Compare 1"]
pub mod taccr1;
#[doc = "TACCR2 (rw) register accessor: Timer0_A5 Capture/Compare 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`taccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`taccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@taccr2`]
module"]
pub type TACCR2 = crate::Reg<taccr2::TACCR2_SPEC>;
#[doc = "Timer0_A5 Capture/Compare 2"]
pub mod taccr2;
#[doc = "TACCR3 (rw) register accessor: Timer0_A5 Capture/Compare 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`taccr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`taccr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@taccr3`]
module"]
pub type TACCR3 = crate::Reg<taccr3::TACCR3_SPEC>;
#[doc = "Timer0_A5 Capture/Compare 3"]
pub mod taccr3;
#[doc = "TACCR4 (rw) register accessor: Timer0_A5 Capture/Compare 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`taccr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`taccr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@taccr4`]
module"]
pub type TACCR4 = crate::Reg<taccr4::TACCR4_SPEC>;
#[doc = "Timer0_A5 Capture/Compare 4"]
pub mod taccr4;
#[doc = "TA0EX0 (rw) register accessor: Timer0_A5 Expansion Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta0ex0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta0ex0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0ex0`]
module"]
pub type TA0EX0 = crate::Reg<ta0ex0::TA0EX0_SPEC>;
#[doc = "Timer0_A5 Expansion Register 0"]
pub mod ta0ex0;
#[doc = "TAIV (rw) register accessor: Timer0_A5 Interrupt Vector Word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`taiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`taiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@taiv`]
module"]
pub type TAIV = crate::Reg<taiv::TAIV_SPEC>;
#[doc = "Timer0_A5 Interrupt Vector Word"]
pub mod taiv;
