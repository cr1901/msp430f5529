#[doc = "Register `PMMCTL1` reader"]
pub struct R(crate::R<PMMCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMMCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMMCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMMCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMMCTL1` writer"]
pub struct W(crate::W<PMMCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMMCTL1_SPEC>;
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
impl From<crate::W<PMMCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMMCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMMREFMD` reader - PMM Reference Mode"]
pub struct PMMREFMD_R(crate::FieldReader<bool, bool>);
impl PMMREFMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMMREFMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMMREFMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMMREFMD` writer - PMM Reference Mode"]
pub struct PMMREFMD_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMREFMD_W<'a> {
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
#[doc = "Field `PMMCMD0` reader - PMM Voltage Regulator Current Mode Bit: 0"]
pub struct PMMCMD0_R(crate::FieldReader<bool, bool>);
impl PMMCMD0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMMCMD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMMCMD0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMMCMD0` writer - PMM Voltage Regulator Current Mode Bit: 0"]
pub struct PMMCMD0_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMCMD0_W<'a> {
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
#[doc = "Field `PMMCMD1` reader - PMM Voltage Regulator Current Mode Bit: 1"]
pub struct PMMCMD1_R(crate::FieldReader<bool, bool>);
impl PMMCMD1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMMCMD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMMCMD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMMCMD1` writer - PMM Voltage Regulator Current Mode Bit: 1"]
pub struct PMMCMD1_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMCMD1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - PMM Reference Mode"]
    #[inline(always)]
    pub fn pmmrefmd(&self) -> PMMREFMD_R {
        PMMREFMD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - PMM Voltage Regulator Current Mode Bit: 0"]
    #[inline(always)]
    pub fn pmmcmd0(&self) -> PMMCMD0_R {
        PMMCMD0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PMM Voltage Regulator Current Mode Bit: 1"]
    #[inline(always)]
    pub fn pmmcmd1(&self) -> PMMCMD1_R {
        PMMCMD1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PMM Reference Mode"]
    #[inline(always)]
    pub fn pmmrefmd(&mut self) -> PMMREFMD_W {
        PMMREFMD_W { w: self }
    }
    #[doc = "Bit 4 - PMM Voltage Regulator Current Mode Bit: 0"]
    #[inline(always)]
    pub fn pmmcmd0(&mut self) -> PMMCMD0_W {
        PMMCMD0_W { w: self }
    }
    #[doc = "Bit 5 - PMM Voltage Regulator Current Mode Bit: 1"]
    #[inline(always)]
    pub fn pmmcmd1(&mut self) -> PMMCMD1_W {
        PMMCMD1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMM Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmctl1](index.html) module"]
pub struct PMMCTL1_SPEC;
impl crate::RegisterSpec for PMMCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pmmctl1::R](R) reader structure"]
impl crate::Readable for PMMCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmmctl1::W](W) writer structure"]
impl crate::Writable for PMMCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMMCTL1 to value 0"]
impl crate::Resettable for PMMCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
