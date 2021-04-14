#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Module Control 0"]
    pub dmactl0: crate::Reg<dmactl0::DMACTL0_SPEC>,
    #[doc = "0x02 - DMA Module Control 1"]
    pub dmactl1: crate::Reg<dmactl1::DMACTL1_SPEC>,
    #[doc = "0x04 - DMA Module Control 2"]
    pub dmactl2: crate::Reg<dmactl2::DMACTL2_SPEC>,
    #[doc = "0x06 - DMA Module Control 3"]
    pub dmactl3: crate::Reg<dmactl3::DMACTL3_SPEC>,
    #[doc = "0x08 - DMA Module Control 4"]
    pub dmactl4: crate::Reg<dmactl4::DMACTL4_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x0e - DMA Interrupt Vector Word"]
    pub dmaiv: crate::Reg<dmaiv::DMAIV_SPEC>,
    #[doc = "0x10 - DMA Channel 0 Control"]
    pub dma0ctl: crate::Reg<dma0ctl::DMA0CTL_SPEC>,
    #[doc = "0x12 - DMA Channel 0 Source Address"]
    pub dma0sa: crate::Reg<dma0sa::DMA0SA_SPEC>,
    #[doc = "0x16 - DMA Channel 0 Destination Address"]
    pub dma0da: crate::Reg<dma0da::DMA0DA_SPEC>,
    #[doc = "0x1a - DMA Channel 0 Transfer Size"]
    pub dma0sz: crate::Reg<dma0sz::DMA0SZ_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x20 - DMA Channel 1 Control"]
    pub dma1ctl: crate::Reg<dma1ctl::DMA1CTL_SPEC>,
    #[doc = "0x22 - DMA Channel 1 Source Address"]
    pub dma1sa: crate::Reg<dma1sa::DMA1SA_SPEC>,
    #[doc = "0x26 - DMA Channel 1 Destination Address"]
    pub dma1da: crate::Reg<dma1da::DMA1DA_SPEC>,
    #[doc = "0x2a - DMA Channel 1 Transfer Size"]
    pub dma1sz: crate::Reg<dma1sz::DMA1SZ_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x30 - DMA Channel 2 Control"]
    pub dma2ctl: crate::Reg<dma2ctl::DMA2CTL_SPEC>,
    #[doc = "0x32 - DMA Channel 2 Source Address"]
    pub dma2sa: crate::Reg<dma2sa::DMA2SA_SPEC>,
    #[doc = "0x36 - DMA Channel 2 Destination Address"]
    pub dma2da: crate::Reg<dma2da::DMA2DA_SPEC>,
    #[doc = "0x3a - DMA Channel 2 Transfer Size"]
    pub dma2sz: crate::Reg<dma2sz::DMA2SZ_SPEC>,
}
#[doc = "DMACTL0 register accessor: an alias for `Reg<DMACTL0_SPEC>`"]
pub type DMACTL0 = crate::Reg<dmactl0::DMACTL0_SPEC>;
#[doc = "DMA Module Control 0"]
pub mod dmactl0;
#[doc = "DMACTL1 register accessor: an alias for `Reg<DMACTL1_SPEC>`"]
pub type DMACTL1 = crate::Reg<dmactl1::DMACTL1_SPEC>;
#[doc = "DMA Module Control 1"]
pub mod dmactl1;
#[doc = "DMACTL2 register accessor: an alias for `Reg<DMACTL2_SPEC>`"]
pub type DMACTL2 = crate::Reg<dmactl2::DMACTL2_SPEC>;
#[doc = "DMA Module Control 2"]
pub mod dmactl2;
#[doc = "DMACTL3 register accessor: an alias for `Reg<DMACTL3_SPEC>`"]
pub type DMACTL3 = crate::Reg<dmactl3::DMACTL3_SPEC>;
#[doc = "DMA Module Control 3"]
pub mod dmactl3;
#[doc = "DMACTL4 register accessor: an alias for `Reg<DMACTL4_SPEC>`"]
pub type DMACTL4 = crate::Reg<dmactl4::DMACTL4_SPEC>;
#[doc = "DMA Module Control 4"]
pub mod dmactl4;
#[doc = "DMAIV register accessor: an alias for `Reg<DMAIV_SPEC>`"]
pub type DMAIV = crate::Reg<dmaiv::DMAIV_SPEC>;
#[doc = "DMA Interrupt Vector Word"]
pub mod dmaiv;
#[doc = "DMA0CTL register accessor: an alias for `Reg<DMA0CTL_SPEC>`"]
pub type DMA0CTL = crate::Reg<dma0ctl::DMA0CTL_SPEC>;
#[doc = "DMA Channel 0 Control"]
pub mod dma0ctl;
#[doc = "DMA0SZ register accessor: an alias for `Reg<DMA0SZ_SPEC>`"]
pub type DMA0SZ = crate::Reg<dma0sz::DMA0SZ_SPEC>;
#[doc = "DMA Channel 0 Transfer Size"]
pub mod dma0sz;
#[doc = "DMA1CTL register accessor: an alias for `Reg<DMA1CTL_SPEC>`"]
pub type DMA1CTL = crate::Reg<dma1ctl::DMA1CTL_SPEC>;
#[doc = "DMA Channel 1 Control"]
pub mod dma1ctl;
#[doc = "DMA1SZ register accessor: an alias for `Reg<DMA1SZ_SPEC>`"]
pub type DMA1SZ = crate::Reg<dma1sz::DMA1SZ_SPEC>;
#[doc = "DMA Channel 1 Transfer Size"]
pub mod dma1sz;
#[doc = "DMA2CTL register accessor: an alias for `Reg<DMA2CTL_SPEC>`"]
pub type DMA2CTL = crate::Reg<dma2ctl::DMA2CTL_SPEC>;
#[doc = "DMA Channel 2 Control"]
pub mod dma2ctl;
#[doc = "DMA2SZ register accessor: an alias for `Reg<DMA2SZ_SPEC>`"]
pub type DMA2SZ = crate::Reg<dma2sz::DMA2SZ_SPEC>;
#[doc = "DMA Channel 2 Transfer Size"]
pub mod dma2sz;
#[doc = "DMA0SA register accessor: an alias for `Reg<DMA0SA_SPEC>`"]
pub type DMA0SA = crate::Reg<dma0sa::DMA0SA_SPEC>;
#[doc = "DMA Channel 0 Source Address"]
pub mod dma0sa;
#[doc = "DMA0DA register accessor: an alias for `Reg<DMA0DA_SPEC>`"]
pub type DMA0DA = crate::Reg<dma0da::DMA0DA_SPEC>;
#[doc = "DMA Channel 0 Destination Address"]
pub mod dma0da;
#[doc = "DMA1SA register accessor: an alias for `Reg<DMA1SA_SPEC>`"]
pub type DMA1SA = crate::Reg<dma1sa::DMA1SA_SPEC>;
#[doc = "DMA Channel 1 Source Address"]
pub mod dma1sa;
#[doc = "DMA1DA register accessor: an alias for `Reg<DMA1DA_SPEC>`"]
pub type DMA1DA = crate::Reg<dma1da::DMA1DA_SPEC>;
#[doc = "DMA Channel 1 Destination Address"]
pub mod dma1da;
#[doc = "DMA2SA register accessor: an alias for `Reg<DMA2SA_SPEC>`"]
pub type DMA2SA = crate::Reg<dma2sa::DMA2SA_SPEC>;
#[doc = "DMA Channel 2 Source Address"]
pub mod dma2sa;
#[doc = "DMA2DA register accessor: an alias for `Reg<DMA2DA_SPEC>`"]
pub type DMA2DA = crate::Reg<dma2da::DMA2DA_SPEC>;
#[doc = "DMA Channel 2 Destination Address"]
pub mod dma2da;
