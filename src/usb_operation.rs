#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    usbstabuff: USBSTABUFF,
    _reserved1: [u8; 0x076e],
    usbtopbuff: USBTOPBUFF,
    usboep0buf: USBOEP0BUF,
    _reserved3: [u8; 0x07],
    usbiep0buf: USBIEP0BUF,
    _reserved4: [u8; 0x07],
    usbsublk: USBSUBLK,
    _reserved5: [u8; 0x07],
    usboepcnf_1: USBOEPCNF_1,
    usboepbbax_1: USBOEPBBAX_1,
    usboepbctx_1: USBOEPBCTX_1,
    _reserved8: [u8; 0x02],
    usboepbbay_1: USBOEPBBAY_1,
    usboepbcty_1: USBOEPBCTY_1,
    usboepsizxy_1: USBOEPSIZXY_1,
    usboepcnf_2: USBOEPCNF_2,
    usboepbbax_2: USBOEPBBAX_2,
    usboepbctx_2: USBOEPBCTX_2,
    _reserved14: [u8; 0x02],
    usboepbbay_2: USBOEPBBAY_2,
    usboepbcty_2: USBOEPBCTY_2,
    usboepsizxy_2: USBOEPSIZXY_2,
    usboepcnf_3: USBOEPCNF_3,
    usboepbbax_3: USBOEPBBAX_3,
    usboepbctx_3: USBOEPBCTX_3,
    _reserved20: [u8; 0x02],
    usboepbbay_3: USBOEPBBAY_3,
    usboepbcty_3: USBOEPBCTY_3,
    usboepsizxy_3: USBOEPSIZXY_3,
    usboepcnf_4: USBOEPCNF_4,
    usboepbbax_4: USBOEPBBAX_4,
    usboepbctx_4: USBOEPBCTX_4,
    _reserved26: [u8; 0x02],
    usboepbbay_4: USBOEPBBAY_4,
    usboepbcty_4: USBOEPBCTY_4,
    usboepsizxy_4: USBOEPSIZXY_4,
    usboepcnf_5: USBOEPCNF_5,
    usboepbbax_5: USBOEPBBAX_5,
    usboepbctx_5: USBOEPBCTX_5,
    _reserved32: [u8; 0x02],
    usboepbbay_5: USBOEPBBAY_5,
    usboepbcty_5: USBOEPBCTY_5,
    usboepsizxy_5: USBOEPSIZXY_5,
    usboepcnf_6: USBOEPCNF_6,
    usboepbbax_6: USBOEPBBAX_6,
    usboepbctx_6: USBOEPBCTX_6,
    _reserved38: [u8; 0x02],
    usboepbbay_6: USBOEPBBAY_6,
    usboepbcty_6: USBOEPBCTY_6,
    usboepsizxy_6: USBOEPSIZXY_6,
    usboepcnf_7: USBOEPCNF_7,
    usboepbbax_7: USBOEPBBAX_7,
    usboepbctx_7: USBOEPBCTX_7,
    _reserved44: [u8; 0x02],
    usboepbbay_7: USBOEPBBAY_7,
    usboepbcty_7: USBOEPBCTY_7,
    usboepsizxy_7: USBOEPSIZXY_7,
    _reserved47: [u8; 0x08],
    usbiepcnf_1: USBIEPCNF_1,
    usbiepbbax_1: USBIEPBBAX_1,
    usbiepbctx_1: USBIEPBCTX_1,
    _reserved50: [u8; 0x02],
    usbiepbbay_1: USBIEPBBAY_1,
    usbiepbcty_1: USBIEPBCTY_1,
    usbiepsizxy_1: USBIEPSIZXY_1,
    usbiepcnf_2: USBIEPCNF_2,
    usbiepbbax_2: USBIEPBBAX_2,
    usbiepbctx_2: USBIEPBCTX_2,
    _reserved56: [u8; 0x02],
    usbiepbbay_2: USBIEPBBAY_2,
    usbiepbcty_2: USBIEPBCTY_2,
    usbiepsizxy_2: USBIEPSIZXY_2,
    usbiepcnf_3: USBIEPCNF_3,
    usbiepbbax_3: USBIEPBBAX_3,
    usbiepbctx_3: USBIEPBCTX_3,
    _reserved62: [u8; 0x02],
    usbiepbbay_3: USBIEPBBAY_3,
    usbiepbcty_3: USBIEPBCTY_3,
    usbiepsizxy_3: USBIEPSIZXY_3,
    usbiepcnf_4: USBIEPCNF_4,
    usbiepbbax_4: USBIEPBBAX_4,
    usbiepbctx_4: USBIEPBCTX_4,
    _reserved68: [u8; 0x02],
    usbiepbbay_4: USBIEPBBAY_4,
    usbiepbcty_4: USBIEPBCTY_4,
    usbiepsizxy_4: USBIEPSIZXY_4,
    usbiepcnf_5: USBIEPCNF_5,
    usbiepbbax_5: USBIEPBBAX_5,
    usbiepbctx_5: USBIEPBCTX_5,
    _reserved74: [u8; 0x02],
    usbiepbbay_5: USBIEPBBAY_5,
    usbiepbcty_5: USBIEPBCTY_5,
    usbiepsizxy_5: USBIEPSIZXY_5,
    usbiepcnf_6: USBIEPCNF_6,
    usbiepbbax_6: USBIEPBBAX_6,
    usbiepbctx_6: USBIEPBCTX_6,
    _reserved80: [u8; 0x02],
    usbiepbbay_6: USBIEPBBAY_6,
    usbiepbcty_6: USBIEPBCTY_6,
    usbiepsizxy_6: USBIEPSIZXY_6,
    usbiepcnf_7: USBIEPCNF_7,
    usbiepbbax_7: USBIEPBBAX_7,
    usbiepbctx_7: USBIEPBCTX_7,
    _reserved86: [u8; 0x02],
    usbiepbbay_7: USBIEPBBAY_7,
    usbiepbcty_7: USBIEPBCTY_7,
    usbiepsizxy_7: USBIEPSIZXY_7,
}
impl RegisterBlock {
    #[doc = "0x00 - Start of buffer space"]
    #[inline(always)]
    pub const fn usbstabuff(&self) -> &USBSTABUFF {
        &self.usbstabuff
    }
    #[doc = "0x76f - Top of buffer space"]
    #[inline(always)]
    pub const fn usbtopbuff(&self) -> &USBTOPBUFF {
        &self.usbtopbuff
    }
    #[doc = "0x770 - Output endpoint_0 buffer"]
    #[inline(always)]
    pub const fn usboep0buf(&self) -> &USBOEP0BUF {
        &self.usboep0buf
    }
    #[doc = "0x778 - Input endpoint_0 buffer"]
    #[inline(always)]
    pub const fn usbiep0buf(&self) -> &USBIEP0BUF {
        &self.usbiep0buf
    }
    #[doc = "0x780 - Setup Packet Block"]
    #[inline(always)]
    pub const fn usbsublk(&self) -> &USBSUBLK {
        &self.usbsublk
    }
    #[doc = "0x788 - Output Endpoint_1: Configuration"]
    #[inline(always)]
    pub const fn usboepcnf_1(&self) -> &USBOEPCNF_1 {
        &self.usboepcnf_1
    }
    #[doc = "0x789 - Output Endpoint_1: X-buffer base addr."]
    #[inline(always)]
    pub const fn usboepbbax_1(&self) -> &USBOEPBBAX_1 {
        &self.usboepbbax_1
    }
    #[doc = "0x78a - Output Endpoint_1: X-byte count"]
    #[inline(always)]
    pub const fn usboepbctx_1(&self) -> &USBOEPBCTX_1 {
        &self.usboepbctx_1
    }
    #[doc = "0x78d - Output Endpoint_1: Y-buffer base addr."]
    #[inline(always)]
    pub const fn usboepbbay_1(&self) -> &USBOEPBBAY_1 {
        &self.usboepbbay_1
    }
    #[doc = "0x78e - Output Endpoint_1: Y-byte count"]
    #[inline(always)]
    pub const fn usboepbcty_1(&self) -> &USBOEPBCTY_1 {
        &self.usboepbcty_1
    }
    #[doc = "0x78f - Output Endpoint_1: X/Y-buffer size"]
    #[inline(always)]
    pub const fn usboepsizxy_1(&self) -> &USBOEPSIZXY_1 {
        &self.usboepsizxy_1
    }
    #[doc = "0x790 - Output Endpoint_2: Configuration"]
    #[inline(always)]
    pub const fn usboepcnf_2(&self) -> &USBOEPCNF_2 {
        &self.usboepcnf_2
    }
    #[doc = "0x791 - Output Endpoint_2: X-buffer base addr."]
    #[inline(always)]
    pub const fn usboepbbax_2(&self) -> &USBOEPBBAX_2 {
        &self.usboepbbax_2
    }
    #[doc = "0x792 - Output Endpoint_2: X-byte count"]
    #[inline(always)]
    pub const fn usboepbctx_2(&self) -> &USBOEPBCTX_2 {
        &self.usboepbctx_2
    }
    #[doc = "0x795 - Output Endpoint_2: Y-buffer base addr."]
    #[inline(always)]
    pub const fn usboepbbay_2(&self) -> &USBOEPBBAY_2 {
        &self.usboepbbay_2
    }
    #[doc = "0x796 - Output Endpoint_2: Y-byte count"]
    #[inline(always)]
    pub const fn usboepbcty_2(&self) -> &USBOEPBCTY_2 {
        &self.usboepbcty_2
    }
    #[doc = "0x797 - Output Endpoint_2: X/Y-buffer size"]
    #[inline(always)]
    pub const fn usboepsizxy_2(&self) -> &USBOEPSIZXY_2 {
        &self.usboepsizxy_2
    }
    #[doc = "0x798 - Output Endpoint_3: Configuration"]
    #[inline(always)]
    pub const fn usboepcnf_3(&self) -> &USBOEPCNF_3 {
        &self.usboepcnf_3
    }
    #[doc = "0x799 - Output Endpoint_3: X-buffer base addr."]
    #[inline(always)]
    pub const fn usboepbbax_3(&self) -> &USBOEPBBAX_3 {
        &self.usboepbbax_3
    }
    #[doc = "0x79a - Output Endpoint_3: X-byte count"]
    #[inline(always)]
    pub const fn usboepbctx_3(&self) -> &USBOEPBCTX_3 {
        &self.usboepbctx_3
    }
    #[doc = "0x79d - Output Endpoint_3: Y-buffer base addr."]
    #[inline(always)]
    pub const fn usboepbbay_3(&self) -> &USBOEPBBAY_3 {
        &self.usboepbbay_3
    }
    #[doc = "0x79e - Output Endpoint_3: Y-byte count"]
    #[inline(always)]
    pub const fn usboepbcty_3(&self) -> &USBOEPBCTY_3 {
        &self.usboepbcty_3
    }
    #[doc = "0x79f - Output Endpoint_3: X/Y-buffer size"]
    #[inline(always)]
    pub const fn usboepsizxy_3(&self) -> &USBOEPSIZXY_3 {
        &self.usboepsizxy_3
    }
    #[doc = "0x7a0 - Output Endpoint_4: Configuration"]
    #[inline(always)]
    pub const fn usboepcnf_4(&self) -> &USBOEPCNF_4 {
        &self.usboepcnf_4
    }
    #[doc = "0x7a1 - Output Endpoint_4: X-buffer base addr."]
    #[inline(always)]
    pub const fn usboepbbax_4(&self) -> &USBOEPBBAX_4 {
        &self.usboepbbax_4
    }
    #[doc = "0x7a2 - Output Endpoint_4: X-byte count"]
    #[inline(always)]
    pub const fn usboepbctx_4(&self) -> &USBOEPBCTX_4 {
        &self.usboepbctx_4
    }
    #[doc = "0x7a5 - Output Endpoint_4: Y-buffer base addr."]
    #[inline(always)]
    pub const fn usboepbbay_4(&self) -> &USBOEPBBAY_4 {
        &self.usboepbbay_4
    }
    #[doc = "0x7a6 - Output Endpoint_4: Y-byte count"]
    #[inline(always)]
    pub const fn usboepbcty_4(&self) -> &USBOEPBCTY_4 {
        &self.usboepbcty_4
    }
    #[doc = "0x7a7 - Output Endpoint_4: X/Y-buffer size"]
    #[inline(always)]
    pub const fn usboepsizxy_4(&self) -> &USBOEPSIZXY_4 {
        &self.usboepsizxy_4
    }
    #[doc = "0x7a8 - Output Endpoint_5: Configuration"]
    #[inline(always)]
    pub const fn usboepcnf_5(&self) -> &USBOEPCNF_5 {
        &self.usboepcnf_5
    }
    #[doc = "0x7a9 - Output Endpoint_5: X-buffer base addr."]
    #[inline(always)]
    pub const fn usboepbbax_5(&self) -> &USBOEPBBAX_5 {
        &self.usboepbbax_5
    }
    #[doc = "0x7aa - Output Endpoint_5: X-byte count"]
    #[inline(always)]
    pub const fn usboepbctx_5(&self) -> &USBOEPBCTX_5 {
        &self.usboepbctx_5
    }
    #[doc = "0x7ad - Output Endpoint_5: Y-buffer base addr."]
    #[inline(always)]
    pub const fn usboepbbay_5(&self) -> &USBOEPBBAY_5 {
        &self.usboepbbay_5
    }
    #[doc = "0x7ae - Output Endpoint_5: Y-byte count"]
    #[inline(always)]
    pub const fn usboepbcty_5(&self) -> &USBOEPBCTY_5 {
        &self.usboepbcty_5
    }
    #[doc = "0x7af - Output Endpoint_5: X/Y-buffer size"]
    #[inline(always)]
    pub const fn usboepsizxy_5(&self) -> &USBOEPSIZXY_5 {
        &self.usboepsizxy_5
    }
    #[doc = "0x7b0 - Output Endpoint_6: Configuration"]
    #[inline(always)]
    pub const fn usboepcnf_6(&self) -> &USBOEPCNF_6 {
        &self.usboepcnf_6
    }
    #[doc = "0x7b1 - Output Endpoint_6: X-buffer base addr."]
    #[inline(always)]
    pub const fn usboepbbax_6(&self) -> &USBOEPBBAX_6 {
        &self.usboepbbax_6
    }
    #[doc = "0x7b2 - Output Endpoint_6: X-byte count"]
    #[inline(always)]
    pub const fn usboepbctx_6(&self) -> &USBOEPBCTX_6 {
        &self.usboepbctx_6
    }
    #[doc = "0x7b5 - Output Endpoint_6: Y-buffer base addr."]
    #[inline(always)]
    pub const fn usboepbbay_6(&self) -> &USBOEPBBAY_6 {
        &self.usboepbbay_6
    }
    #[doc = "0x7b6 - Output Endpoint_6: Y-byte count"]
    #[inline(always)]
    pub const fn usboepbcty_6(&self) -> &USBOEPBCTY_6 {
        &self.usboepbcty_6
    }
    #[doc = "0x7b7 - Output Endpoint_6: X/Y-buffer size"]
    #[inline(always)]
    pub const fn usboepsizxy_6(&self) -> &USBOEPSIZXY_6 {
        &self.usboepsizxy_6
    }
    #[doc = "0x7b8 - Output Endpoint_7: Configuration"]
    #[inline(always)]
    pub const fn usboepcnf_7(&self) -> &USBOEPCNF_7 {
        &self.usboepcnf_7
    }
    #[doc = "0x7b9 - Output Endpoint_7: X-buffer base addr."]
    #[inline(always)]
    pub const fn usboepbbax_7(&self) -> &USBOEPBBAX_7 {
        &self.usboepbbax_7
    }
    #[doc = "0x7ba - Output Endpoint_7: X-byte count"]
    #[inline(always)]
    pub const fn usboepbctx_7(&self) -> &USBOEPBCTX_7 {
        &self.usboepbctx_7
    }
    #[doc = "0x7bd - Output Endpoint_7: Y-buffer base addr."]
    #[inline(always)]
    pub const fn usboepbbay_7(&self) -> &USBOEPBBAY_7 {
        &self.usboepbbay_7
    }
    #[doc = "0x7be - Output Endpoint_7: Y-byte count"]
    #[inline(always)]
    pub const fn usboepbcty_7(&self) -> &USBOEPBCTY_7 {
        &self.usboepbcty_7
    }
    #[doc = "0x7bf - Output Endpoint_7: X/Y-buffer size"]
    #[inline(always)]
    pub const fn usboepsizxy_7(&self) -> &USBOEPSIZXY_7 {
        &self.usboepsizxy_7
    }
    #[doc = "0x7c8 - Input Endpoint_1: Configuration"]
    #[inline(always)]
    pub const fn usbiepcnf_1(&self) -> &USBIEPCNF_1 {
        &self.usbiepcnf_1
    }
    #[doc = "0x7c9 - Input Endpoint_1: X-buffer base addr."]
    #[inline(always)]
    pub const fn usbiepbbax_1(&self) -> &USBIEPBBAX_1 {
        &self.usbiepbbax_1
    }
    #[doc = "0x7ca - Input Endpoint_1: X-byte count"]
    #[inline(always)]
    pub const fn usbiepbctx_1(&self) -> &USBIEPBCTX_1 {
        &self.usbiepbctx_1
    }
    #[doc = "0x7cd - Input Endpoint_1: Y-buffer base addr."]
    #[inline(always)]
    pub const fn usbiepbbay_1(&self) -> &USBIEPBBAY_1 {
        &self.usbiepbbay_1
    }
    #[doc = "0x7ce - Input Endpoint_1: Y-byte count"]
    #[inline(always)]
    pub const fn usbiepbcty_1(&self) -> &USBIEPBCTY_1 {
        &self.usbiepbcty_1
    }
    #[doc = "0x7cf - Input Endpoint_1: X/Y-buffer size"]
    #[inline(always)]
    pub const fn usbiepsizxy_1(&self) -> &USBIEPSIZXY_1 {
        &self.usbiepsizxy_1
    }
    #[doc = "0x7d0 - Input Endpoint_2: Configuration"]
    #[inline(always)]
    pub const fn usbiepcnf_2(&self) -> &USBIEPCNF_2 {
        &self.usbiepcnf_2
    }
    #[doc = "0x7d1 - Input Endpoint_2: X-buffer base addr."]
    #[inline(always)]
    pub const fn usbiepbbax_2(&self) -> &USBIEPBBAX_2 {
        &self.usbiepbbax_2
    }
    #[doc = "0x7d2 - Input Endpoint_2: X-byte count"]
    #[inline(always)]
    pub const fn usbiepbctx_2(&self) -> &USBIEPBCTX_2 {
        &self.usbiepbctx_2
    }
    #[doc = "0x7d5 - Input Endpoint_2: Y-buffer base addr."]
    #[inline(always)]
    pub const fn usbiepbbay_2(&self) -> &USBIEPBBAY_2 {
        &self.usbiepbbay_2
    }
    #[doc = "0x7d6 - Input Endpoint_2: Y-byte count"]
    #[inline(always)]
    pub const fn usbiepbcty_2(&self) -> &USBIEPBCTY_2 {
        &self.usbiepbcty_2
    }
    #[doc = "0x7d7 - Input Endpoint_2: X/Y-buffer size"]
    #[inline(always)]
    pub const fn usbiepsizxy_2(&self) -> &USBIEPSIZXY_2 {
        &self.usbiepsizxy_2
    }
    #[doc = "0x7d8 - Input Endpoint_3: Configuration"]
    #[inline(always)]
    pub const fn usbiepcnf_3(&self) -> &USBIEPCNF_3 {
        &self.usbiepcnf_3
    }
    #[doc = "0x7d9 - Input Endpoint_3: X-buffer base addr."]
    #[inline(always)]
    pub const fn usbiepbbax_3(&self) -> &USBIEPBBAX_3 {
        &self.usbiepbbax_3
    }
    #[doc = "0x7da - Input Endpoint_3: X-byte count"]
    #[inline(always)]
    pub const fn usbiepbctx_3(&self) -> &USBIEPBCTX_3 {
        &self.usbiepbctx_3
    }
    #[doc = "0x7dd - Input Endpoint_3: Y-buffer base addr."]
    #[inline(always)]
    pub const fn usbiepbbay_3(&self) -> &USBIEPBBAY_3 {
        &self.usbiepbbay_3
    }
    #[doc = "0x7de - Input Endpoint_3: Y-byte count"]
    #[inline(always)]
    pub const fn usbiepbcty_3(&self) -> &USBIEPBCTY_3 {
        &self.usbiepbcty_3
    }
    #[doc = "0x7df - Input Endpoint_3: X/Y-buffer size"]
    #[inline(always)]
    pub const fn usbiepsizxy_3(&self) -> &USBIEPSIZXY_3 {
        &self.usbiepsizxy_3
    }
    #[doc = "0x7e0 - Input Endpoint_4: Configuration"]
    #[inline(always)]
    pub const fn usbiepcnf_4(&self) -> &USBIEPCNF_4 {
        &self.usbiepcnf_4
    }
    #[doc = "0x7e1 - Input Endpoint_4: X-buffer base addr."]
    #[inline(always)]
    pub const fn usbiepbbax_4(&self) -> &USBIEPBBAX_4 {
        &self.usbiepbbax_4
    }
    #[doc = "0x7e2 - Input Endpoint_4: X-byte count"]
    #[inline(always)]
    pub const fn usbiepbctx_4(&self) -> &USBIEPBCTX_4 {
        &self.usbiepbctx_4
    }
    #[doc = "0x7e5 - Input Endpoint_4: Y-buffer base addr."]
    #[inline(always)]
    pub const fn usbiepbbay_4(&self) -> &USBIEPBBAY_4 {
        &self.usbiepbbay_4
    }
    #[doc = "0x7e6 - Input Endpoint_4: Y-byte count"]
    #[inline(always)]
    pub const fn usbiepbcty_4(&self) -> &USBIEPBCTY_4 {
        &self.usbiepbcty_4
    }
    #[doc = "0x7e7 - Input Endpoint_4: X/Y-buffer size"]
    #[inline(always)]
    pub const fn usbiepsizxy_4(&self) -> &USBIEPSIZXY_4 {
        &self.usbiepsizxy_4
    }
    #[doc = "0x7e8 - Input Endpoint_5: Configuration"]
    #[inline(always)]
    pub const fn usbiepcnf_5(&self) -> &USBIEPCNF_5 {
        &self.usbiepcnf_5
    }
    #[doc = "0x7e9 - Input Endpoint_5: X-buffer base addr."]
    #[inline(always)]
    pub const fn usbiepbbax_5(&self) -> &USBIEPBBAX_5 {
        &self.usbiepbbax_5
    }
    #[doc = "0x7ea - Input Endpoint_5: X-byte count"]
    #[inline(always)]
    pub const fn usbiepbctx_5(&self) -> &USBIEPBCTX_5 {
        &self.usbiepbctx_5
    }
    #[doc = "0x7ed - Input Endpoint_5: Y-buffer base addr."]
    #[inline(always)]
    pub const fn usbiepbbay_5(&self) -> &USBIEPBBAY_5 {
        &self.usbiepbbay_5
    }
    #[doc = "0x7ee - Input Endpoint_5: Y-byte count"]
    #[inline(always)]
    pub const fn usbiepbcty_5(&self) -> &USBIEPBCTY_5 {
        &self.usbiepbcty_5
    }
    #[doc = "0x7ef - Input Endpoint_5: X/Y-buffer size"]
    #[inline(always)]
    pub const fn usbiepsizxy_5(&self) -> &USBIEPSIZXY_5 {
        &self.usbiepsizxy_5
    }
    #[doc = "0x7f0 - Input Endpoint_6: Configuration"]
    #[inline(always)]
    pub const fn usbiepcnf_6(&self) -> &USBIEPCNF_6 {
        &self.usbiepcnf_6
    }
    #[doc = "0x7f1 - Input Endpoint_6: X-buffer base addr."]
    #[inline(always)]
    pub const fn usbiepbbax_6(&self) -> &USBIEPBBAX_6 {
        &self.usbiepbbax_6
    }
    #[doc = "0x7f2 - Input Endpoint_6: X-byte count"]
    #[inline(always)]
    pub const fn usbiepbctx_6(&self) -> &USBIEPBCTX_6 {
        &self.usbiepbctx_6
    }
    #[doc = "0x7f5 - Input Endpoint_6: Y-buffer base addr."]
    #[inline(always)]
    pub const fn usbiepbbay_6(&self) -> &USBIEPBBAY_6 {
        &self.usbiepbbay_6
    }
    #[doc = "0x7f6 - Input Endpoint_6: Y-byte count"]
    #[inline(always)]
    pub const fn usbiepbcty_6(&self) -> &USBIEPBCTY_6 {
        &self.usbiepbcty_6
    }
    #[doc = "0x7f7 - Input Endpoint_6: X/Y-buffer size"]
    #[inline(always)]
    pub const fn usbiepsizxy_6(&self) -> &USBIEPSIZXY_6 {
        &self.usbiepsizxy_6
    }
    #[doc = "0x7f8 - Input Endpoint_7: Configuration"]
    #[inline(always)]
    pub const fn usbiepcnf_7(&self) -> &USBIEPCNF_7 {
        &self.usbiepcnf_7
    }
    #[doc = "0x7f9 - Input Endpoint_7: X-buffer base addr."]
    #[inline(always)]
    pub const fn usbiepbbax_7(&self) -> &USBIEPBBAX_7 {
        &self.usbiepbbax_7
    }
    #[doc = "0x7fa - Input Endpoint_7: X-byte count"]
    #[inline(always)]
    pub const fn usbiepbctx_7(&self) -> &USBIEPBCTX_7 {
        &self.usbiepbctx_7
    }
    #[doc = "0x7fd - Input Endpoint_7: Y-buffer base addr."]
    #[inline(always)]
    pub const fn usbiepbbay_7(&self) -> &USBIEPBBAY_7 {
        &self.usbiepbbay_7
    }
    #[doc = "0x7fe - Input Endpoint_7: Y-byte count"]
    #[inline(always)]
    pub const fn usbiepbcty_7(&self) -> &USBIEPBCTY_7 {
        &self.usbiepbcty_7
    }
    #[doc = "0x7ff - Input Endpoint_7: X/Y-buffer size"]
    #[inline(always)]
    pub const fn usbiepsizxy_7(&self) -> &USBIEPSIZXY_7 {
        &self.usbiepsizxy_7
    }
}
#[doc = "USBSTABUFF (rw) register accessor: Start of buffer space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbstabuff::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbstabuff::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbstabuff`]
module"]
pub type USBSTABUFF = crate::Reg<usbstabuff::USBSTABUFF_SPEC>;
#[doc = "Start of buffer space"]
pub mod usbstabuff;
#[doc = "USBTOPBUFF (rw) register accessor: Top of buffer space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbtopbuff::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbtopbuff::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbtopbuff`]
module"]
pub type USBTOPBUFF = crate::Reg<usbtopbuff::USBTOPBUFF_SPEC>;
#[doc = "Top of buffer space"]
pub mod usbtopbuff;
#[doc = "USBOEP0BUF (rw) register accessor: Output endpoint_0 buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboep0buf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboep0buf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboep0buf`]
module"]
pub type USBOEP0BUF = crate::Reg<usboep0buf::USBOEP0BUF_SPEC>;
#[doc = "Output endpoint_0 buffer"]
pub mod usboep0buf;
#[doc = "USBIEP0BUF (rw) register accessor: Input endpoint_0 buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiep0buf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiep0buf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiep0buf`]
module"]
pub type USBIEP0BUF = crate::Reg<usbiep0buf::USBIEP0BUF_SPEC>;
#[doc = "Input endpoint_0 buffer"]
pub mod usbiep0buf;
#[doc = "USBSUBLK (rw) register accessor: Setup Packet Block\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbsublk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbsublk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbsublk`]
module"]
pub type USBSUBLK = crate::Reg<usbsublk::USBSUBLK_SPEC>;
#[doc = "Setup Packet Block"]
pub mod usbsublk;
#[doc = "USBOEPCNF_1 (rw) register accessor: Output Endpoint_1: Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepcnf_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepcnf_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepcnf_1`]
module"]
pub type USBOEPCNF_1 = crate::Reg<usboepcnf_1::USBOEPCNF_1_SPEC>;
#[doc = "Output Endpoint_1: Configuration"]
pub mod usboepcnf_1;
#[doc = "USBOEPBBAX_1 (rw) register accessor: Output Endpoint_1: X-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbbax_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbbax_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbbax_1`]
module"]
pub type USBOEPBBAX_1 = crate::Reg<usboepbbax_1::USBOEPBBAX_1_SPEC>;
#[doc = "Output Endpoint_1: X-buffer base addr."]
pub mod usboepbbax_1;
#[doc = "USBOEPBCTX_1 (rw) register accessor: Output Endpoint_1: X-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbctx_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbctx_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbctx_1`]
module"]
pub type USBOEPBCTX_1 = crate::Reg<usboepbctx_1::USBOEPBCTX_1_SPEC>;
#[doc = "Output Endpoint_1: X-byte count"]
pub mod usboepbctx_1;
#[doc = "USBOEPBBAY_1 (rw) register accessor: Output Endpoint_1: Y-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbbay_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbbay_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbbay_1`]
module"]
pub type USBOEPBBAY_1 = crate::Reg<usboepbbay_1::USBOEPBBAY_1_SPEC>;
#[doc = "Output Endpoint_1: Y-buffer base addr."]
pub mod usboepbbay_1;
#[doc = "USBOEPBCTY_1 (rw) register accessor: Output Endpoint_1: Y-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbcty_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbcty_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbcty_1`]
module"]
pub type USBOEPBCTY_1 = crate::Reg<usboepbcty_1::USBOEPBCTY_1_SPEC>;
#[doc = "Output Endpoint_1: Y-byte count"]
pub mod usboepbcty_1;
#[doc = "USBOEPSIZXY_1 (rw) register accessor: Output Endpoint_1: X/Y-buffer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepsizxy_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepsizxy_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepsizxy_1`]
module"]
pub type USBOEPSIZXY_1 = crate::Reg<usboepsizxy_1::USBOEPSIZXY_1_SPEC>;
#[doc = "Output Endpoint_1: X/Y-buffer size"]
pub mod usboepsizxy_1;
#[doc = "USBOEPCNF_2 (rw) register accessor: Output Endpoint_2: Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepcnf_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepcnf_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepcnf_2`]
module"]
pub type USBOEPCNF_2 = crate::Reg<usboepcnf_2::USBOEPCNF_2_SPEC>;
#[doc = "Output Endpoint_2: Configuration"]
pub mod usboepcnf_2;
#[doc = "USBOEPBBAX_2 (rw) register accessor: Output Endpoint_2: X-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbbax_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbbax_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbbax_2`]
module"]
pub type USBOEPBBAX_2 = crate::Reg<usboepbbax_2::USBOEPBBAX_2_SPEC>;
#[doc = "Output Endpoint_2: X-buffer base addr."]
pub mod usboepbbax_2;
#[doc = "USBOEPBCTX_2 (rw) register accessor: Output Endpoint_2: X-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbctx_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbctx_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbctx_2`]
module"]
pub type USBOEPBCTX_2 = crate::Reg<usboepbctx_2::USBOEPBCTX_2_SPEC>;
#[doc = "Output Endpoint_2: X-byte count"]
pub mod usboepbctx_2;
#[doc = "USBOEPBBAY_2 (rw) register accessor: Output Endpoint_2: Y-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbbay_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbbay_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbbay_2`]
module"]
pub type USBOEPBBAY_2 = crate::Reg<usboepbbay_2::USBOEPBBAY_2_SPEC>;
#[doc = "Output Endpoint_2: Y-buffer base addr."]
pub mod usboepbbay_2;
#[doc = "USBOEPBCTY_2 (rw) register accessor: Output Endpoint_2: Y-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbcty_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbcty_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbcty_2`]
module"]
pub type USBOEPBCTY_2 = crate::Reg<usboepbcty_2::USBOEPBCTY_2_SPEC>;
#[doc = "Output Endpoint_2: Y-byte count"]
pub mod usboepbcty_2;
#[doc = "USBOEPSIZXY_2 (rw) register accessor: Output Endpoint_2: X/Y-buffer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepsizxy_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepsizxy_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepsizxy_2`]
module"]
pub type USBOEPSIZXY_2 = crate::Reg<usboepsizxy_2::USBOEPSIZXY_2_SPEC>;
#[doc = "Output Endpoint_2: X/Y-buffer size"]
pub mod usboepsizxy_2;
#[doc = "USBOEPCNF_3 (rw) register accessor: Output Endpoint_3: Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepcnf_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepcnf_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepcnf_3`]
module"]
pub type USBOEPCNF_3 = crate::Reg<usboepcnf_3::USBOEPCNF_3_SPEC>;
#[doc = "Output Endpoint_3: Configuration"]
pub mod usboepcnf_3;
#[doc = "USBOEPBBAX_3 (rw) register accessor: Output Endpoint_3: X-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbbax_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbbax_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbbax_3`]
module"]
pub type USBOEPBBAX_3 = crate::Reg<usboepbbax_3::USBOEPBBAX_3_SPEC>;
#[doc = "Output Endpoint_3: X-buffer base addr."]
pub mod usboepbbax_3;
#[doc = "USBOEPBCTX_3 (rw) register accessor: Output Endpoint_3: X-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbctx_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbctx_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbctx_3`]
module"]
pub type USBOEPBCTX_3 = crate::Reg<usboepbctx_3::USBOEPBCTX_3_SPEC>;
#[doc = "Output Endpoint_3: X-byte count"]
pub mod usboepbctx_3;
#[doc = "USBOEPBBAY_3 (rw) register accessor: Output Endpoint_3: Y-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbbay_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbbay_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbbay_3`]
module"]
pub type USBOEPBBAY_3 = crate::Reg<usboepbbay_3::USBOEPBBAY_3_SPEC>;
#[doc = "Output Endpoint_3: Y-buffer base addr."]
pub mod usboepbbay_3;
#[doc = "USBOEPBCTY_3 (rw) register accessor: Output Endpoint_3: Y-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbcty_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbcty_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbcty_3`]
module"]
pub type USBOEPBCTY_3 = crate::Reg<usboepbcty_3::USBOEPBCTY_3_SPEC>;
#[doc = "Output Endpoint_3: Y-byte count"]
pub mod usboepbcty_3;
#[doc = "USBOEPSIZXY_3 (rw) register accessor: Output Endpoint_3: X/Y-buffer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepsizxy_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepsizxy_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepsizxy_3`]
module"]
pub type USBOEPSIZXY_3 = crate::Reg<usboepsizxy_3::USBOEPSIZXY_3_SPEC>;
#[doc = "Output Endpoint_3: X/Y-buffer size"]
pub mod usboepsizxy_3;
#[doc = "USBOEPCNF_4 (rw) register accessor: Output Endpoint_4: Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepcnf_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepcnf_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepcnf_4`]
module"]
pub type USBOEPCNF_4 = crate::Reg<usboepcnf_4::USBOEPCNF_4_SPEC>;
#[doc = "Output Endpoint_4: Configuration"]
pub mod usboepcnf_4;
#[doc = "USBOEPBBAX_4 (rw) register accessor: Output Endpoint_4: X-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbbax_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbbax_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbbax_4`]
module"]
pub type USBOEPBBAX_4 = crate::Reg<usboepbbax_4::USBOEPBBAX_4_SPEC>;
#[doc = "Output Endpoint_4: X-buffer base addr."]
pub mod usboepbbax_4;
#[doc = "USBOEPBCTX_4 (rw) register accessor: Output Endpoint_4: X-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbctx_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbctx_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbctx_4`]
module"]
pub type USBOEPBCTX_4 = crate::Reg<usboepbctx_4::USBOEPBCTX_4_SPEC>;
#[doc = "Output Endpoint_4: X-byte count"]
pub mod usboepbctx_4;
#[doc = "USBOEPBBAY_4 (rw) register accessor: Output Endpoint_4: Y-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbbay_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbbay_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbbay_4`]
module"]
pub type USBOEPBBAY_4 = crate::Reg<usboepbbay_4::USBOEPBBAY_4_SPEC>;
#[doc = "Output Endpoint_4: Y-buffer base addr."]
pub mod usboepbbay_4;
#[doc = "USBOEPBCTY_4 (rw) register accessor: Output Endpoint_4: Y-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbcty_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbcty_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbcty_4`]
module"]
pub type USBOEPBCTY_4 = crate::Reg<usboepbcty_4::USBOEPBCTY_4_SPEC>;
#[doc = "Output Endpoint_4: Y-byte count"]
pub mod usboepbcty_4;
#[doc = "USBOEPSIZXY_4 (rw) register accessor: Output Endpoint_4: X/Y-buffer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepsizxy_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepsizxy_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepsizxy_4`]
module"]
pub type USBOEPSIZXY_4 = crate::Reg<usboepsizxy_4::USBOEPSIZXY_4_SPEC>;
#[doc = "Output Endpoint_4: X/Y-buffer size"]
pub mod usboepsizxy_4;
#[doc = "USBOEPCNF_5 (rw) register accessor: Output Endpoint_5: Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepcnf_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepcnf_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepcnf_5`]
module"]
pub type USBOEPCNF_5 = crate::Reg<usboepcnf_5::USBOEPCNF_5_SPEC>;
#[doc = "Output Endpoint_5: Configuration"]
pub mod usboepcnf_5;
#[doc = "USBOEPBBAX_5 (rw) register accessor: Output Endpoint_5: X-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbbax_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbbax_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbbax_5`]
module"]
pub type USBOEPBBAX_5 = crate::Reg<usboepbbax_5::USBOEPBBAX_5_SPEC>;
#[doc = "Output Endpoint_5: X-buffer base addr."]
pub mod usboepbbax_5;
#[doc = "USBOEPBCTX_5 (rw) register accessor: Output Endpoint_5: X-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbctx_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbctx_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbctx_5`]
module"]
pub type USBOEPBCTX_5 = crate::Reg<usboepbctx_5::USBOEPBCTX_5_SPEC>;
#[doc = "Output Endpoint_5: X-byte count"]
pub mod usboepbctx_5;
#[doc = "USBOEPBBAY_5 (rw) register accessor: Output Endpoint_5: Y-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbbay_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbbay_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbbay_5`]
module"]
pub type USBOEPBBAY_5 = crate::Reg<usboepbbay_5::USBOEPBBAY_5_SPEC>;
#[doc = "Output Endpoint_5: Y-buffer base addr."]
pub mod usboepbbay_5;
#[doc = "USBOEPBCTY_5 (rw) register accessor: Output Endpoint_5: Y-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbcty_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbcty_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbcty_5`]
module"]
pub type USBOEPBCTY_5 = crate::Reg<usboepbcty_5::USBOEPBCTY_5_SPEC>;
#[doc = "Output Endpoint_5: Y-byte count"]
pub mod usboepbcty_5;
#[doc = "USBOEPSIZXY_5 (rw) register accessor: Output Endpoint_5: X/Y-buffer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepsizxy_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepsizxy_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepsizxy_5`]
module"]
pub type USBOEPSIZXY_5 = crate::Reg<usboepsizxy_5::USBOEPSIZXY_5_SPEC>;
#[doc = "Output Endpoint_5: X/Y-buffer size"]
pub mod usboepsizxy_5;
#[doc = "USBOEPCNF_6 (rw) register accessor: Output Endpoint_6: Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepcnf_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepcnf_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepcnf_6`]
module"]
pub type USBOEPCNF_6 = crate::Reg<usboepcnf_6::USBOEPCNF_6_SPEC>;
#[doc = "Output Endpoint_6: Configuration"]
pub mod usboepcnf_6;
#[doc = "USBOEPBBAX_6 (rw) register accessor: Output Endpoint_6: X-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbbax_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbbax_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbbax_6`]
module"]
pub type USBOEPBBAX_6 = crate::Reg<usboepbbax_6::USBOEPBBAX_6_SPEC>;
#[doc = "Output Endpoint_6: X-buffer base addr."]
pub mod usboepbbax_6;
#[doc = "USBOEPBCTX_6 (rw) register accessor: Output Endpoint_6: X-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbctx_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbctx_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbctx_6`]
module"]
pub type USBOEPBCTX_6 = crate::Reg<usboepbctx_6::USBOEPBCTX_6_SPEC>;
#[doc = "Output Endpoint_6: X-byte count"]
pub mod usboepbctx_6;
#[doc = "USBOEPBBAY_6 (rw) register accessor: Output Endpoint_6: Y-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbbay_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbbay_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbbay_6`]
module"]
pub type USBOEPBBAY_6 = crate::Reg<usboepbbay_6::USBOEPBBAY_6_SPEC>;
#[doc = "Output Endpoint_6: Y-buffer base addr."]
pub mod usboepbbay_6;
#[doc = "USBOEPBCTY_6 (rw) register accessor: Output Endpoint_6: Y-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbcty_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbcty_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbcty_6`]
module"]
pub type USBOEPBCTY_6 = crate::Reg<usboepbcty_6::USBOEPBCTY_6_SPEC>;
#[doc = "Output Endpoint_6: Y-byte count"]
pub mod usboepbcty_6;
#[doc = "USBOEPSIZXY_6 (rw) register accessor: Output Endpoint_6: X/Y-buffer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepsizxy_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepsizxy_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepsizxy_6`]
module"]
pub type USBOEPSIZXY_6 = crate::Reg<usboepsizxy_6::USBOEPSIZXY_6_SPEC>;
#[doc = "Output Endpoint_6: X/Y-buffer size"]
pub mod usboepsizxy_6;
#[doc = "USBOEPCNF_7 (rw) register accessor: Output Endpoint_7: Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepcnf_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepcnf_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepcnf_7`]
module"]
pub type USBOEPCNF_7 = crate::Reg<usboepcnf_7::USBOEPCNF_7_SPEC>;
#[doc = "Output Endpoint_7: Configuration"]
pub mod usboepcnf_7;
#[doc = "USBOEPBBAX_7 (rw) register accessor: Output Endpoint_7: X-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbbax_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbbax_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbbax_7`]
module"]
pub type USBOEPBBAX_7 = crate::Reg<usboepbbax_7::USBOEPBBAX_7_SPEC>;
#[doc = "Output Endpoint_7: X-buffer base addr."]
pub mod usboepbbax_7;
#[doc = "USBOEPBCTX_7 (rw) register accessor: Output Endpoint_7: X-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbctx_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbctx_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbctx_7`]
module"]
pub type USBOEPBCTX_7 = crate::Reg<usboepbctx_7::USBOEPBCTX_7_SPEC>;
#[doc = "Output Endpoint_7: X-byte count"]
pub mod usboepbctx_7;
#[doc = "USBOEPBBAY_7 (rw) register accessor: Output Endpoint_7: Y-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbbay_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbbay_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbbay_7`]
module"]
pub type USBOEPBBAY_7 = crate::Reg<usboepbbay_7::USBOEPBBAY_7_SPEC>;
#[doc = "Output Endpoint_7: Y-buffer base addr."]
pub mod usboepbbay_7;
#[doc = "USBOEPBCTY_7 (rw) register accessor: Output Endpoint_7: Y-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepbcty_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepbcty_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepbcty_7`]
module"]
pub type USBOEPBCTY_7 = crate::Reg<usboepbcty_7::USBOEPBCTY_7_SPEC>;
#[doc = "Output Endpoint_7: Y-byte count"]
pub mod usboepbcty_7;
#[doc = "USBOEPSIZXY_7 (rw) register accessor: Output Endpoint_7: X/Y-buffer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usboepsizxy_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usboepsizxy_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usboepsizxy_7`]
module"]
pub type USBOEPSIZXY_7 = crate::Reg<usboepsizxy_7::USBOEPSIZXY_7_SPEC>;
#[doc = "Output Endpoint_7: X/Y-buffer size"]
pub mod usboepsizxy_7;
#[doc = "USBIEPCNF_1 (rw) register accessor: Input Endpoint_1: Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepcnf_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepcnf_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepcnf_1`]
module"]
pub type USBIEPCNF_1 = crate::Reg<usbiepcnf_1::USBIEPCNF_1_SPEC>;
#[doc = "Input Endpoint_1: Configuration"]
pub mod usbiepcnf_1;
#[doc = "USBIEPBBAX_1 (rw) register accessor: Input Endpoint_1: X-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbbax_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbbax_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbbax_1`]
module"]
pub type USBIEPBBAX_1 = crate::Reg<usbiepbbax_1::USBIEPBBAX_1_SPEC>;
#[doc = "Input Endpoint_1: X-buffer base addr."]
pub mod usbiepbbax_1;
#[doc = "USBIEPBCTX_1 (rw) register accessor: Input Endpoint_1: X-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbctx_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbctx_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbctx_1`]
module"]
pub type USBIEPBCTX_1 = crate::Reg<usbiepbctx_1::USBIEPBCTX_1_SPEC>;
#[doc = "Input Endpoint_1: X-byte count"]
pub mod usbiepbctx_1;
#[doc = "USBIEPBBAY_1 (rw) register accessor: Input Endpoint_1: Y-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbbay_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbbay_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbbay_1`]
module"]
pub type USBIEPBBAY_1 = crate::Reg<usbiepbbay_1::USBIEPBBAY_1_SPEC>;
#[doc = "Input Endpoint_1: Y-buffer base addr."]
pub mod usbiepbbay_1;
#[doc = "USBIEPBCTY_1 (rw) register accessor: Input Endpoint_1: Y-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbcty_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbcty_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbcty_1`]
module"]
pub type USBIEPBCTY_1 = crate::Reg<usbiepbcty_1::USBIEPBCTY_1_SPEC>;
#[doc = "Input Endpoint_1: Y-byte count"]
pub mod usbiepbcty_1;
#[doc = "USBIEPSIZXY_1 (rw) register accessor: Input Endpoint_1: X/Y-buffer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepsizxy_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepsizxy_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepsizxy_1`]
module"]
pub type USBIEPSIZXY_1 = crate::Reg<usbiepsizxy_1::USBIEPSIZXY_1_SPEC>;
#[doc = "Input Endpoint_1: X/Y-buffer size"]
pub mod usbiepsizxy_1;
#[doc = "USBIEPCNF_2 (rw) register accessor: Input Endpoint_2: Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepcnf_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepcnf_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepcnf_2`]
module"]
pub type USBIEPCNF_2 = crate::Reg<usbiepcnf_2::USBIEPCNF_2_SPEC>;
#[doc = "Input Endpoint_2: Configuration"]
pub mod usbiepcnf_2;
#[doc = "USBIEPBBAX_2 (rw) register accessor: Input Endpoint_2: X-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbbax_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbbax_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbbax_2`]
module"]
pub type USBIEPBBAX_2 = crate::Reg<usbiepbbax_2::USBIEPBBAX_2_SPEC>;
#[doc = "Input Endpoint_2: X-buffer base addr."]
pub mod usbiepbbax_2;
#[doc = "USBIEPBCTX_2 (rw) register accessor: Input Endpoint_2: X-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbctx_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbctx_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbctx_2`]
module"]
pub type USBIEPBCTX_2 = crate::Reg<usbiepbctx_2::USBIEPBCTX_2_SPEC>;
#[doc = "Input Endpoint_2: X-byte count"]
pub mod usbiepbctx_2;
#[doc = "USBIEPBBAY_2 (rw) register accessor: Input Endpoint_2: Y-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbbay_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbbay_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbbay_2`]
module"]
pub type USBIEPBBAY_2 = crate::Reg<usbiepbbay_2::USBIEPBBAY_2_SPEC>;
#[doc = "Input Endpoint_2: Y-buffer base addr."]
pub mod usbiepbbay_2;
#[doc = "USBIEPBCTY_2 (rw) register accessor: Input Endpoint_2: Y-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbcty_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbcty_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbcty_2`]
module"]
pub type USBIEPBCTY_2 = crate::Reg<usbiepbcty_2::USBIEPBCTY_2_SPEC>;
#[doc = "Input Endpoint_2: Y-byte count"]
pub mod usbiepbcty_2;
#[doc = "USBIEPSIZXY_2 (rw) register accessor: Input Endpoint_2: X/Y-buffer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepsizxy_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepsizxy_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepsizxy_2`]
module"]
pub type USBIEPSIZXY_2 = crate::Reg<usbiepsizxy_2::USBIEPSIZXY_2_SPEC>;
#[doc = "Input Endpoint_2: X/Y-buffer size"]
pub mod usbiepsizxy_2;
#[doc = "USBIEPCNF_3 (rw) register accessor: Input Endpoint_3: Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepcnf_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepcnf_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepcnf_3`]
module"]
pub type USBIEPCNF_3 = crate::Reg<usbiepcnf_3::USBIEPCNF_3_SPEC>;
#[doc = "Input Endpoint_3: Configuration"]
pub mod usbiepcnf_3;
#[doc = "USBIEPBBAX_3 (rw) register accessor: Input Endpoint_3: X-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbbax_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbbax_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbbax_3`]
module"]
pub type USBIEPBBAX_3 = crate::Reg<usbiepbbax_3::USBIEPBBAX_3_SPEC>;
#[doc = "Input Endpoint_3: X-buffer base addr."]
pub mod usbiepbbax_3;
#[doc = "USBIEPBCTX_3 (rw) register accessor: Input Endpoint_3: X-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbctx_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbctx_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbctx_3`]
module"]
pub type USBIEPBCTX_3 = crate::Reg<usbiepbctx_3::USBIEPBCTX_3_SPEC>;
#[doc = "Input Endpoint_3: X-byte count"]
pub mod usbiepbctx_3;
#[doc = "USBIEPBBAY_3 (rw) register accessor: Input Endpoint_3: Y-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbbay_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbbay_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbbay_3`]
module"]
pub type USBIEPBBAY_3 = crate::Reg<usbiepbbay_3::USBIEPBBAY_3_SPEC>;
#[doc = "Input Endpoint_3: Y-buffer base addr."]
pub mod usbiepbbay_3;
#[doc = "USBIEPBCTY_3 (rw) register accessor: Input Endpoint_3: Y-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbcty_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbcty_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbcty_3`]
module"]
pub type USBIEPBCTY_3 = crate::Reg<usbiepbcty_3::USBIEPBCTY_3_SPEC>;
#[doc = "Input Endpoint_3: Y-byte count"]
pub mod usbiepbcty_3;
#[doc = "USBIEPSIZXY_3 (rw) register accessor: Input Endpoint_3: X/Y-buffer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepsizxy_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepsizxy_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepsizxy_3`]
module"]
pub type USBIEPSIZXY_3 = crate::Reg<usbiepsizxy_3::USBIEPSIZXY_3_SPEC>;
#[doc = "Input Endpoint_3: X/Y-buffer size"]
pub mod usbiepsizxy_3;
#[doc = "USBIEPCNF_4 (rw) register accessor: Input Endpoint_4: Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepcnf_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepcnf_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepcnf_4`]
module"]
pub type USBIEPCNF_4 = crate::Reg<usbiepcnf_4::USBIEPCNF_4_SPEC>;
#[doc = "Input Endpoint_4: Configuration"]
pub mod usbiepcnf_4;
#[doc = "USBIEPBBAX_4 (rw) register accessor: Input Endpoint_4: X-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbbax_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbbax_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbbax_4`]
module"]
pub type USBIEPBBAX_4 = crate::Reg<usbiepbbax_4::USBIEPBBAX_4_SPEC>;
#[doc = "Input Endpoint_4: X-buffer base addr."]
pub mod usbiepbbax_4;
#[doc = "USBIEPBCTX_4 (rw) register accessor: Input Endpoint_4: X-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbctx_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbctx_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbctx_4`]
module"]
pub type USBIEPBCTX_4 = crate::Reg<usbiepbctx_4::USBIEPBCTX_4_SPEC>;
#[doc = "Input Endpoint_4: X-byte count"]
pub mod usbiepbctx_4;
#[doc = "USBIEPBBAY_4 (rw) register accessor: Input Endpoint_4: Y-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbbay_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbbay_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbbay_4`]
module"]
pub type USBIEPBBAY_4 = crate::Reg<usbiepbbay_4::USBIEPBBAY_4_SPEC>;
#[doc = "Input Endpoint_4: Y-buffer base addr."]
pub mod usbiepbbay_4;
#[doc = "USBIEPBCTY_4 (rw) register accessor: Input Endpoint_4: Y-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbcty_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbcty_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbcty_4`]
module"]
pub type USBIEPBCTY_4 = crate::Reg<usbiepbcty_4::USBIEPBCTY_4_SPEC>;
#[doc = "Input Endpoint_4: Y-byte count"]
pub mod usbiepbcty_4;
#[doc = "USBIEPSIZXY_4 (rw) register accessor: Input Endpoint_4: X/Y-buffer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepsizxy_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepsizxy_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepsizxy_4`]
module"]
pub type USBIEPSIZXY_4 = crate::Reg<usbiepsizxy_4::USBIEPSIZXY_4_SPEC>;
#[doc = "Input Endpoint_4: X/Y-buffer size"]
pub mod usbiepsizxy_4;
#[doc = "USBIEPCNF_5 (rw) register accessor: Input Endpoint_5: Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepcnf_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepcnf_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepcnf_5`]
module"]
pub type USBIEPCNF_5 = crate::Reg<usbiepcnf_5::USBIEPCNF_5_SPEC>;
#[doc = "Input Endpoint_5: Configuration"]
pub mod usbiepcnf_5;
#[doc = "USBIEPBBAX_5 (rw) register accessor: Input Endpoint_5: X-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbbax_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbbax_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbbax_5`]
module"]
pub type USBIEPBBAX_5 = crate::Reg<usbiepbbax_5::USBIEPBBAX_5_SPEC>;
#[doc = "Input Endpoint_5: X-buffer base addr."]
pub mod usbiepbbax_5;
#[doc = "USBIEPBCTX_5 (rw) register accessor: Input Endpoint_5: X-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbctx_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbctx_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbctx_5`]
module"]
pub type USBIEPBCTX_5 = crate::Reg<usbiepbctx_5::USBIEPBCTX_5_SPEC>;
#[doc = "Input Endpoint_5: X-byte count"]
pub mod usbiepbctx_5;
#[doc = "USBIEPBBAY_5 (rw) register accessor: Input Endpoint_5: Y-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbbay_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbbay_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbbay_5`]
module"]
pub type USBIEPBBAY_5 = crate::Reg<usbiepbbay_5::USBIEPBBAY_5_SPEC>;
#[doc = "Input Endpoint_5: Y-buffer base addr."]
pub mod usbiepbbay_5;
#[doc = "USBIEPBCTY_5 (rw) register accessor: Input Endpoint_5: Y-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbcty_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbcty_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbcty_5`]
module"]
pub type USBIEPBCTY_5 = crate::Reg<usbiepbcty_5::USBIEPBCTY_5_SPEC>;
#[doc = "Input Endpoint_5: Y-byte count"]
pub mod usbiepbcty_5;
#[doc = "USBIEPSIZXY_5 (rw) register accessor: Input Endpoint_5: X/Y-buffer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepsizxy_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepsizxy_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepsizxy_5`]
module"]
pub type USBIEPSIZXY_5 = crate::Reg<usbiepsizxy_5::USBIEPSIZXY_5_SPEC>;
#[doc = "Input Endpoint_5: X/Y-buffer size"]
pub mod usbiepsizxy_5;
#[doc = "USBIEPCNF_6 (rw) register accessor: Input Endpoint_6: Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepcnf_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepcnf_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepcnf_6`]
module"]
pub type USBIEPCNF_6 = crate::Reg<usbiepcnf_6::USBIEPCNF_6_SPEC>;
#[doc = "Input Endpoint_6: Configuration"]
pub mod usbiepcnf_6;
#[doc = "USBIEPBBAX_6 (rw) register accessor: Input Endpoint_6: X-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbbax_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbbax_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbbax_6`]
module"]
pub type USBIEPBBAX_6 = crate::Reg<usbiepbbax_6::USBIEPBBAX_6_SPEC>;
#[doc = "Input Endpoint_6: X-buffer base addr."]
pub mod usbiepbbax_6;
#[doc = "USBIEPBCTX_6 (rw) register accessor: Input Endpoint_6: X-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbctx_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbctx_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbctx_6`]
module"]
pub type USBIEPBCTX_6 = crate::Reg<usbiepbctx_6::USBIEPBCTX_6_SPEC>;
#[doc = "Input Endpoint_6: X-byte count"]
pub mod usbiepbctx_6;
#[doc = "USBIEPBBAY_6 (rw) register accessor: Input Endpoint_6: Y-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbbay_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbbay_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbbay_6`]
module"]
pub type USBIEPBBAY_6 = crate::Reg<usbiepbbay_6::USBIEPBBAY_6_SPEC>;
#[doc = "Input Endpoint_6: Y-buffer base addr."]
pub mod usbiepbbay_6;
#[doc = "USBIEPBCTY_6 (rw) register accessor: Input Endpoint_6: Y-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbcty_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbcty_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbcty_6`]
module"]
pub type USBIEPBCTY_6 = crate::Reg<usbiepbcty_6::USBIEPBCTY_6_SPEC>;
#[doc = "Input Endpoint_6: Y-byte count"]
pub mod usbiepbcty_6;
#[doc = "USBIEPSIZXY_6 (rw) register accessor: Input Endpoint_6: X/Y-buffer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepsizxy_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepsizxy_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepsizxy_6`]
module"]
pub type USBIEPSIZXY_6 = crate::Reg<usbiepsizxy_6::USBIEPSIZXY_6_SPEC>;
#[doc = "Input Endpoint_6: X/Y-buffer size"]
pub mod usbiepsizxy_6;
#[doc = "USBIEPCNF_7 (rw) register accessor: Input Endpoint_7: Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepcnf_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepcnf_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepcnf_7`]
module"]
pub type USBIEPCNF_7 = crate::Reg<usbiepcnf_7::USBIEPCNF_7_SPEC>;
#[doc = "Input Endpoint_7: Configuration"]
pub mod usbiepcnf_7;
#[doc = "USBIEPBBAX_7 (rw) register accessor: Input Endpoint_7: X-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbbax_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbbax_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbbax_7`]
module"]
pub type USBIEPBBAX_7 = crate::Reg<usbiepbbax_7::USBIEPBBAX_7_SPEC>;
#[doc = "Input Endpoint_7: X-buffer base addr."]
pub mod usbiepbbax_7;
#[doc = "USBIEPBCTX_7 (rw) register accessor: Input Endpoint_7: X-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbctx_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbctx_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbctx_7`]
module"]
pub type USBIEPBCTX_7 = crate::Reg<usbiepbctx_7::USBIEPBCTX_7_SPEC>;
#[doc = "Input Endpoint_7: X-byte count"]
pub mod usbiepbctx_7;
#[doc = "USBIEPBBAY_7 (rw) register accessor: Input Endpoint_7: Y-buffer base addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbbay_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbbay_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbbay_7`]
module"]
pub type USBIEPBBAY_7 = crate::Reg<usbiepbbay_7::USBIEPBBAY_7_SPEC>;
#[doc = "Input Endpoint_7: Y-buffer base addr."]
pub mod usbiepbbay_7;
#[doc = "USBIEPBCTY_7 (rw) register accessor: Input Endpoint_7: Y-byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepbcty_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepbcty_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepbcty_7`]
module"]
pub type USBIEPBCTY_7 = crate::Reg<usbiepbcty_7::USBIEPBCTY_7_SPEC>;
#[doc = "Input Endpoint_7: Y-byte count"]
pub mod usbiepbcty_7;
#[doc = "USBIEPSIZXY_7 (rw) register accessor: Input Endpoint_7: X/Y-buffer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbiepsizxy_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbiepsizxy_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbiepsizxy_7`]
module"]
pub type USBIEPSIZXY_7 = crate::Reg<usbiepsizxy_7::USBIEPSIZXY_7_SPEC>;
#[doc = "Input Endpoint_7: X/Y-buffer size"]
pub mod usbiepsizxy_7;
