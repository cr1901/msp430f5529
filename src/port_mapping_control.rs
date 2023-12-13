#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pmapkeyid: PMAPKEYID,
    pmapctl: PMAPCTL,
}
impl RegisterBlock {
    #[doc = "0x00 - Port Mapping Key register"]
    #[inline(always)]
    pub const fn pmapkeyid(&self) -> &PMAPKEYID {
        &self.pmapkeyid
    }
    #[doc = "0x02 - Port Mapping control register"]
    #[inline(always)]
    pub const fn pmapctl(&self) -> &PMAPCTL {
        &self.pmapctl
    }
}
#[doc = "PMAPKEYID (rw) register accessor: Port Mapping Key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmapkeyid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmapkeyid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmapkeyid`]
module"]
pub type PMAPKEYID = crate::Reg<pmapkeyid::PMAPKEYID_SPEC>;
#[doc = "Port Mapping Key register"]
pub mod pmapkeyid;
#[doc = "PMAPCTL (rw) register accessor: Port Mapping control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmapctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmapctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmapctl`]
module"]
pub type PMAPCTL = crate::Reg<pmapctl::PMAPCTL_SPEC>;
#[doc = "Port Mapping control register"]
pub mod pmapctl;
