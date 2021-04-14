#[doc = "Register `P1DIR` reader"]
pub struct R(crate::R<P1DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P1DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P1DIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P1DIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P1DIR` writer"]
pub struct W(crate::W<P1DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P1DIR_SPEC>;
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
impl From<crate::W<P1DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P1DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1DIR0` reader - P1DIR0"]
pub struct P1DIR0_R(crate::FieldReader<bool, bool>);
impl P1DIR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1DIR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1DIR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1DIR0` writer - P1DIR0"]
pub struct P1DIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DIR0_W<'a> {
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
#[doc = "Field `P1DIR1` reader - P1DIR1"]
pub struct P1DIR1_R(crate::FieldReader<bool, bool>);
impl P1DIR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1DIR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1DIR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1DIR1` writer - P1DIR1"]
pub struct P1DIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DIR1_W<'a> {
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
#[doc = "Field `P1DIR2` reader - P1DIR2"]
pub struct P1DIR2_R(crate::FieldReader<bool, bool>);
impl P1DIR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1DIR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1DIR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1DIR2` writer - P1DIR2"]
pub struct P1DIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DIR2_W<'a> {
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
#[doc = "Field `P1DIR3` reader - P1DIR3"]
pub struct P1DIR3_R(crate::FieldReader<bool, bool>);
impl P1DIR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1DIR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1DIR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1DIR3` writer - P1DIR3"]
pub struct P1DIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DIR3_W<'a> {
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
#[doc = "Field `P1DIR4` reader - P1DIR4"]
pub struct P1DIR4_R(crate::FieldReader<bool, bool>);
impl P1DIR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1DIR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1DIR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1DIR4` writer - P1DIR4"]
pub struct P1DIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DIR4_W<'a> {
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
#[doc = "Field `P1DIR5` reader - P1DIR5"]
pub struct P1DIR5_R(crate::FieldReader<bool, bool>);
impl P1DIR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1DIR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1DIR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1DIR5` writer - P1DIR5"]
pub struct P1DIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DIR5_W<'a> {
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
#[doc = "Field `P1DIR6` reader - P1DIR6"]
pub struct P1DIR6_R(crate::FieldReader<bool, bool>);
impl P1DIR6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1DIR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1DIR6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1DIR6` writer - P1DIR6"]
pub struct P1DIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DIR6_W<'a> {
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
#[doc = "Field `P1DIR7` reader - P1DIR7"]
pub struct P1DIR7_R(crate::FieldReader<bool, bool>);
impl P1DIR7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1DIR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1DIR7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1DIR7` writer - P1DIR7"]
pub struct P1DIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DIR7_W<'a> {
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
    #[doc = "Bit 0 - P1DIR0"]
    #[inline(always)]
    pub fn p1dir0(&self) -> P1DIR0_R {
        P1DIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P1DIR1"]
    #[inline(always)]
    pub fn p1dir1(&self) -> P1DIR1_R {
        P1DIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P1DIR2"]
    #[inline(always)]
    pub fn p1dir2(&self) -> P1DIR2_R {
        P1DIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P1DIR3"]
    #[inline(always)]
    pub fn p1dir3(&self) -> P1DIR3_R {
        P1DIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P1DIR4"]
    #[inline(always)]
    pub fn p1dir4(&self) -> P1DIR4_R {
        P1DIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P1DIR5"]
    #[inline(always)]
    pub fn p1dir5(&self) -> P1DIR5_R {
        P1DIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P1DIR6"]
    #[inline(always)]
    pub fn p1dir6(&self) -> P1DIR6_R {
        P1DIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P1DIR7"]
    #[inline(always)]
    pub fn p1dir7(&self) -> P1DIR7_R {
        P1DIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1DIR0"]
    #[inline(always)]
    pub fn p1dir0(&mut self) -> P1DIR0_W {
        P1DIR0_W { w: self }
    }
    #[doc = "Bit 1 - P1DIR1"]
    #[inline(always)]
    pub fn p1dir1(&mut self) -> P1DIR1_W {
        P1DIR1_W { w: self }
    }
    #[doc = "Bit 2 - P1DIR2"]
    #[inline(always)]
    pub fn p1dir2(&mut self) -> P1DIR2_W {
        P1DIR2_W { w: self }
    }
    #[doc = "Bit 3 - P1DIR3"]
    #[inline(always)]
    pub fn p1dir3(&mut self) -> P1DIR3_W {
        P1DIR3_W { w: self }
    }
    #[doc = "Bit 4 - P1DIR4"]
    #[inline(always)]
    pub fn p1dir4(&mut self) -> P1DIR4_W {
        P1DIR4_W { w: self }
    }
    #[doc = "Bit 5 - P1DIR5"]
    #[inline(always)]
    pub fn p1dir5(&mut self) -> P1DIR5_W {
        P1DIR5_W { w: self }
    }
    #[doc = "Bit 6 - P1DIR6"]
    #[inline(always)]
    pub fn p1dir6(&mut self) -> P1DIR6_W {
        P1DIR6_W { w: self }
    }
    #[doc = "Bit 7 - P1DIR7"]
    #[inline(always)]
    pub fn p1dir7(&mut self) -> P1DIR7_W {
        P1DIR7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 1 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1dir](index.html) module"]
pub struct P1DIR_SPEC;
impl crate::RegisterSpec for P1DIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p1dir::R](R) reader structure"]
impl crate::Readable for P1DIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p1dir::W](W) writer structure"]
impl crate::Writable for P1DIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P1DIR to value 0"]
impl crate::Resettable for P1DIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
