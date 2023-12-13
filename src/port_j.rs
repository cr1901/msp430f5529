#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pjin: PJIN,
    pjout: PJOUT,
    pjdir: PJDIR,
    pjren: PJREN,
    pjds: PJDS,
}
impl RegisterBlock {
    #[doc = "0x00 - Port J Input"]
    #[inline(always)]
    pub const fn pjin(&self) -> &PJIN {
        &self.pjin
    }
    #[doc = "0x02 - Port J Output"]
    #[inline(always)]
    pub const fn pjout(&self) -> &PJOUT {
        &self.pjout
    }
    #[doc = "0x04 - Port J Direction"]
    #[inline(always)]
    pub const fn pjdir(&self) -> &PJDIR {
        &self.pjdir
    }
    #[doc = "0x06 - Port J Resistor Enable"]
    #[inline(always)]
    pub const fn pjren(&self) -> &PJREN {
        &self.pjren
    }
    #[doc = "0x08 - Port J Drive Strenght"]
    #[inline(always)]
    pub const fn pjds(&self) -> &PJDS {
        &self.pjds
    }
}
#[doc = "PJIN (rw) register accessor: Port J Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pjin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pjin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjin`]
module"]
pub type PJIN = crate::Reg<pjin::PJIN_SPEC>;
#[doc = "Port J Input"]
pub mod pjin;
#[doc = "PJOUT (rw) register accessor: Port J Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pjout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pjout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjout`]
module"]
pub type PJOUT = crate::Reg<pjout::PJOUT_SPEC>;
#[doc = "Port J Output"]
pub mod pjout;
#[doc = "PJDIR (rw) register accessor: Port J Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pjdir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pjdir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjdir`]
module"]
pub type PJDIR = crate::Reg<pjdir::PJDIR_SPEC>;
#[doc = "Port J Direction"]
pub mod pjdir;
#[doc = "PJREN (rw) register accessor: Port J Resistor Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pjren::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pjren::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjren`]
module"]
pub type PJREN = crate::Reg<pjren::PJREN_SPEC>;
#[doc = "Port J Resistor Enable"]
pub mod pjren;
#[doc = "PJDS (rw) register accessor: Port J Drive Strenght\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pjds::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pjds::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjds`]
module"]
pub type PJDS = crate::Reg<pjds::PJDS_SPEC>;
#[doc = "Port J Drive Strenght"]
pub mod pjds;
