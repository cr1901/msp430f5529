#[doc = "Register `UCB0IFG` reader"]
pub struct R(crate::R<UCB0IFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0IFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0IFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0IFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0IFG` writer"]
pub struct W(crate::W<UCB0IFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0IFG_SPEC>;
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
impl From<crate::W<UCB0IFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0IFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCRXIFG` reader - USCI Receive Interrupt Flag"]
pub struct UCRXIFG_R(crate::FieldReader<bool, bool>);
impl UCRXIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCRXIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCRXIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCRXIFG` writer - USCI Receive Interrupt Flag"]
pub struct UCRXIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIFG_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `UCTXIFG` reader - USCI Transmit Interrupt Flag"]
pub struct UCTXIFG_R(crate::FieldReader<bool, bool>);
impl UCTXIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCTXIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCTXIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXIFG` writer - USCI Transmit Interrupt Flag"]
pub struct UCTXIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `UCSTTIFG` reader - START Condition interrupt Flag"]
pub struct UCSTTIFG_R(crate::FieldReader<bool, bool>);
impl UCSTTIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCSTTIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSTTIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSTTIFG` writer - START Condition interrupt Flag"]
pub struct UCSTTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTTIFG_W<'a> {
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
#[doc = "Field `UCSTPIFG` reader - STOP Condition interrupt Flag"]
pub struct UCSTPIFG_R(crate::FieldReader<bool, bool>);
impl UCSTPIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCSTPIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSTPIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSTPIFG` writer - STOP Condition interrupt Flag"]
pub struct UCSTPIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTPIFG_W<'a> {
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
#[doc = "Field `UCALIFG` reader - Arbitration Lost interrupt Flag"]
pub struct UCALIFG_R(crate::FieldReader<bool, bool>);
impl UCALIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCALIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCALIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCALIFG` writer - Arbitration Lost interrupt Flag"]
pub struct UCALIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCALIFG_W<'a> {
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
#[doc = "Field `UCNACKIFG` reader - NAK Condition interrupt Flag"]
pub struct UCNACKIFG_R(crate::FieldReader<bool, bool>);
impl UCNACKIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCNACKIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCNACKIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCNACKIFG` writer - NAK Condition interrupt Flag"]
pub struct UCNACKIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCNACKIFG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - USCI Receive Interrupt Flag"]
    #[inline(always)]
    pub fn ucrxifg(&self) -> UCRXIFG_R {
        UCRXIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Flag"]
    #[inline(always)]
    pub fn uctxifg(&self) -> UCTXIFG_R {
        UCTXIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - START Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucsttifg(&self) -> UCSTTIFG_R {
        UCSTTIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - STOP Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucstpifg(&self) -> UCSTPIFG_R {
        UCSTPIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Arbitration Lost interrupt Flag"]
    #[inline(always)]
    pub fn ucalifg(&self) -> UCALIFG_R {
        UCALIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - NAK Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucnackifg(&self) -> UCNACKIFG_R {
        UCNACKIFG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Receive Interrupt Flag"]
    #[inline(always)]
    pub fn ucrxifg(&mut self) -> UCRXIFG_W {
        UCRXIFG_W { w: self }
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Flag"]
    #[inline(always)]
    pub fn uctxifg(&mut self) -> UCTXIFG_W {
        UCTXIFG_W { w: self }
    }
    #[doc = "Bit 2 - START Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucsttifg(&mut self) -> UCSTTIFG_W {
        UCSTTIFG_W { w: self }
    }
    #[doc = "Bit 3 - STOP Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucstpifg(&mut self) -> UCSTPIFG_W {
        UCSTPIFG_W { w: self }
    }
    #[doc = "Bit 4 - Arbitration Lost interrupt Flag"]
    #[inline(always)]
    pub fn ucalifg(&mut self) -> UCALIFG_W {
        UCALIFG_W { w: self }
    }
    #[doc = "Bit 5 - NAK Condition interrupt Flag"]
    #[inline(always)]
    pub fn ucnackifg(&mut self) -> UCNACKIFG_W {
        UCNACKIFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 Interrupt Flags Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ifg](index.html) module"]
pub struct UCB0IFG_SPEC;
impl crate::RegisterSpec for UCB0IFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucb0ifg::R](R) reader structure"]
impl crate::Readable for UCB0IFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0ifg::W](W) writer structure"]
impl crate::Writable for UCB0IFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0IFG to value 0"]
impl crate::Resettable for UCB0IFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
