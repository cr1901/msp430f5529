#[doc = "Register `P7OUT` reader"]
pub struct R(crate::R<P7OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P7OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P7OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P7OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P7OUT` writer"]
pub struct W(crate::W<P7OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P7OUT_SPEC>;
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
impl From<crate::W<P7OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P7OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P7OUT0` reader - P7OUT0"]
pub struct P7OUT0_R(crate::FieldReader<bool, bool>);
impl P7OUT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7OUT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7OUT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7OUT0` writer - P7OUT0"]
pub struct P7OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> P7OUT0_W<'a> {
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
#[doc = "Field `P7OUT1` reader - P7OUT1"]
pub struct P7OUT1_R(crate::FieldReader<bool, bool>);
impl P7OUT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7OUT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7OUT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7OUT1` writer - P7OUT1"]
pub struct P7OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> P7OUT1_W<'a> {
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
#[doc = "Field `P7OUT2` reader - P7OUT2"]
pub struct P7OUT2_R(crate::FieldReader<bool, bool>);
impl P7OUT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7OUT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7OUT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7OUT2` writer - P7OUT2"]
pub struct P7OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> P7OUT2_W<'a> {
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
#[doc = "Field `P7OUT3` reader - P7OUT3"]
pub struct P7OUT3_R(crate::FieldReader<bool, bool>);
impl P7OUT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7OUT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7OUT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7OUT3` writer - P7OUT3"]
pub struct P7OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> P7OUT3_W<'a> {
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
#[doc = "Field `P7OUT4` reader - P7OUT4"]
pub struct P7OUT4_R(crate::FieldReader<bool, bool>);
impl P7OUT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7OUT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7OUT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7OUT4` writer - P7OUT4"]
pub struct P7OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> P7OUT4_W<'a> {
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
#[doc = "Field `P7OUT5` reader - P7OUT5"]
pub struct P7OUT5_R(crate::FieldReader<bool, bool>);
impl P7OUT5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7OUT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7OUT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7OUT5` writer - P7OUT5"]
pub struct P7OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> P7OUT5_W<'a> {
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
#[doc = "Field `P7OUT6` reader - P7OUT6"]
pub struct P7OUT6_R(crate::FieldReader<bool, bool>);
impl P7OUT6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7OUT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7OUT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7OUT6` writer - P7OUT6"]
pub struct P7OUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> P7OUT6_W<'a> {
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
#[doc = "Field `P7OUT7` reader - P7OUT7"]
pub struct P7OUT7_R(crate::FieldReader<bool, bool>);
impl P7OUT7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7OUT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7OUT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7OUT7` writer - P7OUT7"]
pub struct P7OUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> P7OUT7_W<'a> {
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
    #[doc = "Bit 0 - P7OUT0"]
    #[inline(always)]
    pub fn p7out0(&self) -> P7OUT0_R {
        P7OUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P7OUT1"]
    #[inline(always)]
    pub fn p7out1(&self) -> P7OUT1_R {
        P7OUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P7OUT2"]
    #[inline(always)]
    pub fn p7out2(&self) -> P7OUT2_R {
        P7OUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P7OUT3"]
    #[inline(always)]
    pub fn p7out3(&self) -> P7OUT3_R {
        P7OUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P7OUT4"]
    #[inline(always)]
    pub fn p7out4(&self) -> P7OUT4_R {
        P7OUT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P7OUT5"]
    #[inline(always)]
    pub fn p7out5(&self) -> P7OUT5_R {
        P7OUT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P7OUT6"]
    #[inline(always)]
    pub fn p7out6(&self) -> P7OUT6_R {
        P7OUT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P7OUT7"]
    #[inline(always)]
    pub fn p7out7(&self) -> P7OUT7_R {
        P7OUT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7OUT0"]
    #[inline(always)]
    pub fn p7out0(&mut self) -> P7OUT0_W {
        P7OUT0_W { w: self }
    }
    #[doc = "Bit 1 - P7OUT1"]
    #[inline(always)]
    pub fn p7out1(&mut self) -> P7OUT1_W {
        P7OUT1_W { w: self }
    }
    #[doc = "Bit 2 - P7OUT2"]
    #[inline(always)]
    pub fn p7out2(&mut self) -> P7OUT2_W {
        P7OUT2_W { w: self }
    }
    #[doc = "Bit 3 - P7OUT3"]
    #[inline(always)]
    pub fn p7out3(&mut self) -> P7OUT3_W {
        P7OUT3_W { w: self }
    }
    #[doc = "Bit 4 - P7OUT4"]
    #[inline(always)]
    pub fn p7out4(&mut self) -> P7OUT4_W {
        P7OUT4_W { w: self }
    }
    #[doc = "Bit 5 - P7OUT5"]
    #[inline(always)]
    pub fn p7out5(&mut self) -> P7OUT5_W {
        P7OUT5_W { w: self }
    }
    #[doc = "Bit 6 - P7OUT6"]
    #[inline(always)]
    pub fn p7out6(&mut self) -> P7OUT6_W {
        P7OUT6_W { w: self }
    }
    #[doc = "Bit 7 - P7OUT7"]
    #[inline(always)]
    pub fn p7out7(&mut self) -> P7OUT7_W {
        P7OUT7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 7 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7out](index.html) module"]
pub struct P7OUT_SPEC;
impl crate::RegisterSpec for P7OUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p7out::R](R) reader structure"]
impl crate::Readable for P7OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p7out::W](W) writer structure"]
impl crate::Writable for P7OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P7OUT to value 0"]
impl crate::Resettable for P7OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
