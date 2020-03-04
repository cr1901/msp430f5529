#[doc = "Reader of register DMACTL4"]
pub type R = crate::R<u16, super::DMACTL4>;
#[doc = "Writer for register DMACTL4"]
pub type W = crate::W<u16, super::DMACTL4>;
#[doc = "Register DMACTL4 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACTL4 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENNMI`"]
pub type ENNMI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENNMI`"]
pub struct ENNMI_W<'a> {
    w: &'a mut W,
}
impl<'a> ENNMI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ROUNDROBIN`"]
pub type ROUNDROBIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROUNDROBIN`"]
pub struct ROUNDROBIN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROUNDROBIN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DMARMWDIS`"]
pub type DMARMWDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMARMWDIS`"]
pub struct DMARMWDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARMWDIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable NMI interruption of DMA"]
    #[inline(always)]
    pub fn ennmi(&self) -> ENNMI_R {
        ENNMI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Round-Robin DMA channel priorities"]
    #[inline(always)]
    pub fn roundrobin(&self) -> ROUNDROBIN_R {
        ROUNDROBIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Inhibited DMA transfers during read-modify-write CPU operations"]
    #[inline(always)]
    pub fn dmarmwdis(&self) -> DMARMWDIS_R {
        DMARMWDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable NMI interruption of DMA"]
    #[inline(always)]
    pub fn ennmi(&mut self) -> ENNMI_W {
        ENNMI_W { w: self }
    }
    #[doc = "Bit 1 - Round-Robin DMA channel priorities"]
    #[inline(always)]
    pub fn roundrobin(&mut self) -> ROUNDROBIN_W {
        ROUNDROBIN_W { w: self }
    }
    #[doc = "Bit 2 - Inhibited DMA transfers during read-modify-write CPU operations"]
    #[inline(always)]
    pub fn dmarmwdis(&mut self) -> DMARMWDIS_W {
        DMARMWDIS_W { w: self }
    }
}
