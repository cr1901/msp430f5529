#[doc = "Register `P6OUT` reader"]
pub struct R(crate::R<P6OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P6OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P6OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P6OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P6OUT` writer"]
pub struct W(crate::W<P6OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P6OUT_SPEC>;
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
impl From<crate::W<P6OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P6OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P6OUT0` reader - P6OUT0"]
pub struct P6OUT0_R(crate::FieldReader<bool, bool>);
impl P6OUT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6OUT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6OUT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6OUT0` writer - P6OUT0"]
pub struct P6OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> P6OUT0_W<'a> {
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
#[doc = "Field `P6OUT1` reader - P6OUT1"]
pub struct P6OUT1_R(crate::FieldReader<bool, bool>);
impl P6OUT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6OUT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6OUT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6OUT1` writer - P6OUT1"]
pub struct P6OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> P6OUT1_W<'a> {
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
#[doc = "Field `P6OUT2` reader - P6OUT2"]
pub struct P6OUT2_R(crate::FieldReader<bool, bool>);
impl P6OUT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6OUT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6OUT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6OUT2` writer - P6OUT2"]
pub struct P6OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> P6OUT2_W<'a> {
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
#[doc = "Field `P6OUT3` reader - P6OUT3"]
pub struct P6OUT3_R(crate::FieldReader<bool, bool>);
impl P6OUT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6OUT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6OUT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6OUT3` writer - P6OUT3"]
pub struct P6OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> P6OUT3_W<'a> {
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
#[doc = "Field `P6OUT4` reader - P6OUT4"]
pub struct P6OUT4_R(crate::FieldReader<bool, bool>);
impl P6OUT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6OUT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6OUT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6OUT4` writer - P6OUT4"]
pub struct P6OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> P6OUT4_W<'a> {
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
#[doc = "Field `P6OUT5` reader - P6OUT5"]
pub struct P6OUT5_R(crate::FieldReader<bool, bool>);
impl P6OUT5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6OUT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6OUT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6OUT5` writer - P6OUT5"]
pub struct P6OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> P6OUT5_W<'a> {
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
#[doc = "Field `P6OUT6` reader - P6OUT6"]
pub struct P6OUT6_R(crate::FieldReader<bool, bool>);
impl P6OUT6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6OUT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6OUT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6OUT6` writer - P6OUT6"]
pub struct P6OUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> P6OUT6_W<'a> {
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
#[doc = "Field `P6OUT7` reader - P6OUT7"]
pub struct P6OUT7_R(crate::FieldReader<bool, bool>);
impl P6OUT7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6OUT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6OUT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6OUT7` writer - P6OUT7"]
pub struct P6OUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> P6OUT7_W<'a> {
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
    #[doc = "Bit 0 - P6OUT0"]
    #[inline(always)]
    pub fn p6out0(&self) -> P6OUT0_R {
        P6OUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P6OUT1"]
    #[inline(always)]
    pub fn p6out1(&self) -> P6OUT1_R {
        P6OUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P6OUT2"]
    #[inline(always)]
    pub fn p6out2(&self) -> P6OUT2_R {
        P6OUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P6OUT3"]
    #[inline(always)]
    pub fn p6out3(&self) -> P6OUT3_R {
        P6OUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P6OUT4"]
    #[inline(always)]
    pub fn p6out4(&self) -> P6OUT4_R {
        P6OUT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P6OUT5"]
    #[inline(always)]
    pub fn p6out5(&self) -> P6OUT5_R {
        P6OUT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P6OUT6"]
    #[inline(always)]
    pub fn p6out6(&self) -> P6OUT6_R {
        P6OUT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P6OUT7"]
    #[inline(always)]
    pub fn p6out7(&self) -> P6OUT7_R {
        P6OUT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P6OUT0"]
    #[inline(always)]
    pub fn p6out0(&mut self) -> P6OUT0_W {
        P6OUT0_W { w: self }
    }
    #[doc = "Bit 1 - P6OUT1"]
    #[inline(always)]
    pub fn p6out1(&mut self) -> P6OUT1_W {
        P6OUT1_W { w: self }
    }
    #[doc = "Bit 2 - P6OUT2"]
    #[inline(always)]
    pub fn p6out2(&mut self) -> P6OUT2_W {
        P6OUT2_W { w: self }
    }
    #[doc = "Bit 3 - P6OUT3"]
    #[inline(always)]
    pub fn p6out3(&mut self) -> P6OUT3_W {
        P6OUT3_W { w: self }
    }
    #[doc = "Bit 4 - P6OUT4"]
    #[inline(always)]
    pub fn p6out4(&mut self) -> P6OUT4_W {
        P6OUT4_W { w: self }
    }
    #[doc = "Bit 5 - P6OUT5"]
    #[inline(always)]
    pub fn p6out5(&mut self) -> P6OUT5_W {
        P6OUT5_W { w: self }
    }
    #[doc = "Bit 6 - P6OUT6"]
    #[inline(always)]
    pub fn p6out6(&mut self) -> P6OUT6_W {
        P6OUT6_W { w: self }
    }
    #[doc = "Bit 7 - P6OUT7"]
    #[inline(always)]
    pub fn p6out7(&mut self) -> P6OUT7_W {
        P6OUT7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 6 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6out](index.html) module"]
pub struct P6OUT_SPEC;
impl crate::RegisterSpec for P6OUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p6out::R](R) reader structure"]
impl crate::Readable for P6OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p6out::W](W) writer structure"]
impl crate::Writable for P6OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P6OUT to value 0"]
impl crate::Resettable for P6OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
