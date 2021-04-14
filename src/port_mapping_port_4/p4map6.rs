#[doc = "Register `P4MAP6` reader"]
pub struct R(crate::R<P4MAP6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P4MAP6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P4MAP6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P4MAP6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P4MAP6` writer"]
pub struct W(crate::W<P4MAP6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P4MAP6_SPEC>;
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
impl From<crate::W<P4MAP6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P4MAP6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMAP0` reader - PMAP0"]
pub struct PMAP0_R(crate::FieldReader<bool, bool>);
impl PMAP0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMAP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMAP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMAP0` writer - PMAP0"]
pub struct PMAP0_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAP0_W<'a> {
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
#[doc = "Field `PMAP1` reader - PMAP1"]
pub struct PMAP1_R(crate::FieldReader<bool, bool>);
impl PMAP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMAP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMAP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMAP1` writer - PMAP1"]
pub struct PMAP1_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAP1_W<'a> {
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
#[doc = "Field `PMAP2` reader - PMAP2"]
pub struct PMAP2_R(crate::FieldReader<bool, bool>);
impl PMAP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMAP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMAP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMAP2` writer - PMAP2"]
pub struct PMAP2_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAP2_W<'a> {
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
#[doc = "Field `PMAP3` reader - PMAP3"]
pub struct PMAP3_R(crate::FieldReader<bool, bool>);
impl PMAP3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMAP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMAP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMAP3` writer - PMAP3"]
pub struct PMAP3_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAP3_W<'a> {
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
#[doc = "Field `PMAP4` reader - PMAP4"]
pub struct PMAP4_R(crate::FieldReader<bool, bool>);
impl PMAP4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMAP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMAP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMAP4` writer - PMAP4"]
pub struct PMAP4_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAP4_W<'a> {
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
#[doc = "Field `PMAP5` reader - PMAP5"]
pub struct PMAP5_R(crate::FieldReader<bool, bool>);
impl PMAP5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMAP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMAP5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMAP5` writer - PMAP5"]
pub struct PMAP5_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAP5_W<'a> {
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
#[doc = "Field `PMAP6` reader - PMAP6"]
pub struct PMAP6_R(crate::FieldReader<bool, bool>);
impl PMAP6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMAP6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMAP6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMAP6` writer - PMAP6"]
pub struct PMAP6_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAP6_W<'a> {
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
#[doc = "Field `PMAP7` reader - PMAP7"]
pub struct PMAP7_R(crate::FieldReader<bool, bool>);
impl PMAP7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMAP7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMAP7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMAP7` writer - PMAP7"]
pub struct PMAP7_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAP7_W<'a> {
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
    #[doc = "Bit 0 - PMAP0"]
    #[inline(always)]
    pub fn pmap0(&self) -> PMAP0_R {
        PMAP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PMAP1"]
    #[inline(always)]
    pub fn pmap1(&self) -> PMAP1_R {
        PMAP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PMAP2"]
    #[inline(always)]
    pub fn pmap2(&self) -> PMAP2_R {
        PMAP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PMAP3"]
    #[inline(always)]
    pub fn pmap3(&self) -> PMAP3_R {
        PMAP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PMAP4"]
    #[inline(always)]
    pub fn pmap4(&self) -> PMAP4_R {
        PMAP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PMAP5"]
    #[inline(always)]
    pub fn pmap5(&self) -> PMAP5_R {
        PMAP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PMAP6"]
    #[inline(always)]
    pub fn pmap6(&self) -> PMAP6_R {
        PMAP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PMAP7"]
    #[inline(always)]
    pub fn pmap7(&self) -> PMAP7_R {
        PMAP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PMAP0"]
    #[inline(always)]
    pub fn pmap0(&mut self) -> PMAP0_W {
        PMAP0_W { w: self }
    }
    #[doc = "Bit 1 - PMAP1"]
    #[inline(always)]
    pub fn pmap1(&mut self) -> PMAP1_W {
        PMAP1_W { w: self }
    }
    #[doc = "Bit 2 - PMAP2"]
    #[inline(always)]
    pub fn pmap2(&mut self) -> PMAP2_W {
        PMAP2_W { w: self }
    }
    #[doc = "Bit 3 - PMAP3"]
    #[inline(always)]
    pub fn pmap3(&mut self) -> PMAP3_W {
        PMAP3_W { w: self }
    }
    #[doc = "Bit 4 - PMAP4"]
    #[inline(always)]
    pub fn pmap4(&mut self) -> PMAP4_W {
        PMAP4_W { w: self }
    }
    #[doc = "Bit 5 - PMAP5"]
    #[inline(always)]
    pub fn pmap5(&mut self) -> PMAP5_W {
        PMAP5_W { w: self }
    }
    #[doc = "Bit 6 - PMAP6"]
    #[inline(always)]
    pub fn pmap6(&mut self) -> PMAP6_W {
        PMAP6_W { w: self }
    }
    #[doc = "Bit 7 - PMAP7"]
    #[inline(always)]
    pub fn pmap7(&mut self) -> PMAP7_W {
        PMAP7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port P4.6 mapping register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4map6](index.html) module"]
pub struct P4MAP6_SPEC;
impl crate::RegisterSpec for P4MAP6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p4map6::R](R) reader structure"]
impl crate::Readable for P4MAP6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p4map6::W](W) writer structure"]
impl crate::Writable for P4MAP6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P4MAP6 to value 0"]
impl crate::Resettable for P4MAP6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
