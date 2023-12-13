#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    dmactl0: DMACTL0,
    dmactl1: DMACTL1,
    dmactl2: DMACTL2,
    dmactl3: DMACTL3,
    dmactl4: DMACTL4,
    _reserved5: [u8; 0x04],
    dmaiv: DMAIV,
    dma0ctl: DMA0CTL,
    dma0sa: DMA0SA,
    dma0da: DMA0DA,
    dma0sz: DMA0SZ,
    _reserved10: [u8; 0x04],
    dma1ctl: DMA1CTL,
    dma1sa: DMA1SA,
    dma1da: DMA1DA,
    dma1sz: DMA1SZ,
    _reserved14: [u8; 0x04],
    dma2ctl: DMA2CTL,
    dma2sa: DMA2SA,
    dma2da: DMA2DA,
    dma2sz: DMA2SZ,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Module Control 0"]
    #[inline(always)]
    pub const fn dmactl0(&self) -> &DMACTL0 {
        &self.dmactl0
    }
    #[doc = "0x02 - DMA Module Control 1"]
    #[inline(always)]
    pub const fn dmactl1(&self) -> &DMACTL1 {
        &self.dmactl1
    }
    #[doc = "0x04 - DMA Module Control 2"]
    #[inline(always)]
    pub const fn dmactl2(&self) -> &DMACTL2 {
        &self.dmactl2
    }
    #[doc = "0x06 - DMA Module Control 3"]
    #[inline(always)]
    pub const fn dmactl3(&self) -> &DMACTL3 {
        &self.dmactl3
    }
    #[doc = "0x08 - DMA Module Control 4"]
    #[inline(always)]
    pub const fn dmactl4(&self) -> &DMACTL4 {
        &self.dmactl4
    }
    #[doc = "0x0e - DMA Interrupt Vector Word"]
    #[inline(always)]
    pub const fn dmaiv(&self) -> &DMAIV {
        &self.dmaiv
    }
    #[doc = "0x10 - DMA Channel 0 Control"]
    #[inline(always)]
    pub const fn dma0ctl(&self) -> &DMA0CTL {
        &self.dma0ctl
    }
    #[doc = "0x12 - DMA Channel 0 Source Address"]
    #[inline(always)]
    pub const fn dma0sa(&self) -> &DMA0SA {
        &self.dma0sa
    }
    #[doc = "0x16 - DMA Channel 0 Destination Address"]
    #[inline(always)]
    pub const fn dma0da(&self) -> &DMA0DA {
        &self.dma0da
    }
    #[doc = "0x1a - DMA Channel 0 Transfer Size"]
    #[inline(always)]
    pub const fn dma0sz(&self) -> &DMA0SZ {
        &self.dma0sz
    }
    #[doc = "0x20 - DMA Channel 1 Control"]
    #[inline(always)]
    pub const fn dma1ctl(&self) -> &DMA1CTL {
        &self.dma1ctl
    }
    #[doc = "0x22 - DMA Channel 1 Source Address"]
    #[inline(always)]
    pub const fn dma1sa(&self) -> &DMA1SA {
        &self.dma1sa
    }
    #[doc = "0x26 - DMA Channel 1 Destination Address"]
    #[inline(always)]
    pub const fn dma1da(&self) -> &DMA1DA {
        &self.dma1da
    }
    #[doc = "0x2a - DMA Channel 1 Transfer Size"]
    #[inline(always)]
    pub const fn dma1sz(&self) -> &DMA1SZ {
        &self.dma1sz
    }
    #[doc = "0x30 - DMA Channel 2 Control"]
    #[inline(always)]
    pub const fn dma2ctl(&self) -> &DMA2CTL {
        &self.dma2ctl
    }
    #[doc = "0x32 - DMA Channel 2 Source Address"]
    #[inline(always)]
    pub const fn dma2sa(&self) -> &DMA2SA {
        &self.dma2sa
    }
    #[doc = "0x36 - DMA Channel 2 Destination Address"]
    #[inline(always)]
    pub const fn dma2da(&self) -> &DMA2DA {
        &self.dma2da
    }
    #[doc = "0x3a - DMA Channel 2 Transfer Size"]
    #[inline(always)]
    pub const fn dma2sz(&self) -> &DMA2SZ {
        &self.dma2sz
    }
}
#[doc = "DMACTL0 (rw) register accessor: DMA Module Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactl0`]
module"]
pub type DMACTL0 = crate::Reg<dmactl0::DMACTL0_SPEC>;
#[doc = "DMA Module Control 0"]
pub mod dmactl0;
#[doc = "DMACTL1 (rw) register accessor: DMA Module Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactl1`]
module"]
pub type DMACTL1 = crate::Reg<dmactl1::DMACTL1_SPEC>;
#[doc = "DMA Module Control 1"]
pub mod dmactl1;
#[doc = "DMACTL2 (rw) register accessor: DMA Module Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactl2`]
module"]
pub type DMACTL2 = crate::Reg<dmactl2::DMACTL2_SPEC>;
#[doc = "DMA Module Control 2"]
pub mod dmactl2;
#[doc = "DMACTL3 (rw) register accessor: DMA Module Control 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactl3`]
module"]
pub type DMACTL3 = crate::Reg<dmactl3::DMACTL3_SPEC>;
#[doc = "DMA Module Control 3"]
pub mod dmactl3;
#[doc = "DMACTL4 (rw) register accessor: DMA Module Control 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactl4`]
module"]
pub type DMACTL4 = crate::Reg<dmactl4::DMACTL4_SPEC>;
#[doc = "DMA Module Control 4"]
pub mod dmactl4;
#[doc = "DMAIV (rw) register accessor: DMA Interrupt Vector Word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaiv`]
module"]
pub type DMAIV = crate::Reg<dmaiv::DMAIV_SPEC>;
#[doc = "DMA Interrupt Vector Word"]
pub mod dmaiv;
#[doc = "DMA0CTL (rw) register accessor: DMA Channel 0 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0ctl`]
module"]
pub type DMA0CTL = crate::Reg<dma0ctl::DMA0CTL_SPEC>;
#[doc = "DMA Channel 0 Control"]
pub mod dma0ctl;
#[doc = "DMA0SZ (rw) register accessor: DMA Channel 0 Transfer Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma0sz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma0sz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0sz`]
module"]
pub type DMA0SZ = crate::Reg<dma0sz::DMA0SZ_SPEC>;
#[doc = "DMA Channel 0 Transfer Size"]
pub mod dma0sz;
#[doc = "DMA1CTL (rw) register accessor: DMA Channel 1 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma1ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma1ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1ctl`]
module"]
pub type DMA1CTL = crate::Reg<dma1ctl::DMA1CTL_SPEC>;
#[doc = "DMA Channel 1 Control"]
pub mod dma1ctl;
#[doc = "DMA1SZ (rw) register accessor: DMA Channel 1 Transfer Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma1sz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma1sz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1sz`]
module"]
pub type DMA1SZ = crate::Reg<dma1sz::DMA1SZ_SPEC>;
#[doc = "DMA Channel 1 Transfer Size"]
pub mod dma1sz;
#[doc = "DMA2CTL (rw) register accessor: DMA Channel 2 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma2ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma2ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2ctl`]
module"]
pub type DMA2CTL = crate::Reg<dma2ctl::DMA2CTL_SPEC>;
#[doc = "DMA Channel 2 Control"]
pub mod dma2ctl;
#[doc = "DMA2SZ (rw) register accessor: DMA Channel 2 Transfer Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma2sz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma2sz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2sz`]
module"]
pub type DMA2SZ = crate::Reg<dma2sz::DMA2SZ_SPEC>;
#[doc = "DMA Channel 2 Transfer Size"]
pub mod dma2sz;
#[doc = "DMA0SA (rw) register accessor: DMA Channel 0 Source Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma0sa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma0sa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0sa`]
module"]
pub type DMA0SA = crate::Reg<dma0sa::DMA0SA_SPEC>;
#[doc = "DMA Channel 0 Source Address"]
pub mod dma0sa;
#[doc = "DMA0DA (rw) register accessor: DMA Channel 0 Destination Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma0da::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma0da::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma0da`]
module"]
pub type DMA0DA = crate::Reg<dma0da::DMA0DA_SPEC>;
#[doc = "DMA Channel 0 Destination Address"]
pub mod dma0da;
#[doc = "DMA1SA (rw) register accessor: DMA Channel 1 Source Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma1sa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma1sa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1sa`]
module"]
pub type DMA1SA = crate::Reg<dma1sa::DMA1SA_SPEC>;
#[doc = "DMA Channel 1 Source Address"]
pub mod dma1sa;
#[doc = "DMA1DA (rw) register accessor: DMA Channel 1 Destination Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma1da::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma1da::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1da`]
module"]
pub type DMA1DA = crate::Reg<dma1da::DMA1DA_SPEC>;
#[doc = "DMA Channel 1 Destination Address"]
pub mod dma1da;
#[doc = "DMA2SA (rw) register accessor: DMA Channel 2 Source Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma2sa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma2sa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2sa`]
module"]
pub type DMA2SA = crate::Reg<dma2sa::DMA2SA_SPEC>;
#[doc = "DMA Channel 2 Source Address"]
pub mod dma2sa;
#[doc = "DMA2DA (rw) register accessor: DMA Channel 2 Destination Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma2da::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma2da::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2da`]
module"]
pub type DMA2DA = crate::Reg<dma2da::DMA2DA_SPEC>;
#[doc = "DMA Channel 2 Destination Address"]
pub mod dma2da;
