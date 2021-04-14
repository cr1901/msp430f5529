#[doc = "Register `UCSCTL0` reader"]
pub struct R(crate::R<UCSCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSCTL0` writer"]
pub struct W(crate::W<UCSCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSCTL0_SPEC>;
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
impl From<crate::W<UCSCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOD0` reader - Modulation Bit Counter Bit : 0"]
pub struct MOD0_R(crate::FieldReader<bool, bool>);
impl MOD0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MOD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOD0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOD0` writer - Modulation Bit Counter Bit : 0"]
pub struct MOD0_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD0_W<'a> {
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
#[doc = "Field `MOD1` reader - Modulation Bit Counter Bit : 1"]
pub struct MOD1_R(crate::FieldReader<bool, bool>);
impl MOD1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MOD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOD1` writer - Modulation Bit Counter Bit : 1"]
pub struct MOD1_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD1_W<'a> {
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
#[doc = "Field `MOD2` reader - Modulation Bit Counter Bit : 2"]
pub struct MOD2_R(crate::FieldReader<bool, bool>);
impl MOD2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MOD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOD2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOD2` writer - Modulation Bit Counter Bit : 2"]
pub struct MOD2_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD2_W<'a> {
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
#[doc = "Field `MOD3` reader - Modulation Bit Counter Bit : 3"]
pub struct MOD3_R(crate::FieldReader<bool, bool>);
impl MOD3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MOD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOD3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOD3` writer - Modulation Bit Counter Bit : 3"]
pub struct MOD3_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD3_W<'a> {
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
#[doc = "Field `MOD4` reader - Modulation Bit Counter Bit : 4"]
pub struct MOD4_R(crate::FieldReader<bool, bool>);
impl MOD4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MOD4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOD4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOD4` writer - Modulation Bit Counter Bit : 4"]
pub struct MOD4_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD4_W<'a> {
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
#[doc = "Field `DCO0` reader - DCO TAP Bit : 0"]
pub struct DCO0_R(crate::FieldReader<bool, bool>);
impl DCO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCO0` writer - DCO TAP Bit : 0"]
pub struct DCO0_W<'a> {
    w: &'a mut W,
}
impl<'a> DCO0_W<'a> {
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
#[doc = "Field `DCO1` reader - DCO TAP Bit : 1"]
pub struct DCO1_R(crate::FieldReader<bool, bool>);
impl DCO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCO1` writer - DCO TAP Bit : 1"]
pub struct DCO1_W<'a> {
    w: &'a mut W,
}
impl<'a> DCO1_W<'a> {
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
#[doc = "Field `DCO2` reader - DCO TAP Bit : 2"]
pub struct DCO2_R(crate::FieldReader<bool, bool>);
impl DCO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCO2` writer - DCO TAP Bit : 2"]
pub struct DCO2_W<'a> {
    w: &'a mut W,
}
impl<'a> DCO2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `DCO3` reader - DCO TAP Bit : 3"]
pub struct DCO3_R(crate::FieldReader<bool, bool>);
impl DCO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCO3` writer - DCO TAP Bit : 3"]
pub struct DCO3_W<'a> {
    w: &'a mut W,
}
impl<'a> DCO3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u16 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `DCO4` reader - DCO TAP Bit : 4"]
pub struct DCO4_R(crate::FieldReader<bool, bool>);
impl DCO4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCO4` writer - DCO TAP Bit : 4"]
pub struct DCO4_W<'a> {
    w: &'a mut W,
}
impl<'a> DCO4_W<'a> {
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
impl R {
    #[doc = "Bit 3 - Modulation Bit Counter Bit : 0"]
    #[inline(always)]
    pub fn mod0(&self) -> MOD0_R {
        MOD0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Modulation Bit Counter Bit : 1"]
    #[inline(always)]
    pub fn mod1(&self) -> MOD1_R {
        MOD1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Modulation Bit Counter Bit : 2"]
    #[inline(always)]
    pub fn mod2(&self) -> MOD2_R {
        MOD2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Modulation Bit Counter Bit : 3"]
    #[inline(always)]
    pub fn mod3(&self) -> MOD3_R {
        MOD3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Modulation Bit Counter Bit : 4"]
    #[inline(always)]
    pub fn mod4(&self) -> MOD4_R {
        MOD4_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DCO TAP Bit : 0"]
    #[inline(always)]
    pub fn dco0(&self) -> DCO0_R {
        DCO0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DCO TAP Bit : 1"]
    #[inline(always)]
    pub fn dco1(&self) -> DCO1_R {
        DCO1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DCO TAP Bit : 2"]
    #[inline(always)]
    pub fn dco2(&self) -> DCO2_R {
        DCO2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DCO TAP Bit : 3"]
    #[inline(always)]
    pub fn dco3(&self) -> DCO3_R {
        DCO3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DCO TAP Bit : 4"]
    #[inline(always)]
    pub fn dco4(&self) -> DCO4_R {
        DCO4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Modulation Bit Counter Bit : 0"]
    #[inline(always)]
    pub fn mod0(&mut self) -> MOD0_W {
        MOD0_W { w: self }
    }
    #[doc = "Bit 4 - Modulation Bit Counter Bit : 1"]
    #[inline(always)]
    pub fn mod1(&mut self) -> MOD1_W {
        MOD1_W { w: self }
    }
    #[doc = "Bit 5 - Modulation Bit Counter Bit : 2"]
    #[inline(always)]
    pub fn mod2(&mut self) -> MOD2_W {
        MOD2_W { w: self }
    }
    #[doc = "Bit 6 - Modulation Bit Counter Bit : 3"]
    #[inline(always)]
    pub fn mod3(&mut self) -> MOD3_W {
        MOD3_W { w: self }
    }
    #[doc = "Bit 7 - Modulation Bit Counter Bit : 4"]
    #[inline(always)]
    pub fn mod4(&mut self) -> MOD4_W {
        MOD4_W { w: self }
    }
    #[doc = "Bit 8 - DCO TAP Bit : 0"]
    #[inline(always)]
    pub fn dco0(&mut self) -> DCO0_W {
        DCO0_W { w: self }
    }
    #[doc = "Bit 9 - DCO TAP Bit : 1"]
    #[inline(always)]
    pub fn dco1(&mut self) -> DCO1_W {
        DCO1_W { w: self }
    }
    #[doc = "Bit 10 - DCO TAP Bit : 2"]
    #[inline(always)]
    pub fn dco2(&mut self) -> DCO2_W {
        DCO2_W { w: self }
    }
    #[doc = "Bit 11 - DCO TAP Bit : 3"]
    #[inline(always)]
    pub fn dco3(&mut self) -> DCO3_W {
        DCO3_W { w: self }
    }
    #[doc = "Bit 12 - DCO TAP Bit : 4"]
    #[inline(always)]
    pub fn dco4(&mut self) -> DCO4_W {
        DCO4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCS Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsctl0](index.html) module"]
pub struct UCSCTL0_SPEC;
impl crate::RegisterSpec for UCSCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucsctl0::R](R) reader structure"]
impl crate::Readable for UCSCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsctl0::W](W) writer structure"]
impl crate::Writable for UCSCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCSCTL0 to value 0"]
impl crate::Resettable for UCSCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
