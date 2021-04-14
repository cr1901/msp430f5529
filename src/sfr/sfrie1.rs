#[doc = "Register `SFRIE1` reader"]
pub struct R(crate::R<SFRIE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFRIE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFRIE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFRIE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFRIE1` writer"]
pub struct W(crate::W<SFRIE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFRIE1_SPEC>;
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
impl From<crate::W<SFRIE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFRIE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTIE` reader - WDT Interrupt Enable"]
pub struct WDTIE_R(crate::FieldReader<bool, bool>);
impl WDTIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTIE` writer - WDT Interrupt Enable"]
pub struct WDTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTIE_W<'a> {
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
#[doc = "Field `OFIE` reader - Osc Fault Enable"]
pub struct OFIE_R(crate::FieldReader<bool, bool>);
impl OFIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFIE` writer - Osc Fault Enable"]
pub struct OFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OFIE_W<'a> {
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
#[doc = "Field `VMAIE` reader - Vacant Memory Interrupt Enable"]
pub struct VMAIE_R(crate::FieldReader<bool, bool>);
impl VMAIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VMAIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VMAIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VMAIE` writer - Vacant Memory Interrupt Enable"]
pub struct VMAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VMAIE_W<'a> {
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
#[doc = "Field `NMIIE` reader - NMI Interrupt Enable"]
pub struct NMIIE_R(crate::FieldReader<bool, bool>);
impl NMIIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NMIIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NMIIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMIIE` writer - NMI Interrupt Enable"]
pub struct NMIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIIE_W<'a> {
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
#[doc = "Field `ACCVIE` reader - Flash Access Violation Interrupt Enable"]
pub struct ACCVIE_R(crate::FieldReader<bool, bool>);
impl ACCVIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACCVIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACCVIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACCVIE` writer - Flash Access Violation Interrupt Enable"]
pub struct ACCVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCVIE_W<'a> {
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
#[doc = "Field `JMBINIE` reader - JTAG Mail Box input Interrupt Enable"]
pub struct JMBINIE_R(crate::FieldReader<bool, bool>);
impl JMBINIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JMBINIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JMBINIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JMBINIE` writer - JTAG Mail Box input Interrupt Enable"]
pub struct JMBINIE_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBINIE_W<'a> {
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
#[doc = "Field `JMBOUTIE` reader - JTAG Mail Box output Interrupt Enable"]
pub struct JMBOUTIE_R(crate::FieldReader<bool, bool>);
impl JMBOUTIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JMBOUTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JMBOUTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JMBOUTIE` writer - JTAG Mail Box output Interrupt Enable"]
pub struct JMBOUTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBOUTIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - WDT Interrupt Enable"]
    #[inline(always)]
    pub fn wdtie(&self) -> WDTIE_R {
        WDTIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Osc Fault Enable"]
    #[inline(always)]
    pub fn ofie(&self) -> OFIE_R {
        OFIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Vacant Memory Interrupt Enable"]
    #[inline(always)]
    pub fn vmaie(&self) -> VMAIE_R {
        VMAIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NMI Interrupt Enable"]
    #[inline(always)]
    pub fn nmiie(&self) -> NMIIE_R {
        NMIIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Flash Access Violation Interrupt Enable"]
    #[inline(always)]
    pub fn accvie(&self) -> ACCVIE_R {
        ACCVIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - JTAG Mail Box input Interrupt Enable"]
    #[inline(always)]
    pub fn jmbinie(&self) -> JMBINIE_R {
        JMBINIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - JTAG Mail Box output Interrupt Enable"]
    #[inline(always)]
    pub fn jmboutie(&self) -> JMBOUTIE_R {
        JMBOUTIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT Interrupt Enable"]
    #[inline(always)]
    pub fn wdtie(&mut self) -> WDTIE_W {
        WDTIE_W { w: self }
    }
    #[doc = "Bit 1 - Osc Fault Enable"]
    #[inline(always)]
    pub fn ofie(&mut self) -> OFIE_W {
        OFIE_W { w: self }
    }
    #[doc = "Bit 3 - Vacant Memory Interrupt Enable"]
    #[inline(always)]
    pub fn vmaie(&mut self) -> VMAIE_W {
        VMAIE_W { w: self }
    }
    #[doc = "Bit 4 - NMI Interrupt Enable"]
    #[inline(always)]
    pub fn nmiie(&mut self) -> NMIIE_W {
        NMIIE_W { w: self }
    }
    #[doc = "Bit 5 - Flash Access Violation Interrupt Enable"]
    #[inline(always)]
    pub fn accvie(&mut self) -> ACCVIE_W {
        ACCVIE_W { w: self }
    }
    #[doc = "Bit 6 - JTAG Mail Box input Interrupt Enable"]
    #[inline(always)]
    pub fn jmbinie(&mut self) -> JMBINIE_W {
        JMBINIE_W { w: self }
    }
    #[doc = "Bit 7 - JTAG Mail Box output Interrupt Enable"]
    #[inline(always)]
    pub fn jmboutie(&mut self) -> JMBOUTIE_W {
        JMBOUTIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfrie1](index.html) module"]
pub struct SFRIE1_SPEC;
impl crate::RegisterSpec for SFRIE1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sfrie1::R](R) reader structure"]
impl crate::Readable for SFRIE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfrie1::W](W) writer structure"]
impl crate::Writable for SFRIE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SFRIE1 to value 0"]
impl crate::Resettable for SFRIE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
