#[doc = "Register `USBOEPCNF_5` reader"]
pub struct R(crate::R<USBOEPCNF_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBOEPCNF_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBOEPCNF_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBOEPCNF_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBOEPCNF_5` writer"]
pub struct W(crate::W<USBOEPCNF_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBOEPCNF_5_SPEC>;
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
impl From<crate::W<USBOEPCNF_5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBOEPCNF_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBIIE` reader - USB - Transaction Interrupt indication enable"]
pub struct USBIIE_R(crate::FieldReader<bool, bool>);
impl USBIIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBIIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBIIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBIIE` writer - USB - Transaction Interrupt indication enable"]
pub struct USBIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBIIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `STALL` reader - USB - Stall Condition"]
pub struct STALL_R(crate::FieldReader<bool, bool>);
impl STALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL` writer - USB - Stall Condition"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DBUF` reader - USB - Double Buffer Enable"]
pub struct DBUF_R(crate::FieldReader<bool, bool>);
impl DBUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUF` writer - USB - Double Buffer Enable"]
pub struct DBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> DBUF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TOGGLE` reader - USB - Toggle Bit"]
pub struct TOGGLE_R(crate::FieldReader<bool, bool>);
impl TOGGLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOGGLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOGGLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOGGLE` writer - USB - Toggle Bit"]
pub struct TOGGLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOGGLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `UBME` reader - USB - UBM In-Endpoint Enable"]
pub struct UBME_R(crate::FieldReader<bool, bool>);
impl UBME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UBME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UBME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UBME` writer - USB - UBM In-Endpoint Enable"]
pub struct UBME_W<'a> {
    w: &'a mut W,
}
impl<'a> UBME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - USB - Transaction Interrupt indication enable"]
    #[inline(always)]
    pub fn usbiie(&self) -> USBIIE_R {
        USBIIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB - Stall Condition"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB - Double Buffer Enable"]
    #[inline(always)]
    pub fn dbuf(&self) -> DBUF_R {
        DBUF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB - Toggle Bit"]
    #[inline(always)]
    pub fn toggle(&self) -> TOGGLE_R {
        TOGGLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB - UBM In-Endpoint Enable"]
    #[inline(always)]
    pub fn ubme(&self) -> UBME_R {
        UBME_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - USB - Transaction Interrupt indication enable"]
    #[inline(always)]
    pub fn usbiie(&mut self) -> USBIIE_W {
        USBIIE_W { w: self }
    }
    #[doc = "Bit 3 - USB - Stall Condition"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Bit 4 - USB - Double Buffer Enable"]
    #[inline(always)]
    pub fn dbuf(&mut self) -> DBUF_W {
        DBUF_W { w: self }
    }
    #[doc = "Bit 5 - USB - Toggle Bit"]
    #[inline(always)]
    pub fn toggle(&mut self) -> TOGGLE_W {
        TOGGLE_W { w: self }
    }
    #[doc = "Bit 7 - USB - UBM In-Endpoint Enable"]
    #[inline(always)]
    pub fn ubme(&mut self) -> UBME_W {
        UBME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Endpoint_5: Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usboepcnf_5](index.html) module"]
pub struct USBOEPCNF_5_SPEC;
impl crate::RegisterSpec for USBOEPCNF_5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usboepcnf_5::R](R) reader structure"]
impl crate::Readable for USBOEPCNF_5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usboepcnf_5::W](W) writer structure"]
impl crate::Writable for USBOEPCNF_5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBOEPCNF_5 to value 0"]
impl crate::Resettable for USBOEPCNF_5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
