#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 3 Input"]
    pub p3in: crate::Reg<p3in::P3IN_SPEC>,
    #[doc = "0x01 - Port 4 Input"]
    pub p4in: crate::Reg<p4in::P4IN_SPEC>,
    #[doc = "0x02 - Port 3 Output"]
    pub p3out: crate::Reg<p3out::P3OUT_SPEC>,
    #[doc = "0x03 - Port 4 Output"]
    pub p4out: crate::Reg<p4out::P4OUT_SPEC>,
    #[doc = "0x04 - Port 3 Direction"]
    pub p3dir: crate::Reg<p3dir::P3DIR_SPEC>,
    #[doc = "0x05 - Port 4 Direction"]
    pub p4dir: crate::Reg<p4dir::P4DIR_SPEC>,
    #[doc = "0x06 - Port 3 Resistor Enable"]
    pub p3ren: crate::Reg<p3ren::P3REN_SPEC>,
    #[doc = "0x07 - Port 4 Resistor Enable"]
    pub p4ren: crate::Reg<p4ren::P4REN_SPEC>,
    #[doc = "0x08 - Port 3 Drive Strenght"]
    pub p3ds: crate::Reg<p3ds::P3DS_SPEC>,
    #[doc = "0x09 - Port 4 Drive Strenght"]
    pub p4ds: crate::Reg<p4ds::P4DS_SPEC>,
    #[doc = "0x0a - Port 3 Selection"]
    pub p3sel: crate::Reg<p3sel::P3SEL_SPEC>,
    #[doc = "0x0b - Port 4 Selection"]
    pub p4sel: crate::Reg<p4sel::P4SEL_SPEC>,
}
#[doc = "P3IN register accessor: an alias for `Reg<P3IN_SPEC>`"]
pub type P3IN = crate::Reg<p3in::P3IN_SPEC>;
#[doc = "Port 3 Input"]
pub mod p3in;
#[doc = "P4IN register accessor: an alias for `Reg<P4IN_SPEC>`"]
pub type P4IN = crate::Reg<p4in::P4IN_SPEC>;
#[doc = "Port 4 Input"]
pub mod p4in;
#[doc = "P3OUT register accessor: an alias for `Reg<P3OUT_SPEC>`"]
pub type P3OUT = crate::Reg<p3out::P3OUT_SPEC>;
#[doc = "Port 3 Output"]
pub mod p3out;
#[doc = "P4OUT register accessor: an alias for `Reg<P4OUT_SPEC>`"]
pub type P4OUT = crate::Reg<p4out::P4OUT_SPEC>;
#[doc = "Port 4 Output"]
pub mod p4out;
#[doc = "P3DIR register accessor: an alias for `Reg<P3DIR_SPEC>`"]
pub type P3DIR = crate::Reg<p3dir::P3DIR_SPEC>;
#[doc = "Port 3 Direction"]
pub mod p3dir;
#[doc = "P4DIR register accessor: an alias for `Reg<P4DIR_SPEC>`"]
pub type P4DIR = crate::Reg<p4dir::P4DIR_SPEC>;
#[doc = "Port 4 Direction"]
pub mod p4dir;
#[doc = "P3REN register accessor: an alias for `Reg<P3REN_SPEC>`"]
pub type P3REN = crate::Reg<p3ren::P3REN_SPEC>;
#[doc = "Port 3 Resistor Enable"]
pub mod p3ren;
#[doc = "P4REN register accessor: an alias for `Reg<P4REN_SPEC>`"]
pub type P4REN = crate::Reg<p4ren::P4REN_SPEC>;
#[doc = "Port 4 Resistor Enable"]
pub mod p4ren;
#[doc = "P3DS register accessor: an alias for `Reg<P3DS_SPEC>`"]
pub type P3DS = crate::Reg<p3ds::P3DS_SPEC>;
#[doc = "Port 3 Drive Strenght"]
pub mod p3ds;
#[doc = "P4DS register accessor: an alias for `Reg<P4DS_SPEC>`"]
pub type P4DS = crate::Reg<p4ds::P4DS_SPEC>;
#[doc = "Port 4 Drive Strenght"]
pub mod p4ds;
#[doc = "P3SEL register accessor: an alias for `Reg<P3SEL_SPEC>`"]
pub type P3SEL = crate::Reg<p3sel::P3SEL_SPEC>;
#[doc = "Port 3 Selection"]
pub mod p3sel;
#[doc = "P4SEL register accessor: an alias for `Reg<P4SEL_SPEC>`"]
pub type P4SEL = crate::Reg<p4sel::P4SEL_SPEC>;
#[doc = "Port 4 Selection"]
pub mod p4sel;
