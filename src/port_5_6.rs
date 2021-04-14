#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 5 Input"]
    pub p5in: crate::Reg<p5in::P5IN_SPEC>,
    #[doc = "0x01 - Port 6 Input"]
    pub p6in: crate::Reg<p6in::P6IN_SPEC>,
    #[doc = "0x02 - Port 5 Output"]
    pub p5out: crate::Reg<p5out::P5OUT_SPEC>,
    #[doc = "0x03 - Port 6 Output"]
    pub p6out: crate::Reg<p6out::P6OUT_SPEC>,
    #[doc = "0x04 - Port 5 Direction"]
    pub p5dir: crate::Reg<p5dir::P5DIR_SPEC>,
    #[doc = "0x05 - Port 6 Direction"]
    pub p6dir: crate::Reg<p6dir::P6DIR_SPEC>,
    #[doc = "0x06 - Port 5 Resistor Enable"]
    pub p5ren: crate::Reg<p5ren::P5REN_SPEC>,
    #[doc = "0x07 - Port 6 Resistor Enable"]
    pub p6ren: crate::Reg<p6ren::P6REN_SPEC>,
    #[doc = "0x08 - Port 5 Drive Strenght"]
    pub p5ds: crate::Reg<p5ds::P5DS_SPEC>,
    #[doc = "0x09 - Port 6 Drive Strenght"]
    pub p6ds: crate::Reg<p6ds::P6DS_SPEC>,
    #[doc = "0x0a - Port 5 Selection"]
    pub p5sel: crate::Reg<p5sel::P5SEL_SPEC>,
    #[doc = "0x0b - Port 6 Selection"]
    pub p6sel: crate::Reg<p6sel::P6SEL_SPEC>,
}
#[doc = "P5IN register accessor: an alias for `Reg<P5IN_SPEC>`"]
pub type P5IN = crate::Reg<p5in::P5IN_SPEC>;
#[doc = "Port 5 Input"]
pub mod p5in;
#[doc = "P6IN register accessor: an alias for `Reg<P6IN_SPEC>`"]
pub type P6IN = crate::Reg<p6in::P6IN_SPEC>;
#[doc = "Port 6 Input"]
pub mod p6in;
#[doc = "P5OUT register accessor: an alias for `Reg<P5OUT_SPEC>`"]
pub type P5OUT = crate::Reg<p5out::P5OUT_SPEC>;
#[doc = "Port 5 Output"]
pub mod p5out;
#[doc = "P6OUT register accessor: an alias for `Reg<P6OUT_SPEC>`"]
pub type P6OUT = crate::Reg<p6out::P6OUT_SPEC>;
#[doc = "Port 6 Output"]
pub mod p6out;
#[doc = "P5DIR register accessor: an alias for `Reg<P5DIR_SPEC>`"]
pub type P5DIR = crate::Reg<p5dir::P5DIR_SPEC>;
#[doc = "Port 5 Direction"]
pub mod p5dir;
#[doc = "P6DIR register accessor: an alias for `Reg<P6DIR_SPEC>`"]
pub type P6DIR = crate::Reg<p6dir::P6DIR_SPEC>;
#[doc = "Port 6 Direction"]
pub mod p6dir;
#[doc = "P5REN register accessor: an alias for `Reg<P5REN_SPEC>`"]
pub type P5REN = crate::Reg<p5ren::P5REN_SPEC>;
#[doc = "Port 5 Resistor Enable"]
pub mod p5ren;
#[doc = "P6REN register accessor: an alias for `Reg<P6REN_SPEC>`"]
pub type P6REN = crate::Reg<p6ren::P6REN_SPEC>;
#[doc = "Port 6 Resistor Enable"]
pub mod p6ren;
#[doc = "P5DS register accessor: an alias for `Reg<P5DS_SPEC>`"]
pub type P5DS = crate::Reg<p5ds::P5DS_SPEC>;
#[doc = "Port 5 Drive Strenght"]
pub mod p5ds;
#[doc = "P6DS register accessor: an alias for `Reg<P6DS_SPEC>`"]
pub type P6DS = crate::Reg<p6ds::P6DS_SPEC>;
#[doc = "Port 6 Drive Strenght"]
pub mod p6ds;
#[doc = "P5SEL register accessor: an alias for `Reg<P5SEL_SPEC>`"]
pub type P5SEL = crate::Reg<p5sel::P5SEL_SPEC>;
#[doc = "Port 5 Selection"]
pub mod p5sel;
#[doc = "P6SEL register accessor: an alias for `Reg<P6SEL_SPEC>`"]
pub type P6SEL = crate::Reg<p6sel::P6SEL_SPEC>;
#[doc = "Port 6 Selection"]
pub mod p6sel;
