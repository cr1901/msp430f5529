#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    p5in: P5IN,
    p6in: P6IN,
    p5out: P5OUT,
    p6out: P6OUT,
    p5dir: P5DIR,
    p6dir: P6DIR,
    p5ren: P5REN,
    p6ren: P6REN,
    p5ds: P5DS,
    p6ds: P6DS,
    p5sel: P5SEL,
    p6sel: P6SEL,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 5 Input"]
    #[inline(always)]
    pub const fn p5in(&self) -> &P5IN {
        &self.p5in
    }
    #[doc = "0x01 - Port 6 Input"]
    #[inline(always)]
    pub const fn p6in(&self) -> &P6IN {
        &self.p6in
    }
    #[doc = "0x02 - Port 5 Output"]
    #[inline(always)]
    pub const fn p5out(&self) -> &P5OUT {
        &self.p5out
    }
    #[doc = "0x03 - Port 6 Output"]
    #[inline(always)]
    pub const fn p6out(&self) -> &P6OUT {
        &self.p6out
    }
    #[doc = "0x04 - Port 5 Direction"]
    #[inline(always)]
    pub const fn p5dir(&self) -> &P5DIR {
        &self.p5dir
    }
    #[doc = "0x05 - Port 6 Direction"]
    #[inline(always)]
    pub const fn p6dir(&self) -> &P6DIR {
        &self.p6dir
    }
    #[doc = "0x06 - Port 5 Resistor Enable"]
    #[inline(always)]
    pub const fn p5ren(&self) -> &P5REN {
        &self.p5ren
    }
    #[doc = "0x07 - Port 6 Resistor Enable"]
    #[inline(always)]
    pub const fn p6ren(&self) -> &P6REN {
        &self.p6ren
    }
    #[doc = "0x08 - Port 5 Drive Strenght"]
    #[inline(always)]
    pub const fn p5ds(&self) -> &P5DS {
        &self.p5ds
    }
    #[doc = "0x09 - Port 6 Drive Strenght"]
    #[inline(always)]
    pub const fn p6ds(&self) -> &P6DS {
        &self.p6ds
    }
    #[doc = "0x0a - Port 5 Selection"]
    #[inline(always)]
    pub const fn p5sel(&self) -> &P5SEL {
        &self.p5sel
    }
    #[doc = "0x0b - Port 6 Selection"]
    #[inline(always)]
    pub const fn p6sel(&self) -> &P6SEL {
        &self.p6sel
    }
}
#[doc = "P5IN (rw) register accessor: Port 5 Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p5in::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p5in::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5in`]
module"]
pub type P5IN = crate::Reg<p5in::P5IN_SPEC>;
#[doc = "Port 5 Input"]
pub mod p5in;
#[doc = "P6IN (rw) register accessor: Port 6 Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p6in::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p6in::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6in`]
module"]
pub type P6IN = crate::Reg<p6in::P6IN_SPEC>;
#[doc = "Port 6 Input"]
pub mod p6in;
#[doc = "P5OUT (rw) register accessor: Port 5 Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p5out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p5out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5out`]
module"]
pub type P5OUT = crate::Reg<p5out::P5OUT_SPEC>;
#[doc = "Port 5 Output"]
pub mod p5out;
#[doc = "P6OUT (rw) register accessor: Port 6 Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p6out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p6out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6out`]
module"]
pub type P6OUT = crate::Reg<p6out::P6OUT_SPEC>;
#[doc = "Port 6 Output"]
pub mod p6out;
#[doc = "P5DIR (rw) register accessor: Port 5 Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p5dir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p5dir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5dir`]
module"]
pub type P5DIR = crate::Reg<p5dir::P5DIR_SPEC>;
#[doc = "Port 5 Direction"]
pub mod p5dir;
#[doc = "P6DIR (rw) register accessor: Port 6 Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p6dir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p6dir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6dir`]
module"]
pub type P6DIR = crate::Reg<p6dir::P6DIR_SPEC>;
#[doc = "Port 6 Direction"]
pub mod p6dir;
#[doc = "P5REN (rw) register accessor: Port 5 Resistor Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p5ren::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p5ren::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5ren`]
module"]
pub type P5REN = crate::Reg<p5ren::P5REN_SPEC>;
#[doc = "Port 5 Resistor Enable"]
pub mod p5ren;
#[doc = "P6REN (rw) register accessor: Port 6 Resistor Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p6ren::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p6ren::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6ren`]
module"]
pub type P6REN = crate::Reg<p6ren::P6REN_SPEC>;
#[doc = "Port 6 Resistor Enable"]
pub mod p6ren;
#[doc = "P5DS (rw) register accessor: Port 5 Drive Strenght\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p5ds::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p5ds::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5ds`]
module"]
pub type P5DS = crate::Reg<p5ds::P5DS_SPEC>;
#[doc = "Port 5 Drive Strenght"]
pub mod p5ds;
#[doc = "P6DS (rw) register accessor: Port 6 Drive Strenght\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p6ds::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p6ds::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6ds`]
module"]
pub type P6DS = crate::Reg<p6ds::P6DS_SPEC>;
#[doc = "Port 6 Drive Strenght"]
pub mod p6ds;
#[doc = "P5SEL (rw) register accessor: Port 5 Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p5sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p5sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5sel`]
module"]
pub type P5SEL = crate::Reg<p5sel::P5SEL_SPEC>;
#[doc = "Port 5 Selection"]
pub mod p5sel;
#[doc = "P6SEL (rw) register accessor: Port 6 Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p6sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p6sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6sel`]
module"]
pub type P6SEL = crate::Reg<p6sel::P6SEL_SPEC>;
#[doc = "Port 6 Selection"]
pub mod p6sel;
