#[doc = "Register `SFRRPCR` reader"]
pub struct R(crate::R<SFRRPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFRRPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFRRPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFRRPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFRRPCR` writer"]
pub struct W(crate::W<SFRRPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFRRPCR_SPEC>;
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
impl From<crate::W<SFRRPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFRRPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSNMI` reader - NMI select"]
pub struct SYSNMI_R(crate::FieldReader<bool, bool>);
impl SYSNMI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSNMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSNMI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSNMI` writer - NMI select"]
pub struct SYSNMI_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSNMI_W<'a> {
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
#[doc = "Field `SYSNMIIES` reader - NMI edge select"]
pub struct SYSNMIIES_R(crate::FieldReader<bool, bool>);
impl SYSNMIIES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSNMIIES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSNMIIES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSNMIIES` writer - NMI edge select"]
pub struct SYSNMIIES_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSNMIIES_W<'a> {
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
#[doc = "Field `SYSRSTUP` reader - RESET Pin pull down/up select"]
pub struct SYSRSTUP_R(crate::FieldReader<bool, bool>);
impl SYSRSTUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSRSTUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSRSTUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSRSTUP` writer - RESET Pin pull down/up select"]
pub struct SYSRSTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRSTUP_W<'a> {
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
#[doc = "Field `SYSRSTRE` reader - RESET Pin Resistor enable"]
pub struct SYSRSTRE_R(crate::FieldReader<bool, bool>);
impl SYSRSTRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSRSTRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSRSTRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSRSTRE` writer - RESET Pin Resistor enable"]
pub struct SYSRSTRE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRSTRE_W<'a> {
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
    #[doc = "Bit 0 - NMI select"]
    #[inline(always)]
    pub fn sysnmi(&self) -> SYSNMI_R {
        SYSNMI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - NMI edge select"]
    #[inline(always)]
    pub fn sysnmiies(&self) -> SYSNMIIES_R {
        SYSNMIIES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RESET Pin pull down/up select"]
    #[inline(always)]
    pub fn sysrstup(&self) -> SYSRSTUP_R {
        SYSRSTUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RESET Pin Resistor enable"]
    #[inline(always)]
    pub fn sysrstre(&self) -> SYSRSTRE_R {
        SYSRSTRE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NMI select"]
    #[inline(always)]
    pub fn sysnmi(&mut self) -> SYSNMI_W {
        SYSNMI_W { w: self }
    }
    #[doc = "Bit 1 - NMI edge select"]
    #[inline(always)]
    pub fn sysnmiies(&mut self) -> SYSNMIIES_W {
        SYSNMIIES_W { w: self }
    }
    #[doc = "Bit 2 - RESET Pin pull down/up select"]
    #[inline(always)]
    pub fn sysrstup(&mut self) -> SYSRSTUP_W {
        SYSRSTUP_W { w: self }
    }
    #[doc = "Bit 3 - RESET Pin Resistor enable"]
    #[inline(always)]
    pub fn sysrstre(&mut self) -> SYSRSTRE_W {
        SYSRSTRE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RESET Pin Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfrrpcr](index.html) module"]
pub struct SFRRPCR_SPEC;
impl crate::RegisterSpec for SFRRPCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sfrrpcr::R](R) reader structure"]
impl crate::Readable for SFRRPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfrrpcr::W](W) writer structure"]
impl crate::Writable for SFRRPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SFRRPCR to value 0"]
impl crate::Resettable for SFRRPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
