#[doc = "Register `P2IFG` reader"]
pub struct R(crate::R<P2IFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2IFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P2IFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P2IFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P2IFG` writer"]
pub struct W(crate::W<P2IFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P2IFG_SPEC>;
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
impl From<crate::W<P2IFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P2IFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P2IFG0` reader - P2IFG0"]
pub struct P2IFG0_R(crate::FieldReader<bool, bool>);
impl P2IFG0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2IFG0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2IFG0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2IFG0` writer - P2IFG0"]
pub struct P2IFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IFG0_W<'a> {
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
#[doc = "Field `P2IFG1` reader - P2IFG1"]
pub struct P2IFG1_R(crate::FieldReader<bool, bool>);
impl P2IFG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2IFG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2IFG1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2IFG1` writer - P2IFG1"]
pub struct P2IFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IFG1_W<'a> {
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
#[doc = "Field `P2IFG2` reader - P2IFG2"]
pub struct P2IFG2_R(crate::FieldReader<bool, bool>);
impl P2IFG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2IFG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2IFG2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2IFG2` writer - P2IFG2"]
pub struct P2IFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IFG2_W<'a> {
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
#[doc = "Field `P2IFG3` reader - P2IFG3"]
pub struct P2IFG3_R(crate::FieldReader<bool, bool>);
impl P2IFG3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2IFG3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2IFG3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2IFG3` writer - P2IFG3"]
pub struct P2IFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IFG3_W<'a> {
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
#[doc = "Field `P2IFG4` reader - P2IFG4"]
pub struct P2IFG4_R(crate::FieldReader<bool, bool>);
impl P2IFG4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2IFG4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2IFG4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2IFG4` writer - P2IFG4"]
pub struct P2IFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IFG4_W<'a> {
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
#[doc = "Field `P2IFG5` reader - P2IFG5"]
pub struct P2IFG5_R(crate::FieldReader<bool, bool>);
impl P2IFG5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2IFG5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2IFG5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2IFG5` writer - P2IFG5"]
pub struct P2IFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IFG5_W<'a> {
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
#[doc = "Field `P2IFG6` reader - P2IFG6"]
pub struct P2IFG6_R(crate::FieldReader<bool, bool>);
impl P2IFG6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2IFG6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2IFG6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2IFG6` writer - P2IFG6"]
pub struct P2IFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IFG6_W<'a> {
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
#[doc = "Field `P2IFG7` reader - P2IFG7"]
pub struct P2IFG7_R(crate::FieldReader<bool, bool>);
impl P2IFG7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2IFG7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2IFG7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2IFG7` writer - P2IFG7"]
pub struct P2IFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IFG7_W<'a> {
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
    #[doc = "Bit 0 - P2IFG0"]
    #[inline(always)]
    pub fn p2ifg0(&self) -> P2IFG0_R {
        P2IFG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P2IFG1"]
    #[inline(always)]
    pub fn p2ifg1(&self) -> P2IFG1_R {
        P2IFG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P2IFG2"]
    #[inline(always)]
    pub fn p2ifg2(&self) -> P2IFG2_R {
        P2IFG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P2IFG3"]
    #[inline(always)]
    pub fn p2ifg3(&self) -> P2IFG3_R {
        P2IFG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P2IFG4"]
    #[inline(always)]
    pub fn p2ifg4(&self) -> P2IFG4_R {
        P2IFG4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P2IFG5"]
    #[inline(always)]
    pub fn p2ifg5(&self) -> P2IFG5_R {
        P2IFG5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P2IFG6"]
    #[inline(always)]
    pub fn p2ifg6(&self) -> P2IFG6_R {
        P2IFG6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P2IFG7"]
    #[inline(always)]
    pub fn p2ifg7(&self) -> P2IFG7_R {
        P2IFG7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2IFG0"]
    #[inline(always)]
    pub fn p2ifg0(&mut self) -> P2IFG0_W {
        P2IFG0_W { w: self }
    }
    #[doc = "Bit 1 - P2IFG1"]
    #[inline(always)]
    pub fn p2ifg1(&mut self) -> P2IFG1_W {
        P2IFG1_W { w: self }
    }
    #[doc = "Bit 2 - P2IFG2"]
    #[inline(always)]
    pub fn p2ifg2(&mut self) -> P2IFG2_W {
        P2IFG2_W { w: self }
    }
    #[doc = "Bit 3 - P2IFG3"]
    #[inline(always)]
    pub fn p2ifg3(&mut self) -> P2IFG3_W {
        P2IFG3_W { w: self }
    }
    #[doc = "Bit 4 - P2IFG4"]
    #[inline(always)]
    pub fn p2ifg4(&mut self) -> P2IFG4_W {
        P2IFG4_W { w: self }
    }
    #[doc = "Bit 5 - P2IFG5"]
    #[inline(always)]
    pub fn p2ifg5(&mut self) -> P2IFG5_W {
        P2IFG5_W { w: self }
    }
    #[doc = "Bit 6 - P2IFG6"]
    #[inline(always)]
    pub fn p2ifg6(&mut self) -> P2IFG6_W {
        P2IFG6_W { w: self }
    }
    #[doc = "Bit 7 - P2IFG7"]
    #[inline(always)]
    pub fn p2ifg7(&mut self) -> P2IFG7_W {
        P2IFG7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 2 Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2ifg](index.html) module"]
pub struct P2IFG_SPEC;
impl crate::RegisterSpec for P2IFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p2ifg::R](R) reader structure"]
impl crate::Readable for P2IFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p2ifg::W](W) writer structure"]
impl crate::Writable for P2IFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P2IFG to value 0"]
impl crate::Resettable for P2IFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
