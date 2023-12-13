#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mpy32l: MPY32L,
    mpy32h: MPY32H,
    mpys32l: MPYS32L,
    mpys32h: MPYS32H,
    mac32l: MAC32L,
    mac32h: MAC32H,
    macs32l: MACS32L,
    macs32h: MACS32H,
    op2l: OP2L,
    op2h: OP2H,
    res0: RES0,
    res1: RES1,
    res2: RES2,
    res3: RES3,
}
impl RegisterBlock {
    #[doc = "0x00 - 32-bit operand 1 - multiply - low word"]
    #[inline(always)]
    pub const fn mpy32l(&self) -> &MPY32L {
        &self.mpy32l
    }
    #[doc = "0x02 - 32-bit operand 1 - multiply - high word"]
    #[inline(always)]
    pub const fn mpy32h(&self) -> &MPY32H {
        &self.mpy32h
    }
    #[doc = "0x04 - 32-bit operand 1 - signed multiply - low word"]
    #[inline(always)]
    pub const fn mpys32l(&self) -> &MPYS32L {
        &self.mpys32l
    }
    #[doc = "0x06 - 32-bit operand 1 - signed multiply - high word"]
    #[inline(always)]
    pub const fn mpys32h(&self) -> &MPYS32H {
        &self.mpys32h
    }
    #[doc = "0x08 - 32-bit operand 1 - multiply accumulate - low word"]
    #[inline(always)]
    pub const fn mac32l(&self) -> &MAC32L {
        &self.mac32l
    }
    #[doc = "0x0a - 32-bit operand 1 - multiply accumulate - high word"]
    #[inline(always)]
    pub const fn mac32h(&self) -> &MAC32H {
        &self.mac32h
    }
    #[doc = "0x0c - 32-bit operand 1 - signed multiply accumulate - low word"]
    #[inline(always)]
    pub const fn macs32l(&self) -> &MACS32L {
        &self.macs32l
    }
    #[doc = "0x0e - 32-bit operand 1 - signed multiply accumulate - high word"]
    #[inline(always)]
    pub const fn macs32h(&self) -> &MACS32H {
        &self.macs32h
    }
    #[doc = "0x10 - 32-bit operand 2 - low word"]
    #[inline(always)]
    pub const fn op2l(&self) -> &OP2L {
        &self.op2l
    }
    #[doc = "0x12 - 32-bit operand 2 - high word"]
    #[inline(always)]
    pub const fn op2h(&self) -> &OP2H {
        &self.op2h
    }
    #[doc = "0x14 - 32x32-bit result 0 - least significant word"]
    #[inline(always)]
    pub const fn res0(&self) -> &RES0 {
        &self.res0
    }
    #[doc = "0x16 - 32x32-bit result 1"]
    #[inline(always)]
    pub const fn res1(&self) -> &RES1 {
        &self.res1
    }
    #[doc = "0x18 - 32x32-bit result 2"]
    #[inline(always)]
    pub const fn res2(&self) -> &RES2 {
        &self.res2
    }
    #[doc = "0x1a - 32x32-bit result 3 - most significant word"]
    #[inline(always)]
    pub const fn res3(&self) -> &RES3 {
        &self.res3
    }
}
#[doc = "MPY32L (rw) register accessor: 32-bit operand 1 - multiply - low word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpy32l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpy32l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpy32l`]
module"]
pub type MPY32L = crate::Reg<mpy32l::MPY32L_SPEC>;
#[doc = "32-bit operand 1 - multiply - low word"]
pub mod mpy32l;
#[doc = "MPY32H (rw) register accessor: 32-bit operand 1 - multiply - high word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpy32h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpy32h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpy32h`]
module"]
pub type MPY32H = crate::Reg<mpy32h::MPY32H_SPEC>;
#[doc = "32-bit operand 1 - multiply - high word"]
pub mod mpy32h;
#[doc = "MPYS32L (rw) register accessor: 32-bit operand 1 - signed multiply - low word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpys32l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpys32l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpys32l`]
module"]
pub type MPYS32L = crate::Reg<mpys32l::MPYS32L_SPEC>;
#[doc = "32-bit operand 1 - signed multiply - low word"]
pub mod mpys32l;
#[doc = "MPYS32H (rw) register accessor: 32-bit operand 1 - signed multiply - high word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpys32h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpys32h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpys32h`]
module"]
pub type MPYS32H = crate::Reg<mpys32h::MPYS32H_SPEC>;
#[doc = "32-bit operand 1 - signed multiply - high word"]
pub mod mpys32h;
#[doc = "MAC32L (rw) register accessor: 32-bit operand 1 - multiply accumulate - low word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac32l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac32l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac32l`]
module"]
pub type MAC32L = crate::Reg<mac32l::MAC32L_SPEC>;
#[doc = "32-bit operand 1 - multiply accumulate - low word"]
pub mod mac32l;
#[doc = "MAC32H (rw) register accessor: 32-bit operand 1 - multiply accumulate - high word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac32h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac32h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac32h`]
module"]
pub type MAC32H = crate::Reg<mac32h::MAC32H_SPEC>;
#[doc = "32-bit operand 1 - multiply accumulate - high word"]
pub mod mac32h;
#[doc = "MACS32L (rw) register accessor: 32-bit operand 1 - signed multiply accumulate - low word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macs32l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macs32l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macs32l`]
module"]
pub type MACS32L = crate::Reg<macs32l::MACS32L_SPEC>;
#[doc = "32-bit operand 1 - signed multiply accumulate - low word"]
pub mod macs32l;
#[doc = "MACS32H (rw) register accessor: 32-bit operand 1 - signed multiply accumulate - high word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macs32h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macs32h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macs32h`]
module"]
pub type MACS32H = crate::Reg<macs32h::MACS32H_SPEC>;
#[doc = "32-bit operand 1 - signed multiply accumulate - high word"]
pub mod macs32h;
#[doc = "OP2L (rw) register accessor: 32-bit operand 2 - low word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op2l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op2l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op2l`]
module"]
pub type OP2L = crate::Reg<op2l::OP2L_SPEC>;
#[doc = "32-bit operand 2 - low word"]
pub mod op2l;
#[doc = "OP2H (rw) register accessor: 32-bit operand 2 - high word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op2h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op2h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op2h`]
module"]
pub type OP2H = crate::Reg<op2h::OP2H_SPEC>;
#[doc = "32-bit operand 2 - high word"]
pub mod op2h;
#[doc = "RES0 (rw) register accessor: 32x32-bit result 0 - least significant word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`res0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`res0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res0`]
module"]
pub type RES0 = crate::Reg<res0::RES0_SPEC>;
#[doc = "32x32-bit result 0 - least significant word"]
pub mod res0;
#[doc = "RES1 (rw) register accessor: 32x32-bit result 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`res1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`res1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res1`]
module"]
pub type RES1 = crate::Reg<res1::RES1_SPEC>;
#[doc = "32x32-bit result 1"]
pub mod res1;
#[doc = "RES2 (rw) register accessor: 32x32-bit result 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`res2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`res2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res2`]
module"]
pub type RES2 = crate::Reg<res2::RES2_SPEC>;
#[doc = "32x32-bit result 2"]
pub mod res2;
#[doc = "RES3 (rw) register accessor: 32x32-bit result 3 - most significant word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`res3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`res3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res3`]
module"]
pub type RES3 = crate::Reg<res3::RES3_SPEC>;
#[doc = "32x32-bit result 3 - most significant word"]
pub mod res3;
