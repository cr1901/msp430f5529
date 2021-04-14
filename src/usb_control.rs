#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB Controller key register"]
    pub usbkeyid: crate::Reg<usbkeyid::USBKEYID_SPEC>,
    #[doc = "0x02 - USB Module configuration register"]
    pub usbcnf: crate::Reg<usbcnf::USBCNF_SPEC>,
    #[doc = "0x04 - USB PHY control register"]
    pub usbphyctl: crate::Reg<usbphyctl::USBPHYCTL_SPEC>,
    _reserved3: [u8; 0x02],
    #[doc = "0x08 - USB Power control register"]
    pub usbpwrctl: crate::Reg<usbpwrctl::USBPWRCTL_SPEC>,
    _reserved4: [u8; 0x06],
    #[doc = "0x10 - USB PLL control register"]
    pub usbpllctl: crate::Reg<usbpllctl::USBPLLCTL_SPEC>,
    #[doc = "0x12 - USB PLL Clock Divider Buffer control register"]
    pub usbplldivb: crate::Reg<usbplldivb::USBPLLDIVB_SPEC>,
    #[doc = "0x14 - USB PLL Interrupt control register"]
    pub usbpllir: crate::Reg<usbpllir::USBPLLIR_SPEC>,
    _reserved7: [u8; 0x0a],
    #[doc = "0x20 - USB Input endpoint_0: Configuration"]
    pub usbiepcnf_0: crate::Reg<usbiepcnf_0::USBIEPCNF_0_SPEC>,
    #[doc = "0x21 - USB Input endpoint_0: Byte Count"]
    pub usbiepcnt_0: crate::Reg<usbiepcnt_0::USBIEPCNT_0_SPEC>,
    #[doc = "0x22 - USB Output endpoint_0: Configuration"]
    pub usboepcnf_0: crate::Reg<usboepcnf_0::USBOEPCNF_0_SPEC>,
    #[doc = "0x23 - USB Output endpoint_0: byte count"]
    pub usboepcnt_0: crate::Reg<usboepcnt_0::USBOEPCNT_0_SPEC>,
    _reserved11: [u8; 0x0a],
    #[doc = "0x2e - USB Input endpoint interrupt enable flags"]
    pub usbiepie: crate::Reg<usbiepie::USBIEPIE_SPEC>,
    #[doc = "0x2f - USB Output endpoint interrupt enable flags"]
    pub usboepie: crate::Reg<usboepie::USBOEPIE_SPEC>,
    #[doc = "0x30 - USB Input endpoint interrupt flags"]
    pub usbiepifg: crate::Reg<usbiepifg::USBIEPIFG_SPEC>,
    #[doc = "0x31 - USB Output endpoint interrupt flags"]
    pub usboepifg: crate::Reg<usboepifg::USBOEPIFG_SPEC>,
    #[doc = "0x32 - USB Vector interrupt register"]
    pub usbvecint: crate::Reg<usbvecint::USBVECINT_SPEC>,
    _reserved16: [u8; 0x02],
    #[doc = "0x36 - USB maintenance register"]
    pub usbmaint: crate::Reg<usbmaint::USBMAINT_SPEC>,
    #[doc = "0x38 - USB Time Stamp register"]
    pub usbtsreg: crate::Reg<usbtsreg::USBTSREG_SPEC>,
    #[doc = "0x3a - USB Frame number"]
    pub usbfn: crate::Reg<usbfn::USBFN_SPEC>,
    #[doc = "0x3c - USB control register"]
    pub usbctl: crate::Reg<usbctl::USBCTL_SPEC>,
    #[doc = "0x3d - USB interrupt enable register"]
    pub usbie: crate::Reg<usbie::USBIE_SPEC>,
    #[doc = "0x3e - USB interrupt flag register"]
    pub usbifg: crate::Reg<usbifg::USBIFG_SPEC>,
    #[doc = "0x3f - USB Function address register"]
    pub usbfunadr: crate::Reg<usbfunadr::USBFUNADR_SPEC>,
}
#[doc = "USBIEPCNF_0 register accessor: an alias for `Reg<USBIEPCNF_0_SPEC>`"]
pub type USBIEPCNF_0 = crate::Reg<usbiepcnf_0::USBIEPCNF_0_SPEC>;
#[doc = "USB Input endpoint_0: Configuration"]
pub mod usbiepcnf_0;
#[doc = "USBIEPCNT_0 register accessor: an alias for `Reg<USBIEPCNT_0_SPEC>`"]
pub type USBIEPCNT_0 = crate::Reg<usbiepcnt_0::USBIEPCNT_0_SPEC>;
#[doc = "USB Input endpoint_0: Byte Count"]
pub mod usbiepcnt_0;
#[doc = "USBOEPCNF_0 register accessor: an alias for `Reg<USBOEPCNF_0_SPEC>`"]
pub type USBOEPCNF_0 = crate::Reg<usboepcnf_0::USBOEPCNF_0_SPEC>;
#[doc = "USB Output endpoint_0: Configuration"]
pub mod usboepcnf_0;
#[doc = "USBOEPCNT_0 register accessor: an alias for `Reg<USBOEPCNT_0_SPEC>`"]
pub type USBOEPCNT_0 = crate::Reg<usboepcnt_0::USBOEPCNT_0_SPEC>;
#[doc = "USB Output endpoint_0: byte count"]
pub mod usboepcnt_0;
#[doc = "USBIEPIE register accessor: an alias for `Reg<USBIEPIE_SPEC>`"]
pub type USBIEPIE = crate::Reg<usbiepie::USBIEPIE_SPEC>;
#[doc = "USB Input endpoint interrupt enable flags"]
pub mod usbiepie;
#[doc = "USBOEPIE register accessor: an alias for `Reg<USBOEPIE_SPEC>`"]
pub type USBOEPIE = crate::Reg<usboepie::USBOEPIE_SPEC>;
#[doc = "USB Output endpoint interrupt enable flags"]
pub mod usboepie;
#[doc = "USBIEPIFG register accessor: an alias for `Reg<USBIEPIFG_SPEC>`"]
pub type USBIEPIFG = crate::Reg<usbiepifg::USBIEPIFG_SPEC>;
#[doc = "USB Input endpoint interrupt flags"]
pub mod usbiepifg;
#[doc = "USBOEPIFG register accessor: an alias for `Reg<USBOEPIFG_SPEC>`"]
pub type USBOEPIFG = crate::Reg<usboepifg::USBOEPIFG_SPEC>;
#[doc = "USB Output endpoint interrupt flags"]
pub mod usboepifg;
#[doc = "USBCTL register accessor: an alias for `Reg<USBCTL_SPEC>`"]
pub type USBCTL = crate::Reg<usbctl::USBCTL_SPEC>;
#[doc = "USB control register"]
pub mod usbctl;
#[doc = "USBIE register accessor: an alias for `Reg<USBIE_SPEC>`"]
pub type USBIE = crate::Reg<usbie::USBIE_SPEC>;
#[doc = "USB interrupt enable register"]
pub mod usbie;
#[doc = "USBIFG register accessor: an alias for `Reg<USBIFG_SPEC>`"]
pub type USBIFG = crate::Reg<usbifg::USBIFG_SPEC>;
#[doc = "USB interrupt flag register"]
pub mod usbifg;
#[doc = "USBFUNADR register accessor: an alias for `Reg<USBFUNADR_SPEC>`"]
pub type USBFUNADR = crate::Reg<usbfunadr::USBFUNADR_SPEC>;
#[doc = "USB Function address register"]
pub mod usbfunadr;
#[doc = "USBKEYID register accessor: an alias for `Reg<USBKEYID_SPEC>`"]
pub type USBKEYID = crate::Reg<usbkeyid::USBKEYID_SPEC>;
#[doc = "USB Controller key register"]
pub mod usbkeyid;
#[doc = "USBCNF register accessor: an alias for `Reg<USBCNF_SPEC>`"]
pub type USBCNF = crate::Reg<usbcnf::USBCNF_SPEC>;
#[doc = "USB Module configuration register"]
pub mod usbcnf;
#[doc = "USBPHYCTL register accessor: an alias for `Reg<USBPHYCTL_SPEC>`"]
pub type USBPHYCTL = crate::Reg<usbphyctl::USBPHYCTL_SPEC>;
#[doc = "USB PHY control register"]
pub mod usbphyctl;
#[doc = "USBPWRCTL register accessor: an alias for `Reg<USBPWRCTL_SPEC>`"]
pub type USBPWRCTL = crate::Reg<usbpwrctl::USBPWRCTL_SPEC>;
#[doc = "USB Power control register"]
pub mod usbpwrctl;
#[doc = "USBPLLCTL register accessor: an alias for `Reg<USBPLLCTL_SPEC>`"]
pub type USBPLLCTL = crate::Reg<usbpllctl::USBPLLCTL_SPEC>;
#[doc = "USB PLL control register"]
pub mod usbpllctl;
#[doc = "USBPLLDIVB register accessor: an alias for `Reg<USBPLLDIVB_SPEC>`"]
pub type USBPLLDIVB = crate::Reg<usbplldivb::USBPLLDIVB_SPEC>;
#[doc = "USB PLL Clock Divider Buffer control register"]
pub mod usbplldivb;
#[doc = "USBPLLIR register accessor: an alias for `Reg<USBPLLIR_SPEC>`"]
pub type USBPLLIR = crate::Reg<usbpllir::USBPLLIR_SPEC>;
#[doc = "USB PLL Interrupt control register"]
pub mod usbpllir;
#[doc = "USBVECINT register accessor: an alias for `Reg<USBVECINT_SPEC>`"]
pub type USBVECINT = crate::Reg<usbvecint::USBVECINT_SPEC>;
#[doc = "USB Vector interrupt register"]
pub mod usbvecint;
#[doc = "USBMAINT register accessor: an alias for `Reg<USBMAINT_SPEC>`"]
pub type USBMAINT = crate::Reg<usbmaint::USBMAINT_SPEC>;
#[doc = "USB maintenance register"]
pub mod usbmaint;
#[doc = "USBTSREG register accessor: an alias for `Reg<USBTSREG_SPEC>`"]
pub type USBTSREG = crate::Reg<usbtsreg::USBTSREG_SPEC>;
#[doc = "USB Time Stamp register"]
pub mod usbtsreg;
#[doc = "USBFN register accessor: an alias for `Reg<USBFN_SPEC>`"]
pub type USBFN = crate::Reg<usbfn::USBFN_SPEC>;
#[doc = "USB Frame number"]
pub mod usbfn;
