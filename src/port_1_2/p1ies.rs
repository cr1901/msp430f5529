#[doc = "Register `P1IES` reader"]
pub struct R(crate::R<P1IES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P1IES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P1IES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P1IES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P1IES` writer"]
pub struct W(crate::W<P1IES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P1IES_SPEC>;
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
impl From<crate::W<P1IES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P1IES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1IES0` reader - P1IES0"]
pub struct P1IES0_R(crate::FieldReader<bool, bool>);
impl P1IES0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IES0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IES0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IES0` writer - P1IES0"]
pub struct P1IES0_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IES0_W<'a> {
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
#[doc = "Field `P1IES1` reader - P1IES1"]
pub struct P1IES1_R(crate::FieldReader<bool, bool>);
impl P1IES1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IES1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IES1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IES1` writer - P1IES1"]
pub struct P1IES1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IES1_W<'a> {
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
#[doc = "Field `P1IES2` reader - P1IES2"]
pub struct P1IES2_R(crate::FieldReader<bool, bool>);
impl P1IES2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IES2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IES2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IES2` writer - P1IES2"]
pub struct P1IES2_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IES2_W<'a> {
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
#[doc = "Field `P1IES3` reader - P1IES3"]
pub struct P1IES3_R(crate::FieldReader<bool, bool>);
impl P1IES3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IES3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IES3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IES3` writer - P1IES3"]
pub struct P1IES3_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IES3_W<'a> {
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
#[doc = "Field `P1IES4` reader - P1IES4"]
pub struct P1IES4_R(crate::FieldReader<bool, bool>);
impl P1IES4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IES4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IES4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IES4` writer - P1IES4"]
pub struct P1IES4_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IES4_W<'a> {
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
#[doc = "Field `P1IES5` reader - P1IES5"]
pub struct P1IES5_R(crate::FieldReader<bool, bool>);
impl P1IES5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IES5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IES5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IES5` writer - P1IES5"]
pub struct P1IES5_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IES5_W<'a> {
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
#[doc = "Field `P1IES6` reader - P1IES6"]
pub struct P1IES6_R(crate::FieldReader<bool, bool>);
impl P1IES6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IES6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IES6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IES6` writer - P1IES6"]
pub struct P1IES6_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IES6_W<'a> {
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
#[doc = "Field `P1IES7` reader - P1IES7"]
pub struct P1IES7_R(crate::FieldReader<bool, bool>);
impl P1IES7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1IES7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IES7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IES7` writer - P1IES7"]
pub struct P1IES7_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IES7_W<'a> {
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
    #[doc = "Bit 0 - P1IES0"]
    #[inline(always)]
    pub fn p1ies0(&self) -> P1IES0_R {
        P1IES0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P1IES1"]
    #[inline(always)]
    pub fn p1ies1(&self) -> P1IES1_R {
        P1IES1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P1IES2"]
    #[inline(always)]
    pub fn p1ies2(&self) -> P1IES2_R {
        P1IES2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P1IES3"]
    #[inline(always)]
    pub fn p1ies3(&self) -> P1IES3_R {
        P1IES3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P1IES4"]
    #[inline(always)]
    pub fn p1ies4(&self) -> P1IES4_R {
        P1IES4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P1IES5"]
    #[inline(always)]
    pub fn p1ies5(&self) -> P1IES5_R {
        P1IES5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P1IES6"]
    #[inline(always)]
    pub fn p1ies6(&self) -> P1IES6_R {
        P1IES6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P1IES7"]
    #[inline(always)]
    pub fn p1ies7(&self) -> P1IES7_R {
        P1IES7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1IES0"]
    #[inline(always)]
    pub fn p1ies0(&mut self) -> P1IES0_W {
        P1IES0_W { w: self }
    }
    #[doc = "Bit 1 - P1IES1"]
    #[inline(always)]
    pub fn p1ies1(&mut self) -> P1IES1_W {
        P1IES1_W { w: self }
    }
    #[doc = "Bit 2 - P1IES2"]
    #[inline(always)]
    pub fn p1ies2(&mut self) -> P1IES2_W {
        P1IES2_W { w: self }
    }
    #[doc = "Bit 3 - P1IES3"]
    #[inline(always)]
    pub fn p1ies3(&mut self) -> P1IES3_W {
        P1IES3_W { w: self }
    }
    #[doc = "Bit 4 - P1IES4"]
    #[inline(always)]
    pub fn p1ies4(&mut self) -> P1IES4_W {
        P1IES4_W { w: self }
    }
    #[doc = "Bit 5 - P1IES5"]
    #[inline(always)]
    pub fn p1ies5(&mut self) -> P1IES5_W {
        P1IES5_W { w: self }
    }
    #[doc = "Bit 6 - P1IES6"]
    #[inline(always)]
    pub fn p1ies6(&mut self) -> P1IES6_W {
        P1IES6_W { w: self }
    }
    #[doc = "Bit 7 - P1IES7"]
    #[inline(always)]
    pub fn p1ies7(&mut self) -> P1IES7_W {
        P1IES7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 1 Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1ies](index.html) module"]
pub struct P1IES_SPEC;
impl crate::RegisterSpec for P1IES_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p1ies::R](R) reader structure"]
impl crate::Readable for P1IES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p1ies::W](W) writer structure"]
impl crate::Writable for P1IES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P1IES to value 0"]
impl crate::Resettable for P1IES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
