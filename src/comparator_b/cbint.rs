#[doc = "Register `CBINT` reader"]
pub struct R(crate::R<CBINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CBINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CBINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CBINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CBINT` writer"]
pub struct W(crate::W<CBINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CBINT_SPEC>;
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
impl From<crate::W<CBINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CBINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CBIFG` reader - Comp. B Interrupt Flag"]
pub struct CBIFG_R(crate::FieldReader<bool, bool>);
impl CBIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBIFG` writer - Comp. B Interrupt Flag"]
pub struct CBIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CBIFG_W<'a> {
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
#[doc = "Field `CBIIFG` reader - Comp. B Interrupt Flag Inverted Polarity"]
pub struct CBIIFG_R(crate::FieldReader<bool, bool>);
impl CBIIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBIIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBIIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBIIFG` writer - Comp. B Interrupt Flag Inverted Polarity"]
pub struct CBIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CBIIFG_W<'a> {
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
#[doc = "Field `CBIE` reader - Comp. B Interrupt Enable"]
pub struct CBIE_R(crate::FieldReader<bool, bool>);
impl CBIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBIE` writer - Comp. B Interrupt Enable"]
pub struct CBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CBIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CBIIE` reader - Comp. B Interrupt Enable Inverted Polarity"]
pub struct CBIIE_R(crate::FieldReader<bool, bool>);
impl CBIIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBIIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBIIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBIIE` writer - Comp. B Interrupt Enable Inverted Polarity"]
pub struct CBIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CBIIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comp. B Interrupt Flag"]
    #[inline(always)]
    pub fn cbifg(&self) -> CBIFG_R {
        CBIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comp. B Interrupt Flag Inverted Polarity"]
    #[inline(always)]
    pub fn cbiifg(&self) -> CBIIFG_R {
        CBIIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Comp. B Interrupt Enable"]
    #[inline(always)]
    pub fn cbie(&self) -> CBIE_R {
        CBIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Comp. B Interrupt Enable Inverted Polarity"]
    #[inline(always)]
    pub fn cbiie(&self) -> CBIIE_R {
        CBIIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. B Interrupt Flag"]
    #[inline(always)]
    pub fn cbifg(&mut self) -> CBIFG_W {
        CBIFG_W { w: self }
    }
    #[doc = "Bit 1 - Comp. B Interrupt Flag Inverted Polarity"]
    #[inline(always)]
    pub fn cbiifg(&mut self) -> CBIIFG_W {
        CBIIFG_W { w: self }
    }
    #[doc = "Bit 8 - Comp. B Interrupt Enable"]
    #[inline(always)]
    pub fn cbie(&mut self) -> CBIE_W {
        CBIE_W { w: self }
    }
    #[doc = "Bit 9 - Comp. B Interrupt Enable Inverted Polarity"]
    #[inline(always)]
    pub fn cbiie(&mut self) -> CBIIE_W {
        CBIIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator B Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbint](index.html) module"]
pub struct CBINT_SPEC;
impl crate::RegisterSpec for CBINT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cbint::R](R) reader structure"]
impl crate::Readable for CBINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cbint::W](W) writer structure"]
impl crate::Writable for CBINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CBINT to value 0"]
impl crate::Resettable for CBINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
