#[doc = "Reader of register CBCTL0"]
pub type R = crate::R<u16, super::CBCTL0>;
#[doc = "Writer for register CBCTL0"]
pub type W = crate::W<u16, super::CBCTL0>;
#[doc = "Register CBCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CBCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Comp. B Pos. Channel Input Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CBIPSEL_A {
    #[doc = "0: Comp. B V+ terminal Input Select: Channel 0"]
    CBIPSEL_0 = 0,
    #[doc = "1: Comp. B V+ terminal Input Select: Channel 1"]
    CBIPSEL_1 = 1,
    #[doc = "2: Comp. B V+ terminal Input Select: Channel 2"]
    CBIPSEL_2 = 2,
    #[doc = "3: Comp. B V+ terminal Input Select: Channel 3"]
    CBIPSEL_3 = 3,
    #[doc = "4: Comp. B V+ terminal Input Select: Channel 4"]
    CBIPSEL_4 = 4,
    #[doc = "5: Comp. B V+ terminal Input Select: Channel 5"]
    CBIPSEL_5 = 5,
    #[doc = "6: Comp. B V+ terminal Input Select: Channel 6"]
    CBIPSEL_6 = 6,
    #[doc = "7: Comp. B V+ terminal Input Select: Channel 7"]
    CBIPSEL_7 = 7,
    #[doc = "8: Comp. B V+ terminal Input Select: Channel 8"]
    CBIPSEL_8 = 8,
    #[doc = "9: Comp. B V+ terminal Input Select: Channel 9"]
    CBIPSEL_9 = 9,
    #[doc = "10: Comp. B V+ terminal Input Select: Channel 10"]
    CBIPSEL_10 = 10,
    #[doc = "11: Comp. B V+ terminal Input Select: Channel 11"]
    CBIPSEL_11 = 11,
    #[doc = "12: Comp. B V+ terminal Input Select: Channel 12"]
    CBIPSEL_12 = 12,
    #[doc = "13: Comp. B V+ terminal Input Select: Channel 13"]
    CBIPSEL_13 = 13,
    #[doc = "14: Comp. B V+ terminal Input Select: Channel 14"]
    CBIPSEL_14 = 14,
    #[doc = "15: Comp. B V+ terminal Input Select: Channel 15"]
    CBIPSEL_15 = 15,
}
impl From<CBIPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CBIPSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CBIPSEL`"]
pub type CBIPSEL_R = crate::R<u8, CBIPSEL_A>;
impl CBIPSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBIPSEL_A {
        match self.bits {
            0 => CBIPSEL_A::CBIPSEL_0,
            1 => CBIPSEL_A::CBIPSEL_1,
            2 => CBIPSEL_A::CBIPSEL_2,
            3 => CBIPSEL_A::CBIPSEL_3,
            4 => CBIPSEL_A::CBIPSEL_4,
            5 => CBIPSEL_A::CBIPSEL_5,
            6 => CBIPSEL_A::CBIPSEL_6,
            7 => CBIPSEL_A::CBIPSEL_7,
            8 => CBIPSEL_A::CBIPSEL_8,
            9 => CBIPSEL_A::CBIPSEL_9,
            10 => CBIPSEL_A::CBIPSEL_10,
            11 => CBIPSEL_A::CBIPSEL_11,
            12 => CBIPSEL_A::CBIPSEL_12,
            13 => CBIPSEL_A::CBIPSEL_13,
            14 => CBIPSEL_A::CBIPSEL_14,
            15 => CBIPSEL_A::CBIPSEL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_0`"]
    #[inline(always)]
    pub fn is_cbipsel_0(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_0
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_1`"]
    #[inline(always)]
    pub fn is_cbipsel_1(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_1
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_2`"]
    #[inline(always)]
    pub fn is_cbipsel_2(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_2
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_3`"]
    #[inline(always)]
    pub fn is_cbipsel_3(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_3
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_4`"]
    #[inline(always)]
    pub fn is_cbipsel_4(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_4
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_5`"]
    #[inline(always)]
    pub fn is_cbipsel_5(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_5
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_6`"]
    #[inline(always)]
    pub fn is_cbipsel_6(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_6
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_7`"]
    #[inline(always)]
    pub fn is_cbipsel_7(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_7
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_8`"]
    #[inline(always)]
    pub fn is_cbipsel_8(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_8
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_9`"]
    #[inline(always)]
    pub fn is_cbipsel_9(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_9
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_10`"]
    #[inline(always)]
    pub fn is_cbipsel_10(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_10
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_11`"]
    #[inline(always)]
    pub fn is_cbipsel_11(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_11
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_12`"]
    #[inline(always)]
    pub fn is_cbipsel_12(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_12
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_13`"]
    #[inline(always)]
    pub fn is_cbipsel_13(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_13
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_14`"]
    #[inline(always)]
    pub fn is_cbipsel_14(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_14
    }
    #[doc = "Checks if the value of the field is `CBIPSEL_15`"]
    #[inline(always)]
    pub fn is_cbipsel_15(&self) -> bool {
        *self == CBIPSEL_A::CBIPSEL_15
    }
}
#[doc = "Write proxy for field `CBIPSEL`"]
pub struct CBIPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CBIPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBIPSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 0"]
    #[inline(always)]
    pub fn cbipsel_0(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_0)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 1"]
    #[inline(always)]
    pub fn cbipsel_1(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_1)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 2"]
    #[inline(always)]
    pub fn cbipsel_2(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_2)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 3"]
    #[inline(always)]
    pub fn cbipsel_3(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_3)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 4"]
    #[inline(always)]
    pub fn cbipsel_4(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_4)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 5"]
    #[inline(always)]
    pub fn cbipsel_5(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_5)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 6"]
    #[inline(always)]
    pub fn cbipsel_6(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_6)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 7"]
    #[inline(always)]
    pub fn cbipsel_7(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_7)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 8"]
    #[inline(always)]
    pub fn cbipsel_8(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_8)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 9"]
    #[inline(always)]
    pub fn cbipsel_9(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_9)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 10"]
    #[inline(always)]
    pub fn cbipsel_10(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_10)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 11"]
    #[inline(always)]
    pub fn cbipsel_11(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_11)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 12"]
    #[inline(always)]
    pub fn cbipsel_12(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_12)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 13"]
    #[inline(always)]
    pub fn cbipsel_13(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_13)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 14"]
    #[inline(always)]
    pub fn cbipsel_14(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_14)
    }
    #[doc = "Comp. B V+ terminal Input Select: Channel 15"]
    #[inline(always)]
    pub fn cbipsel_15(self) -> &'a mut W {
        self.variant(CBIPSEL_A::CBIPSEL_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CBIPEN`"]
pub type CBIPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBIPEN`"]
pub struct CBIPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CBIPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Comp. B Neg. Channel Input Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CBIMSEL_A {
    #[doc = "0: Comp. B V- Terminal Input Select: Channel 0"]
    CBIMSEL_0 = 0,
    #[doc = "1: Comp. B V- Terminal Input Select: Channel 1"]
    CBIMSEL_1 = 1,
    #[doc = "2: Comp. B V- Terminal Input Select: Channel 2"]
    CBIMSEL_2 = 2,
    #[doc = "3: Comp. B V- Terminal Input Select: Channel 3"]
    CBIMSEL_3 = 3,
    #[doc = "4: Comp. B V- Terminal Input Select: Channel 4"]
    CBIMSEL_4 = 4,
    #[doc = "5: Comp. B V- Terminal Input Select: Channel 5"]
    CBIMSEL_5 = 5,
    #[doc = "6: Comp. B V- Terminal Input Select: Channel 6"]
    CBIMSEL_6 = 6,
    #[doc = "7: Comp. B V- Terminal Input Select: Channel 7"]
    CBIMSEL_7 = 7,
    #[doc = "8: Comp. B V- terminal Input Select: Channel 8"]
    CBIMSEL_8 = 8,
    #[doc = "9: Comp. B V- terminal Input Select: Channel 9"]
    CBIMSEL_9 = 9,
    #[doc = "10: Comp. B V- terminal Input Select: Channel 10"]
    CBIMSEL_10 = 10,
    #[doc = "11: Comp. B V- terminal Input Select: Channel 11"]
    CBIMSEL_11 = 11,
    #[doc = "12: Comp. B V- terminal Input Select: Channel 12"]
    CBIMSEL_12 = 12,
    #[doc = "13: Comp. B V- terminal Input Select: Channel 13"]
    CBIMSEL_13 = 13,
    #[doc = "14: Comp. B V- terminal Input Select: Channel 14"]
    CBIMSEL_14 = 14,
    #[doc = "15: Comp. B V- terminal Input Select: Channel 15"]
    CBIMSEL_15 = 15,
}
impl From<CBIMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CBIMSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CBIMSEL`"]
pub type CBIMSEL_R = crate::R<u8, CBIMSEL_A>;
impl CBIMSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBIMSEL_A {
        match self.bits {
            0 => CBIMSEL_A::CBIMSEL_0,
            1 => CBIMSEL_A::CBIMSEL_1,
            2 => CBIMSEL_A::CBIMSEL_2,
            3 => CBIMSEL_A::CBIMSEL_3,
            4 => CBIMSEL_A::CBIMSEL_4,
            5 => CBIMSEL_A::CBIMSEL_5,
            6 => CBIMSEL_A::CBIMSEL_6,
            7 => CBIMSEL_A::CBIMSEL_7,
            8 => CBIMSEL_A::CBIMSEL_8,
            9 => CBIMSEL_A::CBIMSEL_9,
            10 => CBIMSEL_A::CBIMSEL_10,
            11 => CBIMSEL_A::CBIMSEL_11,
            12 => CBIMSEL_A::CBIMSEL_12,
            13 => CBIMSEL_A::CBIMSEL_13,
            14 => CBIMSEL_A::CBIMSEL_14,
            15 => CBIMSEL_A::CBIMSEL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_0`"]
    #[inline(always)]
    pub fn is_cbimsel_0(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_0
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_1`"]
    #[inline(always)]
    pub fn is_cbimsel_1(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_1
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_2`"]
    #[inline(always)]
    pub fn is_cbimsel_2(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_2
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_3`"]
    #[inline(always)]
    pub fn is_cbimsel_3(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_3
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_4`"]
    #[inline(always)]
    pub fn is_cbimsel_4(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_4
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_5`"]
    #[inline(always)]
    pub fn is_cbimsel_5(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_5
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_6`"]
    #[inline(always)]
    pub fn is_cbimsel_6(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_6
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_7`"]
    #[inline(always)]
    pub fn is_cbimsel_7(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_7
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_8`"]
    #[inline(always)]
    pub fn is_cbimsel_8(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_8
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_9`"]
    #[inline(always)]
    pub fn is_cbimsel_9(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_9
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_10`"]
    #[inline(always)]
    pub fn is_cbimsel_10(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_10
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_11`"]
    #[inline(always)]
    pub fn is_cbimsel_11(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_11
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_12`"]
    #[inline(always)]
    pub fn is_cbimsel_12(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_12
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_13`"]
    #[inline(always)]
    pub fn is_cbimsel_13(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_13
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_14`"]
    #[inline(always)]
    pub fn is_cbimsel_14(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_14
    }
    #[doc = "Checks if the value of the field is `CBIMSEL_15`"]
    #[inline(always)]
    pub fn is_cbimsel_15(&self) -> bool {
        *self == CBIMSEL_A::CBIMSEL_15
    }
}
#[doc = "Write proxy for field `CBIMSEL`"]
pub struct CBIMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CBIMSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBIMSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Comp. B V- Terminal Input Select: Channel 0"]
    #[inline(always)]
    pub fn cbimsel_0(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_0)
    }
    #[doc = "Comp. B V- Terminal Input Select: Channel 1"]
    #[inline(always)]
    pub fn cbimsel_1(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_1)
    }
    #[doc = "Comp. B V- Terminal Input Select: Channel 2"]
    #[inline(always)]
    pub fn cbimsel_2(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_2)
    }
    #[doc = "Comp. B V- Terminal Input Select: Channel 3"]
    #[inline(always)]
    pub fn cbimsel_3(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_3)
    }
    #[doc = "Comp. B V- Terminal Input Select: Channel 4"]
    #[inline(always)]
    pub fn cbimsel_4(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_4)
    }
    #[doc = "Comp. B V- Terminal Input Select: Channel 5"]
    #[inline(always)]
    pub fn cbimsel_5(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_5)
    }
    #[doc = "Comp. B V- Terminal Input Select: Channel 6"]
    #[inline(always)]
    pub fn cbimsel_6(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_6)
    }
    #[doc = "Comp. B V- Terminal Input Select: Channel 7"]
    #[inline(always)]
    pub fn cbimsel_7(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_7)
    }
    #[doc = "Comp. B V- terminal Input Select: Channel 8"]
    #[inline(always)]
    pub fn cbimsel_8(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_8)
    }
    #[doc = "Comp. B V- terminal Input Select: Channel 9"]
    #[inline(always)]
    pub fn cbimsel_9(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_9)
    }
    #[doc = "Comp. B V- terminal Input Select: Channel 10"]
    #[inline(always)]
    pub fn cbimsel_10(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_10)
    }
    #[doc = "Comp. B V- terminal Input Select: Channel 11"]
    #[inline(always)]
    pub fn cbimsel_11(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_11)
    }
    #[doc = "Comp. B V- terminal Input Select: Channel 12"]
    #[inline(always)]
    pub fn cbimsel_12(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_12)
    }
    #[doc = "Comp. B V- terminal Input Select: Channel 13"]
    #[inline(always)]
    pub fn cbimsel_13(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_13)
    }
    #[doc = "Comp. B V- terminal Input Select: Channel 14"]
    #[inline(always)]
    pub fn cbimsel_14(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_14)
    }
    #[doc = "Comp. B V- terminal Input Select: Channel 15"]
    #[inline(always)]
    pub fn cbimsel_15(self) -> &'a mut W {
        self.variant(CBIMSEL_A::CBIMSEL_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CBIMEN`"]
pub type CBIMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBIMEN`"]
pub struct CBIMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CBIMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Comp. B Pos. Channel Input Select 0"]
    #[inline(always)]
    pub fn cbipsel(&self) -> CBIPSEL_R {
        CBIPSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Comp. B Pos. Channel Input Enable"]
    #[inline(always)]
    pub fn cbipen(&self) -> CBIPEN_R {
        CBIPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Comp. B Neg. Channel Input Select 0"]
    #[inline(always)]
    pub fn cbimsel(&self) -> CBIMSEL_R {
        CBIMSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comp. B Neg. Channel Input Enable"]
    #[inline(always)]
    pub fn cbimen(&self) -> CBIMEN_R {
        CBIMEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Comp. B Pos. Channel Input Select 0"]
    #[inline(always)]
    pub fn cbipsel(&mut self) -> CBIPSEL_W {
        CBIPSEL_W { w: self }
    }
    #[doc = "Bit 7 - Comp. B Pos. Channel Input Enable"]
    #[inline(always)]
    pub fn cbipen(&mut self) -> CBIPEN_W {
        CBIPEN_W { w: self }
    }
    #[doc = "Bits 8:11 - Comp. B Neg. Channel Input Select 0"]
    #[inline(always)]
    pub fn cbimsel(&mut self) -> CBIMSEL_W {
        CBIMSEL_W { w: self }
    }
    #[doc = "Bit 15 - Comp. B Neg. Channel Input Enable"]
    #[inline(always)]
    pub fn cbimen(&mut self) -> CBIMEN_W {
        CBIMEN_W { w: self }
    }
}
