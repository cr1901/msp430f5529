#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    crcdi: CRCDI,
    crcdirb: CRCDIRB,
    crcinires: CRCINIRES,
    crcresr: CRCRESR,
}
impl RegisterBlock {
    #[doc = "0x00 - CRC Data In Register"]
    #[inline(always)]
    pub const fn crcdi(&self) -> &CRCDI {
        &self.crcdi
    }
    #[doc = "0x02 - CRC data in reverse byte Register"]
    #[inline(always)]
    pub const fn crcdirb(&self) -> &CRCDIRB {
        &self.crcdirb
    }
    #[doc = "0x04 - CRC Initialisation Register and Result Register"]
    #[inline(always)]
    pub const fn crcinires(&self) -> &CRCINIRES {
        &self.crcinires
    }
    #[doc = "0x06 - CRC reverse result Register"]
    #[inline(always)]
    pub const fn crcresr(&self) -> &CRCRESR {
        &self.crcresr
    }
}
#[doc = "CRCDI (rw) register accessor: CRC Data In Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcdi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcdi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdi`]
module"]
pub type CRCDI = crate::Reg<crcdi::CRCDI_SPEC>;
#[doc = "CRC Data In Register"]
pub mod crcdi;
#[doc = "CRCDIRB (rw) register accessor: CRC data in reverse byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcdirb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcdirb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdirb`]
module"]
pub type CRCDIRB = crate::Reg<crcdirb::CRCDIRB_SPEC>;
#[doc = "CRC data in reverse byte Register"]
pub mod crcdirb;
#[doc = "CRCINIRES (rw) register accessor: CRC Initialisation Register and Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcinires::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcinires::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcinires`]
module"]
pub type CRCINIRES = crate::Reg<crcinires::CRCINIRES_SPEC>;
#[doc = "CRC Initialisation Register and Result Register"]
pub mod crcinires;
#[doc = "CRCRESR (rw) register accessor: CRC reverse result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcresr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcresr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcresr`]
module"]
pub type CRCRESR = crate::Reg<crcresr::CRCRESR_SPEC>;
#[doc = "CRC reverse result Register"]
pub mod crcresr;
