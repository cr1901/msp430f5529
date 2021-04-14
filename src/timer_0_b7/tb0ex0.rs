#[doc = "Register `TB0EX0` reader"]
pub struct R(crate::R<TB0EX0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TB0EX0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TB0EX0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TB0EX0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TB0EX0` writer"]
pub struct W(crate::W<TB0EX0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TB0EX0_SPEC>;
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
impl From<crate::W<TB0EX0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TB0EX0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer0_B7 Input divider expansion Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TBIDEX_A {
    #[doc = "0: Timer0_B7 Input divider expansion : /1"]
    TBIDEX_0 = 0,
    #[doc = "1: Timer0_B7 Input divider expansion : /2"]
    TBIDEX_1 = 1,
    #[doc = "2: Timer0_B7 Input divider expansion : /3"]
    TBIDEX_2 = 2,
    #[doc = "3: Timer0_B7 Input divider expansion : /4"]
    TBIDEX_3 = 3,
    #[doc = "4: Timer0_B7 Input divider expansion : /5"]
    TBIDEX_4 = 4,
    #[doc = "5: Timer0_B7 Input divider expansion : /6"]
    TBIDEX_5 = 5,
    #[doc = "6: Timer0_B7 Input divider expansion : /7"]
    TBIDEX_6 = 6,
    #[doc = "7: Timer0_B7 Input divider expansion : /8"]
    TBIDEX_7 = 7,
}
impl From<TBIDEX_A> for u8 {
    #[inline(always)]
    fn from(variant: TBIDEX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TBIDEX` reader - Timer0_B7 Input divider expansion Bit: 0"]
pub struct TBIDEX_R(crate::FieldReader<u8, TBIDEX_A>);
impl TBIDEX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TBIDEX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBIDEX_A {
        match self.bits {
            0 => TBIDEX_A::TBIDEX_0,
            1 => TBIDEX_A::TBIDEX_1,
            2 => TBIDEX_A::TBIDEX_2,
            3 => TBIDEX_A::TBIDEX_3,
            4 => TBIDEX_A::TBIDEX_4,
            5 => TBIDEX_A::TBIDEX_5,
            6 => TBIDEX_A::TBIDEX_6,
            7 => TBIDEX_A::TBIDEX_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TBIDEX_0`"]
    #[inline(always)]
    pub fn is_tbidex_0(&self) -> bool {
        **self == TBIDEX_A::TBIDEX_0
    }
    #[doc = "Checks if the value of the field is `TBIDEX_1`"]
    #[inline(always)]
    pub fn is_tbidex_1(&self) -> bool {
        **self == TBIDEX_A::TBIDEX_1
    }
    #[doc = "Checks if the value of the field is `TBIDEX_2`"]
    #[inline(always)]
    pub fn is_tbidex_2(&self) -> bool {
        **self == TBIDEX_A::TBIDEX_2
    }
    #[doc = "Checks if the value of the field is `TBIDEX_3`"]
    #[inline(always)]
    pub fn is_tbidex_3(&self) -> bool {
        **self == TBIDEX_A::TBIDEX_3
    }
    #[doc = "Checks if the value of the field is `TBIDEX_4`"]
    #[inline(always)]
    pub fn is_tbidex_4(&self) -> bool {
        **self == TBIDEX_A::TBIDEX_4
    }
    #[doc = "Checks if the value of the field is `TBIDEX_5`"]
    #[inline(always)]
    pub fn is_tbidex_5(&self) -> bool {
        **self == TBIDEX_A::TBIDEX_5
    }
    #[doc = "Checks if the value of the field is `TBIDEX_6`"]
    #[inline(always)]
    pub fn is_tbidex_6(&self) -> bool {
        **self == TBIDEX_A::TBIDEX_6
    }
    #[doc = "Checks if the value of the field is `TBIDEX_7`"]
    #[inline(always)]
    pub fn is_tbidex_7(&self) -> bool {
        **self == TBIDEX_A::TBIDEX_7
    }
}
impl core::ops::Deref for TBIDEX_R {
    type Target = crate::FieldReader<u8, TBIDEX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBIDEX` writer - Timer0_B7 Input divider expansion Bit: 0"]
pub struct TBIDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> TBIDEX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBIDEX_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Timer0_B7 Input divider expansion : /1"]
    #[inline(always)]
    pub fn tbidex_0(self) -> &'a mut W {
        self.variant(TBIDEX_A::TBIDEX_0)
    }
    #[doc = "Timer0_B7 Input divider expansion : /2"]
    #[inline(always)]
    pub fn tbidex_1(self) -> &'a mut W {
        self.variant(TBIDEX_A::TBIDEX_1)
    }
    #[doc = "Timer0_B7 Input divider expansion : /3"]
    #[inline(always)]
    pub fn tbidex_2(self) -> &'a mut W {
        self.variant(TBIDEX_A::TBIDEX_2)
    }
    #[doc = "Timer0_B7 Input divider expansion : /4"]
    #[inline(always)]
    pub fn tbidex_3(self) -> &'a mut W {
        self.variant(TBIDEX_A::TBIDEX_3)
    }
    #[doc = "Timer0_B7 Input divider expansion : /5"]
    #[inline(always)]
    pub fn tbidex_4(self) -> &'a mut W {
        self.variant(TBIDEX_A::TBIDEX_4)
    }
    #[doc = "Timer0_B7 Input divider expansion : /6"]
    #[inline(always)]
    pub fn tbidex_5(self) -> &'a mut W {
        self.variant(TBIDEX_A::TBIDEX_5)
    }
    #[doc = "Timer0_B7 Input divider expansion : /7"]
    #[inline(always)]
    pub fn tbidex_6(self) -> &'a mut W {
        self.variant(TBIDEX_A::TBIDEX_6)
    }
    #[doc = "Timer0_B7 Input divider expansion : /8"]
    #[inline(always)]
    pub fn tbidex_7(self) -> &'a mut W {
        self.variant(TBIDEX_A::TBIDEX_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u16 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Timer0_B7 Input divider expansion Bit: 0"]
    #[inline(always)]
    pub fn tbidex(&self) -> TBIDEX_R {
        TBIDEX_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timer0_B7 Input divider expansion Bit: 0"]
    #[inline(always)]
    pub fn tbidex(&mut self) -> TBIDEX_W {
        TBIDEX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer0_B7 Expansion Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0ex0](index.html) module"]
pub struct TB0EX0_SPEC;
impl crate::RegisterSpec for TB0EX0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tb0ex0::R](R) reader structure"]
impl crate::Readable for TB0EX0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tb0ex0::W](W) writer structure"]
impl crate::Writable for TB0EX0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TB0EX0 to value 0"]
impl crate::Resettable for TB0EX0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
