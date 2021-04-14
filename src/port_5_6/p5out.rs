#[doc = "Register `P5OUT` reader"]
pub struct R(crate::R<P5OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P5OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P5OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P5OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P5OUT` writer"]
pub struct W(crate::W<P5OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P5OUT_SPEC>;
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
impl From<crate::W<P5OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P5OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P5OUT0` reader - P5OUT0"]
pub struct P5OUT0_R(crate::FieldReader<bool, bool>);
impl P5OUT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P5OUT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5OUT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5OUT0` writer - P5OUT0"]
pub struct P5OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> P5OUT0_W<'a> {
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
#[doc = "Field `P5OUT1` reader - P5OUT1"]
pub struct P5OUT1_R(crate::FieldReader<bool, bool>);
impl P5OUT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P5OUT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5OUT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5OUT1` writer - P5OUT1"]
pub struct P5OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> P5OUT1_W<'a> {
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
#[doc = "Field `P5OUT2` reader - P5OUT2"]
pub struct P5OUT2_R(crate::FieldReader<bool, bool>);
impl P5OUT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P5OUT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5OUT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5OUT2` writer - P5OUT2"]
pub struct P5OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> P5OUT2_W<'a> {
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
#[doc = "Field `P5OUT3` reader - P5OUT3"]
pub struct P5OUT3_R(crate::FieldReader<bool, bool>);
impl P5OUT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P5OUT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5OUT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5OUT3` writer - P5OUT3"]
pub struct P5OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> P5OUT3_W<'a> {
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
#[doc = "Field `P5OUT4` reader - P5OUT4"]
pub struct P5OUT4_R(crate::FieldReader<bool, bool>);
impl P5OUT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P5OUT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5OUT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5OUT4` writer - P5OUT4"]
pub struct P5OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> P5OUT4_W<'a> {
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
#[doc = "Field `P5OUT5` reader - P5OUT5"]
pub struct P5OUT5_R(crate::FieldReader<bool, bool>);
impl P5OUT5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P5OUT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5OUT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5OUT5` writer - P5OUT5"]
pub struct P5OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> P5OUT5_W<'a> {
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
#[doc = "Field `P5OUT6` reader - P5OUT6"]
pub struct P5OUT6_R(crate::FieldReader<bool, bool>);
impl P5OUT6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P5OUT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5OUT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5OUT6` writer - P5OUT6"]
pub struct P5OUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> P5OUT6_W<'a> {
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
#[doc = "Field `P5OUT7` reader - P5OUT7"]
pub struct P5OUT7_R(crate::FieldReader<bool, bool>);
impl P5OUT7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P5OUT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5OUT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5OUT7` writer - P5OUT7"]
pub struct P5OUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> P5OUT7_W<'a> {
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
    #[doc = "Bit 0 - P5OUT0"]
    #[inline(always)]
    pub fn p5out0(&self) -> P5OUT0_R {
        P5OUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P5OUT1"]
    #[inline(always)]
    pub fn p5out1(&self) -> P5OUT1_R {
        P5OUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P5OUT2"]
    #[inline(always)]
    pub fn p5out2(&self) -> P5OUT2_R {
        P5OUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P5OUT3"]
    #[inline(always)]
    pub fn p5out3(&self) -> P5OUT3_R {
        P5OUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P5OUT4"]
    #[inline(always)]
    pub fn p5out4(&self) -> P5OUT4_R {
        P5OUT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P5OUT5"]
    #[inline(always)]
    pub fn p5out5(&self) -> P5OUT5_R {
        P5OUT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P5OUT6"]
    #[inline(always)]
    pub fn p5out6(&self) -> P5OUT6_R {
        P5OUT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P5OUT7"]
    #[inline(always)]
    pub fn p5out7(&self) -> P5OUT7_R {
        P5OUT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P5OUT0"]
    #[inline(always)]
    pub fn p5out0(&mut self) -> P5OUT0_W {
        P5OUT0_W { w: self }
    }
    #[doc = "Bit 1 - P5OUT1"]
    #[inline(always)]
    pub fn p5out1(&mut self) -> P5OUT1_W {
        P5OUT1_W { w: self }
    }
    #[doc = "Bit 2 - P5OUT2"]
    #[inline(always)]
    pub fn p5out2(&mut self) -> P5OUT2_W {
        P5OUT2_W { w: self }
    }
    #[doc = "Bit 3 - P5OUT3"]
    #[inline(always)]
    pub fn p5out3(&mut self) -> P5OUT3_W {
        P5OUT3_W { w: self }
    }
    #[doc = "Bit 4 - P5OUT4"]
    #[inline(always)]
    pub fn p5out4(&mut self) -> P5OUT4_W {
        P5OUT4_W { w: self }
    }
    #[doc = "Bit 5 - P5OUT5"]
    #[inline(always)]
    pub fn p5out5(&mut self) -> P5OUT5_W {
        P5OUT5_W { w: self }
    }
    #[doc = "Bit 6 - P5OUT6"]
    #[inline(always)]
    pub fn p5out6(&mut self) -> P5OUT6_W {
        P5OUT6_W { w: self }
    }
    #[doc = "Bit 7 - P5OUT7"]
    #[inline(always)]
    pub fn p5out7(&mut self) -> P5OUT7_W {
        P5OUT7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 5 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5out](index.html) module"]
pub struct P5OUT_SPEC;
impl crate::RegisterSpec for P5OUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p5out::R](R) reader structure"]
impl crate::Readable for P5OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p5out::W](W) writer structure"]
impl crate::Writable for P5OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P5OUT to value 0"]
impl crate::Resettable for P5OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
