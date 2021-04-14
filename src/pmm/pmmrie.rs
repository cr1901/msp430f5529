#[doc = "Register `PMMRIE` reader"]
pub struct R(crate::R<PMMRIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMMRIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMMRIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMMRIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMMRIE` writer"]
pub struct W(crate::W<PMMRIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMMRIE_SPEC>;
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
impl From<crate::W<PMMRIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMMRIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SVSMLDLYIE` reader - SVS and SVM low side Delay expired interrupt enable"]
pub struct SVSMLDLYIE_R(crate::FieldReader<bool, bool>);
impl SVSMLDLYIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SVSMLDLYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SVSMLDLYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVSMLDLYIE` writer - SVS and SVM low side Delay expired interrupt enable"]
pub struct SVSMLDLYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSMLDLYIE_W<'a> {
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
#[doc = "Field `SVMLIE` reader - SVM low side interrupt enable"]
pub struct SVMLIE_R(crate::FieldReader<bool, bool>);
impl SVMLIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SVMLIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SVMLIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVMLIE` writer - SVM low side interrupt enable"]
pub struct SVMLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMLIE_W<'a> {
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
#[doc = "Field `SVMLVLRIE` reader - SVM low side Voltage Level Reached interrupt enable"]
pub struct SVMLVLRIE_R(crate::FieldReader<bool, bool>);
impl SVMLVLRIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SVMLVLRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SVMLVLRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVMLVLRIE` writer - SVM low side Voltage Level Reached interrupt enable"]
pub struct SVMLVLRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMLVLRIE_W<'a> {
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
#[doc = "Field `SVSMHDLYIE` reader - SVS and SVM high side Delay expired interrupt enable"]
pub struct SVSMHDLYIE_R(crate::FieldReader<bool, bool>);
impl SVSMHDLYIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SVSMHDLYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SVSMHDLYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVSMHDLYIE` writer - SVS and SVM high side Delay expired interrupt enable"]
pub struct SVSMHDLYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSMHDLYIE_W<'a> {
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
#[doc = "Field `SVMHIE` reader - SVM high side interrupt enable"]
pub struct SVMHIE_R(crate::FieldReader<bool, bool>);
impl SVMHIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SVMHIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SVMHIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVMHIE` writer - SVM high side interrupt enable"]
pub struct SVMHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMHIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `SVMHVLRIE` reader - SVM high side Voltage Level Reached interrupt enable"]
pub struct SVMHVLRIE_R(crate::FieldReader<bool, bool>);
impl SVMHVLRIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SVMHVLRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SVMHVLRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVMHVLRIE` writer - SVM high side Voltage Level Reached interrupt enable"]
pub struct SVMHVLRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMHVLRIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `SVSLPE` reader - SVS low side POR enable"]
pub struct SVSLPE_R(crate::FieldReader<bool, bool>);
impl SVSLPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SVSLPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SVSLPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVSLPE` writer - SVS low side POR enable"]
pub struct SVSLPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSLPE_W<'a> {
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
#[doc = "Field `SVMLVLRPE` reader - SVM low side Voltage Level reached POR enable"]
pub struct SVMLVLRPE_R(crate::FieldReader<bool, bool>);
impl SVMLVLRPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SVMLVLRPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SVMLVLRPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVMLVLRPE` writer - SVM low side Voltage Level reached POR enable"]
pub struct SVMLVLRPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMLVLRPE_W<'a> {
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
#[doc = "Field `SVSHPE` reader - SVS high side POR enable"]
pub struct SVSHPE_R(crate::FieldReader<bool, bool>);
impl SVSHPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SVSHPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SVSHPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVSHPE` writer - SVS high side POR enable"]
pub struct SVSHPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSHPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `SVMHVLRPE` reader - SVM high side Voltage Level reached POR enable"]
pub struct SVMHVLRPE_R(crate::FieldReader<bool, bool>);
impl SVMHVLRPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SVMHVLRPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SVMHVLRPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVMHVLRPE` writer - SVM high side Voltage Level reached POR enable"]
pub struct SVMHVLRPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMHVLRPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u16 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SVS and SVM low side Delay expired interrupt enable"]
    #[inline(always)]
    pub fn svsmldlyie(&self) -> SVSMLDLYIE_R {
        SVSMLDLYIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SVM low side interrupt enable"]
    #[inline(always)]
    pub fn svmlie(&self) -> SVMLIE_R {
        SVMLIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SVM low side Voltage Level Reached interrupt enable"]
    #[inline(always)]
    pub fn svmlvlrie(&self) -> SVMLVLRIE_R {
        SVMLVLRIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SVS and SVM high side Delay expired interrupt enable"]
    #[inline(always)]
    pub fn svsmhdlyie(&self) -> SVSMHDLYIE_R {
        SVSMHDLYIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SVM high side interrupt enable"]
    #[inline(always)]
    pub fn svmhie(&self) -> SVMHIE_R {
        SVMHIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SVM high side Voltage Level Reached interrupt enable"]
    #[inline(always)]
    pub fn svmhvlrie(&self) -> SVMHVLRIE_R {
        SVMHVLRIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SVS low side POR enable"]
    #[inline(always)]
    pub fn svslpe(&self) -> SVSLPE_R {
        SVSLPE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SVM low side Voltage Level reached POR enable"]
    #[inline(always)]
    pub fn svmlvlrpe(&self) -> SVMLVLRPE_R {
        SVMLVLRPE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SVS high side POR enable"]
    #[inline(always)]
    pub fn svshpe(&self) -> SVSHPE_R {
        SVSHPE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SVM high side Voltage Level reached POR enable"]
    #[inline(always)]
    pub fn svmhvlrpe(&self) -> SVMHVLRPE_R {
        SVMHVLRPE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SVS and SVM low side Delay expired interrupt enable"]
    #[inline(always)]
    pub fn svsmldlyie(&mut self) -> SVSMLDLYIE_W {
        SVSMLDLYIE_W { w: self }
    }
    #[doc = "Bit 1 - SVM low side interrupt enable"]
    #[inline(always)]
    pub fn svmlie(&mut self) -> SVMLIE_W {
        SVMLIE_W { w: self }
    }
    #[doc = "Bit 2 - SVM low side Voltage Level Reached interrupt enable"]
    #[inline(always)]
    pub fn svmlvlrie(&mut self) -> SVMLVLRIE_W {
        SVMLVLRIE_W { w: self }
    }
    #[doc = "Bit 4 - SVS and SVM high side Delay expired interrupt enable"]
    #[inline(always)]
    pub fn svsmhdlyie(&mut self) -> SVSMHDLYIE_W {
        SVSMHDLYIE_W { w: self }
    }
    #[doc = "Bit 5 - SVM high side interrupt enable"]
    #[inline(always)]
    pub fn svmhie(&mut self) -> SVMHIE_W {
        SVMHIE_W { w: self }
    }
    #[doc = "Bit 6 - SVM high side Voltage Level Reached interrupt enable"]
    #[inline(always)]
    pub fn svmhvlrie(&mut self) -> SVMHVLRIE_W {
        SVMHVLRIE_W { w: self }
    }
    #[doc = "Bit 8 - SVS low side POR enable"]
    #[inline(always)]
    pub fn svslpe(&mut self) -> SVSLPE_W {
        SVSLPE_W { w: self }
    }
    #[doc = "Bit 9 - SVM low side Voltage Level reached POR enable"]
    #[inline(always)]
    pub fn svmlvlrpe(&mut self) -> SVMLVLRPE_W {
        SVMLVLRPE_W { w: self }
    }
    #[doc = "Bit 12 - SVS high side POR enable"]
    #[inline(always)]
    pub fn svshpe(&mut self) -> SVSHPE_W {
        SVSHPE_W { w: self }
    }
    #[doc = "Bit 13 - SVM high side Voltage Level reached POR enable"]
    #[inline(always)]
    pub fn svmhvlrpe(&mut self) -> SVMHVLRPE_W {
        SVMHVLRPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMM and RESET Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmrie](index.html) module"]
pub struct PMMRIE_SPEC;
impl crate::RegisterSpec for PMMRIE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pmmrie::R](R) reader structure"]
impl crate::Readable for PMMRIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmmrie::W](W) writer structure"]
impl crate::Writable for PMMRIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMMRIE to value 0"]
impl crate::Resettable for PMMRIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
