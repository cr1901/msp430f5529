#[doc = "Reader of register TB0EX0"]
pub type R = crate::R<u16, super::TB0EX0>;
#[doc = "Writer for register TB0EX0"]
pub type W = crate::W<u16, super::TB0EX0>;
#[doc = "Register TB0EX0 `reset()`'s with value 0"]
impl crate::ResetValue for super::TB0EX0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `TBIDEX`"]
pub type TBIDEX_R = crate::R<u8, TBIDEX_A>;
impl TBIDEX_R {
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
        *self == TBIDEX_A::TBIDEX_0
    }
    #[doc = "Checks if the value of the field is `TBIDEX_1`"]
    #[inline(always)]
    pub fn is_tbidex_1(&self) -> bool {
        *self == TBIDEX_A::TBIDEX_1
    }
    #[doc = "Checks if the value of the field is `TBIDEX_2`"]
    #[inline(always)]
    pub fn is_tbidex_2(&self) -> bool {
        *self == TBIDEX_A::TBIDEX_2
    }
    #[doc = "Checks if the value of the field is `TBIDEX_3`"]
    #[inline(always)]
    pub fn is_tbidex_3(&self) -> bool {
        *self == TBIDEX_A::TBIDEX_3
    }
    #[doc = "Checks if the value of the field is `TBIDEX_4`"]
    #[inline(always)]
    pub fn is_tbidex_4(&self) -> bool {
        *self == TBIDEX_A::TBIDEX_4
    }
    #[doc = "Checks if the value of the field is `TBIDEX_5`"]
    #[inline(always)]
    pub fn is_tbidex_5(&self) -> bool {
        *self == TBIDEX_A::TBIDEX_5
    }
    #[doc = "Checks if the value of the field is `TBIDEX_6`"]
    #[inline(always)]
    pub fn is_tbidex_6(&self) -> bool {
        *self == TBIDEX_A::TBIDEX_6
    }
    #[doc = "Checks if the value of the field is `TBIDEX_7`"]
    #[inline(always)]
    pub fn is_tbidex_7(&self) -> bool {
        *self == TBIDEX_A::TBIDEX_7
    }
}
#[doc = "Write proxy for field `TBIDEX`"]
pub struct TBIDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> TBIDEX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBIDEX_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
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
}
