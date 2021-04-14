#[doc = "Register `P6DIR` reader"]
pub struct R(crate::R<P6DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P6DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P6DIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P6DIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P6DIR` writer"]
pub struct W(crate::W<P6DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P6DIR_SPEC>;
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
impl From<crate::W<P6DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P6DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P6DIR0` reader - P6DIR0"]
pub struct P6DIR0_R(crate::FieldReader<bool, bool>);
impl P6DIR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6DIR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6DIR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6DIR0` writer - P6DIR0"]
pub struct P6DIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DIR0_W<'a> {
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
#[doc = "Field `P6DIR1` reader - P6DIR1"]
pub struct P6DIR1_R(crate::FieldReader<bool, bool>);
impl P6DIR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6DIR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6DIR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6DIR1` writer - P6DIR1"]
pub struct P6DIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DIR1_W<'a> {
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
#[doc = "Field `P6DIR2` reader - P6DIR2"]
pub struct P6DIR2_R(crate::FieldReader<bool, bool>);
impl P6DIR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6DIR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6DIR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6DIR2` writer - P6DIR2"]
pub struct P6DIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DIR2_W<'a> {
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
#[doc = "Field `P6DIR3` reader - P6DIR3"]
pub struct P6DIR3_R(crate::FieldReader<bool, bool>);
impl P6DIR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6DIR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6DIR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6DIR3` writer - P6DIR3"]
pub struct P6DIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DIR3_W<'a> {
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
#[doc = "Field `P6DIR4` reader - P6DIR4"]
pub struct P6DIR4_R(crate::FieldReader<bool, bool>);
impl P6DIR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6DIR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6DIR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6DIR4` writer - P6DIR4"]
pub struct P6DIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DIR4_W<'a> {
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
#[doc = "Field `P6DIR5` reader - P6DIR5"]
pub struct P6DIR5_R(crate::FieldReader<bool, bool>);
impl P6DIR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6DIR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6DIR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6DIR5` writer - P6DIR5"]
pub struct P6DIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DIR5_W<'a> {
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
#[doc = "Field `P6DIR6` reader - P6DIR6"]
pub struct P6DIR6_R(crate::FieldReader<bool, bool>);
impl P6DIR6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6DIR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6DIR6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6DIR6` writer - P6DIR6"]
pub struct P6DIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DIR6_W<'a> {
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
#[doc = "Field `P6DIR7` reader - P6DIR7"]
pub struct P6DIR7_R(crate::FieldReader<bool, bool>);
impl P6DIR7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6DIR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6DIR7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6DIR7` writer - P6DIR7"]
pub struct P6DIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DIR7_W<'a> {
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
    #[doc = "Bit 0 - P6DIR0"]
    #[inline(always)]
    pub fn p6dir0(&self) -> P6DIR0_R {
        P6DIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P6DIR1"]
    #[inline(always)]
    pub fn p6dir1(&self) -> P6DIR1_R {
        P6DIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P6DIR2"]
    #[inline(always)]
    pub fn p6dir2(&self) -> P6DIR2_R {
        P6DIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P6DIR3"]
    #[inline(always)]
    pub fn p6dir3(&self) -> P6DIR3_R {
        P6DIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P6DIR4"]
    #[inline(always)]
    pub fn p6dir4(&self) -> P6DIR4_R {
        P6DIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P6DIR5"]
    #[inline(always)]
    pub fn p6dir5(&self) -> P6DIR5_R {
        P6DIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P6DIR6"]
    #[inline(always)]
    pub fn p6dir6(&self) -> P6DIR6_R {
        P6DIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P6DIR7"]
    #[inline(always)]
    pub fn p6dir7(&self) -> P6DIR7_R {
        P6DIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P6DIR0"]
    #[inline(always)]
    pub fn p6dir0(&mut self) -> P6DIR0_W {
        P6DIR0_W { w: self }
    }
    #[doc = "Bit 1 - P6DIR1"]
    #[inline(always)]
    pub fn p6dir1(&mut self) -> P6DIR1_W {
        P6DIR1_W { w: self }
    }
    #[doc = "Bit 2 - P6DIR2"]
    #[inline(always)]
    pub fn p6dir2(&mut self) -> P6DIR2_W {
        P6DIR2_W { w: self }
    }
    #[doc = "Bit 3 - P6DIR3"]
    #[inline(always)]
    pub fn p6dir3(&mut self) -> P6DIR3_W {
        P6DIR3_W { w: self }
    }
    #[doc = "Bit 4 - P6DIR4"]
    #[inline(always)]
    pub fn p6dir4(&mut self) -> P6DIR4_W {
        P6DIR4_W { w: self }
    }
    #[doc = "Bit 5 - P6DIR5"]
    #[inline(always)]
    pub fn p6dir5(&mut self) -> P6DIR5_W {
        P6DIR5_W { w: self }
    }
    #[doc = "Bit 6 - P6DIR6"]
    #[inline(always)]
    pub fn p6dir6(&mut self) -> P6DIR6_W {
        P6DIR6_W { w: self }
    }
    #[doc = "Bit 7 - P6DIR7"]
    #[inline(always)]
    pub fn p6dir7(&mut self) -> P6DIR7_W {
        P6DIR7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 6 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6dir](index.html) module"]
pub struct P6DIR_SPEC;
impl crate::RegisterSpec for P6DIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p6dir::R](R) reader structure"]
impl crate::Readable for P6DIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p6dir::W](W) writer structure"]
impl crate::Writable for P6DIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P6DIR to value 0"]
impl crate::Resettable for P6DIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
