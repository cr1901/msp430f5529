#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI B1 Control Register 1"]
    pub ucb1ctl1_spi: crate::Reg<ucb1ctl1_spi::UCB1CTL1_SPI_SPEC>,
    #[doc = "0x01 - USCI B1 Control Register 0"]
    pub ucb1ctl0_spi: crate::Reg<ucb1ctl0_spi::UCB1CTL0_SPI_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x06 - USCI B1 Baud Rate 0"]
    pub ucb1br0_spi: crate::Reg<ucb1br0_spi::UCB1BR0_SPI_SPEC>,
    #[doc = "0x07 - USCI B1 Baud Rate 1"]
    pub ucb1br1_spi: crate::Reg<ucb1br1_spi::UCB1BR1_SPI_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x0a - USCI B1 Status Register"]
    pub ucb1stat_spi: crate::Reg<ucb1stat_spi::UCB1STAT_SPI_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x0c - USCI B1 Receive Buffer"]
    pub ucb1rxbuf_spi: crate::Reg<ucb1rxbuf_spi::UCB1RXBUF_SPI_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x0e - USCI B1 Transmit Buffer"]
    pub ucb1txbuf_spi: crate::Reg<ucb1txbuf_spi::UCB1TXBUF_SPI_SPEC>,
    _reserved7: [u8; 0x0d],
    #[doc = "0x1c - USCI B1 Interrupt Enable Register"]
    pub ucb1ie_spi: crate::Reg<ucb1ie_spi::UCB1IE_SPI_SPEC>,
    #[doc = "0x1d - USCI B1 Interrupt Flags Register"]
    pub ucb1ifg_spi: crate::Reg<ucb1ifg_spi::UCB1IFG_SPI_SPEC>,
    #[doc = "0x1e - USCI B1 Interrupt Vector Register"]
    pub ucb1iv_spi: crate::Reg<ucb1iv_spi::UCB1IV_SPI_SPEC>,
}
#[doc = "UCB1CTL1_SPI register accessor: an alias for `Reg<UCB1CTL1_SPI_SPEC>`"]
pub type UCB1CTL1_SPI = crate::Reg<ucb1ctl1_spi::UCB1CTL1_SPI_SPEC>;
#[doc = "USCI B1 Control Register 1"]
pub mod ucb1ctl1_spi;
#[doc = "UCB1CTL0_SPI register accessor: an alias for `Reg<UCB1CTL0_SPI_SPEC>`"]
pub type UCB1CTL0_SPI = crate::Reg<ucb1ctl0_spi::UCB1CTL0_SPI_SPEC>;
#[doc = "USCI B1 Control Register 0"]
pub mod ucb1ctl0_spi;
#[doc = "UCB1BR0_SPI register accessor: an alias for `Reg<UCB1BR0_SPI_SPEC>`"]
pub type UCB1BR0_SPI = crate::Reg<ucb1br0_spi::UCB1BR0_SPI_SPEC>;
#[doc = "USCI B1 Baud Rate 0"]
pub mod ucb1br0_spi;
#[doc = "UCB1BR1_SPI register accessor: an alias for `Reg<UCB1BR1_SPI_SPEC>`"]
pub type UCB1BR1_SPI = crate::Reg<ucb1br1_spi::UCB1BR1_SPI_SPEC>;
#[doc = "USCI B1 Baud Rate 1"]
pub mod ucb1br1_spi;
#[doc = "UCB1STAT_SPI register accessor: an alias for `Reg<UCB1STAT_SPI_SPEC>`"]
pub type UCB1STAT_SPI = crate::Reg<ucb1stat_spi::UCB1STAT_SPI_SPEC>;
#[doc = "USCI B1 Status Register"]
pub mod ucb1stat_spi;
#[doc = "UCB1RXBUF_SPI register accessor: an alias for `Reg<UCB1RXBUF_SPI_SPEC>`"]
pub type UCB1RXBUF_SPI = crate::Reg<ucb1rxbuf_spi::UCB1RXBUF_SPI_SPEC>;
#[doc = "USCI B1 Receive Buffer"]
pub mod ucb1rxbuf_spi;
#[doc = "UCB1TXBUF_SPI register accessor: an alias for `Reg<UCB1TXBUF_SPI_SPEC>`"]
pub type UCB1TXBUF_SPI = crate::Reg<ucb1txbuf_spi::UCB1TXBUF_SPI_SPEC>;
#[doc = "USCI B1 Transmit Buffer"]
pub mod ucb1txbuf_spi;
#[doc = "UCB1IE_SPI register accessor: an alias for `Reg<UCB1IE_SPI_SPEC>`"]
pub type UCB1IE_SPI = crate::Reg<ucb1ie_spi::UCB1IE_SPI_SPEC>;
#[doc = "USCI B1 Interrupt Enable Register"]
pub mod ucb1ie_spi;
#[doc = "UCB1IFG_SPI register accessor: an alias for `Reg<UCB1IFG_SPI_SPEC>`"]
pub type UCB1IFG_SPI = crate::Reg<ucb1ifg_spi::UCB1IFG_SPI_SPEC>;
#[doc = "USCI B1 Interrupt Flags Register"]
pub mod ucb1ifg_spi;
#[doc = "UCB1IV_SPI register accessor: an alias for `Reg<UCB1IV_SPI_SPEC>`"]
pub type UCB1IV_SPI = crate::Reg<ucb1iv_spi::UCB1IV_SPI_SPEC>;
#[doc = "USCI B1 Interrupt Vector Register"]
pub mod ucb1iv_spi;
