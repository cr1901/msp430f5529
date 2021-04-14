#[doc = "Register `P4DIR` reader"]
pub struct R(crate::R<P4DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P4DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P4DIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P4DIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P4DIR` writer"]
pub struct W(crate::W<P4DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P4DIR_SPEC>;
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
impl From<crate::W<P4DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P4DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P4DIR0` reader - P4DIR0"]
pub struct P4DIR0_R(crate::FieldReader<bool, bool>);
impl P4DIR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P4DIR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4DIR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4DIR0` writer - P4DIR0"]
pub struct P4DIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DIR0_W<'a> {
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
#[doc = "Field `P4DIR1` reader - P4DIR1"]
pub struct P4DIR1_R(crate::FieldReader<bool, bool>);
impl P4DIR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P4DIR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4DIR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4DIR1` writer - P4DIR1"]
pub struct P4DIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DIR1_W<'a> {
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
#[doc = "Field `P4DIR2` reader - P4DIR2"]
pub struct P4DIR2_R(crate::FieldReader<bool, bool>);
impl P4DIR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P4DIR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4DIR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4DIR2` writer - P4DIR2"]
pub struct P4DIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DIR2_W<'a> {
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
#[doc = "Field `P4DIR3` reader - P4DIR3"]
pub struct P4DIR3_R(crate::FieldReader<bool, bool>);
impl P4DIR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P4DIR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4DIR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4DIR3` writer - P4DIR3"]
pub struct P4DIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DIR3_W<'a> {
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
#[doc = "Field `P4DIR4` reader - P4DIR4"]
pub struct P4DIR4_R(crate::FieldReader<bool, bool>);
impl P4DIR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P4DIR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4DIR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4DIR4` writer - P4DIR4"]
pub struct P4DIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DIR4_W<'a> {
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
#[doc = "Field `P4DIR5` reader - P4DIR5"]
pub struct P4DIR5_R(crate::FieldReader<bool, bool>);
impl P4DIR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P4DIR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4DIR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4DIR5` writer - P4DIR5"]
pub struct P4DIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DIR5_W<'a> {
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
#[doc = "Field `P4DIR6` reader - P4DIR6"]
pub struct P4DIR6_R(crate::FieldReader<bool, bool>);
impl P4DIR6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P4DIR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4DIR6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4DIR6` writer - P4DIR6"]
pub struct P4DIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DIR6_W<'a> {
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
#[doc = "Field `P4DIR7` reader - P4DIR7"]
pub struct P4DIR7_R(crate::FieldReader<bool, bool>);
impl P4DIR7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P4DIR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4DIR7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4DIR7` writer - P4DIR7"]
pub struct P4DIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DIR7_W<'a> {
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
    #[doc = "Bit 0 - P4DIR0"]
    #[inline(always)]
    pub fn p4dir0(&self) -> P4DIR0_R {
        P4DIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P4DIR1"]
    #[inline(always)]
    pub fn p4dir1(&self) -> P4DIR1_R {
        P4DIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P4DIR2"]
    #[inline(always)]
    pub fn p4dir2(&self) -> P4DIR2_R {
        P4DIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P4DIR3"]
    #[inline(always)]
    pub fn p4dir3(&self) -> P4DIR3_R {
        P4DIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P4DIR4"]
    #[inline(always)]
    pub fn p4dir4(&self) -> P4DIR4_R {
        P4DIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P4DIR5"]
    #[inline(always)]
    pub fn p4dir5(&self) -> P4DIR5_R {
        P4DIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P4DIR6"]
    #[inline(always)]
    pub fn p4dir6(&self) -> P4DIR6_R {
        P4DIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P4DIR7"]
    #[inline(always)]
    pub fn p4dir7(&self) -> P4DIR7_R {
        P4DIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P4DIR0"]
    #[inline(always)]
    pub fn p4dir0(&mut self) -> P4DIR0_W {
        P4DIR0_W { w: self }
    }
    #[doc = "Bit 1 - P4DIR1"]
    #[inline(always)]
    pub fn p4dir1(&mut self) -> P4DIR1_W {
        P4DIR1_W { w: self }
    }
    #[doc = "Bit 2 - P4DIR2"]
    #[inline(always)]
    pub fn p4dir2(&mut self) -> P4DIR2_W {
        P4DIR2_W { w: self }
    }
    #[doc = "Bit 3 - P4DIR3"]
    #[inline(always)]
    pub fn p4dir3(&mut self) -> P4DIR3_W {
        P4DIR3_W { w: self }
    }
    #[doc = "Bit 4 - P4DIR4"]
    #[inline(always)]
    pub fn p4dir4(&mut self) -> P4DIR4_W {
        P4DIR4_W { w: self }
    }
    #[doc = "Bit 5 - P4DIR5"]
    #[inline(always)]
    pub fn p4dir5(&mut self) -> P4DIR5_W {
        P4DIR5_W { w: self }
    }
    #[doc = "Bit 6 - P4DIR6"]
    #[inline(always)]
    pub fn p4dir6(&mut self) -> P4DIR6_W {
        P4DIR6_W { w: self }
    }
    #[doc = "Bit 7 - P4DIR7"]
    #[inline(always)]
    pub fn p4dir7(&mut self) -> P4DIR7_W {
        P4DIR7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 4 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4dir](index.html) module"]
pub struct P4DIR_SPEC;
impl crate::RegisterSpec for P4DIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p4dir::R](R) reader structure"]
impl crate::Readable for P4DIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p4dir::W](W) writer structure"]
impl crate::Writable for P4DIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P4DIR to value 0"]
impl crate::Resettable for P4DIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
