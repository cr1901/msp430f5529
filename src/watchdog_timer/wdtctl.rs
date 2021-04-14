#[doc = "Register `WDTCTL` reader"]
pub struct R(crate::R<WDTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCTL` writer"]
pub struct W(crate::W<WDTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCTL_SPEC>;
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
impl From<crate::W<WDTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "WDT - Timer Interval Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDTIS_A {
    #[doc = "0: WDT - Timer Interval Select: /2G"]
    WDTIS_0 = 0,
    #[doc = "1: WDT - Timer Interval Select: /128M"]
    WDTIS_1 = 1,
    #[doc = "2: WDT - Timer Interval Select: /8192k"]
    WDTIS_2 = 2,
    #[doc = "3: WDT - Timer Interval Select: /512k"]
    WDTIS_3 = 3,
    #[doc = "4: WDT - Timer Interval Select: /32k"]
    WDTIS_4 = 4,
    #[doc = "5: WDT - Timer Interval Select: /8192"]
    WDTIS_5 = 5,
    #[doc = "6: WDT - Timer Interval Select: /512"]
    WDTIS_6 = 6,
    #[doc = "7: WDT - Timer Interval Select: /64"]
    WDTIS_7 = 7,
}
impl From<WDTIS_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTIS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WDTIS` reader - WDT - Timer Interval Select 0"]
pub struct WDTIS_R(crate::FieldReader<u8, WDTIS_A>);
impl WDTIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDTIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTIS_A {
        match self.bits {
            0 => WDTIS_A::WDTIS_0,
            1 => WDTIS_A::WDTIS_1,
            2 => WDTIS_A::WDTIS_2,
            3 => WDTIS_A::WDTIS_3,
            4 => WDTIS_A::WDTIS_4,
            5 => WDTIS_A::WDTIS_5,
            6 => WDTIS_A::WDTIS_6,
            7 => WDTIS_A::WDTIS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WDTIS_0`"]
    #[inline(always)]
    pub fn is_wdtis_0(&self) -> bool {
        **self == WDTIS_A::WDTIS_0
    }
    #[doc = "Checks if the value of the field is `WDTIS_1`"]
    #[inline(always)]
    pub fn is_wdtis_1(&self) -> bool {
        **self == WDTIS_A::WDTIS_1
    }
    #[doc = "Checks if the value of the field is `WDTIS_2`"]
    #[inline(always)]
    pub fn is_wdtis_2(&self) -> bool {
        **self == WDTIS_A::WDTIS_2
    }
    #[doc = "Checks if the value of the field is `WDTIS_3`"]
    #[inline(always)]
    pub fn is_wdtis_3(&self) -> bool {
        **self == WDTIS_A::WDTIS_3
    }
    #[doc = "Checks if the value of the field is `WDTIS_4`"]
    #[inline(always)]
    pub fn is_wdtis_4(&self) -> bool {
        **self == WDTIS_A::WDTIS_4
    }
    #[doc = "Checks if the value of the field is `WDTIS_5`"]
    #[inline(always)]
    pub fn is_wdtis_5(&self) -> bool {
        **self == WDTIS_A::WDTIS_5
    }
    #[doc = "Checks if the value of the field is `WDTIS_6`"]
    #[inline(always)]
    pub fn is_wdtis_6(&self) -> bool {
        **self == WDTIS_A::WDTIS_6
    }
    #[doc = "Checks if the value of the field is `WDTIS_7`"]
    #[inline(always)]
    pub fn is_wdtis_7(&self) -> bool {
        **self == WDTIS_A::WDTIS_7
    }
}
impl core::ops::Deref for WDTIS_R {
    type Target = crate::FieldReader<u8, WDTIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTIS` writer - WDT - Timer Interval Select 0"]
pub struct WDTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTIS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "WDT - Timer Interval Select: /2G"]
    #[inline(always)]
    pub fn wdtis_0(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_0)
    }
    #[doc = "WDT - Timer Interval Select: /128M"]
    #[inline(always)]
    pub fn wdtis_1(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_1)
    }
    #[doc = "WDT - Timer Interval Select: /8192k"]
    #[inline(always)]
    pub fn wdtis_2(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_2)
    }
    #[doc = "WDT - Timer Interval Select: /512k"]
    #[inline(always)]
    pub fn wdtis_3(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_3)
    }
    #[doc = "WDT - Timer Interval Select: /32k"]
    #[inline(always)]
    pub fn wdtis_4(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_4)
    }
    #[doc = "WDT - Timer Interval Select: /8192"]
    #[inline(always)]
    pub fn wdtis_5(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_5)
    }
    #[doc = "WDT - Timer Interval Select: /512"]
    #[inline(always)]
    pub fn wdtis_6(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_6)
    }
    #[doc = "WDT - Timer Interval Select: /64"]
    #[inline(always)]
    pub fn wdtis_7(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u16 & 0x07);
        self.w
    }
}
#[doc = "Field `WDTCNTCL` reader - WDT - Timer Clear"]
pub struct WDTCNTCL_R(crate::FieldReader<bool, bool>);
impl WDTCNTCL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDTCNTCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTCNTCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTCNTCL` writer - WDT - Timer Clear"]
pub struct WDTCNTCL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTCNTCL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `WDTTMSEL` reader - WDT - Timer Mode Select"]
pub struct WDTTMSEL_R(crate::FieldReader<bool, bool>);
impl WDTTMSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDTTMSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTTMSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTTMSEL` writer - WDT - Timer Mode Select"]
pub struct WDTTMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTTMSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "WDT - Timer Clock Source Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDTSSEL_A {
    #[doc = "0: WDT - Timer Clock Source Select: SMCLK"]
    WDTSSEL_0 = 0,
    #[doc = "1: WDT - Timer Clock Source Select: ACLK"]
    WDTSSEL_1 = 1,
    #[doc = "2: WDT - Timer Clock Source Select: VLO_CLK"]
    WDTSSEL_2 = 2,
    #[doc = "3: WDT - Timer Clock Source Select: reserved"]
    WDTSSEL_3 = 3,
}
impl From<WDTSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WDTSSEL` reader - WDT - Timer Clock Source Select 0"]
pub struct WDTSSEL_R(crate::FieldReader<u8, WDTSSEL_A>);
impl WDTSSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDTSSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTSSEL_A {
        match self.bits {
            0 => WDTSSEL_A::WDTSSEL_0,
            1 => WDTSSEL_A::WDTSSEL_1,
            2 => WDTSSEL_A::WDTSSEL_2,
            3 => WDTSSEL_A::WDTSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WDTSSEL_0`"]
    #[inline(always)]
    pub fn is_wdtssel_0(&self) -> bool {
        **self == WDTSSEL_A::WDTSSEL_0
    }
    #[doc = "Checks if the value of the field is `WDTSSEL_1`"]
    #[inline(always)]
    pub fn is_wdtssel_1(&self) -> bool {
        **self == WDTSSEL_A::WDTSSEL_1
    }
    #[doc = "Checks if the value of the field is `WDTSSEL_2`"]
    #[inline(always)]
    pub fn is_wdtssel_2(&self) -> bool {
        **self == WDTSSEL_A::WDTSSEL_2
    }
    #[doc = "Checks if the value of the field is `WDTSSEL_3`"]
    #[inline(always)]
    pub fn is_wdtssel_3(&self) -> bool {
        **self == WDTSSEL_A::WDTSSEL_3
    }
}
impl core::ops::Deref for WDTSSEL_R {
    type Target = crate::FieldReader<u8, WDTSSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTSSEL` writer - WDT - Timer Clock Source Select 0"]
pub struct WDTSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTSSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "WDT - Timer Clock Source Select: SMCLK"]
    #[inline(always)]
    pub fn wdtssel_0(self) -> &'a mut W {
        self.variant(WDTSSEL_A::WDTSSEL_0)
    }
    #[doc = "WDT - Timer Clock Source Select: ACLK"]
    #[inline(always)]
    pub fn wdtssel_1(self) -> &'a mut W {
        self.variant(WDTSSEL_A::WDTSSEL_1)
    }
    #[doc = "WDT - Timer Clock Source Select: VLO_CLK"]
    #[inline(always)]
    pub fn wdtssel_2(self) -> &'a mut W {
        self.variant(WDTSSEL_A::WDTSSEL_2)
    }
    #[doc = "WDT - Timer Clock Source Select: reserved"]
    #[inline(always)]
    pub fn wdtssel_3(self) -> &'a mut W {
        self.variant(WDTSSEL_A::WDTSSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u16 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `WDTHOLD` reader - WDT - Timer hold"]
pub struct WDTHOLD_R(crate::FieldReader<bool, bool>);
impl WDTHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDTHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTHOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTHOLD` writer - WDT - Timer hold"]
pub struct WDTHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTHOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - WDT - Timer Interval Select 0"]
    #[inline(always)]
    pub fn wdtis(&self) -> WDTIS_R {
        WDTIS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - WDT - Timer Clear"]
    #[inline(always)]
    pub fn wdtcntcl(&self) -> WDTCNTCL_R {
        WDTCNTCL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WDT - Timer Mode Select"]
    #[inline(always)]
    pub fn wdttmsel(&self) -> WDTTMSEL_R {
        WDTTMSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - WDT - Timer Clock Source Select 0"]
    #[inline(always)]
    pub fn wdtssel(&self) -> WDTSSEL_R {
        WDTSSEL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - WDT - Timer hold"]
    #[inline(always)]
    pub fn wdthold(&self) -> WDTHOLD_R {
        WDTHOLD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - WDT - Timer Interval Select 0"]
    #[inline(always)]
    pub fn wdtis(&mut self) -> WDTIS_W {
        WDTIS_W { w: self }
    }
    #[doc = "Bit 3 - WDT - Timer Clear"]
    #[inline(always)]
    pub fn wdtcntcl(&mut self) -> WDTCNTCL_W {
        WDTCNTCL_W { w: self }
    }
    #[doc = "Bit 4 - WDT - Timer Mode Select"]
    #[inline(always)]
    pub fn wdttmsel(&mut self) -> WDTTMSEL_W {
        WDTTMSEL_W { w: self }
    }
    #[doc = "Bits 5:6 - WDT - Timer Clock Source Select 0"]
    #[inline(always)]
    pub fn wdtssel(&mut self) -> WDTSSEL_W {
        WDTSSEL_W { w: self }
    }
    #[doc = "Bit 7 - WDT - Timer hold"]
    #[inline(always)]
    pub fn wdthold(&mut self) -> WDTHOLD_W {
        WDTHOLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtctl](index.html) module"]
pub struct WDTCTL_SPEC;
impl crate::RegisterSpec for WDTCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wdtctl::R](R) reader structure"]
impl crate::Readable for WDTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtctl::W](W) writer structure"]
impl crate::Writable for WDTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTCTL to value 0"]
impl crate::Resettable for WDTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
