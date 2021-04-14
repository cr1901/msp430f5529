#[doc = "Register `UCSCTL7` reader"]
pub struct R(crate::R<UCSCTL7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSCTL7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSCTL7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSCTL7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSCTL7` writer"]
pub struct W(crate::W<UCSCTL7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSCTL7_SPEC>;
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
impl From<crate::W<UCSCTL7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSCTL7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCOFFG` reader - DCO Fault Flag"]
pub struct DCOFFG_R(crate::FieldReader<bool, bool>);
impl DCOFFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCOFFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCOFFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCOFFG` writer - DCO Fault Flag"]
pub struct DCOFFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOFFG_W<'a> {
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
#[doc = "Field `XT1LFOFFG` reader - XT1 Low Frequency Oscillator Fault Flag"]
pub struct XT1LFOFFG_R(crate::FieldReader<bool, bool>);
impl XT1LFOFFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XT1LFOFFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XT1LFOFFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XT1LFOFFG` writer - XT1 Low Frequency Oscillator Fault Flag"]
pub struct XT1LFOFFG_W<'a> {
    w: &'a mut W,
}
impl<'a> XT1LFOFFG_W<'a> {
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
#[doc = "Field `XT2OFFG` reader - High Frequency Oscillator 2 Fault Flag"]
pub struct XT2OFFG_R(crate::FieldReader<bool, bool>);
impl XT2OFFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XT2OFFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XT2OFFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XT2OFFG` writer - High Frequency Oscillator 2 Fault Flag"]
pub struct XT2OFFG_W<'a> {
    w: &'a mut W,
}
impl<'a> XT2OFFG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DCO Fault Flag"]
    #[inline(always)]
    pub fn dcoffg(&self) -> DCOFFG_R {
        DCOFFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XT1 Low Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn xt1lfoffg(&self) -> XT1LFOFFG_R {
        XT1LFOFFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - High Frequency Oscillator 2 Fault Flag"]
    #[inline(always)]
    pub fn xt2offg(&self) -> XT2OFFG_R {
        XT2OFFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCO Fault Flag"]
    #[inline(always)]
    pub fn dcoffg(&mut self) -> DCOFFG_W {
        DCOFFG_W { w: self }
    }
    #[doc = "Bit 1 - XT1 Low Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn xt1lfoffg(&mut self) -> XT1LFOFFG_W {
        XT1LFOFFG_W { w: self }
    }
    #[doc = "Bit 3 - High Frequency Oscillator 2 Fault Flag"]
    #[inline(always)]
    pub fn xt2offg(&mut self) -> XT2OFFG_W {
        XT2OFFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCS Control Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsctl7](index.html) module"]
pub struct UCSCTL7_SPEC;
impl crate::RegisterSpec for UCSCTL7_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucsctl7::R](R) reader structure"]
impl crate::Readable for UCSCTL7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsctl7::W](W) writer structure"]
impl crate::Writable for UCSCTL7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCSCTL7 to value 0"]
impl crate::Resettable for UCSCTL7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
