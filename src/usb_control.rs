#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB Controller key register"]
    pub usbkeyid: USBKEYID,
    #[doc = "0x02 - USB Module configuration register"]
    pub usbcnf: USBCNF,
    #[doc = "0x04 - USB PHY control register"]
    pub usbphyctl: USBPHYCTL,
    _reserved3: [u8; 2usize],
    #[doc = "0x08 - USB Power control register"]
    pub usbpwrctl: USBPWRCTL,
    _reserved4: [u8; 6usize],
    #[doc = "0x10 - USB PLL control register"]
    pub usbpllctl: USBPLLCTL,
    #[doc = "0x12 - USB PLL Clock Divider Buffer control register"]
    pub usbplldivb: USBPLLDIVB,
    #[doc = "0x14 - USB PLL Interrupt control register"]
    pub usbpllir: USBPLLIR,
    _reserved7: [u8; 10usize],
    #[doc = "0x20 - USB Input endpoint_0: Configuration"]
    pub usbiepcnf_0: USBIEPCNF_0,
    #[doc = "0x21 - USB Input endpoint_0: Byte Count"]
    pub usbiepcnt_0: USBIEPCNT_0,
    #[doc = "0x22 - USB Output endpoint_0: Configuration"]
    pub usboepcnf_0: USBOEPCNF_0,
    #[doc = "0x23 - USB Output endpoint_0: byte count"]
    pub usboepcnt_0: USBOEPCNT_0,
    _reserved11: [u8; 10usize],
    #[doc = "0x2e - USB Input endpoint interrupt enable flags"]
    pub usbiepie: USBIEPIE,
    #[doc = "0x2f - USB Output endpoint interrupt enable flags"]
    pub usboepie: USBOEPIE,
    #[doc = "0x30 - USB Input endpoint interrupt flags"]
    pub usbiepifg: USBIEPIFG,
    #[doc = "0x31 - USB Output endpoint interrupt flags"]
    pub usboepifg: USBOEPIFG,
    #[doc = "0x32 - USB Vector interrupt register"]
    pub usbvecint: USBVECINT,
    _reserved16: [u8; 2usize],
    #[doc = "0x36 - USB maintenance register"]
    pub usbmaint: USBMAINT,
    #[doc = "0x38 - USB Time Stamp register"]
    pub usbtsreg: USBTSREG,
    #[doc = "0x3a - USB Frame number"]
    pub usbfn: USBFN,
    #[doc = "0x3c - USB control register"]
    pub usbctl: USBCTL,
    #[doc = "0x3d - USB interrupt enable register"]
    pub usbie: USBIE,
    #[doc = "0x3e - USB interrupt flag register"]
    pub usbifg: USBIFG,
    #[doc = "0x3f - USB Function address register"]
    pub usbfunadr: USBFUNADR,
}
#[doc = "USB Input endpoint_0: Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepcnf_0](usbiepcnf_0) module"]
pub type USBIEPCNF_0 = crate::Reg<u8, _USBIEPCNF_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPCNF_0;
#[doc = "`read()` method returns [usbiepcnf_0::R](usbiepcnf_0::R) reader structure"]
impl crate::Readable for USBIEPCNF_0 {}
#[doc = "`write(|w| ..)` method takes [usbiepcnf_0::W](usbiepcnf_0::W) writer structure"]
impl crate::Writable for USBIEPCNF_0 {}
#[doc = "USB Input endpoint_0: Configuration"]
pub mod usbiepcnf_0;
#[doc = "USB Input endpoint_0: Byte Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepcnt_0](usbiepcnt_0) module"]
pub type USBIEPCNT_0 = crate::Reg<u8, _USBIEPCNT_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPCNT_0;
#[doc = "`read()` method returns [usbiepcnt_0::R](usbiepcnt_0::R) reader structure"]
impl crate::Readable for USBIEPCNT_0 {}
#[doc = "`write(|w| ..)` method takes [usbiepcnt_0::W](usbiepcnt_0::W) writer structure"]
impl crate::Writable for USBIEPCNT_0 {}
#[doc = "USB Input endpoint_0: Byte Count"]
pub mod usbiepcnt_0;
#[doc = "USB Output endpoint_0: Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepcnf_0](usboepcnf_0) module"]
pub type USBOEPCNF_0 = crate::Reg<u8, _USBOEPCNF_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPCNF_0;
#[doc = "`read()` method returns [usboepcnf_0::R](usboepcnf_0::R) reader structure"]
impl crate::Readable for USBOEPCNF_0 {}
#[doc = "`write(|w| ..)` method takes [usboepcnf_0::W](usboepcnf_0::W) writer structure"]
impl crate::Writable for USBOEPCNF_0 {}
#[doc = "USB Output endpoint_0: Configuration"]
pub mod usboepcnf_0;
#[doc = "USB Output endpoint_0: byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepcnt_0](usboepcnt_0) module"]
pub type USBOEPCNT_0 = crate::Reg<u8, _USBOEPCNT_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPCNT_0;
#[doc = "`read()` method returns [usboepcnt_0::R](usboepcnt_0::R) reader structure"]
impl crate::Readable for USBOEPCNT_0 {}
#[doc = "`write(|w| ..)` method takes [usboepcnt_0::W](usboepcnt_0::W) writer structure"]
impl crate::Writable for USBOEPCNT_0 {}
#[doc = "USB Output endpoint_0: byte count"]
pub mod usboepcnt_0;
#[doc = "USB Input endpoint interrupt enable flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepie](usbiepie) module"]
pub type USBIEPIE = crate::Reg<u8, _USBIEPIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPIE;
#[doc = "`read()` method returns [usbiepie::R](usbiepie::R) reader structure"]
impl crate::Readable for USBIEPIE {}
#[doc = "`write(|w| ..)` method takes [usbiepie::W](usbiepie::W) writer structure"]
impl crate::Writable for USBIEPIE {}
#[doc = "USB Input endpoint interrupt enable flags"]
pub mod usbiepie;
#[doc = "USB Output endpoint interrupt enable flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepie](usboepie) module"]
pub type USBOEPIE = crate::Reg<u8, _USBOEPIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPIE;
#[doc = "`read()` method returns [usboepie::R](usboepie::R) reader structure"]
impl crate::Readable for USBOEPIE {}
#[doc = "`write(|w| ..)` method takes [usboepie::W](usboepie::W) writer structure"]
impl crate::Writable for USBOEPIE {}
#[doc = "USB Output endpoint interrupt enable flags"]
pub mod usboepie;
#[doc = "USB Input endpoint interrupt flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbiepifg](usbiepifg) module"]
pub type USBIEPIFG = crate::Reg<u8, _USBIEPIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIEPIFG;
#[doc = "`read()` method returns [usbiepifg::R](usbiepifg::R) reader structure"]
impl crate::Readable for USBIEPIFG {}
#[doc = "`write(|w| ..)` method takes [usbiepifg::W](usbiepifg::W) writer structure"]
impl crate::Writable for USBIEPIFG {}
#[doc = "USB Input endpoint interrupt flags"]
pub mod usbiepifg;
#[doc = "USB Output endpoint interrupt flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepifg](usboepifg) module"]
pub type USBOEPIFG = crate::Reg<u8, _USBOEPIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBOEPIFG;
#[doc = "`read()` method returns [usboepifg::R](usboepifg::R) reader structure"]
impl crate::Readable for USBOEPIFG {}
#[doc = "`write(|w| ..)` method takes [usboepifg::W](usboepifg::W) writer structure"]
impl crate::Writable for USBOEPIFG {}
#[doc = "USB Output endpoint interrupt flags"]
pub mod usboepifg;
#[doc = "USB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbctl](usbctl) module"]
pub type USBCTL = crate::Reg<u8, _USBCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBCTL;
#[doc = "`read()` method returns [usbctl::R](usbctl::R) reader structure"]
impl crate::Readable for USBCTL {}
#[doc = "`write(|w| ..)` method takes [usbctl::W](usbctl::W) writer structure"]
impl crate::Writable for USBCTL {}
#[doc = "USB control register"]
pub mod usbctl;
#[doc = "USB interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbie](usbie) module"]
pub type USBIE = crate::Reg<u8, _USBIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIE;
#[doc = "`read()` method returns [usbie::R](usbie::R) reader structure"]
impl crate::Readable for USBIE {}
#[doc = "`write(|w| ..)` method takes [usbie::W](usbie::W) writer structure"]
impl crate::Writable for USBIE {}
#[doc = "USB interrupt enable register"]
pub mod usbie;
#[doc = "USB interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbifg](usbifg) module"]
pub type USBIFG = crate::Reg<u8, _USBIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBIFG;
#[doc = "`read()` method returns [usbifg::R](usbifg::R) reader structure"]
impl crate::Readable for USBIFG {}
#[doc = "`write(|w| ..)` method takes [usbifg::W](usbifg::W) writer structure"]
impl crate::Writable for USBIFG {}
#[doc = "USB interrupt flag register"]
pub mod usbifg;
#[doc = "USB Function address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbfunadr](usbfunadr) module"]
pub type USBFUNADR = crate::Reg<u8, _USBFUNADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBFUNADR;
#[doc = "`read()` method returns [usbfunadr::R](usbfunadr::R) reader structure"]
impl crate::Readable for USBFUNADR {}
#[doc = "`write(|w| ..)` method takes [usbfunadr::W](usbfunadr::W) writer structure"]
impl crate::Writable for USBFUNADR {}
#[doc = "USB Function address register"]
pub mod usbfunadr;
#[doc = "USB Controller key register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbkeyid](usbkeyid) module"]
pub type USBKEYID = crate::Reg<u16, _USBKEYID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBKEYID;
#[doc = "`read()` method returns [usbkeyid::R](usbkeyid::R) reader structure"]
impl crate::Readable for USBKEYID {}
#[doc = "`write(|w| ..)` method takes [usbkeyid::W](usbkeyid::W) writer structure"]
impl crate::Writable for USBKEYID {}
#[doc = "USB Controller key register"]
pub mod usbkeyid;
#[doc = "USB Module configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbcnf](usbcnf) module"]
pub type USBCNF = crate::Reg<u16, _USBCNF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBCNF;
#[doc = "`read()` method returns [usbcnf::R](usbcnf::R) reader structure"]
impl crate::Readable for USBCNF {}
#[doc = "`write(|w| ..)` method takes [usbcnf::W](usbcnf::W) writer structure"]
impl crate::Writable for USBCNF {}
#[doc = "USB Module configuration register"]
pub mod usbcnf;
#[doc = "USB PHY control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbphyctl](usbphyctl) module"]
pub type USBPHYCTL = crate::Reg<u16, _USBPHYCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPHYCTL;
#[doc = "`read()` method returns [usbphyctl::R](usbphyctl::R) reader structure"]
impl crate::Readable for USBPHYCTL {}
#[doc = "`write(|w| ..)` method takes [usbphyctl::W](usbphyctl::W) writer structure"]
impl crate::Writable for USBPHYCTL {}
#[doc = "USB PHY control register"]
pub mod usbphyctl;
#[doc = "USB Power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpwrctl](usbpwrctl) module"]
pub type USBPWRCTL = crate::Reg<u16, _USBPWRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPWRCTL;
#[doc = "`read()` method returns [usbpwrctl::R](usbpwrctl::R) reader structure"]
impl crate::Readable for USBPWRCTL {}
#[doc = "`write(|w| ..)` method takes [usbpwrctl::W](usbpwrctl::W) writer structure"]
impl crate::Writable for USBPWRCTL {}
#[doc = "USB Power control register"]
pub mod usbpwrctl;
#[doc = "USB PLL control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpllctl](usbpllctl) module"]
pub type USBPLLCTL = crate::Reg<u16, _USBPLLCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPLLCTL;
#[doc = "`read()` method returns [usbpllctl::R](usbpllctl::R) reader structure"]
impl crate::Readable for USBPLLCTL {}
#[doc = "`write(|w| ..)` method takes [usbpllctl::W](usbpllctl::W) writer structure"]
impl crate::Writable for USBPLLCTL {}
#[doc = "USB PLL control register"]
pub mod usbpllctl;
#[doc = "USB PLL Clock Divider Buffer control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbplldivb](usbplldivb) module"]
pub type USBPLLDIVB = crate::Reg<u16, _USBPLLDIVB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPLLDIVB;
#[doc = "`read()` method returns [usbplldivb::R](usbplldivb::R) reader structure"]
impl crate::Readable for USBPLLDIVB {}
#[doc = "`write(|w| ..)` method takes [usbplldivb::W](usbplldivb::W) writer structure"]
impl crate::Writable for USBPLLDIVB {}
#[doc = "USB PLL Clock Divider Buffer control register"]
pub mod usbplldivb;
#[doc = "USB PLL Interrupt control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpllir](usbpllir) module"]
pub type USBPLLIR = crate::Reg<u16, _USBPLLIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPLLIR;
#[doc = "`read()` method returns [usbpllir::R](usbpllir::R) reader structure"]
impl crate::Readable for USBPLLIR {}
#[doc = "`write(|w| ..)` method takes [usbpllir::W](usbpllir::W) writer structure"]
impl crate::Writable for USBPLLIR {}
#[doc = "USB PLL Interrupt control register"]
pub mod usbpllir;
#[doc = "USB Vector interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbvecint](usbvecint) module"]
pub type USBVECINT = crate::Reg<u16, _USBVECINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBVECINT;
#[doc = "`read()` method returns [usbvecint::R](usbvecint::R) reader structure"]
impl crate::Readable for USBVECINT {}
#[doc = "`write(|w| ..)` method takes [usbvecint::W](usbvecint::W) writer structure"]
impl crate::Writable for USBVECINT {}
#[doc = "USB Vector interrupt register"]
pub mod usbvecint;
#[doc = "USB maintenance register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbmaint](usbmaint) module"]
pub type USBMAINT = crate::Reg<u16, _USBMAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBMAINT;
#[doc = "`read()` method returns [usbmaint::R](usbmaint::R) reader structure"]
impl crate::Readable for USBMAINT {}
#[doc = "`write(|w| ..)` method takes [usbmaint::W](usbmaint::W) writer structure"]
impl crate::Writable for USBMAINT {}
#[doc = "USB maintenance register"]
pub mod usbmaint;
#[doc = "USB Time Stamp register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbtsreg](usbtsreg) module"]
pub type USBTSREG = crate::Reg<u16, _USBTSREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBTSREG;
#[doc = "`read()` method returns [usbtsreg::R](usbtsreg::R) reader structure"]
impl crate::Readable for USBTSREG {}
#[doc = "`write(|w| ..)` method takes [usbtsreg::W](usbtsreg::W) writer structure"]
impl crate::Writable for USBTSREG {}
#[doc = "USB Time Stamp register"]
pub mod usbtsreg;
#[doc = "USB Frame number\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbfn](usbfn) module"]
pub type USBFN = crate::Reg<u16, _USBFN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBFN;
#[doc = "`read()` method returns [usbfn::R](usbfn::R) reader structure"]
impl crate::Readable for USBFN {}
#[doc = "`write(|w| ..)` method takes [usbfn::W](usbfn::W) writer structure"]
impl crate::Writable for USBFN {}
#[doc = "USB Frame number"]
pub mod usbfn;
