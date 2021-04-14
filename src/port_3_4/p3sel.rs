#[doc = "Register `P3SEL` reader"]
pub struct R(crate::R<P3SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P3SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P3SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P3SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P3SEL` writer"]
pub struct W(crate::W<P3SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P3SEL_SPEC>;
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
impl From<crate::W<P3SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P3SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P3SEL0` reader - P3SEL0"]
pub struct P3SEL0_R(crate::FieldReader<bool, bool>);
impl P3SEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P3SEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3SEL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3SEL0` writer - P3SEL0"]
pub struct P3SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL0_W<'a> {
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
#[doc = "Field `P3SEL1` reader - P3SEL1"]
pub struct P3SEL1_R(crate::FieldReader<bool, bool>);
impl P3SEL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P3SEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3SEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3SEL1` writer - P3SEL1"]
pub struct P3SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL1_W<'a> {
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
#[doc = "Field `P3SEL2` reader - P3SEL2"]
pub struct P3SEL2_R(crate::FieldReader<bool, bool>);
impl P3SEL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P3SEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3SEL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3SEL2` writer - P3SEL2"]
pub struct P3SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL2_W<'a> {
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
#[doc = "Field `P3SEL3` reader - P3SEL3"]
pub struct P3SEL3_R(crate::FieldReader<bool, bool>);
impl P3SEL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P3SEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3SEL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3SEL3` writer - P3SEL3"]
pub struct P3SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL3_W<'a> {
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
#[doc = "Field `P3SEL4` reader - P3SEL4"]
pub struct P3SEL4_R(crate::FieldReader<bool, bool>);
impl P3SEL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P3SEL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3SEL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3SEL4` writer - P3SEL4"]
pub struct P3SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL4_W<'a> {
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
#[doc = "Field `P3SEL5` reader - P3SEL5"]
pub struct P3SEL5_R(crate::FieldReader<bool, bool>);
impl P3SEL5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P3SEL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3SEL5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3SEL5` writer - P3SEL5"]
pub struct P3SEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL5_W<'a> {
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
#[doc = "Field `P3SEL6` reader - P3SEL6"]
pub struct P3SEL6_R(crate::FieldReader<bool, bool>);
impl P3SEL6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P3SEL6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3SEL6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3SEL6` writer - P3SEL6"]
pub struct P3SEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL6_W<'a> {
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
#[doc = "Field `P3SEL7` reader - P3SEL7"]
pub struct P3SEL7_R(crate::FieldReader<bool, bool>);
impl P3SEL7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P3SEL7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3SEL7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3SEL7` writer - P3SEL7"]
pub struct P3SEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL7_W<'a> {
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
    #[doc = "Bit 0 - P3SEL0"]
    #[inline(always)]
    pub fn p3sel0(&self) -> P3SEL0_R {
        P3SEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P3SEL1"]
    #[inline(always)]
    pub fn p3sel1(&self) -> P3SEL1_R {
        P3SEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P3SEL2"]
    #[inline(always)]
    pub fn p3sel2(&self) -> P3SEL2_R {
        P3SEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P3SEL3"]
    #[inline(always)]
    pub fn p3sel3(&self) -> P3SEL3_R {
        P3SEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P3SEL4"]
    #[inline(always)]
    pub fn p3sel4(&self) -> P3SEL4_R {
        P3SEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P3SEL5"]
    #[inline(always)]
    pub fn p3sel5(&self) -> P3SEL5_R {
        P3SEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P3SEL6"]
    #[inline(always)]
    pub fn p3sel6(&self) -> P3SEL6_R {
        P3SEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P3SEL7"]
    #[inline(always)]
    pub fn p3sel7(&self) -> P3SEL7_R {
        P3SEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3SEL0"]
    #[inline(always)]
    pub fn p3sel0(&mut self) -> P3SEL0_W {
        P3SEL0_W { w: self }
    }
    #[doc = "Bit 1 - P3SEL1"]
    #[inline(always)]
    pub fn p3sel1(&mut self) -> P3SEL1_W {
        P3SEL1_W { w: self }
    }
    #[doc = "Bit 2 - P3SEL2"]
    #[inline(always)]
    pub fn p3sel2(&mut self) -> P3SEL2_W {
        P3SEL2_W { w: self }
    }
    #[doc = "Bit 3 - P3SEL3"]
    #[inline(always)]
    pub fn p3sel3(&mut self) -> P3SEL3_W {
        P3SEL3_W { w: self }
    }
    #[doc = "Bit 4 - P3SEL4"]
    #[inline(always)]
    pub fn p3sel4(&mut self) -> P3SEL4_W {
        P3SEL4_W { w: self }
    }
    #[doc = "Bit 5 - P3SEL5"]
    #[inline(always)]
    pub fn p3sel5(&mut self) -> P3SEL5_W {
        P3SEL5_W { w: self }
    }
    #[doc = "Bit 6 - P3SEL6"]
    #[inline(always)]
    pub fn p3sel6(&mut self) -> P3SEL6_W {
        P3SEL6_W { w: self }
    }
    #[doc = "Bit 7 - P3SEL7"]
    #[inline(always)]
    pub fn p3sel7(&mut self) -> P3SEL7_W {
        P3SEL7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 3 Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3sel](index.html) module"]
pub struct P3SEL_SPEC;
impl crate::RegisterSpec for P3SEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p3sel::R](R) reader structure"]
impl crate::Readable for P3SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p3sel::W](W) writer structure"]
impl crate::Writable for P3SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P3SEL to value 0"]
impl crate::Resettable for P3SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
