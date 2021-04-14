#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI A0 Control Register 1"]
    pub uca0ctl1_spi: crate::Reg<uca0ctl1_spi::UCA0CTL1_SPI_SPEC>,
    #[doc = "0x01 - USCI A0 Control Register 0"]
    pub uca0ctl0_spi: crate::Reg<uca0ctl0_spi::UCA0CTL0_SPI_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x06 - USCI A0 Baud Rate 0"]
    pub uca0br0_spi: crate::Reg<uca0br0_spi::UCA0BR0_SPI_SPEC>,
    #[doc = "0x07 - USCI A0 Baud Rate 1"]
    pub uca0br1_spi: crate::Reg<uca0br1_spi::UCA0BR1_SPI_SPEC>,
    #[doc = "0x08 - USCI A0 Modulation Control"]
    pub uca0mctl_spi: crate::Reg<uca0mctl_spi::UCA0MCTL_SPI_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x0a - USCI A0 Status Register"]
    pub uca0stat_spi: crate::Reg<uca0stat_spi::UCA0STAT_SPI_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x0c - USCI A0 Receive Buffer"]
    pub uca0rxbuf_spi: crate::Reg<uca0rxbuf_spi::UCA0RXBUF_SPI_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x0e - USCI A0 Transmit Buffer"]
    pub uca0txbuf_spi: crate::Reg<uca0txbuf_spi::UCA0TXBUF_SPI_SPEC>,
    _reserved8: [u8; 0x0d],
    #[doc = "0x1c - USCI A0 Interrupt Enable Register"]
    pub uca0ie_spi: crate::Reg<uca0ie_spi::UCA0IE_SPI_SPEC>,
    #[doc = "0x1d - USCI A0 Interrupt Flags Register"]
    pub uca0ifg_spi: crate::Reg<uca0ifg_spi::UCA0IFG_SPI_SPEC>,
    #[doc = "0x1e - USCI A0 Interrupt Vector Register"]
    pub uca0iv_spi: crate::Reg<uca0iv_spi::UCA0IV_SPI_SPEC>,
}
#[doc = "UCA0CTL1_SPI register accessor: an alias for `Reg<UCA0CTL1_SPI_SPEC>`"]
pub type UCA0CTL1_SPI = crate::Reg<uca0ctl1_spi::UCA0CTL1_SPI_SPEC>;
#[doc = "USCI A0 Control Register 1"]
pub mod uca0ctl1_spi;
#[doc = "UCA0CTL0_SPI register accessor: an alias for `Reg<UCA0CTL0_SPI_SPEC>`"]
pub type UCA0CTL0_SPI = crate::Reg<uca0ctl0_spi::UCA0CTL0_SPI_SPEC>;
#[doc = "USCI A0 Control Register 0"]
pub mod uca0ctl0_spi;
#[doc = "UCA0BR0_SPI register accessor: an alias for `Reg<UCA0BR0_SPI_SPEC>`"]
pub type UCA0BR0_SPI = crate::Reg<uca0br0_spi::UCA0BR0_SPI_SPEC>;
#[doc = "USCI A0 Baud Rate 0"]
pub mod uca0br0_spi;
#[doc = "UCA0BR1_SPI register accessor: an alias for `Reg<UCA0BR1_SPI_SPEC>`"]
pub type UCA0BR1_SPI = crate::Reg<uca0br1_spi::UCA0BR1_SPI_SPEC>;
#[doc = "USCI A0 Baud Rate 1"]
pub mod uca0br1_spi;
#[doc = "UCA0MCTL_SPI register accessor: an alias for `Reg<UCA0MCTL_SPI_SPEC>`"]
pub type UCA0MCTL_SPI = crate::Reg<uca0mctl_spi::UCA0MCTL_SPI_SPEC>;
#[doc = "USCI A0 Modulation Control"]
pub mod uca0mctl_spi;
#[doc = "UCA0STAT_SPI register accessor: an alias for `Reg<UCA0STAT_SPI_SPEC>`"]
pub type UCA0STAT_SPI = crate::Reg<uca0stat_spi::UCA0STAT_SPI_SPEC>;
#[doc = "USCI A0 Status Register"]
pub mod uca0stat_spi;
#[doc = "UCA0RXBUF_SPI register accessor: an alias for `Reg<UCA0RXBUF_SPI_SPEC>`"]
pub type UCA0RXBUF_SPI = crate::Reg<uca0rxbuf_spi::UCA0RXBUF_SPI_SPEC>;
#[doc = "USCI A0 Receive Buffer"]
pub mod uca0rxbuf_spi;
#[doc = "UCA0TXBUF_SPI register accessor: an alias for `Reg<UCA0TXBUF_SPI_SPEC>`"]
pub type UCA0TXBUF_SPI = crate::Reg<uca0txbuf_spi::UCA0TXBUF_SPI_SPEC>;
#[doc = "USCI A0 Transmit Buffer"]
pub mod uca0txbuf_spi;
#[doc = "UCA0IE_SPI register accessor: an alias for `Reg<UCA0IE_SPI_SPEC>`"]
pub type UCA0IE_SPI = crate::Reg<uca0ie_spi::UCA0IE_SPI_SPEC>;
#[doc = "USCI A0 Interrupt Enable Register"]
pub mod uca0ie_spi;
#[doc = "UCA0IFG_SPI register accessor: an alias for `Reg<UCA0IFG_SPI_SPEC>`"]
pub type UCA0IFG_SPI = crate::Reg<uca0ifg_spi::UCA0IFG_SPI_SPEC>;
#[doc = "USCI A0 Interrupt Flags Register"]
pub mod uca0ifg_spi;
#[doc = "UCA0IV_SPI register accessor: an alias for `Reg<UCA0IV_SPI_SPEC>`"]
pub type UCA0IV_SPI = crate::Reg<uca0iv_spi::UCA0IV_SPI_SPEC>;
#[doc = "USCI A0 Interrupt Vector Register"]
pub mod uca0iv_spi;
