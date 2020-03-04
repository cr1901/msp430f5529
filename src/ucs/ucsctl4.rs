#[doc = "Reader of register UCSCTL4"]
pub type R = crate::R<u16, super::UCSCTL4>;
#[doc = "Writer for register UCSCTL4"]
pub type W = crate::W<u16, super::UCSCTL4>;
#[doc = "Register UCSCTL4 `reset()`'s with value 0"]
impl crate::ResetValue for super::UCSCTL4 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MCLK Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELM_A {
    #[doc = "0: MCLK Source Select 0"]
    SELM_0 = 0,
    #[doc = "1: MCLK Source Select 1"]
    SELM_1 = 1,
    #[doc = "2: MCLK Source Select 2"]
    SELM_2 = 2,
    #[doc = "3: MCLK Source Select 3"]
    SELM_3 = 3,
    #[doc = "4: MCLK Source Select 4"]
    SELM_4 = 4,
    #[doc = "5: MCLK Source Select 5"]
    SELM_5 = 5,
    #[doc = "6: MCLK Source Select 6"]
    SELM_6 = 6,
    #[doc = "7: MCLK Source Select 7"]
    SELM_7 = 7,
}
impl From<SELM_A> for u8 {
    #[inline(always)]
    fn from(variant: SELM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SELM`"]
pub type SELM_R = crate::R<u8, SELM_A>;
impl SELM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELM_A {
        match self.bits {
            0 => SELM_A::SELM_0,
            1 => SELM_A::SELM_1,
            2 => SELM_A::SELM_2,
            3 => SELM_A::SELM_3,
            4 => SELM_A::SELM_4,
            5 => SELM_A::SELM_5,
            6 => SELM_A::SELM_6,
            7 => SELM_A::SELM_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELM_0`"]
    #[inline(always)]
    pub fn is_selm_0(&self) -> bool {
        *self == SELM_A::SELM_0
    }
    #[doc = "Checks if the value of the field is `SELM_1`"]
    #[inline(always)]
    pub fn is_selm_1(&self) -> bool {
        *self == SELM_A::SELM_1
    }
    #[doc = "Checks if the value of the field is `SELM_2`"]
    #[inline(always)]
    pub fn is_selm_2(&self) -> bool {
        *self == SELM_A::SELM_2
    }
    #[doc = "Checks if the value of the field is `SELM_3`"]
    #[inline(always)]
    pub fn is_selm_3(&self) -> bool {
        *self == SELM_A::SELM_3
    }
    #[doc = "Checks if the value of the field is `SELM_4`"]
    #[inline(always)]
    pub fn is_selm_4(&self) -> bool {
        *self == SELM_A::SELM_4
    }
    #[doc = "Checks if the value of the field is `SELM_5`"]
    #[inline(always)]
    pub fn is_selm_5(&self) -> bool {
        *self == SELM_A::SELM_5
    }
    #[doc = "Checks if the value of the field is `SELM_6`"]
    #[inline(always)]
    pub fn is_selm_6(&self) -> bool {
        *self == SELM_A::SELM_6
    }
    #[doc = "Checks if the value of the field is `SELM_7`"]
    #[inline(always)]
    pub fn is_selm_7(&self) -> bool {
        *self == SELM_A::SELM_7
    }
}
#[doc = "Write proxy for field `SELM`"]
pub struct SELM_W<'a> {
    w: &'a mut W,
}
impl<'a> SELM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "MCLK Source Select 0"]
    #[inline(always)]
    pub fn selm_0(self) -> &'a mut W {
        self.variant(SELM_A::SELM_0)
    }
    #[doc = "MCLK Source Select 1"]
    #[inline(always)]
    pub fn selm_1(self) -> &'a mut W {
        self.variant(SELM_A::SELM_1)
    }
    #[doc = "MCLK Source Select 2"]
    #[inline(always)]
    pub fn selm_2(self) -> &'a mut W {
        self.variant(SELM_A::SELM_2)
    }
    #[doc = "MCLK Source Select 3"]
    #[inline(always)]
    pub fn selm_3(self) -> &'a mut W {
        self.variant(SELM_A::SELM_3)
    }
    #[doc = "MCLK Source Select 4"]
    #[inline(always)]
    pub fn selm_4(self) -> &'a mut W {
        self.variant(SELM_A::SELM_4)
    }
    #[doc = "MCLK Source Select 5"]
    #[inline(always)]
    pub fn selm_5(self) -> &'a mut W {
        self.variant(SELM_A::SELM_5)
    }
    #[doc = "MCLK Source Select 6"]
    #[inline(always)]
    pub fn selm_6(self) -> &'a mut W {
        self.variant(SELM_A::SELM_6)
    }
    #[doc = "MCLK Source Select 7"]
    #[inline(always)]
    pub fn selm_7(self) -> &'a mut W {
        self.variant(SELM_A::SELM_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "SMCLK Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELS_A {
    #[doc = "0: SMCLK Source Select 0"]
    SELS_0 = 0,
    #[doc = "1: SMCLK Source Select 1"]
    SELS_1 = 1,
    #[doc = "2: SMCLK Source Select 2"]
    SELS_2 = 2,
    #[doc = "3: SMCLK Source Select 3"]
    SELS_3 = 3,
    #[doc = "4: SMCLK Source Select 4"]
    SELS_4 = 4,
    #[doc = "5: SMCLK Source Select 5"]
    SELS_5 = 5,
    #[doc = "6: SMCLK Source Select 6"]
    SELS_6 = 6,
    #[doc = "7: SMCLK Source Select 7"]
    SELS_7 = 7,
}
impl From<SELS_A> for u8 {
    #[inline(always)]
    fn from(variant: SELS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SELS`"]
pub type SELS_R = crate::R<u8, SELS_A>;
impl SELS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELS_A {
        match self.bits {
            0 => SELS_A::SELS_0,
            1 => SELS_A::SELS_1,
            2 => SELS_A::SELS_2,
            3 => SELS_A::SELS_3,
            4 => SELS_A::SELS_4,
            5 => SELS_A::SELS_5,
            6 => SELS_A::SELS_6,
            7 => SELS_A::SELS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELS_0`"]
    #[inline(always)]
    pub fn is_sels_0(&self) -> bool {
        *self == SELS_A::SELS_0
    }
    #[doc = "Checks if the value of the field is `SELS_1`"]
    #[inline(always)]
    pub fn is_sels_1(&self) -> bool {
        *self == SELS_A::SELS_1
    }
    #[doc = "Checks if the value of the field is `SELS_2`"]
    #[inline(always)]
    pub fn is_sels_2(&self) -> bool {
        *self == SELS_A::SELS_2
    }
    #[doc = "Checks if the value of the field is `SELS_3`"]
    #[inline(always)]
    pub fn is_sels_3(&self) -> bool {
        *self == SELS_A::SELS_3
    }
    #[doc = "Checks if the value of the field is `SELS_4`"]
    #[inline(always)]
    pub fn is_sels_4(&self) -> bool {
        *self == SELS_A::SELS_4
    }
    #[doc = "Checks if the value of the field is `SELS_5`"]
    #[inline(always)]
    pub fn is_sels_5(&self) -> bool {
        *self == SELS_A::SELS_5
    }
    #[doc = "Checks if the value of the field is `SELS_6`"]
    #[inline(always)]
    pub fn is_sels_6(&self) -> bool {
        *self == SELS_A::SELS_6
    }
    #[doc = "Checks if the value of the field is `SELS_7`"]
    #[inline(always)]
    pub fn is_sels_7(&self) -> bool {
        *self == SELS_A::SELS_7
    }
}
#[doc = "Write proxy for field `SELS`"]
pub struct SELS_W<'a> {
    w: &'a mut W,
}
impl<'a> SELS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SMCLK Source Select 0"]
    #[inline(always)]
    pub fn sels_0(self) -> &'a mut W {
        self.variant(SELS_A::SELS_0)
    }
    #[doc = "SMCLK Source Select 1"]
    #[inline(always)]
    pub fn sels_1(self) -> &'a mut W {
        self.variant(SELS_A::SELS_1)
    }
    #[doc = "SMCLK Source Select 2"]
    #[inline(always)]
    pub fn sels_2(self) -> &'a mut W {
        self.variant(SELS_A::SELS_2)
    }
    #[doc = "SMCLK Source Select 3"]
    #[inline(always)]
    pub fn sels_3(self) -> &'a mut W {
        self.variant(SELS_A::SELS_3)
    }
    #[doc = "SMCLK Source Select 4"]
    #[inline(always)]
    pub fn sels_4(self) -> &'a mut W {
        self.variant(SELS_A::SELS_4)
    }
    #[doc = "SMCLK Source Select 5"]
    #[inline(always)]
    pub fn sels_5(self) -> &'a mut W {
        self.variant(SELS_A::SELS_5)
    }
    #[doc = "SMCLK Source Select 6"]
    #[inline(always)]
    pub fn sels_6(self) -> &'a mut W {
        self.variant(SELS_A::SELS_6)
    }
    #[doc = "SMCLK Source Select 7"]
    #[inline(always)]
    pub fn sels_7(self) -> &'a mut W {
        self.variant(SELS_A::SELS_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u16) & 0x07) << 4);
        self.w
    }
}
#[doc = "ACLK Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELA_A {
    #[doc = "0: ACLK Source Select 0"]
    SELA_0 = 0,
    #[doc = "1: ACLK Source Select 1"]
    SELA_1 = 1,
    #[doc = "2: ACLK Source Select 2"]
    SELA_2 = 2,
    #[doc = "3: ACLK Source Select 3"]
    SELA_3 = 3,
    #[doc = "4: ACLK Source Select 4"]
    SELA_4 = 4,
    #[doc = "5: ACLK Source Select 5"]
    SELA_5 = 5,
    #[doc = "6: ACLK Source Select 6"]
    SELA_6 = 6,
    #[doc = "7: ACLK Source Select 7"]
    SELA_7 = 7,
}
impl From<SELA_A> for u8 {
    #[inline(always)]
    fn from(variant: SELA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SELA`"]
pub type SELA_R = crate::R<u8, SELA_A>;
impl SELA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELA_A {
        match self.bits {
            0 => SELA_A::SELA_0,
            1 => SELA_A::SELA_1,
            2 => SELA_A::SELA_2,
            3 => SELA_A::SELA_3,
            4 => SELA_A::SELA_4,
            5 => SELA_A::SELA_5,
            6 => SELA_A::SELA_6,
            7 => SELA_A::SELA_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELA_0`"]
    #[inline(always)]
    pub fn is_sela_0(&self) -> bool {
        *self == SELA_A::SELA_0
    }
    #[doc = "Checks if the value of the field is `SELA_1`"]
    #[inline(always)]
    pub fn is_sela_1(&self) -> bool {
        *self == SELA_A::SELA_1
    }
    #[doc = "Checks if the value of the field is `SELA_2`"]
    #[inline(always)]
    pub fn is_sela_2(&self) -> bool {
        *self == SELA_A::SELA_2
    }
    #[doc = "Checks if the value of the field is `SELA_3`"]
    #[inline(always)]
    pub fn is_sela_3(&self) -> bool {
        *self == SELA_A::SELA_3
    }
    #[doc = "Checks if the value of the field is `SELA_4`"]
    #[inline(always)]
    pub fn is_sela_4(&self) -> bool {
        *self == SELA_A::SELA_4
    }
    #[doc = "Checks if the value of the field is `SELA_5`"]
    #[inline(always)]
    pub fn is_sela_5(&self) -> bool {
        *self == SELA_A::SELA_5
    }
    #[doc = "Checks if the value of the field is `SELA_6`"]
    #[inline(always)]
    pub fn is_sela_6(&self) -> bool {
        *self == SELA_A::SELA_6
    }
    #[doc = "Checks if the value of the field is `SELA_7`"]
    #[inline(always)]
    pub fn is_sela_7(&self) -> bool {
        *self == SELA_A::SELA_7
    }
}
#[doc = "Write proxy for field `SELA`"]
pub struct SELA_W<'a> {
    w: &'a mut W,
}
impl<'a> SELA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELA_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ACLK Source Select 0"]
    #[inline(always)]
    pub fn sela_0(self) -> &'a mut W {
        self.variant(SELA_A::SELA_0)
    }
    #[doc = "ACLK Source Select 1"]
    #[inline(always)]
    pub fn sela_1(self) -> &'a mut W {
        self.variant(SELA_A::SELA_1)
    }
    #[doc = "ACLK Source Select 2"]
    #[inline(always)]
    pub fn sela_2(self) -> &'a mut W {
        self.variant(SELA_A::SELA_2)
    }
    #[doc = "ACLK Source Select 3"]
    #[inline(always)]
    pub fn sela_3(self) -> &'a mut W {
        self.variant(SELA_A::SELA_3)
    }
    #[doc = "ACLK Source Select 4"]
    #[inline(always)]
    pub fn sela_4(self) -> &'a mut W {
        self.variant(SELA_A::SELA_4)
    }
    #[doc = "ACLK Source Select 5"]
    #[inline(always)]
    pub fn sela_5(self) -> &'a mut W {
        self.variant(SELA_A::SELA_5)
    }
    #[doc = "ACLK Source Select 6"]
    #[inline(always)]
    pub fn sela_6(self) -> &'a mut W {
        self.variant(SELA_A::SELA_6)
    }
    #[doc = "ACLK Source Select 7"]
    #[inline(always)]
    pub fn sela_7(self) -> &'a mut W {
        self.variant(SELA_A::SELA_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - MCLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn selm(&self) -> SELM_R {
        SELM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - SMCLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn sels(&self) -> SELS_R {
        SELS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - ACLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn sela(&self) -> SELA_R {
        SELA_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn selm(&mut self) -> SELM_W {
        SELM_W { w: self }
    }
    #[doc = "Bits 4:6 - SMCLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn sels(&mut self) -> SELS_W {
        SELS_W { w: self }
    }
    #[doc = "Bits 8:10 - ACLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn sela(&mut self) -> SELA_W {
        SELA_W { w: self }
    }
}
