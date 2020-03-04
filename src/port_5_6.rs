#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 5 Input"]
    pub p5in: P5IN,
    #[doc = "0x01 - Port 6 Input"]
    pub p6in: P6IN,
    #[doc = "0x02 - Port 5 Output"]
    pub p5out: P5OUT,
    #[doc = "0x03 - Port 6 Output"]
    pub p6out: P6OUT,
    #[doc = "0x04 - Port 5 Direction"]
    pub p5dir: P5DIR,
    #[doc = "0x05 - Port 6 Direction"]
    pub p6dir: P6DIR,
    #[doc = "0x06 - Port 5 Resistor Enable"]
    pub p5ren: P5REN,
    #[doc = "0x07 - Port 6 Resistor Enable"]
    pub p6ren: P6REN,
    #[doc = "0x08 - Port 5 Drive Strenght"]
    pub p5ds: P5DS,
    #[doc = "0x09 - Port 6 Drive Strenght"]
    pub p6ds: P6DS,
    #[doc = "0x0a - Port 5 Selection"]
    pub p5sel: P5SEL,
    #[doc = "0x0b - Port 6 Selection"]
    pub p6sel: P6SEL,
}
#[doc = "Port 5 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5in](p5in) module"]
pub type P5IN = crate::Reg<u8, _P5IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5IN;
#[doc = "`read()` method returns [p5in::R](p5in::R) reader structure"]
impl crate::Readable for P5IN {}
#[doc = "`write(|w| ..)` method takes [p5in::W](p5in::W) writer structure"]
impl crate::Writable for P5IN {}
#[doc = "Port 5 Input"]
pub mod p5in;
#[doc = "Port 6 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6in](p6in) module"]
pub type P6IN = crate::Reg<u8, _P6IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6IN;
#[doc = "`read()` method returns [p6in::R](p6in::R) reader structure"]
impl crate::Readable for P6IN {}
#[doc = "`write(|w| ..)` method takes [p6in::W](p6in::W) writer structure"]
impl crate::Writable for P6IN {}
#[doc = "Port 6 Input"]
pub mod p6in;
#[doc = "Port 5 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5out](p5out) module"]
pub type P5OUT = crate::Reg<u8, _P5OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5OUT;
#[doc = "`read()` method returns [p5out::R](p5out::R) reader structure"]
impl crate::Readable for P5OUT {}
#[doc = "`write(|w| ..)` method takes [p5out::W](p5out::W) writer structure"]
impl crate::Writable for P5OUT {}
#[doc = "Port 5 Output"]
pub mod p5out;
#[doc = "Port 6 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6out](p6out) module"]
pub type P6OUT = crate::Reg<u8, _P6OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6OUT;
#[doc = "`read()` method returns [p6out::R](p6out::R) reader structure"]
impl crate::Readable for P6OUT {}
#[doc = "`write(|w| ..)` method takes [p6out::W](p6out::W) writer structure"]
impl crate::Writable for P6OUT {}
#[doc = "Port 6 Output"]
pub mod p6out;
#[doc = "Port 5 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5dir](p5dir) module"]
pub type P5DIR = crate::Reg<u8, _P5DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5DIR;
#[doc = "`read()` method returns [p5dir::R](p5dir::R) reader structure"]
impl crate::Readable for P5DIR {}
#[doc = "`write(|w| ..)` method takes [p5dir::W](p5dir::W) writer structure"]
impl crate::Writable for P5DIR {}
#[doc = "Port 5 Direction"]
pub mod p5dir;
#[doc = "Port 6 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6dir](p6dir) module"]
pub type P6DIR = crate::Reg<u8, _P6DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6DIR;
#[doc = "`read()` method returns [p6dir::R](p6dir::R) reader structure"]
impl crate::Readable for P6DIR {}
#[doc = "`write(|w| ..)` method takes [p6dir::W](p6dir::W) writer structure"]
impl crate::Writable for P6DIR {}
#[doc = "Port 6 Direction"]
pub mod p6dir;
#[doc = "Port 5 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5ren](p5ren) module"]
pub type P5REN = crate::Reg<u8, _P5REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5REN;
#[doc = "`read()` method returns [p5ren::R](p5ren::R) reader structure"]
impl crate::Readable for P5REN {}
#[doc = "`write(|w| ..)` method takes [p5ren::W](p5ren::W) writer structure"]
impl crate::Writable for P5REN {}
#[doc = "Port 5 Resistor Enable"]
pub mod p5ren;
#[doc = "Port 6 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6ren](p6ren) module"]
pub type P6REN = crate::Reg<u8, _P6REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6REN;
#[doc = "`read()` method returns [p6ren::R](p6ren::R) reader structure"]
impl crate::Readable for P6REN {}
#[doc = "`write(|w| ..)` method takes [p6ren::W](p6ren::W) writer structure"]
impl crate::Writable for P6REN {}
#[doc = "Port 6 Resistor Enable"]
pub mod p6ren;
#[doc = "Port 5 Drive Strenght\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5ds](p5ds) module"]
pub type P5DS = crate::Reg<u8, _P5DS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5DS;
#[doc = "`read()` method returns [p5ds::R](p5ds::R) reader structure"]
impl crate::Readable for P5DS {}
#[doc = "`write(|w| ..)` method takes [p5ds::W](p5ds::W) writer structure"]
impl crate::Writable for P5DS {}
#[doc = "Port 5 Drive Strenght"]
pub mod p5ds;
#[doc = "Port 6 Drive Strenght\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6ds](p6ds) module"]
pub type P6DS = crate::Reg<u8, _P6DS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6DS;
#[doc = "`read()` method returns [p6ds::R](p6ds::R) reader structure"]
impl crate::Readable for P6DS {}
#[doc = "`write(|w| ..)` method takes [p6ds::W](p6ds::W) writer structure"]
impl crate::Writable for P6DS {}
#[doc = "Port 6 Drive Strenght"]
pub mod p6ds;
#[doc = "Port 5 Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5sel](p5sel) module"]
pub type P5SEL = crate::Reg<u8, _P5SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5SEL;
#[doc = "`read()` method returns [p5sel::R](p5sel::R) reader structure"]
impl crate::Readable for P5SEL {}
#[doc = "`write(|w| ..)` method takes [p5sel::W](p5sel::W) writer structure"]
impl crate::Writable for P5SEL {}
#[doc = "Port 5 Selection"]
pub mod p5sel;
#[doc = "Port 6 Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6sel](p6sel) module"]
pub type P6SEL = crate::Reg<u8, _P6SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P6SEL;
#[doc = "`read()` method returns [p6sel::R](p6sel::R) reader structure"]
impl crate::Readable for P6SEL {}
#[doc = "`write(|w| ..)` method takes [p6sel::W](p6sel::W) writer structure"]
impl crate::Writable for P6SEL {}
#[doc = "Port 6 Selection"]
pub mod p6sel;
