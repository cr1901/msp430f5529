#[doc = "Register `TA0EX0` reader"]
pub struct R(crate::R<TA0EX0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TA0EX0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TA0EX0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TA0EX0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TA0EX0` writer"]
pub struct W(crate::W<TA0EX0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TA0EX0_SPEC>;
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
impl From<crate::W<TA0EX0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TA0EX0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer A Input divider expansion Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAIDEX_A {
    #[doc = "0: Timer A Input divider expansion : /1"]
    TAIDEX_0 = 0,
    #[doc = "1: Timer A Input divider expansion : /2"]
    TAIDEX_1 = 1,
    #[doc = "2: Timer A Input divider expansion : /3"]
    TAIDEX_2 = 2,
    #[doc = "3: Timer A Input divider expansion : /4"]
    TAIDEX_3 = 3,
    #[doc = "4: Timer A Input divider expansion : /5"]
    TAIDEX_4 = 4,
    #[doc = "5: Timer A Input divider expansion : /6"]
    TAIDEX_5 = 5,
    #[doc = "6: Timer A Input divider expansion : /7"]
    TAIDEX_6 = 6,
    #[doc = "7: Timer A Input divider expansion : /8"]
    TAIDEX_7 = 7,
}
impl From<TAIDEX_A> for u8 {
    #[inline(always)]
    fn from(variant: TAIDEX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TAIDEX` reader - Timer A Input divider expansion Bit: 0"]
pub struct TAIDEX_R(crate::FieldReader<u8, TAIDEX_A>);
impl TAIDEX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TAIDEX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAIDEX_A {
        match self.bits {
            0 => TAIDEX_A::TAIDEX_0,
            1 => TAIDEX_A::TAIDEX_1,
            2 => TAIDEX_A::TAIDEX_2,
            3 => TAIDEX_A::TAIDEX_3,
            4 => TAIDEX_A::TAIDEX_4,
            5 => TAIDEX_A::TAIDEX_5,
            6 => TAIDEX_A::TAIDEX_6,
            7 => TAIDEX_A::TAIDEX_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TAIDEX_0`"]
    #[inline(always)]
    pub fn is_taidex_0(&self) -> bool {
        **self == TAIDEX_A::TAIDEX_0
    }
    #[doc = "Checks if the value of the field is `TAIDEX_1`"]
    #[inline(always)]
    pub fn is_taidex_1(&self) -> bool {
        **self == TAIDEX_A::TAIDEX_1
    }
    #[doc = "Checks if the value of the field is `TAIDEX_2`"]
    #[inline(always)]
    pub fn is_taidex_2(&self) -> bool {
        **self == TAIDEX_A::TAIDEX_2
    }
    #[doc = "Checks if the value of the field is `TAIDEX_3`"]
    #[inline(always)]
    pub fn is_taidex_3(&self) -> bool {
        **self == TAIDEX_A::TAIDEX_3
    }
    #[doc = "Checks if the value of the field is `TAIDEX_4`"]
    #[inline(always)]
    pub fn is_taidex_4(&self) -> bool {
        **self == TAIDEX_A::TAIDEX_4
    }
    #[doc = "Checks if the value of the field is `TAIDEX_5`"]
    #[inline(always)]
    pub fn is_taidex_5(&self) -> bool {
        **self == TAIDEX_A::TAIDEX_5
    }
    #[doc = "Checks if the value of the field is `TAIDEX_6`"]
    #[inline(always)]
    pub fn is_taidex_6(&self) -> bool {
        **self == TAIDEX_A::TAIDEX_6
    }
    #[doc = "Checks if the value of the field is `TAIDEX_7`"]
    #[inline(always)]
    pub fn is_taidex_7(&self) -> bool {
        **self == TAIDEX_A::TAIDEX_7
    }
}
impl core::ops::Deref for TAIDEX_R {
    type Target = crate::FieldReader<u8, TAIDEX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAIDEX` writer - Timer A Input divider expansion Bit: 0"]
pub struct TAIDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> TAIDEX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAIDEX_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Timer A Input divider expansion : /1"]
    #[inline(always)]
    pub fn taidex_0(self) -> &'a mut W {
        self.variant(TAIDEX_A::TAIDEX_0)
    }
    #[doc = "Timer A Input divider expansion : /2"]
    #[inline(always)]
    pub fn taidex_1(self) -> &'a mut W {
        self.variant(TAIDEX_A::TAIDEX_1)
    }
    #[doc = "Timer A Input divider expansion : /3"]
    #[inline(always)]
    pub fn taidex_2(self) -> &'a mut W {
        self.variant(TAIDEX_A::TAIDEX_2)
    }
    #[doc = "Timer A Input divider expansion : /4"]
    #[inline(always)]
    pub fn taidex_3(self) -> &'a mut W {
        self.variant(TAIDEX_A::TAIDEX_3)
    }
    #[doc = "Timer A Input divider expansion : /5"]
    #[inline(always)]
    pub fn taidex_4(self) -> &'a mut W {
        self.variant(TAIDEX_A::TAIDEX_4)
    }
    #[doc = "Timer A Input divider expansion : /6"]
    #[inline(always)]
    pub fn taidex_5(self) -> &'a mut W {
        self.variant(TAIDEX_A::TAIDEX_5)
    }
    #[doc = "Timer A Input divider expansion : /7"]
    #[inline(always)]
    pub fn taidex_6(self) -> &'a mut W {
        self.variant(TAIDEX_A::TAIDEX_6)
    }
    #[doc = "Timer A Input divider expansion : /8"]
    #[inline(always)]
    pub fn taidex_7(self) -> &'a mut W {
        self.variant(TAIDEX_A::TAIDEX_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u16 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Timer A Input divider expansion Bit: 0"]
    #[inline(always)]
    pub fn taidex(&self) -> TAIDEX_R {
        TAIDEX_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timer A Input divider expansion Bit: 0"]
    #[inline(always)]
    pub fn taidex(&mut self) -> TAIDEX_W {
        TAIDEX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer0_A5 Expansion Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta0ex0](index.html) module"]
pub struct TA0EX0_SPEC;
impl crate::RegisterSpec for TA0EX0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ta0ex0::R](R) reader structure"]
impl crate::Readable for TA0EX0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ta0ex0::W](W) writer structure"]
impl crate::Writable for TA0EX0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TA0EX0 to value 0"]
impl crate::Resettable for TA0EX0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
