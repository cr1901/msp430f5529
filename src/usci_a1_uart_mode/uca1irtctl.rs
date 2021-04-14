#[doc = "Register `UCA1IRTCTL` reader"]
pub struct R(crate::R<UCA1IRTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA1IRTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA1IRTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA1IRTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA1IRTCTL` writer"]
pub struct W(crate::W<UCA1IRTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA1IRTCTL_SPEC>;
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
impl From<crate::W<UCA1IRTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA1IRTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCIREN` reader - IRDA Encoder/Decoder enable"]
pub struct UCIREN_R(crate::FieldReader<bool, bool>);
impl UCIREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCIREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIREN` writer - IRDA Encoder/Decoder enable"]
pub struct UCIREN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIREN_W<'a> {
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
#[doc = "Field `UCIRTXCLK` reader - IRDA Transmit Pulse Clock Select"]
pub struct UCIRTXCLK_R(crate::FieldReader<bool, bool>);
impl UCIRTXCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCIRTXCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIRTXCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRTXCLK` writer - IRDA Transmit Pulse Clock Select"]
pub struct UCIRTXCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXCLK_W<'a> {
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
#[doc = "Field `UCIRTXPL0` reader - IRDA Transmit Pulse Length 0"]
pub struct UCIRTXPL0_R(crate::FieldReader<bool, bool>);
impl UCIRTXPL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCIRTXPL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIRTXPL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRTXPL0` writer - IRDA Transmit Pulse Length 0"]
pub struct UCIRTXPL0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXPL0_W<'a> {
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
#[doc = "Field `UCIRTXPL1` reader - IRDA Transmit Pulse Length 1"]
pub struct UCIRTXPL1_R(crate::FieldReader<bool, bool>);
impl UCIRTXPL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCIRTXPL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIRTXPL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRTXPL1` writer - IRDA Transmit Pulse Length 1"]
pub struct UCIRTXPL1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXPL1_W<'a> {
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
#[doc = "Field `UCIRTXPL2` reader - IRDA Transmit Pulse Length 2"]
pub struct UCIRTXPL2_R(crate::FieldReader<bool, bool>);
impl UCIRTXPL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCIRTXPL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIRTXPL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRTXPL2` writer - IRDA Transmit Pulse Length 2"]
pub struct UCIRTXPL2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXPL2_W<'a> {
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
#[doc = "Field `UCIRTXPL3` reader - IRDA Transmit Pulse Length 3"]
pub struct UCIRTXPL3_R(crate::FieldReader<bool, bool>);
impl UCIRTXPL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCIRTXPL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIRTXPL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRTXPL3` writer - IRDA Transmit Pulse Length 3"]
pub struct UCIRTXPL3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXPL3_W<'a> {
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
#[doc = "Field `UCIRTXPL4` reader - IRDA Transmit Pulse Length 4"]
pub struct UCIRTXPL4_R(crate::FieldReader<bool, bool>);
impl UCIRTXPL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCIRTXPL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIRTXPL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRTXPL4` writer - IRDA Transmit Pulse Length 4"]
pub struct UCIRTXPL4_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXPL4_W<'a> {
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
#[doc = "Field `UCIRTXPL5` reader - IRDA Transmit Pulse Length 5"]
pub struct UCIRTXPL5_R(crate::FieldReader<bool, bool>);
impl UCIRTXPL5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UCIRTXPL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIRTXPL5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRTXPL5` writer - IRDA Transmit Pulse Length 5"]
pub struct UCIRTXPL5_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXPL5_W<'a> {
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
    #[doc = "Bit 0 - IRDA Encoder/Decoder enable"]
    #[inline(always)]
    pub fn uciren(&self) -> UCIREN_R {
        UCIREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IRDA Transmit Pulse Clock Select"]
    #[inline(always)]
    pub fn ucirtxclk(&self) -> UCIRTXCLK_R {
        UCIRTXCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IRDA Transmit Pulse Length 0"]
    #[inline(always)]
    pub fn ucirtxpl0(&self) -> UCIRTXPL0_R {
        UCIRTXPL0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IRDA Transmit Pulse Length 1"]
    #[inline(always)]
    pub fn ucirtxpl1(&self) -> UCIRTXPL1_R {
        UCIRTXPL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IRDA Transmit Pulse Length 2"]
    #[inline(always)]
    pub fn ucirtxpl2(&self) -> UCIRTXPL2_R {
        UCIRTXPL2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IRDA Transmit Pulse Length 3"]
    #[inline(always)]
    pub fn ucirtxpl3(&self) -> UCIRTXPL3_R {
        UCIRTXPL3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IRDA Transmit Pulse Length 4"]
    #[inline(always)]
    pub fn ucirtxpl4(&self) -> UCIRTXPL4_R {
        UCIRTXPL4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IRDA Transmit Pulse Length 5"]
    #[inline(always)]
    pub fn ucirtxpl5(&self) -> UCIRTXPL5_R {
        UCIRTXPL5_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRDA Encoder/Decoder enable"]
    #[inline(always)]
    pub fn uciren(&mut self) -> UCIREN_W {
        UCIREN_W { w: self }
    }
    #[doc = "Bit 1 - IRDA Transmit Pulse Clock Select"]
    #[inline(always)]
    pub fn ucirtxclk(&mut self) -> UCIRTXCLK_W {
        UCIRTXCLK_W { w: self }
    }
    #[doc = "Bit 2 - IRDA Transmit Pulse Length 0"]
    #[inline(always)]
    pub fn ucirtxpl0(&mut self) -> UCIRTXPL0_W {
        UCIRTXPL0_W { w: self }
    }
    #[doc = "Bit 3 - IRDA Transmit Pulse Length 1"]
    #[inline(always)]
    pub fn ucirtxpl1(&mut self) -> UCIRTXPL1_W {
        UCIRTXPL1_W { w: self }
    }
    #[doc = "Bit 4 - IRDA Transmit Pulse Length 2"]
    #[inline(always)]
    pub fn ucirtxpl2(&mut self) -> UCIRTXPL2_W {
        UCIRTXPL2_W { w: self }
    }
    #[doc = "Bit 5 - IRDA Transmit Pulse Length 3"]
    #[inline(always)]
    pub fn ucirtxpl3(&mut self) -> UCIRTXPL3_W {
        UCIRTXPL3_W { w: self }
    }
    #[doc = "Bit 6 - IRDA Transmit Pulse Length 4"]
    #[inline(always)]
    pub fn ucirtxpl4(&mut self) -> UCIRTXPL4_W {
        UCIRTXPL4_W { w: self }
    }
    #[doc = "Bit 7 - IRDA Transmit Pulse Length 5"]
    #[inline(always)]
    pub fn ucirtxpl5(&mut self) -> UCIRTXPL5_W {
        UCIRTXPL5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A1 IrDA Transmit Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1irtctl](index.html) module"]
pub struct UCA1IRTCTL_SPEC;
impl crate::RegisterSpec for UCA1IRTCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca1irtctl::R](R) reader structure"]
impl crate::Readable for UCA1IRTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca1irtctl::W](W) writer structure"]
impl crate::Writable for UCA1IRTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA1IRTCTL to value 0"]
impl crate::Resettable for UCA1IRTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
