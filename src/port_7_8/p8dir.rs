#[doc = "Register `P8DIR` reader"]
pub struct R(crate::R<P8DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P8DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P8DIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P8DIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P8DIR` writer"]
pub struct W(crate::W<P8DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P8DIR_SPEC>;
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
impl From<crate::W<P8DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P8DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P8DIR0` reader - P8DIR0"]
pub struct P8DIR0_R(crate::FieldReader<bool, bool>);
impl P8DIR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8DIR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8DIR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8DIR0` writer - P8DIR0"]
pub struct P8DIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DIR0_W<'a> {
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
#[doc = "Field `P8DIR1` reader - P8DIR1"]
pub struct P8DIR1_R(crate::FieldReader<bool, bool>);
impl P8DIR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8DIR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8DIR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8DIR1` writer - P8DIR1"]
pub struct P8DIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DIR1_W<'a> {
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
#[doc = "Field `P8DIR2` reader - P8DIR2"]
pub struct P8DIR2_R(crate::FieldReader<bool, bool>);
impl P8DIR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8DIR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8DIR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8DIR2` writer - P8DIR2"]
pub struct P8DIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DIR2_W<'a> {
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
#[doc = "Field `P8DIR3` reader - P8DIR3"]
pub struct P8DIR3_R(crate::FieldReader<bool, bool>);
impl P8DIR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8DIR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8DIR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8DIR3` writer - P8DIR3"]
pub struct P8DIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DIR3_W<'a> {
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
#[doc = "Field `P8DIR4` reader - P8DIR4"]
pub struct P8DIR4_R(crate::FieldReader<bool, bool>);
impl P8DIR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8DIR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8DIR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8DIR4` writer - P8DIR4"]
pub struct P8DIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DIR4_W<'a> {
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
#[doc = "Field `P8DIR5` reader - P8DIR5"]
pub struct P8DIR5_R(crate::FieldReader<bool, bool>);
impl P8DIR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8DIR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8DIR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8DIR5` writer - P8DIR5"]
pub struct P8DIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DIR5_W<'a> {
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
#[doc = "Field `P8DIR6` reader - P8DIR6"]
pub struct P8DIR6_R(crate::FieldReader<bool, bool>);
impl P8DIR6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8DIR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8DIR6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8DIR6` writer - P8DIR6"]
pub struct P8DIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DIR6_W<'a> {
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
#[doc = "Field `P8DIR7` reader - P8DIR7"]
pub struct P8DIR7_R(crate::FieldReader<bool, bool>);
impl P8DIR7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P8DIR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8DIR7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8DIR7` writer - P8DIR7"]
pub struct P8DIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DIR7_W<'a> {
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
    #[doc = "Bit 0 - P8DIR0"]
    #[inline(always)]
    pub fn p8dir0(&self) -> P8DIR0_R {
        P8DIR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P8DIR1"]
    #[inline(always)]
    pub fn p8dir1(&self) -> P8DIR1_R {
        P8DIR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P8DIR2"]
    #[inline(always)]
    pub fn p8dir2(&self) -> P8DIR2_R {
        P8DIR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P8DIR3"]
    #[inline(always)]
    pub fn p8dir3(&self) -> P8DIR3_R {
        P8DIR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P8DIR4"]
    #[inline(always)]
    pub fn p8dir4(&self) -> P8DIR4_R {
        P8DIR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P8DIR5"]
    #[inline(always)]
    pub fn p8dir5(&self) -> P8DIR5_R {
        P8DIR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P8DIR6"]
    #[inline(always)]
    pub fn p8dir6(&self) -> P8DIR6_R {
        P8DIR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P8DIR7"]
    #[inline(always)]
    pub fn p8dir7(&self) -> P8DIR7_R {
        P8DIR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P8DIR0"]
    #[inline(always)]
    pub fn p8dir0(&mut self) -> P8DIR0_W {
        P8DIR0_W { w: self }
    }
    #[doc = "Bit 1 - P8DIR1"]
    #[inline(always)]
    pub fn p8dir1(&mut self) -> P8DIR1_W {
        P8DIR1_W { w: self }
    }
    #[doc = "Bit 2 - P8DIR2"]
    #[inline(always)]
    pub fn p8dir2(&mut self) -> P8DIR2_W {
        P8DIR2_W { w: self }
    }
    #[doc = "Bit 3 - P8DIR3"]
    #[inline(always)]
    pub fn p8dir3(&mut self) -> P8DIR3_W {
        P8DIR3_W { w: self }
    }
    #[doc = "Bit 4 - P8DIR4"]
    #[inline(always)]
    pub fn p8dir4(&mut self) -> P8DIR4_W {
        P8DIR4_W { w: self }
    }
    #[doc = "Bit 5 - P8DIR5"]
    #[inline(always)]
    pub fn p8dir5(&mut self) -> P8DIR5_W {
        P8DIR5_W { w: self }
    }
    #[doc = "Bit 6 - P8DIR6"]
    #[inline(always)]
    pub fn p8dir6(&mut self) -> P8DIR6_W {
        P8DIR6_W { w: self }
    }
    #[doc = "Bit 7 - P8DIR7"]
    #[inline(always)]
    pub fn p8dir7(&mut self) -> P8DIR7_W {
        P8DIR7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 8 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p8dir](index.html) module"]
pub struct P8DIR_SPEC;
impl crate::RegisterSpec for P8DIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p8dir::R](R) reader structure"]
impl crate::Readable for P8DIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p8dir::W](W) writer structure"]
impl crate::Writable for P8DIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P8DIR to value 0"]
impl crate::Resettable for P8DIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
