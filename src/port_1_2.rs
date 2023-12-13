#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    p1in: P1IN,
    p2in: P2IN,
    p1out: P1OUT,
    p2out: P2OUT,
    p1dir: P1DIR,
    p2dir: P2DIR,
    p1ren: P1REN,
    p2ren: P2REN,
    p1ds: P1DS,
    p2ds: P2DS,
    p1sel: P1SEL,
    p2sel: P2SEL,
    _reserved12: [u8; 0x02],
    p1iv: P1IV,
    _reserved13: [u8; 0x08],
    p1ies: P1IES,
    p2ies: P2IES,
    p1ie: P1IE,
    p2ie: P2IE,
    p1ifg: P1IFG,
    p2ifg: P2IFG,
    p2iv: P2IV,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 1 Input"]
    #[inline(always)]
    pub const fn p1in(&self) -> &P1IN {
        &self.p1in
    }
    #[doc = "0x01 - Port 2 Input"]
    #[inline(always)]
    pub const fn p2in(&self) -> &P2IN {
        &self.p2in
    }
    #[doc = "0x02 - Port 1 Output"]
    #[inline(always)]
    pub const fn p1out(&self) -> &P1OUT {
        &self.p1out
    }
    #[doc = "0x03 - Port 2 Output"]
    #[inline(always)]
    pub const fn p2out(&self) -> &P2OUT {
        &self.p2out
    }
    #[doc = "0x04 - Port 1 Direction"]
    #[inline(always)]
    pub const fn p1dir(&self) -> &P1DIR {
        &self.p1dir
    }
    #[doc = "0x05 - Port 2 Direction"]
    #[inline(always)]
    pub const fn p2dir(&self) -> &P2DIR {
        &self.p2dir
    }
    #[doc = "0x06 - Port 1 Resistor Enable"]
    #[inline(always)]
    pub const fn p1ren(&self) -> &P1REN {
        &self.p1ren
    }
    #[doc = "0x07 - Port 2 Resistor Enable"]
    #[inline(always)]
    pub const fn p2ren(&self) -> &P2REN {
        &self.p2ren
    }
    #[doc = "0x08 - Port 1 Drive Strenght"]
    #[inline(always)]
    pub const fn p1ds(&self) -> &P1DS {
        &self.p1ds
    }
    #[doc = "0x09 - Port 2 Drive Strenght"]
    #[inline(always)]
    pub const fn p2ds(&self) -> &P2DS {
        &self.p2ds
    }
    #[doc = "0x0a - Port 1 Selection"]
    #[inline(always)]
    pub const fn p1sel(&self) -> &P1SEL {
        &self.p1sel
    }
    #[doc = "0x0b - Port 2 Selection"]
    #[inline(always)]
    pub const fn p2sel(&self) -> &P2SEL {
        &self.p2sel
    }
    #[doc = "0x0e - Port 1 Interrupt Vector Word"]
    #[inline(always)]
    pub const fn p1iv(&self) -> &P1IV {
        &self.p1iv
    }
    #[doc = "0x18 - Port 1 Interrupt Edge Select"]
    #[inline(always)]
    pub const fn p1ies(&self) -> &P1IES {
        &self.p1ies
    }
    #[doc = "0x19 - Port 2 Interrupt Edge Select"]
    #[inline(always)]
    pub const fn p2ies(&self) -> &P2IES {
        &self.p2ies
    }
    #[doc = "0x1a - Port 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn p1ie(&self) -> &P1IE {
        &self.p1ie
    }
    #[doc = "0x1b - Port 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn p2ie(&self) -> &P2IE {
        &self.p2ie
    }
    #[doc = "0x1c - Port 1 Interrupt Flag"]
    #[inline(always)]
    pub const fn p1ifg(&self) -> &P1IFG {
        &self.p1ifg
    }
    #[doc = "0x1d - Port 2 Interrupt Flag"]
    #[inline(always)]
    pub const fn p2ifg(&self) -> &P2IFG {
        &self.p2ifg
    }
    #[doc = "0x1e - Port 2 Interrupt Vector Word"]
    #[inline(always)]
    pub const fn p2iv(&self) -> &P2IV {
        &self.p2iv
    }
}
#[doc = "P1IN (rw) register accessor: Port 1 Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1in::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1in::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1in`]
module"]
pub type P1IN = crate::Reg<p1in::P1IN_SPEC>;
#[doc = "Port 1 Input"]
pub mod p1in;
#[doc = "P2IN (rw) register accessor: Port 2 Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p2in::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p2in::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2in`]
module"]
pub type P2IN = crate::Reg<p2in::P2IN_SPEC>;
#[doc = "Port 2 Input"]
pub mod p2in;
#[doc = "P1OUT (rw) register accessor: Port 1 Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1out`]
module"]
pub type P1OUT = crate::Reg<p1out::P1OUT_SPEC>;
#[doc = "Port 1 Output"]
pub mod p1out;
#[doc = "P2OUT (rw) register accessor: Port 2 Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p2out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p2out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2out`]
module"]
pub type P2OUT = crate::Reg<p2out::P2OUT_SPEC>;
#[doc = "Port 2 Output"]
pub mod p2out;
#[doc = "P1DIR (rw) register accessor: Port 1 Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1dir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1dir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1dir`]
module"]
pub type P1DIR = crate::Reg<p1dir::P1DIR_SPEC>;
#[doc = "Port 1 Direction"]
pub mod p1dir;
#[doc = "P2DIR (rw) register accessor: Port 2 Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p2dir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p2dir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2dir`]
module"]
pub type P2DIR = crate::Reg<p2dir::P2DIR_SPEC>;
#[doc = "Port 2 Direction"]
pub mod p2dir;
#[doc = "P1REN (rw) register accessor: Port 1 Resistor Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1ren::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1ren::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1ren`]
module"]
pub type P1REN = crate::Reg<p1ren::P1REN_SPEC>;
#[doc = "Port 1 Resistor Enable"]
pub mod p1ren;
#[doc = "P2REN (rw) register accessor: Port 2 Resistor Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p2ren::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p2ren::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2ren`]
module"]
pub type P2REN = crate::Reg<p2ren::P2REN_SPEC>;
#[doc = "Port 2 Resistor Enable"]
pub mod p2ren;
#[doc = "P1DS (rw) register accessor: Port 1 Drive Strenght\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1ds::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1ds::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1ds`]
module"]
pub type P1DS = crate::Reg<p1ds::P1DS_SPEC>;
#[doc = "Port 1 Drive Strenght"]
pub mod p1ds;
#[doc = "P2DS (rw) register accessor: Port 2 Drive Strenght\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p2ds::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p2ds::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2ds`]
module"]
pub type P2DS = crate::Reg<p2ds::P2DS_SPEC>;
#[doc = "Port 2 Drive Strenght"]
pub mod p2ds;
#[doc = "P1SEL (rw) register accessor: Port 1 Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1sel`]
module"]
pub type P1SEL = crate::Reg<p1sel::P1SEL_SPEC>;
#[doc = "Port 1 Selection"]
pub mod p1sel;
#[doc = "P2SEL (rw) register accessor: Port 2 Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p2sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p2sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2sel`]
module"]
pub type P2SEL = crate::Reg<p2sel::P2SEL_SPEC>;
#[doc = "Port 2 Selection"]
pub mod p2sel;
#[doc = "P1IES (rw) register accessor: Port 1 Interrupt Edge Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1ies::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1ies::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1ies`]
module"]
pub type P1IES = crate::Reg<p1ies::P1IES_SPEC>;
#[doc = "Port 1 Interrupt Edge Select"]
pub mod p1ies;
#[doc = "P2IES (rw) register accessor: Port 2 Interrupt Edge Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p2ies::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p2ies::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2ies`]
module"]
pub type P2IES = crate::Reg<p2ies::P2IES_SPEC>;
#[doc = "Port 2 Interrupt Edge Select"]
pub mod p2ies;
#[doc = "P1IE (rw) register accessor: Port 1 Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1ie`]
module"]
pub type P1IE = crate::Reg<p1ie::P1IE_SPEC>;
#[doc = "Port 1 Interrupt Enable"]
pub mod p1ie;
#[doc = "P2IE (rw) register accessor: Port 2 Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p2ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p2ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2ie`]
module"]
pub type P2IE = crate::Reg<p2ie::P2IE_SPEC>;
#[doc = "Port 2 Interrupt Enable"]
pub mod p2ie;
#[doc = "P1IFG (rw) register accessor: Port 1 Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1ifg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1ifg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1ifg`]
module"]
pub type P1IFG = crate::Reg<p1ifg::P1IFG_SPEC>;
#[doc = "Port 1 Interrupt Flag"]
pub mod p1ifg;
#[doc = "P2IFG (rw) register accessor: Port 2 Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p2ifg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p2ifg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2ifg`]
module"]
pub type P2IFG = crate::Reg<p2ifg::P2IFG_SPEC>;
#[doc = "Port 2 Interrupt Flag"]
pub mod p2ifg;
#[doc = "P1IV (rw) register accessor: Port 1 Interrupt Vector Word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1iv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1iv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1iv`]
module"]
pub type P1IV = crate::Reg<p1iv::P1IV_SPEC>;
#[doc = "Port 1 Interrupt Vector Word"]
pub mod p1iv;
#[doc = "P2IV (rw) register accessor: Port 2 Interrupt Vector Word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p2iv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p2iv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2iv`]
module"]
pub type P2IV = crate::Reg<p2iv::P2IV_SPEC>;
#[doc = "Port 2 Interrupt Vector Word"]
pub mod p2iv;
