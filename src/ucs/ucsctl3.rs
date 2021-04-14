#[doc = "Register `UCSCTL3` reader"]
pub struct R(crate::R<UCSCTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCSCTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCSCTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCSCTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCSCTL3` writer"]
pub struct W(crate::W<UCSCTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCSCTL3_SPEC>;
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
impl From<crate::W<UCSCTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCSCTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reference Divider Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLLREFDIV_A {
    #[doc = "0: Reference Divider: f(LFCLK)/1"]
    FLLREFDIV_0 = 0,
    #[doc = "1: Reference Divider: f(LFCLK)/2"]
    FLLREFDIV_1 = 1,
    #[doc = "2: Reference Divider: f(LFCLK)/4"]
    FLLREFDIV_2 = 2,
    #[doc = "3: Reference Divider: f(LFCLK)/8"]
    FLLREFDIV_3 = 3,
    #[doc = "4: Reference Divider: f(LFCLK)/12"]
    FLLREFDIV_4 = 4,
    #[doc = "5: Reference Divider: f(LFCLK)/16"]
    FLLREFDIV_5 = 5,
    #[doc = "6: Reference Divider: f(LFCLK)/16"]
    FLLREFDIV_6 = 6,
    #[doc = "7: Reference Divider: f(LFCLK)/16"]
    FLLREFDIV_7 = 7,
}
impl From<FLLREFDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: FLLREFDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLLREFDIV` reader - Reference Divider Bit : 0"]
pub struct FLLREFDIV_R(crate::FieldReader<u8, FLLREFDIV_A>);
impl FLLREFDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLLREFDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLLREFDIV_A {
        match self.bits {
            0 => FLLREFDIV_A::FLLREFDIV_0,
            1 => FLLREFDIV_A::FLLREFDIV_1,
            2 => FLLREFDIV_A::FLLREFDIV_2,
            3 => FLLREFDIV_A::FLLREFDIV_3,
            4 => FLLREFDIV_A::FLLREFDIV_4,
            5 => FLLREFDIV_A::FLLREFDIV_5,
            6 => FLLREFDIV_A::FLLREFDIV_6,
            7 => FLLREFDIV_A::FLLREFDIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_0`"]
    #[inline(always)]
    pub fn is_fllrefdiv_0(&self) -> bool {
        **self == FLLREFDIV_A::FLLREFDIV_0
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_1`"]
    #[inline(always)]
    pub fn is_fllrefdiv_1(&self) -> bool {
        **self == FLLREFDIV_A::FLLREFDIV_1
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_2`"]
    #[inline(always)]
    pub fn is_fllrefdiv_2(&self) -> bool {
        **self == FLLREFDIV_A::FLLREFDIV_2
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_3`"]
    #[inline(always)]
    pub fn is_fllrefdiv_3(&self) -> bool {
        **self == FLLREFDIV_A::FLLREFDIV_3
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_4`"]
    #[inline(always)]
    pub fn is_fllrefdiv_4(&self) -> bool {
        **self == FLLREFDIV_A::FLLREFDIV_4
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_5`"]
    #[inline(always)]
    pub fn is_fllrefdiv_5(&self) -> bool {
        **self == FLLREFDIV_A::FLLREFDIV_5
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_6`"]
    #[inline(always)]
    pub fn is_fllrefdiv_6(&self) -> bool {
        **self == FLLREFDIV_A::FLLREFDIV_6
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_7`"]
    #[inline(always)]
    pub fn is_fllrefdiv_7(&self) -> bool {
        **self == FLLREFDIV_A::FLLREFDIV_7
    }
}
impl core::ops::Deref for FLLREFDIV_R {
    type Target = crate::FieldReader<u8, FLLREFDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLLREFDIV` writer - Reference Divider Bit : 0"]
pub struct FLLREFDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FLLREFDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLLREFDIV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Reference Divider: f(LFCLK)/1"]
    #[inline(always)]
    pub fn fllrefdiv_0(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::FLLREFDIV_0)
    }
    #[doc = "Reference Divider: f(LFCLK)/2"]
    #[inline(always)]
    pub fn fllrefdiv_1(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::FLLREFDIV_1)
    }
    #[doc = "Reference Divider: f(LFCLK)/4"]
    #[inline(always)]
    pub fn fllrefdiv_2(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::FLLREFDIV_2)
    }
    #[doc = "Reference Divider: f(LFCLK)/8"]
    #[inline(always)]
    pub fn fllrefdiv_3(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::FLLREFDIV_3)
    }
    #[doc = "Reference Divider: f(LFCLK)/12"]
    #[inline(always)]
    pub fn fllrefdiv_4(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::FLLREFDIV_4)
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline(always)]
    pub fn fllrefdiv_5(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::FLLREFDIV_5)
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline(always)]
    pub fn fllrefdiv_6(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::FLLREFDIV_6)
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline(always)]
    pub fn fllrefdiv_7(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::FLLREFDIV_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u16 & 0x07);
        self.w
    }
}
#[doc = "FLL Reference Clock Select Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELREF_A {
    #[doc = "0: FLL Reference Clock Select 0"]
    SELREF_0 = 0,
    #[doc = "1: FLL Reference Clock Select 1"]
    SELREF_1 = 1,
    #[doc = "2: FLL Reference Clock Select 2"]
    SELREF_2 = 2,
    #[doc = "3: FLL Reference Clock Select 3"]
    SELREF_3 = 3,
    #[doc = "4: FLL Reference Clock Select 4"]
    SELREF_4 = 4,
    #[doc = "5: FLL Reference Clock Select 5"]
    SELREF_5 = 5,
    #[doc = "6: FLL Reference Clock Select 6"]
    SELREF_6 = 6,
    #[doc = "7: FLL Reference Clock Select 7"]
    SELREF_7 = 7,
}
impl From<SELREF_A> for u8 {
    #[inline(always)]
    fn from(variant: SELREF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SELREF` reader - FLL Reference Clock Select Bit : 0"]
pub struct SELREF_R(crate::FieldReader<u8, SELREF_A>);
impl SELREF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SELREF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELREF_A {
        match self.bits {
            0 => SELREF_A::SELREF_0,
            1 => SELREF_A::SELREF_1,
            2 => SELREF_A::SELREF_2,
            3 => SELREF_A::SELREF_3,
            4 => SELREF_A::SELREF_4,
            5 => SELREF_A::SELREF_5,
            6 => SELREF_A::SELREF_6,
            7 => SELREF_A::SELREF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELREF_0`"]
    #[inline(always)]
    pub fn is_selref_0(&self) -> bool {
        **self == SELREF_A::SELREF_0
    }
    #[doc = "Checks if the value of the field is `SELREF_1`"]
    #[inline(always)]
    pub fn is_selref_1(&self) -> bool {
        **self == SELREF_A::SELREF_1
    }
    #[doc = "Checks if the value of the field is `SELREF_2`"]
    #[inline(always)]
    pub fn is_selref_2(&self) -> bool {
        **self == SELREF_A::SELREF_2
    }
    #[doc = "Checks if the value of the field is `SELREF_3`"]
    #[inline(always)]
    pub fn is_selref_3(&self) -> bool {
        **self == SELREF_A::SELREF_3
    }
    #[doc = "Checks if the value of the field is `SELREF_4`"]
    #[inline(always)]
    pub fn is_selref_4(&self) -> bool {
        **self == SELREF_A::SELREF_4
    }
    #[doc = "Checks if the value of the field is `SELREF_5`"]
    #[inline(always)]
    pub fn is_selref_5(&self) -> bool {
        **self == SELREF_A::SELREF_5
    }
    #[doc = "Checks if the value of the field is `SELREF_6`"]
    #[inline(always)]
    pub fn is_selref_6(&self) -> bool {
        **self == SELREF_A::SELREF_6
    }
    #[doc = "Checks if the value of the field is `SELREF_7`"]
    #[inline(always)]
    pub fn is_selref_7(&self) -> bool {
        **self == SELREF_A::SELREF_7
    }
}
impl core::ops::Deref for SELREF_R {
    type Target = crate::FieldReader<u8, SELREF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELREF` writer - FLL Reference Clock Select Bit : 0"]
pub struct SELREF_W<'a> {
    w: &'a mut W,
}
impl<'a> SELREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELREF_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FLL Reference Clock Select 0"]
    #[inline(always)]
    pub fn selref_0(self) -> &'a mut W {
        self.variant(SELREF_A::SELREF_0)
    }
    #[doc = "FLL Reference Clock Select 1"]
    #[inline(always)]
    pub fn selref_1(self) -> &'a mut W {
        self.variant(SELREF_A::SELREF_1)
    }
    #[doc = "FLL Reference Clock Select 2"]
    #[inline(always)]
    pub fn selref_2(self) -> &'a mut W {
        self.variant(SELREF_A::SELREF_2)
    }
    #[doc = "FLL Reference Clock Select 3"]
    #[inline(always)]
    pub fn selref_3(self) -> &'a mut W {
        self.variant(SELREF_A::SELREF_3)
    }
    #[doc = "FLL Reference Clock Select 4"]
    #[inline(always)]
    pub fn selref_4(self) -> &'a mut W {
        self.variant(SELREF_A::SELREF_4)
    }
    #[doc = "FLL Reference Clock Select 5"]
    #[inline(always)]
    pub fn selref_5(self) -> &'a mut W {
        self.variant(SELREF_A::SELREF_5)
    }
    #[doc = "FLL Reference Clock Select 6"]
    #[inline(always)]
    pub fn selref_6(self) -> &'a mut W {
        self.variant(SELREF_A::SELREF_6)
    }
    #[doc = "FLL Reference Clock Select 7"]
    #[inline(always)]
    pub fn selref_7(self) -> &'a mut W {
        self.variant(SELREF_A::SELREF_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u16 & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Reference Divider Bit : 0"]
    #[inline(always)]
    pub fn fllrefdiv(&self) -> FLLREFDIV_R {
        FLLREFDIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - FLL Reference Clock Select Bit : 0"]
    #[inline(always)]
    pub fn selref(&self) -> SELREF_R {
        SELREF_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reference Divider Bit : 0"]
    #[inline(always)]
    pub fn fllrefdiv(&mut self) -> FLLREFDIV_W {
        FLLREFDIV_W { w: self }
    }
    #[doc = "Bits 4:6 - FLL Reference Clock Select Bit : 0"]
    #[inline(always)]
    pub fn selref(&mut self) -> SELREF_W {
        SELREF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCS Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucsctl3](index.html) module"]
pub struct UCSCTL3_SPEC;
impl crate::RegisterSpec for UCSCTL3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucsctl3::R](R) reader structure"]
impl crate::Readable for UCSCTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucsctl3::W](W) writer structure"]
impl crate::Writable for UCSCTL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCSCTL3 to value 0"]
impl crate::Resettable for UCSCTL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
