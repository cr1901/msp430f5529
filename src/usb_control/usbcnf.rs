#[doc = "Register `USBCNF` reader"]
pub struct R(crate::R<USBCNF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCNF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCNF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCNF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCNF` writer"]
pub struct W(crate::W<USBCNF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCNF_SPEC>;
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
impl From<crate::W<USBCNF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCNF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_EN` reader - USB - Module enable"]
pub struct USB_EN_R(crate::FieldReader<bool, bool>);
impl USB_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_EN` writer - USB - Module enable"]
pub struct USB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_EN_W<'a> {
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
#[doc = "Field `PUR_EN` reader - USB - PUR pin enable"]
pub struct PUR_EN_R(crate::FieldReader<bool, bool>);
impl PUR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PUR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUR_EN` writer - USB - PUR pin enable"]
pub struct PUR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PUR_EN_W<'a> {
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
#[doc = "Field `PUR_IN` reader - USB - PUR pin input value"]
pub struct PUR_IN_R(crate::FieldReader<bool, bool>);
impl PUR_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PUR_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUR_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUR_IN` writer - USB - PUR pin input value"]
pub struct PUR_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> PUR_IN_W<'a> {
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
#[doc = "Field `BLKRDY` reader - USB - Block ready signal for DMA"]
pub struct BLKRDY_R(crate::FieldReader<bool, bool>);
impl BLKRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLKRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLKRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLKRDY` writer - USB - Block ready signal for DMA"]
pub struct BLKRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `FNTEN` reader - USB - Frame Number receive Trigger enable for DMA"]
pub struct FNTEN_R(crate::FieldReader<bool, bool>);
impl FNTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FNTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FNTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FNTEN` writer - USB - Frame Number receive Trigger enable for DMA"]
pub struct FNTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FNTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB - Module enable"]
    #[inline(always)]
    pub fn usb_en(&self) -> USB_EN_R {
        USB_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB - PUR pin enable"]
    #[inline(always)]
    pub fn pur_en(&self) -> PUR_EN_R {
        PUR_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB - PUR pin input value"]
    #[inline(always)]
    pub fn pur_in(&self) -> PUR_IN_R {
        PUR_IN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB - Block ready signal for DMA"]
    #[inline(always)]
    pub fn blkrdy(&self) -> BLKRDY_R {
        BLKRDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB - Frame Number receive Trigger enable for DMA"]
    #[inline(always)]
    pub fn fnten(&self) -> FNTEN_R {
        FNTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB - Module enable"]
    #[inline(always)]
    pub fn usb_en(&mut self) -> USB_EN_W {
        USB_EN_W { w: self }
    }
    #[doc = "Bit 1 - USB - PUR pin enable"]
    #[inline(always)]
    pub fn pur_en(&mut self) -> PUR_EN_W {
        PUR_EN_W { w: self }
    }
    #[doc = "Bit 2 - USB - PUR pin input value"]
    #[inline(always)]
    pub fn pur_in(&mut self) -> PUR_IN_W {
        PUR_IN_W { w: self }
    }
    #[doc = "Bit 3 - USB - Block ready signal for DMA"]
    #[inline(always)]
    pub fn blkrdy(&mut self) -> BLKRDY_W {
        BLKRDY_W { w: self }
    }
    #[doc = "Bit 4 - USB - Frame Number receive Trigger enable for DMA"]
    #[inline(always)]
    pub fn fnten(&mut self) -> FNTEN_W {
        FNTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Module configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbcnf](index.html) module"]
pub struct USBCNF_SPEC;
impl crate::RegisterSpec for USBCNF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbcnf::R](R) reader structure"]
impl crate::Readable for USBCNF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbcnf::W](W) writer structure"]
impl crate::Writable for USBCNF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBCNF to value 0"]
impl crate::Resettable for USBCNF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
