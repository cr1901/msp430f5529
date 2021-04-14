#[doc = "Register `DMACTL4` reader"]
pub struct R(crate::R<DMACTL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACTL4` writer"]
pub struct W(crate::W<DMACTL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMACTL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENNMI` reader - Enable NMI interruption of DMA"]
pub struct ENNMI_R(crate::FieldReader<bool, bool>);
impl ENNMI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENNMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENNMI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENNMI` writer - Enable NMI interruption of DMA"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `ROUNDROBIN` reader - Round-Robin DMA channel priorities"]
pub struct ROUNDROBIN_R(crate::FieldReader<bool, bool>);
impl ROUNDROBIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROUNDROBIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROUNDROBIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROUNDROBIN` writer - Round-Robin DMA channel priorities"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DMARMWDIS` reader - Inhibited DMA transfers during read-modify-write CPU operations"]
pub struct DMARMWDIS_R(crate::FieldReader<bool, bool>);
impl DMARMWDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMARMWDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMARMWDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMARMWDIS` writer - Inhibited DMA transfers during read-modify-write CPU operations"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Module Control 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl4](index.html) module"]
pub struct DMACTL4_SPEC;
impl crate::RegisterSpec for DMACTL4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dmactl4::R](R) reader structure"]
impl crate::Readable for DMACTL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmactl4::W](W) writer structure"]
impl crate::Writable for DMACTL4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACTL4 to value 0"]
impl crate::Resettable for DMACTL4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
