#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ucb0ctl1_spi: UCB0CTL1_SPI,
    ucb0ctl0_spi: UCB0CTL0_SPI,
    _reserved2: [u8; 0x04],
    ucb0br0_spi: UCB0BR0_SPI,
    ucb0br1_spi: UCB0BR1_SPI,
    _reserved4: [u8; 0x02],
    ucb0stat_spi: UCB0STAT_SPI,
    _reserved5: [u8; 0x01],
    ucb0rxbuf_spi: UCB0RXBUF_SPI,
    _reserved6: [u8; 0x01],
    ucb0txbuf_spi: UCB0TXBUF_SPI,
    _reserved7: [u8; 0x0d],
    ucb0ie_spi: UCB0IE_SPI,
    ucb0ifg_spi: UCB0IFG_SPI,
    ucb0iv_spi: UCB0IV_SPI,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI B0 Control Register 1"]
    #[inline(always)]
    pub const fn ucb0ctl1_spi(&self) -> &UCB0CTL1_SPI {
        &self.ucb0ctl1_spi
    }
    #[doc = "0x01 - USCI B0 Control Register 0"]
    #[inline(always)]
    pub const fn ucb0ctl0_spi(&self) -> &UCB0CTL0_SPI {
        &self.ucb0ctl0_spi
    }
    #[doc = "0x06 - USCI B0 Baud Rate 0"]
    #[inline(always)]
    pub const fn ucb0br0_spi(&self) -> &UCB0BR0_SPI {
        &self.ucb0br0_spi
    }
    #[doc = "0x07 - USCI B0 Baud Rate 1"]
    #[inline(always)]
    pub const fn ucb0br1_spi(&self) -> &UCB0BR1_SPI {
        &self.ucb0br1_spi
    }
    #[doc = "0x0a - USCI B0 Status Register"]
    #[inline(always)]
    pub const fn ucb0stat_spi(&self) -> &UCB0STAT_SPI {
        &self.ucb0stat_spi
    }
    #[doc = "0x0c - USCI B0 Receive Buffer"]
    #[inline(always)]
    pub const fn ucb0rxbuf_spi(&self) -> &UCB0RXBUF_SPI {
        &self.ucb0rxbuf_spi
    }
    #[doc = "0x0e - USCI B0 Transmit Buffer"]
    #[inline(always)]
    pub const fn ucb0txbuf_spi(&self) -> &UCB0TXBUF_SPI {
        &self.ucb0txbuf_spi
    }
    #[doc = "0x1c - USCI B0 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucb0ie_spi(&self) -> &UCB0IE_SPI {
        &self.ucb0ie_spi
    }
    #[doc = "0x1d - USCI B0 Interrupt Flags Register"]
    #[inline(always)]
    pub const fn ucb0ifg_spi(&self) -> &UCB0IFG_SPI {
        &self.ucb0ifg_spi
    }
    #[doc = "0x1e - USCI B0 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ucb0iv_spi(&self) -> &UCB0IV_SPI {
        &self.ucb0iv_spi
    }
}
#[doc = "UCB0CTL1_SPI (rw) register accessor: USCI B0 Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0ctl1_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0ctl1_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctl1_spi`]
module"]
pub type UCB0CTL1_SPI = crate::Reg<ucb0ctl1_spi::UCB0CTL1_SPI_SPEC>;
#[doc = "USCI B0 Control Register 1"]
pub mod ucb0ctl1_spi;
#[doc = "UCB0CTL0_SPI (rw) register accessor: USCI B0 Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0ctl0_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0ctl0_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctl0_spi`]
module"]
pub type UCB0CTL0_SPI = crate::Reg<ucb0ctl0_spi::UCB0CTL0_SPI_SPEC>;
#[doc = "USCI B0 Control Register 0"]
pub mod ucb0ctl0_spi;
#[doc = "UCB0BR0_SPI (rw) register accessor: USCI B0 Baud Rate 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0br0_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0br0_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0br0_spi`]
module"]
pub type UCB0BR0_SPI = crate::Reg<ucb0br0_spi::UCB0BR0_SPI_SPEC>;
#[doc = "USCI B0 Baud Rate 0"]
pub mod ucb0br0_spi;
#[doc = "UCB0BR1_SPI (rw) register accessor: USCI B0 Baud Rate 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0br1_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0br1_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0br1_spi`]
module"]
pub type UCB0BR1_SPI = crate::Reg<ucb0br1_spi::UCB0BR1_SPI_SPEC>;
#[doc = "USCI B0 Baud Rate 1"]
pub mod ucb0br1_spi;
#[doc = "UCB0STAT_SPI (rw) register accessor: USCI B0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0stat_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0stat_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0stat_spi`]
module"]
pub type UCB0STAT_SPI = crate::Reg<ucb0stat_spi::UCB0STAT_SPI_SPEC>;
#[doc = "USCI B0 Status Register"]
pub mod ucb0stat_spi;
#[doc = "UCB0RXBUF_SPI (rw) register accessor: USCI B0 Receive Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0rxbuf_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0rxbuf_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0rxbuf_spi`]
module"]
pub type UCB0RXBUF_SPI = crate::Reg<ucb0rxbuf_spi::UCB0RXBUF_SPI_SPEC>;
#[doc = "USCI B0 Receive Buffer"]
pub mod ucb0rxbuf_spi;
#[doc = "UCB0TXBUF_SPI (rw) register accessor: USCI B0 Transmit Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0txbuf_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0txbuf_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0txbuf_spi`]
module"]
pub type UCB0TXBUF_SPI = crate::Reg<ucb0txbuf_spi::UCB0TXBUF_SPI_SPEC>;
#[doc = "USCI B0 Transmit Buffer"]
pub mod ucb0txbuf_spi;
#[doc = "UCB0IE_SPI (rw) register accessor: USCI B0 Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0ie_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0ie_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ie_spi`]
module"]
pub type UCB0IE_SPI = crate::Reg<ucb0ie_spi::UCB0IE_SPI_SPEC>;
#[doc = "USCI B0 Interrupt Enable Register"]
pub mod ucb0ie_spi;
#[doc = "UCB0IFG_SPI (rw) register accessor: USCI B0 Interrupt Flags Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0ifg_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0ifg_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ifg_spi`]
module"]
pub type UCB0IFG_SPI = crate::Reg<ucb0ifg_spi::UCB0IFG_SPI_SPEC>;
#[doc = "USCI B0 Interrupt Flags Register"]
pub mod ucb0ifg_spi;
#[doc = "UCB0IV_SPI (rw) register accessor: USCI B0 Interrupt Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0iv_spi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0iv_spi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0iv_spi`]
module"]
pub type UCB0IV_SPI = crate::Reg<ucb0iv_spi::UCB0IV_SPI_SPEC>;
#[doc = "USCI B0 Interrupt Vector Register"]
pub mod ucb0iv_spi;
