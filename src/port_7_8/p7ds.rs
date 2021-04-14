#[doc = "Register `P7DS` reader"]
pub struct R(crate::R<P7DS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P7DS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P7DS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P7DS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P7DS` writer"]
pub struct W(crate::W<P7DS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P7DS_SPEC>;
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
impl From<crate::W<P7DS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P7DS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P7DS0` reader - P7DS0"]
pub struct P7DS0_R(crate::FieldReader<bool, bool>);
impl P7DS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7DS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7DS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7DS0` writer - P7DS0"]
pub struct P7DS0_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DS0_W<'a> {
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
#[doc = "Field `P7DS1` reader - P7DS1"]
pub struct P7DS1_R(crate::FieldReader<bool, bool>);
impl P7DS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7DS1` writer - P7DS1"]
pub struct P7DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DS1_W<'a> {
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
#[doc = "Field `P7DS2` reader - P7DS2"]
pub struct P7DS2_R(crate::FieldReader<bool, bool>);
impl P7DS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7DS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7DS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7DS2` writer - P7DS2"]
pub struct P7DS2_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DS2_W<'a> {
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
#[doc = "Field `P7DS3` reader - P7DS3"]
pub struct P7DS3_R(crate::FieldReader<bool, bool>);
impl P7DS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7DS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7DS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7DS3` writer - P7DS3"]
pub struct P7DS3_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DS3_W<'a> {
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
#[doc = "Field `P7DS4` reader - P7DS4"]
pub struct P7DS4_R(crate::FieldReader<bool, bool>);
impl P7DS4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7DS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7DS4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7DS4` writer - P7DS4"]
pub struct P7DS4_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DS4_W<'a> {
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
#[doc = "Field `P7DS5` reader - P7DS5"]
pub struct P7DS5_R(crate::FieldReader<bool, bool>);
impl P7DS5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7DS5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7DS5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7DS5` writer - P7DS5"]
pub struct P7DS5_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DS5_W<'a> {
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
#[doc = "Field `P7DS6` reader - P7DS6"]
pub struct P7DS6_R(crate::FieldReader<bool, bool>);
impl P7DS6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7DS6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7DS6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7DS6` writer - P7DS6"]
pub struct P7DS6_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DS6_W<'a> {
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
#[doc = "Field `P7DS7` reader - P7DS7"]
pub struct P7DS7_R(crate::FieldReader<bool, bool>);
impl P7DS7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P7DS7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7DS7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7DS7` writer - P7DS7"]
pub struct P7DS7_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DS7_W<'a> {
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
    #[doc = "Bit 0 - P7DS0"]
    #[inline(always)]
    pub fn p7ds0(&self) -> P7DS0_R {
        P7DS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P7DS1"]
    #[inline(always)]
    pub fn p7ds1(&self) -> P7DS1_R {
        P7DS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P7DS2"]
    #[inline(always)]
    pub fn p7ds2(&self) -> P7DS2_R {
        P7DS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P7DS3"]
    #[inline(always)]
    pub fn p7ds3(&self) -> P7DS3_R {
        P7DS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P7DS4"]
    #[inline(always)]
    pub fn p7ds4(&self) -> P7DS4_R {
        P7DS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P7DS5"]
    #[inline(always)]
    pub fn p7ds5(&self) -> P7DS5_R {
        P7DS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P7DS6"]
    #[inline(always)]
    pub fn p7ds6(&self) -> P7DS6_R {
        P7DS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P7DS7"]
    #[inline(always)]
    pub fn p7ds7(&self) -> P7DS7_R {
        P7DS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P7DS0"]
    #[inline(always)]
    pub fn p7ds0(&mut self) -> P7DS0_W {
        P7DS0_W { w: self }
    }
    #[doc = "Bit 1 - P7DS1"]
    #[inline(always)]
    pub fn p7ds1(&mut self) -> P7DS1_W {
        P7DS1_W { w: self }
    }
    #[doc = "Bit 2 - P7DS2"]
    #[inline(always)]
    pub fn p7ds2(&mut self) -> P7DS2_W {
        P7DS2_W { w: self }
    }
    #[doc = "Bit 3 - P7DS3"]
    #[inline(always)]
    pub fn p7ds3(&mut self) -> P7DS3_W {
        P7DS3_W { w: self }
    }
    #[doc = "Bit 4 - P7DS4"]
    #[inline(always)]
    pub fn p7ds4(&mut self) -> P7DS4_W {
        P7DS4_W { w: self }
    }
    #[doc = "Bit 5 - P7DS5"]
    #[inline(always)]
    pub fn p7ds5(&mut self) -> P7DS5_W {
        P7DS5_W { w: self }
    }
    #[doc = "Bit 6 - P7DS6"]
    #[inline(always)]
    pub fn p7ds6(&mut self) -> P7DS6_W {
        P7DS6_W { w: self }
    }
    #[doc = "Bit 7 - P7DS7"]
    #[inline(always)]
    pub fn p7ds7(&mut self) -> P7DS7_W {
        P7DS7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 7 Drive Strenght\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7ds](index.html) module"]
pub struct P7DS_SPEC;
impl crate::RegisterSpec for P7DS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p7ds::R](R) reader structure"]
impl crate::Readable for P7DS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p7ds::W](W) writer structure"]
impl crate::Writable for P7DS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P7DS to value 0"]
impl crate::Resettable for P7DS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
