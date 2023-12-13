#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mpy: MPY,
    mpys: MPYS,
    mac: MAC,
    macs: MACS,
    op2: OP2,
    reslo: RESLO,
    reshi: RESHI,
    sumext: SUMEXT,
    _reserved8: [u8; 0x1c],
    mpy32ctl0: MPY32CTL0,
}
impl RegisterBlock {
    #[doc = "0x00 - Multiply Unsigned/Operand 1"]
    #[inline(always)]
    pub const fn mpy(&self) -> &MPY {
        &self.mpy
    }
    #[doc = "0x02 - Multiply Signed/Operand 1"]
    #[inline(always)]
    pub const fn mpys(&self) -> &MPYS {
        &self.mpys
    }
    #[doc = "0x04 - Multiply Unsigned and Accumulate/Operand 1"]
    #[inline(always)]
    pub const fn mac(&self) -> &MAC {
        &self.mac
    }
    #[doc = "0x06 - Multiply Signed and Accumulate/Operand 1"]
    #[inline(always)]
    pub const fn macs(&self) -> &MACS {
        &self.macs
    }
    #[doc = "0x08 - Operand 2"]
    #[inline(always)]
    pub const fn op2(&self) -> &OP2 {
        &self.op2
    }
    #[doc = "0x0a - Result Low Word"]
    #[inline(always)]
    pub const fn reslo(&self) -> &RESLO {
        &self.reslo
    }
    #[doc = "0x0c - Result High Word"]
    #[inline(always)]
    pub const fn reshi(&self) -> &RESHI {
        &self.reshi
    }
    #[doc = "0x0e - Sum Extend"]
    #[inline(always)]
    pub const fn sumext(&self) -> &SUMEXT {
        &self.sumext
    }
    #[doc = "0x2c - MPY32 Control Register 0"]
    #[inline(always)]
    pub const fn mpy32ctl0(&self) -> &MPY32CTL0 {
        &self.mpy32ctl0
    }
}
#[doc = "MPY (rw) register accessor: Multiply Unsigned/Operand 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpy`]
module"]
pub type MPY = crate::Reg<mpy::MPY_SPEC>;
#[doc = "Multiply Unsigned/Operand 1"]
pub mod mpy;
#[doc = "MPYS (rw) register accessor: Multiply Signed/Operand 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpys::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpys::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpys`]
module"]
pub type MPYS = crate::Reg<mpys::MPYS_SPEC>;
#[doc = "Multiply Signed/Operand 1"]
pub mod mpys;
#[doc = "MAC (rw) register accessor: Multiply Unsigned and Accumulate/Operand 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac`]
module"]
pub type MAC = crate::Reg<mac::MAC_SPEC>;
#[doc = "Multiply Unsigned and Accumulate/Operand 1"]
pub mod mac;
#[doc = "MACS (rw) register accessor: Multiply Signed and Accumulate/Operand 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macs`]
module"]
pub type MACS = crate::Reg<macs::MACS_SPEC>;
#[doc = "Multiply Signed and Accumulate/Operand 1"]
pub mod macs;
#[doc = "OP2 (rw) register accessor: Operand 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op2`]
module"]
pub type OP2 = crate::Reg<op2::OP2_SPEC>;
#[doc = "Operand 2"]
pub mod op2;
#[doc = "RESLO (rw) register accessor: Result Low Word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reslo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reslo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reslo`]
module"]
pub type RESLO = crate::Reg<reslo::RESLO_SPEC>;
#[doc = "Result Low Word"]
pub mod reslo;
#[doc = "RESHI (rw) register accessor: Result High Word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reshi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reshi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reshi`]
module"]
pub type RESHI = crate::Reg<reshi::RESHI_SPEC>;
#[doc = "Result High Word"]
pub mod reshi;
#[doc = "SUMEXT (rw) register accessor: Sum Extend\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sumext::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sumext::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sumext`]
module"]
pub type SUMEXT = crate::Reg<sumext::SUMEXT_SPEC>;
#[doc = "Sum Extend"]
pub mod sumext;
#[doc = "MPY32CTL0 (rw) register accessor: MPY32 Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpy32ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpy32ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpy32ctl0`]
module"]
pub type MPY32CTL0 = crate::Reg<mpy32ctl0::MPY32CTL0_SPEC>;
#[doc = "MPY32 Control Register 0"]
pub mod mpy32ctl0;
