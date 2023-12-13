#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    uca1ctl1_spi: UCA1CTL1_SPI,
    uca1ctl0_spi: UCA1CTL0_SPI,
    _reserved2: [u8; 0x04],
    uca1br0_spi: UCA1BR0_SPI,
    uca1br1_spi: UCA1BR1_SPI,
    uca1mctl_spi: UCA1MCTL_SPI,
    _reserved5: [u8; 0x01],
    uca1stat_spi: UCA1STAT_SPI,
    _reserved6: [u8; 0x01],
    uca1rxbuf_spi: UCA1RXBUF_SPI,
    _reserved7: [u8; 0x01],
    uca1txbuf_spi: UCA1TXBUF_SPI,
    _reserved8: [u8; 0x0d],
    uca1ie_spi: UCA1IE_SPI,
    uca1ifg_spi: UCA1IFG_SPI,
    uca1iv_spi: UCA1IV_SPI,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI A1 Control Register 1"]
    #[inline(always)]
    pub const fn uca1ctl1_spi(&self) -> &UCA1CTL1_SPI {
        &self.uca1ctl1_spi
    }
    #[doc = "0x01 - USCI A1 Control Register 0"]
    #[inline(always)]
    pub const fn uca1ctl0_spi(&self) -> &UCA1CTL0_SPI {
        &self.uca1ctl0_spi
    }
    #[doc = "0x06 - USCI A1 Baud Rate 0"]
    #[inline(always)]
    pub const fn uca1br0_spi(&self) -> &UCA1BR0_SPI {
        &self.uca1br0_spi
    }
    #[doc = "0x07 - USCI A1 Baud Rate 1"]
    #[inline(always)]
    pub const fn uca1br1_spi(&self) -> &UCA1BR1_SPI {
        &self.uca1br1_spi
    }
    #[doc = "0x08 - USCI A1 Modulation Control"]
    #[inline(always)]
    pub const fn uca1mctl_spi(&self) -> &UCA1MCTL_SPI {
        &self.uca1mctl_spi
    }
    #[doc = "0x0a - USCI A1 Status Register"]
    #[inline(always)]
    pub const fn uca1stat_spi(&self) -> &UCA1STAT_SPI {
        &self.uca1stat_spi
    }
    #[doc = "0x0c - USCI A1 Receive Buffer"]
    #[inline(always)]
    pub const fn uca1rxbuf_spi(&self) -> &UCA1RXBUF_SPI {
        &self.uca1rxbuf_spi
    }
    #[doc = "0x0e - USCI A1 Transmit Buffer"]
    #[inline(always)]
    pub const fn uca1txbuf_spi(&self) -> &UCA1TXBUF_SPI {
        &self.uca1txbuf_spi
    }
    #[doc = "0x1c - USCI A1 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn uca1ie_spi(&self) -> &UCA1IE_SPI {
        &self.uca1ie_spi
    }
    #[doc = "0x1d - USCI A1 Interrupt Flags Register"]
    #[inline(always)]
    pub const fn uca1ifg_spi(&self) -> &UCA1IFG_SPI {
        &self.uca1ifg_spi
    }
    #[doc = "0x1e - USCI A1 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn uca1iv_spi(&self) -> &UCA1IV_SPI {
        &self.uca1iv_spi
    }
}
#[doc = "UCA1CTL1_SPI (rw) register accessor: USCI A1 Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1ctl1_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1ctl1_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ctl1_spi`]
module"]
pub type UCA1CTL1_SPI = crate::Reg<uca1ctl1_spi::UCA1CTL1_SPI_SPEC>;
#[doc = "USCI A1 Control Register 1"]
pub mod uca1ctl1_spi;
#[doc = "UCA1CTL0_SPI (rw) register accessor: USCI A1 Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1ctl0_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1ctl0_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ctl0_spi`]
module"]
pub type UCA1CTL0_SPI = crate::Reg<uca1ctl0_spi::UCA1CTL0_SPI_SPEC>;
#[doc = "USCI A1 Control Register 0"]
pub mod uca1ctl0_spi;
#[doc = "UCA1BR0_SPI (rw) register accessor: USCI A1 Baud Rate 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1br0_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1br0_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1br0_spi`]
module"]
pub type UCA1BR0_SPI = crate::Reg<uca1br0_spi::UCA1BR0_SPI_SPEC>;
#[doc = "USCI A1 Baud Rate 0"]
pub mod uca1br0_spi;
#[doc = "UCA1BR1_SPI (rw) register accessor: USCI A1 Baud Rate 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1br1_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1br1_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1br1_spi`]
module"]
pub type UCA1BR1_SPI = crate::Reg<uca1br1_spi::UCA1BR1_SPI_SPEC>;
#[doc = "USCI A1 Baud Rate 1"]
pub mod uca1br1_spi;
#[doc = "UCA1MCTL_SPI (rw) register accessor: USCI A1 Modulation Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1mctl_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1mctl_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1mctl_spi`]
module"]
pub type UCA1MCTL_SPI = crate::Reg<uca1mctl_spi::UCA1MCTL_SPI_SPEC>;
#[doc = "USCI A1 Modulation Control"]
pub mod uca1mctl_spi;
#[doc = "UCA1STAT_SPI (rw) register accessor: USCI A1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1stat_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1stat_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1stat_spi`]
module"]
pub type UCA1STAT_SPI = crate::Reg<uca1stat_spi::UCA1STAT_SPI_SPEC>;
#[doc = "USCI A1 Status Register"]
pub mod uca1stat_spi;
#[doc = "UCA1RXBUF_SPI (rw) register accessor: USCI A1 Receive Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1rxbuf_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1rxbuf_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1rxbuf_spi`]
module"]
pub type UCA1RXBUF_SPI = crate::Reg<uca1rxbuf_spi::UCA1RXBUF_SPI_SPEC>;
#[doc = "USCI A1 Receive Buffer"]
pub mod uca1rxbuf_spi;
#[doc = "UCA1TXBUF_SPI (rw) register accessor: USCI A1 Transmit Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1txbuf_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1txbuf_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1txbuf_spi`]
module"]
pub type UCA1TXBUF_SPI = crate::Reg<uca1txbuf_spi::UCA1TXBUF_SPI_SPEC>;
#[doc = "USCI A1 Transmit Buffer"]
pub mod uca1txbuf_spi;
#[doc = "UCA1IE_SPI (rw) register accessor: USCI A1 Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1ie_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1ie_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ie_spi`]
module"]
pub type UCA1IE_SPI = crate::Reg<uca1ie_spi::UCA1IE_SPI_SPEC>;
#[doc = "USCI A1 Interrupt Enable Register"]
pub mod uca1ie_spi;
#[doc = "UCA1IFG_SPI (rw) register accessor: USCI A1 Interrupt Flags Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1ifg_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1ifg_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ifg_spi`]
module"]
pub type UCA1IFG_SPI = crate::Reg<uca1ifg_spi::UCA1IFG_SPI_SPEC>;
#[doc = "USCI A1 Interrupt Flags Register"]
pub mod uca1ifg_spi;
#[doc = "UCA1IV_SPI (rw) register accessor: USCI A1 Interrupt Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1iv_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1iv_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1iv_spi`]
module"]
pub type UCA1IV_SPI = crate::Reg<uca1iv_spi::UCA1IV_SPI_SPEC>;
#[doc = "USCI A1 Interrupt Vector Register"]
pub mod uca1iv_spi;
