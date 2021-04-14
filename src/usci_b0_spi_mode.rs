#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI B0 Control Register 1"]
    pub ucb0ctl1_spi: crate::Reg<ucb0ctl1_spi::UCB0CTL1_SPI_SPEC>,
    #[doc = "0x01 - USCI B0 Control Register 0"]
    pub ucb0ctl0_spi: crate::Reg<ucb0ctl0_spi::UCB0CTL0_SPI_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x06 - USCI B0 Baud Rate 0"]
    pub ucb0br0_spi: crate::Reg<ucb0br0_spi::UCB0BR0_SPI_SPEC>,
    #[doc = "0x07 - USCI B0 Baud Rate 1"]
    pub ucb0br1_spi: crate::Reg<ucb0br1_spi::UCB0BR1_SPI_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x0a - USCI B0 Status Register"]
    pub ucb0stat_spi: crate::Reg<ucb0stat_spi::UCB0STAT_SPI_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x0c - USCI B0 Receive Buffer"]
    pub ucb0rxbuf_spi: crate::Reg<ucb0rxbuf_spi::UCB0RXBUF_SPI_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x0e - USCI B0 Transmit Buffer"]
    pub ucb0txbuf_spi: crate::Reg<ucb0txbuf_spi::UCB0TXBUF_SPI_SPEC>,
    _reserved7: [u8; 0x0d],
    #[doc = "0x1c - USCI B0 Interrupt Enable Register"]
    pub ucb0ie_spi: crate::Reg<ucb0ie_spi::UCB0IE_SPI_SPEC>,
    #[doc = "0x1d - USCI B0 Interrupt Flags Register"]
    pub ucb0ifg_spi: crate::Reg<ucb0ifg_spi::UCB0IFG_SPI_SPEC>,
    #[doc = "0x1e - USCI B0 Interrupt Vector Register"]
    pub ucb0iv_spi: crate::Reg<ucb0iv_spi::UCB0IV_SPI_SPEC>,
}
#[doc = "UCB0CTL1_SPI register accessor: an alias for `Reg<UCB0CTL1_SPI_SPEC>`"]
pub type UCB0CTL1_SPI = crate::Reg<ucb0ctl1_spi::UCB0CTL1_SPI_SPEC>;
#[doc = "USCI B0 Control Register 1"]
pub mod ucb0ctl1_spi;
#[doc = "UCB0CTL0_SPI register accessor: an alias for `Reg<UCB0CTL0_SPI_SPEC>`"]
pub type UCB0CTL0_SPI = crate::Reg<ucb0ctl0_spi::UCB0CTL0_SPI_SPEC>;
#[doc = "USCI B0 Control Register 0"]
pub mod ucb0ctl0_spi;
#[doc = "UCB0BR0_SPI register accessor: an alias for `Reg<UCB0BR0_SPI_SPEC>`"]
pub type UCB0BR0_SPI = crate::Reg<ucb0br0_spi::UCB0BR0_SPI_SPEC>;
#[doc = "USCI B0 Baud Rate 0"]
pub mod ucb0br0_spi;
#[doc = "UCB0BR1_SPI register accessor: an alias for `Reg<UCB0BR1_SPI_SPEC>`"]
pub type UCB0BR1_SPI = crate::Reg<ucb0br1_spi::UCB0BR1_SPI_SPEC>;
#[doc = "USCI B0 Baud Rate 1"]
pub mod ucb0br1_spi;
#[doc = "UCB0STAT_SPI register accessor: an alias for `Reg<UCB0STAT_SPI_SPEC>`"]
pub type UCB0STAT_SPI = crate::Reg<ucb0stat_spi::UCB0STAT_SPI_SPEC>;
#[doc = "USCI B0 Status Register"]
pub mod ucb0stat_spi;
#[doc = "UCB0RXBUF_SPI register accessor: an alias for `Reg<UCB0RXBUF_SPI_SPEC>`"]
pub type UCB0RXBUF_SPI = crate::Reg<ucb0rxbuf_spi::UCB0RXBUF_SPI_SPEC>;
#[doc = "USCI B0 Receive Buffer"]
pub mod ucb0rxbuf_spi;
#[doc = "UCB0TXBUF_SPI register accessor: an alias for `Reg<UCB0TXBUF_SPI_SPEC>`"]
pub type UCB0TXBUF_SPI = crate::Reg<ucb0txbuf_spi::UCB0TXBUF_SPI_SPEC>;
#[doc = "USCI B0 Transmit Buffer"]
pub mod ucb0txbuf_spi;
#[doc = "UCB0IE_SPI register accessor: an alias for `Reg<UCB0IE_SPI_SPEC>`"]
pub type UCB0IE_SPI = crate::Reg<ucb0ie_spi::UCB0IE_SPI_SPEC>;
#[doc = "USCI B0 Interrupt Enable Register"]
pub mod ucb0ie_spi;
#[doc = "UCB0IFG_SPI register accessor: an alias for `Reg<UCB0IFG_SPI_SPEC>`"]
pub type UCB0IFG_SPI = crate::Reg<ucb0ifg_spi::UCB0IFG_SPI_SPEC>;
#[doc = "USCI B0 Interrupt Flags Register"]
pub mod ucb0ifg_spi;
#[doc = "UCB0IV_SPI register accessor: an alias for `Reg<UCB0IV_SPI_SPEC>`"]
pub type UCB0IV_SPI = crate::Reg<ucb0iv_spi::UCB0IV_SPI_SPEC>;
#[doc = "USCI B0 Interrupt Vector Register"]
pub mod ucb0iv_spi;
