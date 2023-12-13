#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    usbkeyid: USBKEYID,
    usbcnf: USBCNF,
    usbphyctl: USBPHYCTL,
    _reserved3: [u8; 0x02],
    usbpwrctl: USBPWRCTL,
    _reserved4: [u8; 0x06],
    usbpllctl: USBPLLCTL,
    usbplldivb: USBPLLDIVB,
    usbpllir: USBPLLIR,
    _reserved7: [u8; 0x0a],
    usbiepcnf_0: USBIEPCNF_0,
    usbiepcnt_0: USBIEPCNT_0,
    usboepcnf_0: USBOEPCNF_0,
    usboepcnt_0: USBOEPCNT_0,
    _reserved11: [u8; 0x0a],
    usbiepie: USBIEPIE,
    usboepie: USBOEPIE,
    usbiepifg: USBIEPIFG,
    usboepifg: USBOEPIFG,
    usbvecint: USBVECINT,
    _reserved16: [u8; 0x02],
    usbmaint: USBMAINT,
    usbtsreg: USBTSREG,
    usbfn: USBFN,
    usbctl: USBCTL,
    usbie: USBIE,
    usbifg: USBIFG,
    usbfunadr: USBFUNADR,
}
impl RegisterBlock {
    #[doc = "0x00 - USB Controller key register"]
    #[inline(always)]
    pub const fn usbkeyid(&self) -> &USBKEYID {
        &self.usbkeyid
    }
    #[doc = "0x02 - USB Module configuration register"]
    #[inline(always)]
    pub const fn usbcnf(&self) -> &USBCNF {
        &self.usbcnf
    }
    #[doc = "0x04 - USB PHY control register"]
    #[inline(always)]
    pub const fn usbphyctl(&self) -> &USBPHYCTL {
        &self.usbphyctl
    }
    #[doc = "0x08 - USB Power control register"]
    #[inline(always)]
    pub const fn usbpwrctl(&self) -> &USBPWRCTL {
        &self.usbpwrctl
    }
    #[doc = "0x10 - USB PLL control register"]
    #[inline(always)]
    pub const fn usbpllctl(&self) -> &USBPLLCTL {
        &self.usbpllctl
    }
    #[doc = "0x12 - USB PLL Clock Divider Buffer control register"]
    #[inline(always)]
    pub const fn usbplldivb(&self) -> &USBPLLDIVB {
        &self.usbplldivb
    }
    #[doc = "0x14 - USB PLL Interrupt control register"]
    #[inline(always)]
    pub const fn usbpllir(&self) -> &USBPLLIR {
        &self.usbpllir
    }
    #[doc = "0x20 - USB Input endpoint_0: Configuration"]
    #[inline(always)]
    pub const fn usbiepcnf_0(&self) -> &USBIEPCNF_0 {
        &self.usbiepcnf_0
    }
    #[doc = "0x21 - USB Input endpoint_0: Byte Count"]
    #[inline(always)]
    pub const fn usbiepcnt_0(&self) -> &USBIEPCNT_0 {
        &self.usbiepcnt_0
    }
    #[doc = "0x22 - USB Output endpoint_0: Configuration"]
    #[inline(always)]
    pub const fn usboepcnf_0(&self) -> &USBOEPCNF_0 {
        &self.usboepcnf_0
    }
    #[doc = "0x23 - USB Output endpoint_0: byte count"]
    #[inline(always)]
    pub const fn usboepcnt_0(&self) -> &USBOEPCNT_0 {
        &self.usboepcnt_0
    }
    #[doc = "0x2e - USB Input endpoint interrupt enable flags"]
    #[inline(always)]
    pub const fn usbiepie(&self) -> &USBIEPIE {
        &self.usbiepie
    }
    #[doc = "0x2f - USB Output endpoint interrupt enable flags"]
    #[inline(always)]
    pub const fn usboepie(&self) -> &USBOEPIE {
        &self.usboepie
    }
    #[doc = "0x30 - USB Input endpoint interrupt flags"]
    #[inline(always)]
    pub const fn usbiepifg(&self) -> &USBIEPIFG {
        &self.usbiepifg
    }
    #[doc = "0x31 - USB Output endpoint interrupt flags"]
    #[inline(always)]
    pub const fn usboepifg(&self) -> &USBOEPIFG {
        &self.usboepifg
    }
    #[doc = "0x32 - USB Vector interrupt register"]
    #[inline(always)]
    pub const fn usbvecint(&self) -> &USBVECINT {
        &self.usbvecint
    }
    #[doc = "0x36 - USB maintenance register"]
    #[inline(always)]
    pub const fn usbmaint(&self) -> &USBMAINT {
        &self.usbmaint
    }
    #[doc = "0x38 - USB Time Stamp register"]
    #[inline(always)]
    pub const fn usbtsreg(&self) -> &USBTSREG {
        &self.usbtsreg
    }
    #[doc = "0x3a - USB Frame number"]
    #[inline(always)]
    pub const fn usbfn(&self) -> &USBFN {
        &self.usbfn
    }
    #[doc = "0x3c - USB control register"]
    #[inline(always)]
    pub const fn usbctl(&self) -> &USBCTL {
        &self.usbctl
    }
    #[doc = "0x3d - USB interrupt enable register"]
    #[inline(always)]
    pub const fn usbie(&self) -> &USBIE {
        &self.usbie
    }
    #[doc = "0x3e - USB interrupt flag register"]
    #[inline(always)]
    pub const fn usbifg(&self) -> &USBIFG {
        &self.usbifg
    }
    #[doc = "0x3f - USB Function address register"]
    #[inline(always)]
    pub const fn usbfunadr(&self) -> &USBFUNADR {
        &self.usbfunadr
    }
}
#[doc = "USBIEPCNF_0 (rw) register accessor: USB Input endpoint_0: Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepcnf_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepcnf_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepcnf_0`]
module"]
pub type USBIEPCNF_0 = crate::Reg<usbiepcnf_0::USBIEPCNF_0_SPEC>;
#[doc = "USB Input endpoint_0: Configuration"]
pub mod usbiepcnf_0;
#[doc = "USBIEPCNT_0 (rw) register accessor: USB Input endpoint_0: Byte Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepcnt_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepcnt_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepcnt_0`]
module"]
pub type USBIEPCNT_0 = crate::Reg<usbiepcnt_0::USBIEPCNT_0_SPEC>;
#[doc = "USB Input endpoint_0: Byte Count"]
pub mod usbiepcnt_0;
#[doc = "USBOEPCNF_0 (rw) register accessor: USB Output endpoint_0: Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepcnf_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepcnf_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepcnf_0`]
module"]
pub type USBOEPCNF_0 = crate::Reg<usboepcnf_0::USBOEPCNF_0_SPEC>;
#[doc = "USB Output endpoint_0: Configuration"]
pub mod usboepcnf_0;
#[doc = "USBOEPCNT_0 (rw) register accessor: USB Output endpoint_0: byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepcnt_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepcnt_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepcnt_0`]
module"]
pub type USBOEPCNT_0 = crate::Reg<usboepcnt_0::USBOEPCNT_0_SPEC>;
#[doc = "USB Output endpoint_0: byte count"]
pub mod usboepcnt_0;
#[doc = "USBIEPIE (rw) register accessor: USB Input endpoint interrupt enable flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepie`]
module"]
pub type USBIEPIE = crate::Reg<usbiepie::USBIEPIE_SPEC>;
#[doc = "USB Input endpoint interrupt enable flags"]
pub mod usbiepie;
#[doc = "USBOEPIE (rw) register accessor: USB Output endpoint interrupt enable flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepie`]
module"]
pub type USBOEPIE = crate::Reg<usboepie::USBOEPIE_SPEC>;
#[doc = "USB Output endpoint interrupt enable flags"]
pub mod usboepie;
#[doc = "USBIEPIFG (rw) register accessor: USB Input endpoint interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepifg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepifg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepifg`]
module"]
pub type USBIEPIFG = crate::Reg<usbiepifg::USBIEPIFG_SPEC>;
#[doc = "USB Input endpoint interrupt flags"]
pub mod usbiepifg;
#[doc = "USBOEPIFG (rw) register accessor: USB Output endpoint interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepifg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepifg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepifg`]
module"]
pub type USBOEPIFG = crate::Reg<usboepifg::USBOEPIFG_SPEC>;
#[doc = "USB Output endpoint interrupt flags"]
pub mod usboepifg;
#[doc = "USBCTL (rw) register accessor: USB control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbctl`]
module"]
pub type USBCTL = crate::Reg<usbctl::USBCTL_SPEC>;
#[doc = "USB control register"]
pub mod usbctl;
#[doc = "USBIE (rw) register accessor: USB interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbie`]
module"]
pub type USBIE = crate::Reg<usbie::USBIE_SPEC>;
#[doc = "USB interrupt enable register"]
pub mod usbie;
#[doc = "USBIFG (rw) register accessor: USB interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbifg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbifg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbifg`]
module"]
pub type USBIFG = crate::Reg<usbifg::USBIFG_SPEC>;
#[doc = "USB interrupt flag register"]
pub mod usbifg;
#[doc = "USBFUNADR (rw) register accessor: USB Function address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbfunadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbfunadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbfunadr`]
module"]
pub type USBFUNADR = crate::Reg<usbfunadr::USBFUNADR_SPEC>;
#[doc = "USB Function address register"]
pub mod usbfunadr;
#[doc = "USBKEYID (rw) register accessor: USB Controller key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbkeyid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbkeyid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbkeyid`]
module"]
pub type USBKEYID = crate::Reg<usbkeyid::USBKEYID_SPEC>;
#[doc = "USB Controller key register"]
pub mod usbkeyid;
#[doc = "USBCNF (rw) register accessor: USB Module configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbcnf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbcnf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbcnf`]
module"]
pub type USBCNF = crate::Reg<usbcnf::USBCNF_SPEC>;
#[doc = "USB Module configuration register"]
pub mod usbcnf;
#[doc = "USBPHYCTL (rw) register accessor: USB PHY control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphyctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphyctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphyctl`]
module"]
pub type USBPHYCTL = crate::Reg<usbphyctl::USBPHYCTL_SPEC>;
#[doc = "USB PHY control register"]
pub mod usbphyctl;
#[doc = "USBPWRCTL (rw) register accessor: USB Power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbpwrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbpwrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbpwrctl`]
module"]
pub type USBPWRCTL = crate::Reg<usbpwrctl::USBPWRCTL_SPEC>;
#[doc = "USB Power control register"]
pub mod usbpwrctl;
#[doc = "USBPLLCTL (rw) register accessor: USB PLL control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbpllctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbpllctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbpllctl`]
module"]
pub type USBPLLCTL = crate::Reg<usbpllctl::USBPLLCTL_SPEC>;
#[doc = "USB PLL control register"]
pub mod usbpllctl;
#[doc = "USBPLLDIVB (rw) register accessor: USB PLL Clock Divider Buffer control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbplldivb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbplldivb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbplldivb`]
module"]
pub type USBPLLDIVB = crate::Reg<usbplldivb::USBPLLDIVB_SPEC>;
#[doc = "USB PLL Clock Divider Buffer control register"]
pub mod usbplldivb;
#[doc = "USBPLLIR (rw) register accessor: USB PLL Interrupt control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbpllir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbpllir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbpllir`]
module"]
pub type USBPLLIR = crate::Reg<usbpllir::USBPLLIR_SPEC>;
#[doc = "USB PLL Interrupt control register"]
pub mod usbpllir;
#[doc = "USBVECINT (rw) register accessor: USB Vector interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbvecint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbvecint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbvecint`]
module"]
pub type USBVECINT = crate::Reg<usbvecint::USBVECINT_SPEC>;
#[doc = "USB Vector interrupt register"]
pub mod usbvecint;
#[doc = "USBMAINT (rw) register accessor: USB maintenance register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbmaint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbmaint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbmaint`]
module"]
pub type USBMAINT = crate::Reg<usbmaint::USBMAINT_SPEC>;
#[doc = "USB maintenance register"]
pub mod usbmaint;
#[doc = "USBTSREG (rw) register accessor: USB Time Stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbtsreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbtsreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbtsreg`]
module"]
pub type USBTSREG = crate::Reg<usbtsreg::USBTSREG_SPEC>;
#[doc = "USB Time Stamp register"]
pub mod usbtsreg;
#[doc = "USBFN (rw) register accessor: USB Frame number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbfn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbfn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbfn`]
module"]
pub type USBFN = crate::Reg<usbfn::USBFN_SPEC>;
#[doc = "USB Frame number"]
pub mod usbfn;
