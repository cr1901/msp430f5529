#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    rtcctl01: RTCCTL01,
    rtcctl23: RTCCTL23,
    _reserved2: [u8; 0x04],
    rtcps0ctl: RTCPS0CTL,
    rtcps1ctl: RTCPS1CTL,
    rtcps: RTCPS,
    rtciv: RTCIV,
    rtcsec: RTCSEC,
    rtcmin: RTCMIN,
    rtchour: RTCHOUR,
    rtcdow: RTCDOW,
    rtcday: RTCDAY,
    rtcmon: RTCMON,
    rtcyear: RTCYEAR,
    rtcamin: RTCAMIN,
    rtcahour: RTCAHOUR,
    rtcadow: RTCADOW,
    rtcaday: RTCADAY,
}
impl RegisterBlock {
    #[doc = "0x00 - Real Timer Control 0/1"]
    #[inline(always)]
    pub const fn rtcctl01(&self) -> &RTCCTL01 {
        &self.rtcctl01
    }
    #[doc = "0x02 - Real Timer Control 2/3"]
    #[inline(always)]
    pub const fn rtcctl23(&self) -> &RTCCTL23 {
        &self.rtcctl23
    }
    #[doc = "0x08 - Real Timer Prescale Timer 0 Control"]
    #[inline(always)]
    pub const fn rtcps0ctl(&self) -> &RTCPS0CTL {
        &self.rtcps0ctl
    }
    #[doc = "0x0a - Real Timer Prescale Timer 1 Control"]
    #[inline(always)]
    pub const fn rtcps1ctl(&self) -> &RTCPS1CTL {
        &self.rtcps1ctl
    }
    #[doc = "0x0c - Real Timer Prescale Timer Control"]
    #[inline(always)]
    pub const fn rtcps(&self) -> &RTCPS {
        &self.rtcps
    }
    #[doc = "0x0e - Real Time Clock Interrupt Vector"]
    #[inline(always)]
    pub const fn rtciv(&self) -> &RTCIV {
        &self.rtciv
    }
    #[doc = "0x10 - Real Time Clock Seconds"]
    #[inline(always)]
    pub const fn rtcsec(&self) -> &RTCSEC {
        &self.rtcsec
    }
    #[doc = "0x11 - Real Time Clock Minutes"]
    #[inline(always)]
    pub const fn rtcmin(&self) -> &RTCMIN {
        &self.rtcmin
    }
    #[doc = "0x12 - Real Time Clock Hour"]
    #[inline(always)]
    pub const fn rtchour(&self) -> &RTCHOUR {
        &self.rtchour
    }
    #[doc = "0x13 - Real Time Clock Day of week"]
    #[inline(always)]
    pub const fn rtcdow(&self) -> &RTCDOW {
        &self.rtcdow
    }
    #[doc = "0x14 - Real Time Clock Day"]
    #[inline(always)]
    pub const fn rtcday(&self) -> &RTCDAY {
        &self.rtcday
    }
    #[doc = "0x15 - Real Time Clock Month"]
    #[inline(always)]
    pub const fn rtcmon(&self) -> &RTCMON {
        &self.rtcmon
    }
    #[doc = "0x16 - Real Time Clock Year"]
    #[inline(always)]
    pub const fn rtcyear(&self) -> &RTCYEAR {
        &self.rtcyear
    }
    #[doc = "0x18 - Real Time Clock Alarm Min"]
    #[inline(always)]
    pub const fn rtcamin(&self) -> &RTCAMIN {
        &self.rtcamin
    }
    #[doc = "0x19 - Real Time Clock Alarm Hour"]
    #[inline(always)]
    pub const fn rtcahour(&self) -> &RTCAHOUR {
        &self.rtcahour
    }
    #[doc = "0x1a - Real Time Clock Alarm Day of week"]
    #[inline(always)]
    pub const fn rtcadow(&self) -> &RTCADOW {
        &self.rtcadow
    }
    #[doc = "0x1b - Real Time Clock Alarm Day"]
    #[inline(always)]
    pub const fn rtcaday(&self) -> &RTCADAY {
        &self.rtcaday
    }
}
#[doc = "RTCSEC (rw) register accessor: Real Time Clock Seconds\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcsec`]
module"]
pub type RTCSEC = crate::Reg<rtcsec::RTCSEC_SPEC>;
#[doc = "Real Time Clock Seconds"]
pub mod rtcsec;
#[doc = "RTCMIN (rw) register accessor: Real Time Clock Minutes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcmin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcmin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcmin`]
module"]
pub type RTCMIN = crate::Reg<rtcmin::RTCMIN_SPEC>;
#[doc = "Real Time Clock Minutes"]
pub mod rtcmin;
#[doc = "RTCHOUR (rw) register accessor: Real Time Clock Hour\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtchour::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtchour::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtchour`]
module"]
pub type RTCHOUR = crate::Reg<rtchour::RTCHOUR_SPEC>;
#[doc = "Real Time Clock Hour"]
pub mod rtchour;
#[doc = "RTCDOW (rw) register accessor: Real Time Clock Day of week\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcdow::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcdow::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcdow`]
module"]
pub type RTCDOW = crate::Reg<rtcdow::RTCDOW_SPEC>;
#[doc = "Real Time Clock Day of week"]
pub mod rtcdow;
#[doc = "RTCDAY (rw) register accessor: Real Time Clock Day\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcday::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcday::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcday`]
module"]
pub type RTCDAY = crate::Reg<rtcday::RTCDAY_SPEC>;
#[doc = "Real Time Clock Day"]
pub mod rtcday;
#[doc = "RTCMON (rw) register accessor: Real Time Clock Month\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcmon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcmon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcmon`]
module"]
pub type RTCMON = crate::Reg<rtcmon::RTCMON_SPEC>;
#[doc = "Real Time Clock Month"]
pub mod rtcmon;
#[doc = "RTCAMIN (rw) register accessor: Real Time Clock Alarm Min\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcamin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcamin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcamin`]
module"]
pub type RTCAMIN = crate::Reg<rtcamin::RTCAMIN_SPEC>;
#[doc = "Real Time Clock Alarm Min"]
pub mod rtcamin;
#[doc = "RTCAHOUR (rw) register accessor: Real Time Clock Alarm Hour\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcahour::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcahour::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcahour`]
module"]
pub type RTCAHOUR = crate::Reg<rtcahour::RTCAHOUR_SPEC>;
#[doc = "Real Time Clock Alarm Hour"]
pub mod rtcahour;
#[doc = "RTCADOW (rw) register accessor: Real Time Clock Alarm Day of week\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcadow::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcadow::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcadow`]
module"]
pub type RTCADOW = crate::Reg<rtcadow::RTCADOW_SPEC>;
#[doc = "Real Time Clock Alarm Day of week"]
pub mod rtcadow;
#[doc = "RTCADAY (rw) register accessor: Real Time Clock Alarm Day\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcaday::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcaday::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcaday`]
module"]
pub type RTCADAY = crate::Reg<rtcaday::RTCADAY_SPEC>;
#[doc = "Real Time Clock Alarm Day"]
pub mod rtcaday;
#[doc = "RTCCTL01 (rw) register accessor: Real Timer Control 0/1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcctl01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcctl01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcctl01`]
module"]
pub type RTCCTL01 = crate::Reg<rtcctl01::RTCCTL01_SPEC>;
#[doc = "Real Timer Control 0/1"]
pub mod rtcctl01;
#[doc = "RTCCTL23 (rw) register accessor: Real Timer Control 2/3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcctl23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcctl23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcctl23`]
module"]
pub type RTCCTL23 = crate::Reg<rtcctl23::RTCCTL23_SPEC>;
#[doc = "Real Timer Control 2/3"]
pub mod rtcctl23;
#[doc = "RTCPS0CTL (rw) register accessor: Real Timer Prescale Timer 0 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcps0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcps0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcps0ctl`]
module"]
pub type RTCPS0CTL = crate::Reg<rtcps0ctl::RTCPS0CTL_SPEC>;
#[doc = "Real Timer Prescale Timer 0 Control"]
pub mod rtcps0ctl;
#[doc = "RTCPS1CTL (rw) register accessor: Real Timer Prescale Timer 1 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcps1ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcps1ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcps1ctl`]
module"]
pub type RTCPS1CTL = crate::Reg<rtcps1ctl::RTCPS1CTL_SPEC>;
#[doc = "Real Timer Prescale Timer 1 Control"]
pub mod rtcps1ctl;
#[doc = "RTCPS (rw) register accessor: Real Timer Prescale Timer Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcps::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcps::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcps`]
module"]
pub type RTCPS = crate::Reg<rtcps::RTCPS_SPEC>;
#[doc = "Real Timer Prescale Timer Control"]
pub mod rtcps;
#[doc = "RTCIV (rw) register accessor: Real Time Clock Interrupt Vector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtciv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtciv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtciv`]
module"]
pub type RTCIV = crate::Reg<rtciv::RTCIV_SPEC>;
#[doc = "Real Time Clock Interrupt Vector"]
pub mod rtciv;
#[doc = "RTCYEAR (rw) register accessor: Real Time Clock Year\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcyear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcyear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcyear`]
module"]
pub type RTCYEAR = crate::Reg<rtcyear::RTCYEAR_SPEC>;
#[doc = "Real Time Clock Year"]
pub mod rtcyear;
