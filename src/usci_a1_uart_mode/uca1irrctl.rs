#[doc = "Register `UCA1IRRCTL` reader"]
pub struct R(crate::R<UCA1IRRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA1IRRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA1IRRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA1IRRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA1IRRCTL` writer"]
pub struct W(crate::W<UCA1IRRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA1IRRCTL_SPEC>;
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
impl From<crate::W<UCA1IRRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA1IRRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCIRRXFE` reader - IRDA Receive Filter enable"]
pub struct UCIRRXFE_R(crate::FieldReader<bool, bool>);
impl UCIRRXFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCIRRXFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIRRXFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRRXFE` writer - IRDA Receive Filter enable"]
pub struct UCIRRXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXFE_W<'a> {
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
#[doc = "Field `UCIRRXPL` reader - IRDA Receive Input Polarity"]
pub struct UCIRRXPL_R(crate::FieldReader<bool, bool>);
impl UCIRRXPL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCIRRXPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIRRXPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRRXPL` writer - IRDA Receive Input Polarity"]
pub struct UCIRRXPL_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXPL_W<'a> {
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
#[doc = "Field `UCIRRXFL0` reader - IRDA Receive Filter Length 0"]
pub struct UCIRRXFL0_R(crate::FieldReader<bool, bool>);
impl UCIRRXFL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCIRRXFL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIRRXFL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRRXFL0` writer - IRDA Receive Filter Length 0"]
pub struct UCIRRXFL0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXFL0_W<'a> {
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
#[doc = "Field `UCIRRXFL1` reader - IRDA Receive Filter Length 1"]
pub struct UCIRRXFL1_R(crate::FieldReader<bool, bool>);
impl UCIRRXFL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCIRRXFL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIRRXFL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRRXFL1` writer - IRDA Receive Filter Length 1"]
pub struct UCIRRXFL1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXFL1_W<'a> {
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
#[doc = "Field `UCIRRXFL2` reader - IRDA Receive Filter Length 2"]
pub struct UCIRRXFL2_R(crate::FieldReader<bool, bool>);
impl UCIRRXFL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCIRRXFL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIRRXFL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRRXFL2` writer - IRDA Receive Filter Length 2"]
pub struct UCIRRXFL2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXFL2_W<'a> {
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
#[doc = "Field `UCIRRXFL3` reader - IRDA Receive Filter Length 3"]
pub struct UCIRRXFL3_R(crate::FieldReader<bool, bool>);
impl UCIRRXFL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCIRRXFL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIRRXFL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRRXFL3` writer - IRDA Receive Filter Length 3"]
pub struct UCIRRXFL3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXFL3_W<'a> {
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
#[doc = "Field `UCIRRXFL4` reader - IRDA Receive Filter Length 4"]
pub struct UCIRRXFL4_R(crate::FieldReader<bool, bool>);
impl UCIRRXFL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCIRRXFL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIRRXFL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRRXFL4` writer - IRDA Receive Filter Length 4"]
pub struct UCIRRXFL4_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXFL4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `UCIRRXFL5` reader - IRDA Receive Filter Length 5"]
pub struct UCIRRXFL5_R(crate::FieldReader<bool, bool>);
impl UCIRRXFL5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCIRRXFL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIRRXFL5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRRXFL5` writer - IRDA Receive Filter Length 5"]
pub struct UCIRRXFL5_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXFL5_W<'a> {
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
    #[doc = "Bit 0 - IRDA Receive Filter enable"]
    #[inline(always)]
    pub fn ucirrxfe(&self) -> UCIRRXFE_R {
        UCIRRXFE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IRDA Receive Input Polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&self) -> UCIRRXPL_R {
        UCIRRXPL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IRDA Receive Filter Length 0"]
    #[inline(always)]
    pub fn ucirrxfl0(&self) -> UCIRRXFL0_R {
        UCIRRXFL0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IRDA Receive Filter Length 1"]
    #[inline(always)]
    pub fn ucirrxfl1(&self) -> UCIRRXFL1_R {
        UCIRRXFL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IRDA Receive Filter Length 2"]
    #[inline(always)]
    pub fn ucirrxfl2(&self) -> UCIRRXFL2_R {
        UCIRRXFL2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IRDA Receive Filter Length 3"]
    #[inline(always)]
    pub fn ucirrxfl3(&self) -> UCIRRXFL3_R {
        UCIRRXFL3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IRDA Receive Filter Length 4"]
    #[inline(always)]
    pub fn ucirrxfl4(&self) -> UCIRRXFL4_R {
        UCIRRXFL4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IRDA Receive Filter Length 5"]
    #[inline(always)]
    pub fn ucirrxfl5(&self) -> UCIRRXFL5_R {
        UCIRRXFL5_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRDA Receive Filter enable"]
    #[inline(always)]
    pub fn ucirrxfe(&mut self) -> UCIRRXFE_W {
        UCIRRXFE_W { w: self }
    }
    #[doc = "Bit 1 - IRDA Receive Input Polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&mut self) -> UCIRRXPL_W {
        UCIRRXPL_W { w: self }
    }
    #[doc = "Bit 2 - IRDA Receive Filter Length 0"]
    #[inline(always)]
    pub fn ucirrxfl0(&mut self) -> UCIRRXFL0_W {
        UCIRRXFL0_W { w: self }
    }
    #[doc = "Bit 3 - IRDA Receive Filter Length 1"]
    #[inline(always)]
    pub fn ucirrxfl1(&mut self) -> UCIRRXFL1_W {
        UCIRRXFL1_W { w: self }
    }
    #[doc = "Bit 4 - IRDA Receive Filter Length 2"]
    #[inline(always)]
    pub fn ucirrxfl2(&mut self) -> UCIRRXFL2_W {
        UCIRRXFL2_W { w: self }
    }
    #[doc = "Bit 5 - IRDA Receive Filter Length 3"]
    #[inline(always)]
    pub fn ucirrxfl3(&mut self) -> UCIRRXFL3_W {
        UCIRRXFL3_W { w: self }
    }
    #[doc = "Bit 6 - IRDA Receive Filter Length 4"]
    #[inline(always)]
    pub fn ucirrxfl4(&mut self) -> UCIRRXFL4_W {
        UCIRRXFL4_W { w: self }
    }
    #[doc = "Bit 7 - IRDA Receive Filter Length 5"]
    #[inline(always)]
    pub fn ucirrxfl5(&mut self) -> UCIRRXFL5_W {
        UCIRRXFL5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A1 IrDA Receive Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1irrctl](index.html) module"]
pub struct UCA1IRRCTL_SPEC;
impl crate::RegisterSpec for UCA1IRRCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca1irrctl::R](R) reader structure"]
impl crate::Readable for UCA1IRRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca1irrctl::W](W) writer structure"]
impl crate::Writable for UCA1IRRCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA1IRRCTL to value 0"]
impl crate::Resettable for UCA1IRRCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
