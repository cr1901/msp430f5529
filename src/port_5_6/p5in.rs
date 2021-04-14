#[doc = "Register `P5IN` reader"]
pub struct R(crate::R<P5IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P5IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P5IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P5IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P5IN` writer"]
pub struct W(crate::W<P5IN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P5IN_SPEC>;
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
impl From<crate::W<P5IN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P5IN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P5IN0` reader - P5IN0"]
pub struct P5IN0_R(crate::FieldReader<bool, bool>);
impl P5IN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P5IN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5IN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5IN0` writer - P5IN0"]
pub struct P5IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IN0_W<'a> {
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
#[doc = "Field `P5IN1` reader - P5IN1"]
pub struct P5IN1_R(crate::FieldReader<bool, bool>);
impl P5IN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P5IN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5IN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5IN1` writer - P5IN1"]
pub struct P5IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IN1_W<'a> {
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
#[doc = "Field `P5IN2` reader - P5IN2"]
pub struct P5IN2_R(crate::FieldReader<bool, bool>);
impl P5IN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P5IN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5IN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5IN2` writer - P5IN2"]
pub struct P5IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IN2_W<'a> {
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
#[doc = "Field `P5IN3` reader - P5IN3"]
pub struct P5IN3_R(crate::FieldReader<bool, bool>);
impl P5IN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P5IN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5IN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5IN3` writer - P5IN3"]
pub struct P5IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IN3_W<'a> {
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
#[doc = "Field `P5IN4` reader - P5IN4"]
pub struct P5IN4_R(crate::FieldReader<bool, bool>);
impl P5IN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P5IN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5IN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5IN4` writer - P5IN4"]
pub struct P5IN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IN4_W<'a> {
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
#[doc = "Field `P5IN5` reader - P5IN5"]
pub struct P5IN5_R(crate::FieldReader<bool, bool>);
impl P5IN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P5IN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5IN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5IN5` writer - P5IN5"]
pub struct P5IN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IN5_W<'a> {
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
#[doc = "Field `P5IN6` reader - P5IN6"]
pub struct P5IN6_R(crate::FieldReader<bool, bool>);
impl P5IN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P5IN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5IN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5IN6` writer - P5IN6"]
pub struct P5IN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IN6_W<'a> {
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
#[doc = "Field `P5IN7` reader - P5IN7"]
pub struct P5IN7_R(crate::FieldReader<bool, bool>);
impl P5IN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P5IN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5IN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5IN7` writer - P5IN7"]
pub struct P5IN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IN7_W<'a> {
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
    #[doc = "Bit 0 - P5IN0"]
    #[inline(always)]
    pub fn p5in0(&self) -> P5IN0_R {
        P5IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P5IN1"]
    #[inline(always)]
    pub fn p5in1(&self) -> P5IN1_R {
        P5IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P5IN2"]
    #[inline(always)]
    pub fn p5in2(&self) -> P5IN2_R {
        P5IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P5IN3"]
    #[inline(always)]
    pub fn p5in3(&self) -> P5IN3_R {
        P5IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P5IN4"]
    #[inline(always)]
    pub fn p5in4(&self) -> P5IN4_R {
        P5IN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P5IN5"]
    #[inline(always)]
    pub fn p5in5(&self) -> P5IN5_R {
        P5IN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P5IN6"]
    #[inline(always)]
    pub fn p5in6(&self) -> P5IN6_R {
        P5IN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P5IN7"]
    #[inline(always)]
    pub fn p5in7(&self) -> P5IN7_R {
        P5IN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P5IN0"]
    #[inline(always)]
    pub fn p5in0(&mut self) -> P5IN0_W {
        P5IN0_W { w: self }
    }
    #[doc = "Bit 1 - P5IN1"]
    #[inline(always)]
    pub fn p5in1(&mut self) -> P5IN1_W {
        P5IN1_W { w: self }
    }
    #[doc = "Bit 2 - P5IN2"]
    #[inline(always)]
    pub fn p5in2(&mut self) -> P5IN2_W {
        P5IN2_W { w: self }
    }
    #[doc = "Bit 3 - P5IN3"]
    #[inline(always)]
    pub fn p5in3(&mut self) -> P5IN3_W {
        P5IN3_W { w: self }
    }
    #[doc = "Bit 4 - P5IN4"]
    #[inline(always)]
    pub fn p5in4(&mut self) -> P5IN4_W {
        P5IN4_W { w: self }
    }
    #[doc = "Bit 5 - P5IN5"]
    #[inline(always)]
    pub fn p5in5(&mut self) -> P5IN5_W {
        P5IN5_W { w: self }
    }
    #[doc = "Bit 6 - P5IN6"]
    #[inline(always)]
    pub fn p5in6(&mut self) -> P5IN6_W {
        P5IN6_W { w: self }
    }
    #[doc = "Bit 7 - P5IN7"]
    #[inline(always)]
    pub fn p5in7(&mut self) -> P5IN7_W {
        P5IN7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 5 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5in](index.html) module"]
pub struct P5IN_SPEC;
impl crate::RegisterSpec for P5IN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p5in::R](R) reader structure"]
impl crate::Readable for P5IN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p5in::W](W) writer structure"]
impl crate::Writable for P5IN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P5IN to value 0"]
impl crate::Resettable for P5IN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
