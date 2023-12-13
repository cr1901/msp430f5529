#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    p7in: P7IN,
    p8in: P8IN,
    p7out: P7OUT,
    p8out: P8OUT,
    p7dir: P7DIR,
    p8dir: P8DIR,
    p7ren: P7REN,
    p8ren: P8REN,
    p7ds: P7DS,
    p8ds: P8DS,
    p7sel: P7SEL,
    p8sel: P8SEL,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 7 Input"]
    #[inline(always)]
    pub const fn p7in(&self) -> &P7IN {
        &self.p7in
    }
    #[doc = "0x01 - Port 8 Input"]
    #[inline(always)]
    pub const fn p8in(&self) -> &P8IN {
        &self.p8in
    }
    #[doc = "0x02 - Port 7 Output"]
    #[inline(always)]
    pub const fn p7out(&self) -> &P7OUT {
        &self.p7out
    }
    #[doc = "0x03 - Port 8 Output"]
    #[inline(always)]
    pub const fn p8out(&self) -> &P8OUT {
        &self.p8out
    }
    #[doc = "0x04 - Port 7 Direction"]
    #[inline(always)]
    pub const fn p7dir(&self) -> &P7DIR {
        &self.p7dir
    }
    #[doc = "0x05 - Port 8 Direction"]
    #[inline(always)]
    pub const fn p8dir(&self) -> &P8DIR {
        &self.p8dir
    }
    #[doc = "0x06 - Port 7 Resistor Enable"]
    #[inline(always)]
    pub const fn p7ren(&self) -> &P7REN {
        &self.p7ren
    }
    #[doc = "0x07 - Port 8 Resistor Enable"]
    #[inline(always)]
    pub const fn p8ren(&self) -> &P8REN {
        &self.p8ren
    }
    #[doc = "0x08 - Port 7 Drive Strenght"]
    #[inline(always)]
    pub const fn p7ds(&self) -> &P7DS {
        &self.p7ds
    }
    #[doc = "0x09 - Port 8 Drive Strenght"]
    #[inline(always)]
    pub const fn p8ds(&self) -> &P8DS {
        &self.p8ds
    }
    #[doc = "0x0a - Port 7 Selection"]
    #[inline(always)]
    pub const fn p7sel(&self) -> &P7SEL {
        &self.p7sel
    }
    #[doc = "0x0b - Port 8 Selection"]
    #[inline(always)]
    pub const fn p8sel(&self) -> &P8SEL {
        &self.p8sel
    }
}
#[doc = "P7IN (rw) register accessor: Port 7 Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p7in::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p7in::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7in`]
module"]
pub type P7IN = crate::Reg<p7in::P7IN_SPEC>;
#[doc = "Port 7 Input"]
pub mod p7in;
#[doc = "P8IN (rw) register accessor: Port 8 Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p8in::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p8in::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8in`]
module"]
pub type P8IN = crate::Reg<p8in::P8IN_SPEC>;
#[doc = "Port 8 Input"]
pub mod p8in;
#[doc = "P7OUT (rw) register accessor: Port 7 Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p7out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p7out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7out`]
module"]
pub type P7OUT = crate::Reg<p7out::P7OUT_SPEC>;
#[doc = "Port 7 Output"]
pub mod p7out;
#[doc = "P8OUT (rw) register accessor: Port 8 Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p8out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p8out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8out`]
module"]
pub type P8OUT = crate::Reg<p8out::P8OUT_SPEC>;
#[doc = "Port 8 Output"]
pub mod p8out;
#[doc = "P7DIR (rw) register accessor: Port 7 Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p7dir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p7dir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7dir`]
module"]
pub type P7DIR = crate::Reg<p7dir::P7DIR_SPEC>;
#[doc = "Port 7 Direction"]
pub mod p7dir;
#[doc = "P8DIR (rw) register accessor: Port 8 Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p8dir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p8dir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8dir`]
module"]
pub type P8DIR = crate::Reg<p8dir::P8DIR_SPEC>;
#[doc = "Port 8 Direction"]
pub mod p8dir;
#[doc = "P7REN (rw) register accessor: Port 7 Resistor Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p7ren::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p7ren::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7ren`]
module"]
pub type P7REN = crate::Reg<p7ren::P7REN_SPEC>;
#[doc = "Port 7 Resistor Enable"]
pub mod p7ren;
#[doc = "P8REN (rw) register accessor: Port 8 Resistor Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p8ren::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p8ren::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8ren`]
module"]
pub type P8REN = crate::Reg<p8ren::P8REN_SPEC>;
#[doc = "Port 8 Resistor Enable"]
pub mod p8ren;
#[doc = "P7DS (rw) register accessor: Port 7 Drive Strenght\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p7ds::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p7ds::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7ds`]
module"]
pub type P7DS = crate::Reg<p7ds::P7DS_SPEC>;
#[doc = "Port 7 Drive Strenght"]
pub mod p7ds;
#[doc = "P8DS (rw) register accessor: Port 8 Drive Strenght\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p8ds::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p8ds::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8ds`]
module"]
pub type P8DS = crate::Reg<p8ds::P8DS_SPEC>;
#[doc = "Port 8 Drive Strenght"]
pub mod p8ds;
#[doc = "P7SEL (rw) register accessor: Port 7 Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p7sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p7sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7sel`]
module"]
pub type P7SEL = crate::Reg<p7sel::P7SEL_SPEC>;
#[doc = "Port 7 Selection"]
pub mod p7sel;
#[doc = "P8SEL (rw) register accessor: Port 8 Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p8sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p8sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8sel`]
module"]
pub type P8SEL = crate::Reg<p8sel::P8SEL_SPEC>;
#[doc = "Port 8 Selection"]
pub mod p8sel;
