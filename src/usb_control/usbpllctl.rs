#[doc = "Register `USBPLLCTL` reader"]
pub struct R(crate::R<USBPLLCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPLLCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPLLCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPLLCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPLLCTL` writer"]
pub struct W(crate::W<USBPLLCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPLLCTL_SPEC>;
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
impl From<crate::W<USBPLLCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPLLCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB - Module Clock Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UCLKSEL_A {
    #[doc = "0: USB - Module Clock Select: 0"]
    UCLKSEL_0 = 0,
    #[doc = "1: USB - Module Clock Select: 1"]
    UCLKSEL_1 = 1,
    #[doc = "2: USB - Module Clock Select: 2"]
    UCLKSEL_2 = 2,
    #[doc = "3: USB - Module Clock Select: 3 (Reserved)"]
    UCLKSEL_3 = 3,
}
impl From<UCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UCLKSEL` reader - USB - Module Clock Select Bit 0"]
pub struct UCLKSEL_R(crate::FieldReader<u8, UCLKSEL_A>);
impl UCLKSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UCLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCLKSEL_A {
        match self.bits {
            0 => UCLKSEL_A::UCLKSEL_0,
            1 => UCLKSEL_A::UCLKSEL_1,
            2 => UCLKSEL_A::UCLKSEL_2,
            3 => UCLKSEL_A::UCLKSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCLKSEL_0`"]
    #[inline(always)]
    pub fn is_uclksel_0(&self) -> bool {
        **self == UCLKSEL_A::UCLKSEL_0
    }
    #[doc = "Checks if the value of the field is `UCLKSEL_1`"]
    #[inline(always)]
    pub fn is_uclksel_1(&self) -> bool {
        **self == UCLKSEL_A::UCLKSEL_1
    }
    #[doc = "Checks if the value of the field is `UCLKSEL_2`"]
    #[inline(always)]
    pub fn is_uclksel_2(&self) -> bool {
        **self == UCLKSEL_A::UCLKSEL_2
    }
    #[doc = "Checks if the value of the field is `UCLKSEL_3`"]
    #[inline(always)]
    pub fn is_uclksel_3(&self) -> bool {
        **self == UCLKSEL_A::UCLKSEL_3
    }
}
impl core::ops::Deref for UCLKSEL_R {
    type Target = crate::FieldReader<u8, UCLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCLKSEL` writer - USB - Module Clock Select Bit 0"]
pub struct UCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCLKSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "USB - Module Clock Select: 0"]
    #[inline(always)]
    pub fn uclksel_0(self) -> &'a mut W {
        self.variant(UCLKSEL_A::UCLKSEL_0)
    }
    #[doc = "USB - Module Clock Select: 1"]
    #[inline(always)]
    pub fn uclksel_1(self) -> &'a mut W {
        self.variant(UCLKSEL_A::UCLKSEL_1)
    }
    #[doc = "USB - Module Clock Select: 2"]
    #[inline(always)]
    pub fn uclksel_2(self) -> &'a mut W {
        self.variant(UCLKSEL_A::UCLKSEL_2)
    }
    #[doc = "USB - Module Clock Select: 3 (Reserved)"]
    #[inline(always)]
    pub fn uclksel_3(self) -> &'a mut W {
        self.variant(UCLKSEL_A::UCLKSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u16 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `UPLLEN` reader - USB - PLL enable"]
pub struct UPLLEN_R(crate::FieldReader<bool, bool>);
impl UPLLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPLLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPLLEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPLLEN` writer - USB - PLL enable"]
pub struct UPLLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UPLLEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `UPFDEN` reader - USB - Phase Freq. Discriminator enable"]
pub struct UPFDEN_R(crate::FieldReader<bool, bool>);
impl UPFDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPFDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPFDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPFDEN` writer - USB - Phase Freq. Discriminator enable"]
pub struct UPFDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UPFDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - USB - Module Clock Select Bit 0"]
    #[inline(always)]
    pub fn uclksel(&self) -> UCLKSEL_R {
        UCLKSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - USB - PLL enable"]
    #[inline(always)]
    pub fn upllen(&self) -> UPLLEN_R {
        UPLLEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USB - Phase Freq. Discriminator enable"]
    #[inline(always)]
    pub fn upfden(&self) -> UPFDEN_R {
        UPFDEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 6:7 - USB - Module Clock Select Bit 0"]
    #[inline(always)]
    pub fn uclksel(&mut self) -> UCLKSEL_W {
        UCLKSEL_W { w: self }
    }
    #[doc = "Bit 8 - USB - PLL enable"]
    #[inline(always)]
    pub fn upllen(&mut self) -> UPLLEN_W {
        UPLLEN_W { w: self }
    }
    #[doc = "Bit 9 - USB - Phase Freq. Discriminator enable"]
    #[inline(always)]
    pub fn upfden(&mut self) -> UPFDEN_W {
        UPFDEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PLL control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpllctl](index.html) module"]
pub struct USBPLLCTL_SPEC;
impl crate::RegisterSpec for USBPLLCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usbpllctl::R](R) reader structure"]
impl crate::Readable for USBPLLCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbpllctl::W](W) writer structure"]
impl crate::Writable for USBPLLCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBPLLCTL to value 0"]
impl crate::Resettable for USBPLLCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
