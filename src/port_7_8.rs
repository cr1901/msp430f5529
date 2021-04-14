#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 7 Input"]
    pub p7in: crate::Reg<p7in::P7IN_SPEC>,
    #[doc = "0x01 - Port 8 Input"]
    pub p8in: crate::Reg<p8in::P8IN_SPEC>,
    #[doc = "0x02 - Port 7 Output"]
    pub p7out: crate::Reg<p7out::P7OUT_SPEC>,
    #[doc = "0x03 - Port 8 Output"]
    pub p8out: crate::Reg<p8out::P8OUT_SPEC>,
    #[doc = "0x04 - Port 7 Direction"]
    pub p7dir: crate::Reg<p7dir::P7DIR_SPEC>,
    #[doc = "0x05 - Port 8 Direction"]
    pub p8dir: crate::Reg<p8dir::P8DIR_SPEC>,
    #[doc = "0x06 - Port 7 Resistor Enable"]
    pub p7ren: crate::Reg<p7ren::P7REN_SPEC>,
    #[doc = "0x07 - Port 8 Resistor Enable"]
    pub p8ren: crate::Reg<p8ren::P8REN_SPEC>,
    #[doc = "0x08 - Port 7 Drive Strenght"]
    pub p7ds: crate::Reg<p7ds::P7DS_SPEC>,
    #[doc = "0x09 - Port 8 Drive Strenght"]
    pub p8ds: crate::Reg<p8ds::P8DS_SPEC>,
    #[doc = "0x0a - Port 7 Selection"]
    pub p7sel: crate::Reg<p7sel::P7SEL_SPEC>,
    #[doc = "0x0b - Port 8 Selection"]
    pub p8sel: crate::Reg<p8sel::P8SEL_SPEC>,
}
#[doc = "P7IN register accessor: an alias for `Reg<P7IN_SPEC>`"]
pub type P7IN = crate::Reg<p7in::P7IN_SPEC>;
#[doc = "Port 7 Input"]
pub mod p7in;
#[doc = "P8IN register accessor: an alias for `Reg<P8IN_SPEC>`"]
pub type P8IN = crate::Reg<p8in::P8IN_SPEC>;
#[doc = "Port 8 Input"]
pub mod p8in;
#[doc = "P7OUT register accessor: an alias for `Reg<P7OUT_SPEC>`"]
pub type P7OUT = crate::Reg<p7out::P7OUT_SPEC>;
#[doc = "Port 7 Output"]
pub mod p7out;
#[doc = "P8OUT register accessor: an alias for `Reg<P8OUT_SPEC>`"]
pub type P8OUT = crate::Reg<p8out::P8OUT_SPEC>;
#[doc = "Port 8 Output"]
pub mod p8out;
#[doc = "P7DIR register accessor: an alias for `Reg<P7DIR_SPEC>`"]
pub type P7DIR = crate::Reg<p7dir::P7DIR_SPEC>;
#[doc = "Port 7 Direction"]
pub mod p7dir;
#[doc = "P8DIR register accessor: an alias for `Reg<P8DIR_SPEC>`"]
pub type P8DIR = crate::Reg<p8dir::P8DIR_SPEC>;
#[doc = "Port 8 Direction"]
pub mod p8dir;
#[doc = "P7REN register accessor: an alias for `Reg<P7REN_SPEC>`"]
pub type P7REN = crate::Reg<p7ren::P7REN_SPEC>;
#[doc = "Port 7 Resistor Enable"]
pub mod p7ren;
#[doc = "P8REN register accessor: an alias for `Reg<P8REN_SPEC>`"]
pub type P8REN = crate::Reg<p8ren::P8REN_SPEC>;
#[doc = "Port 8 Resistor Enable"]
pub mod p8ren;
#[doc = "P7DS register accessor: an alias for `Reg<P7DS_SPEC>`"]
pub type P7DS = crate::Reg<p7ds::P7DS_SPEC>;
#[doc = "Port 7 Drive Strenght"]
pub mod p7ds;
#[doc = "P8DS register accessor: an alias for `Reg<P8DS_SPEC>`"]
pub type P8DS = crate::Reg<p8ds::P8DS_SPEC>;
#[doc = "Port 8 Drive Strenght"]
pub mod p8ds;
#[doc = "P7SEL register accessor: an alias for `Reg<P7SEL_SPEC>`"]
pub type P7SEL = crate::Reg<p7sel::P7SEL_SPEC>;
#[doc = "Port 7 Selection"]
pub mod p7sel;
#[doc = "P8SEL register accessor: an alias for `Reg<P8SEL_SPEC>`"]
pub type P8SEL = crate::Reg<p8sel::P8SEL_SPEC>;
#[doc = "Port 8 Selection"]
pub mod p8sel;
