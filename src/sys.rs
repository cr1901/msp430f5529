#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    sysctl: SYSCTL,
    sysbslc: SYSBSLC,
    _reserved2: [u8; 0x02],
    sysjmbc: SYSJMBC,
    sysjmbi0: SYSJMBI0,
    sysjmbi1: SYSJMBI1,
    sysjmbo0: SYSJMBO0,
    sysjmbo1: SYSJMBO1,
    _reserved7: [u8; 0x08],
    sysberriv: SYSBERRIV,
    sysuniv: SYSUNIV,
    syssniv: SYSSNIV,
    sysrstiv: SYSRSTIV,
}
impl RegisterBlock {
    #[doc = "0x00 - System control"]
    #[inline(always)]
    pub const fn sysctl(&self) -> &SYSCTL {
        &self.sysctl
    }
    #[doc = "0x02 - Boot strap configuration area"]
    #[inline(always)]
    pub const fn sysbslc(&self) -> &SYSBSLC {
        &self.sysbslc
    }
    #[doc = "0x06 - JTAG mailbox control"]
    #[inline(always)]
    pub const fn sysjmbc(&self) -> &SYSJMBC {
        &self.sysjmbc
    }
    #[doc = "0x08 - JTAG mailbox input 0"]
    #[inline(always)]
    pub const fn sysjmbi0(&self) -> &SYSJMBI0 {
        &self.sysjmbi0
    }
    #[doc = "0x0a - JTAG mailbox input 1"]
    #[inline(always)]
    pub const fn sysjmbi1(&self) -> &SYSJMBI1 {
        &self.sysjmbi1
    }
    #[doc = "0x0c - JTAG mailbox output 0"]
    #[inline(always)]
    pub const fn sysjmbo0(&self) -> &SYSJMBO0 {
        &self.sysjmbo0
    }
    #[doc = "0x0e - JTAG mailbox output 1"]
    #[inline(always)]
    pub const fn sysjmbo1(&self) -> &SYSJMBO1 {
        &self.sysjmbo1
    }
    #[doc = "0x18 - Bus Error vector generator"]
    #[inline(always)]
    pub const fn sysberriv(&self) -> &SYSBERRIV {
        &self.sysberriv
    }
    #[doc = "0x1a - User NMI vector generator"]
    #[inline(always)]
    pub const fn sysuniv(&self) -> &SYSUNIV {
        &self.sysuniv
    }
    #[doc = "0x1c - System NMI vector generator"]
    #[inline(always)]
    pub const fn syssniv(&self) -> &SYSSNIV {
        &self.syssniv
    }
    #[doc = "0x1e - Reset vector generator"]
    #[inline(always)]
    pub const fn sysrstiv(&self) -> &SYSRSTIV {
        &self.sysrstiv
    }
}
#[doc = "SYSCTL (rw) register accessor: System control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctl`]
module"]
pub type SYSCTL = crate::Reg<sysctl::SYSCTL_SPEC>;
#[doc = "System control"]
pub mod sysctl;
#[doc = "SYSBSLC (rw) register accessor: Boot strap configuration area\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysbslc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysbslc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysbslc`]
module"]
pub type SYSBSLC = crate::Reg<sysbslc::SYSBSLC_SPEC>;
#[doc = "Boot strap configuration area"]
pub mod sysbslc;
#[doc = "SYSJMBC (rw) register accessor: JTAG mailbox control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysjmbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysjmbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysjmbc`]
module"]
pub type SYSJMBC = crate::Reg<sysjmbc::SYSJMBC_SPEC>;
#[doc = "JTAG mailbox control"]
pub mod sysjmbc;
#[doc = "SYSJMBI0 (rw) register accessor: JTAG mailbox input 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysjmbi0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysjmbi0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysjmbi0`]
module"]
pub type SYSJMBI0 = crate::Reg<sysjmbi0::SYSJMBI0_SPEC>;
#[doc = "JTAG mailbox input 0"]
pub mod sysjmbi0;
#[doc = "SYSJMBI1 (rw) register accessor: JTAG mailbox input 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysjmbi1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysjmbi1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysjmbi1`]
module"]
pub type SYSJMBI1 = crate::Reg<sysjmbi1::SYSJMBI1_SPEC>;
#[doc = "JTAG mailbox input 1"]
pub mod sysjmbi1;
#[doc = "SYSJMBO0 (rw) register accessor: JTAG mailbox output 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysjmbo0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysjmbo0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysjmbo0`]
module"]
pub type SYSJMBO0 = crate::Reg<sysjmbo0::SYSJMBO0_SPEC>;
#[doc = "JTAG mailbox output 0"]
pub mod sysjmbo0;
#[doc = "SYSJMBO1 (rw) register accessor: JTAG mailbox output 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysjmbo1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysjmbo1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysjmbo1`]
module"]
pub type SYSJMBO1 = crate::Reg<sysjmbo1::SYSJMBO1_SPEC>;
#[doc = "JTAG mailbox output 1"]
pub mod sysjmbo1;
#[doc = "SYSBERRIV (rw) register accessor: Bus Error vector generator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysberriv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysberriv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysberriv`]
module"]
pub type SYSBERRIV = crate::Reg<sysberriv::SYSBERRIV_SPEC>;
#[doc = "Bus Error vector generator"]
pub mod sysberriv;
#[doc = "SYSUNIV (rw) register accessor: User NMI vector generator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysuniv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysuniv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysuniv`]
module"]
pub type SYSUNIV = crate::Reg<sysuniv::SYSUNIV_SPEC>;
#[doc = "User NMI vector generator"]
pub mod sysuniv;
#[doc = "SYSSNIV (rw) register accessor: System NMI vector generator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syssniv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syssniv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syssniv`]
module"]
pub type SYSSNIV = crate::Reg<syssniv::SYSSNIV_SPEC>;
#[doc = "System NMI vector generator"]
pub mod syssniv;
#[doc = "SYSRSTIV (rw) register accessor: Reset vector generator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysrstiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysrstiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysrstiv`]
module"]
pub type SYSRSTIV = crate::Reg<sysrstiv::SYSRSTIV_SPEC>;
#[doc = "Reset vector generator"]
pub mod sysrstiv;
