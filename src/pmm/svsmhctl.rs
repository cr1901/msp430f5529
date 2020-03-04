#[doc = "Reader of register SVSMHCTL"]
pub type R = crate::R<u16, super::SVSMHCTL>;
#[doc = "Writer for register SVSMHCTL"]
pub type W = crate::W<u16, super::SVSMHCTL>;
#[doc = "Register SVSMHCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::SVSMHCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SVS and SVM high side Reset Release Voltage Level Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SVSMHRRL_A {
    #[doc = "0: SVS and SVM high side Reset Release Voltage Level 0"]
    SVSMHRRL_0 = 0,
    #[doc = "1: SVS and SVM high side Reset Release Voltage Level 1"]
    SVSMHRRL_1 = 1,
    #[doc = "2: SVS and SVM high side Reset Release Voltage Level 2"]
    SVSMHRRL_2 = 2,
    #[doc = "3: SVS and SVM high side Reset Release Voltage Level 3"]
    SVSMHRRL_3 = 3,
    #[doc = "4: SVS and SVM high side Reset Release Voltage Level 4"]
    SVSMHRRL_4 = 4,
    #[doc = "5: SVS and SVM high side Reset Release Voltage Level 5"]
    SVSMHRRL_5 = 5,
    #[doc = "6: SVS and SVM high side Reset Release Voltage Level 6"]
    SVSMHRRL_6 = 6,
    #[doc = "7: SVS and SVM high side Reset Release Voltage Level 7"]
    SVSMHRRL_7 = 7,
}
impl From<SVSMHRRL_A> for u8 {
    #[inline(always)]
    fn from(variant: SVSMHRRL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SVSMHRRL`"]
pub type SVSMHRRL_R = crate::R<u8, SVSMHRRL_A>;
impl SVSMHRRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSMHRRL_A {
        match self.bits {
            0 => SVSMHRRL_A::SVSMHRRL_0,
            1 => SVSMHRRL_A::SVSMHRRL_1,
            2 => SVSMHRRL_A::SVSMHRRL_2,
            3 => SVSMHRRL_A::SVSMHRRL_3,
            4 => SVSMHRRL_A::SVSMHRRL_4,
            5 => SVSMHRRL_A::SVSMHRRL_5,
            6 => SVSMHRRL_A::SVSMHRRL_6,
            7 => SVSMHRRL_A::SVSMHRRL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SVSMHRRL_0`"]
    #[inline(always)]
    pub fn is_svsmhrrl_0(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_0
    }
    #[doc = "Checks if the value of the field is `SVSMHRRL_1`"]
    #[inline(always)]
    pub fn is_svsmhrrl_1(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_1
    }
    #[doc = "Checks if the value of the field is `SVSMHRRL_2`"]
    #[inline(always)]
    pub fn is_svsmhrrl_2(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_2
    }
    #[doc = "Checks if the value of the field is `SVSMHRRL_3`"]
    #[inline(always)]
    pub fn is_svsmhrrl_3(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_3
    }
    #[doc = "Checks if the value of the field is `SVSMHRRL_4`"]
    #[inline(always)]
    pub fn is_svsmhrrl_4(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_4
    }
    #[doc = "Checks if the value of the field is `SVSMHRRL_5`"]
    #[inline(always)]
    pub fn is_svsmhrrl_5(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_5
    }
    #[doc = "Checks if the value of the field is `SVSMHRRL_6`"]
    #[inline(always)]
    pub fn is_svsmhrrl_6(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_6
    }
    #[doc = "Checks if the value of the field is `SVSMHRRL_7`"]
    #[inline(always)]
    pub fn is_svsmhrrl_7(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_7
    }
}
#[doc = "Write proxy for field `SVSMHRRL`"]
pub struct SVSMHRRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSMHRRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVSMHRRL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 0"]
    #[inline(always)]
    pub fn svsmhrrl_0(self) -> &'a mut W {
        self.variant(SVSMHRRL_A::SVSMHRRL_0)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 1"]
    #[inline(always)]
    pub fn svsmhrrl_1(self) -> &'a mut W {
        self.variant(SVSMHRRL_A::SVSMHRRL_1)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 2"]
    #[inline(always)]
    pub fn svsmhrrl_2(self) -> &'a mut W {
        self.variant(SVSMHRRL_A::SVSMHRRL_2)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 3"]
    #[inline(always)]
    pub fn svsmhrrl_3(self) -> &'a mut W {
        self.variant(SVSMHRRL_A::SVSMHRRL_3)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 4"]
    #[inline(always)]
    pub fn svsmhrrl_4(self) -> &'a mut W {
        self.variant(SVSMHRRL_A::SVSMHRRL_4)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 5"]
    #[inline(always)]
    pub fn svsmhrrl_5(self) -> &'a mut W {
        self.variant(SVSMHRRL_A::SVSMHRRL_5)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 6"]
    #[inline(always)]
    pub fn svsmhrrl_6(self) -> &'a mut W {
        self.variant(SVSMHRRL_A::SVSMHRRL_6)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 7"]
    #[inline(always)]
    pub fn svsmhrrl_7(self) -> &'a mut W {
        self.variant(SVSMHRRL_A::SVSMHRRL_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `SVSMHDLYST`"]
pub type SVSMHDLYST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSMHDLYST`"]
pub struct SVSMHDLYST_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSMHDLYST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SVSHMD`"]
pub type SVSHMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSHMD`"]
pub struct SVSHMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSHMD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SVSMHEVM`"]
pub type SVSMHEVM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSMHEVM`"]
pub struct SVSMHEVM_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSMHEVM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SVSMHACE`"]
pub type SVSMHACE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSMHACE`"]
pub struct SVSMHACE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSMHACE_W<'a> {
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
#[doc = "SVS high side reset voltage level Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SVSHRVL_A {
    #[doc = "0: SVS high side Reset Release Voltage Level 0"]
    SVSHRVL_0 = 0,
    #[doc = "1: SVS high side Reset Release Voltage Level 1"]
    SVSHRVL_1 = 1,
    #[doc = "2: SVS high side Reset Release Voltage Level 2"]
    SVSHRVL_2 = 2,
    #[doc = "3: SVS high side Reset Release Voltage Level 3"]
    SVSHRVL_3 = 3,
}
impl From<SVSHRVL_A> for u8 {
    #[inline(always)]
    fn from(variant: SVSHRVL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SVSHRVL`"]
pub type SVSHRVL_R = crate::R<u8, SVSHRVL_A>;
impl SVSHRVL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSHRVL_A {
        match self.bits {
            0 => SVSHRVL_A::SVSHRVL_0,
            1 => SVSHRVL_A::SVSHRVL_1,
            2 => SVSHRVL_A::SVSHRVL_2,
            3 => SVSHRVL_A::SVSHRVL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SVSHRVL_0`"]
    #[inline(always)]
    pub fn is_svshrvl_0(&self) -> bool {
        *self == SVSHRVL_A::SVSHRVL_0
    }
    #[doc = "Checks if the value of the field is `SVSHRVL_1`"]
    #[inline(always)]
    pub fn is_svshrvl_1(&self) -> bool {
        *self == SVSHRVL_A::SVSHRVL_1
    }
    #[doc = "Checks if the value of the field is `SVSHRVL_2`"]
    #[inline(always)]
    pub fn is_svshrvl_2(&self) -> bool {
        *self == SVSHRVL_A::SVSHRVL_2
    }
    #[doc = "Checks if the value of the field is `SVSHRVL_3`"]
    #[inline(always)]
    pub fn is_svshrvl_3(&self) -> bool {
        *self == SVSHRVL_A::SVSHRVL_3
    }
}
#[doc = "Write proxy for field `SVSHRVL`"]
pub struct SVSHRVL_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSHRVL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVSHRVL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SVS high side Reset Release Voltage Level 0"]
    #[inline(always)]
    pub fn svshrvl_0(self) -> &'a mut W {
        self.variant(SVSHRVL_A::SVSHRVL_0)
    }
    #[doc = "SVS high side Reset Release Voltage Level 1"]
    #[inline(always)]
    pub fn svshrvl_1(self) -> &'a mut W {
        self.variant(SVSHRVL_A::SVSHRVL_1)
    }
    #[doc = "SVS high side Reset Release Voltage Level 2"]
    #[inline(always)]
    pub fn svshrvl_2(self) -> &'a mut W {
        self.variant(SVSHRVL_A::SVSHRVL_2)
    }
    #[doc = "SVS high side Reset Release Voltage Level 3"]
    #[inline(always)]
    pub fn svshrvl_3(self) -> &'a mut W {
        self.variant(SVSHRVL_A::SVSHRVL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `SVSHE`"]
pub type SVSHE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSHE`"]
pub struct SVSHE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSHE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SVSHFP`"]
pub type SVSHFP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSHFP`"]
pub struct SVSHFP_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSHFP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SVMHOVPE`"]
pub type SVMHOVPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMHOVPE`"]
pub struct SVMHOVPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMHOVPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SVMHE`"]
pub type SVMHE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMHE`"]
pub struct SVMHE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMHE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SVMHFP`"]
pub type SVMHFP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMHFP`"]
pub struct SVMHFP_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMHFP_W<'a> {
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
    #[doc = "Bits 0:2 - SVS and SVM high side Reset Release Voltage Level Bit: 0"]
    #[inline(always)]
    pub fn svsmhrrl(&self) -> SVSMHRRL_R {
        SVSMHRRL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - SVS and SVM high side delay status"]
    #[inline(always)]
    pub fn svsmhdlyst(&self) -> SVSMHDLYST_R {
        SVSMHDLYST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SVS high side mode"]
    #[inline(always)]
    pub fn svshmd(&self) -> SVSHMD_R {
        SVSHMD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SVS and SVM high side event mask"]
    #[inline(always)]
    pub fn svsmhevm(&self) -> SVSMHEVM_R {
        SVSMHEVM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SVS and SVM high side auto control enable"]
    #[inline(always)]
    pub fn svsmhace(&self) -> SVSMHACE_R {
        SVSMHACE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - SVS high side reset voltage level Bit: 0"]
    #[inline(always)]
    pub fn svshrvl(&self) -> SVSHRVL_R {
        SVSHRVL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - SVS high side enable"]
    #[inline(always)]
    pub fn svshe(&self) -> SVSHE_R {
        SVSHE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SVS high side full performace mode"]
    #[inline(always)]
    pub fn svshfp(&self) -> SVSHFP_R {
        SVSHFP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SVM high side over-voltage enable"]
    #[inline(always)]
    pub fn svmhovpe(&self) -> SVMHOVPE_R {
        SVMHOVPE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SVM high side enable"]
    #[inline(always)]
    pub fn svmhe(&self) -> SVMHE_R {
        SVMHE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SVM high side full performace mode"]
    #[inline(always)]
    pub fn svmhfp(&self) -> SVMHFP_R {
        SVMHFP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SVS and SVM high side Reset Release Voltage Level Bit: 0"]
    #[inline(always)]
    pub fn svsmhrrl(&mut self) -> SVSMHRRL_W {
        SVSMHRRL_W { w: self }
    }
    #[doc = "Bit 3 - SVS and SVM high side delay status"]
    #[inline(always)]
    pub fn svsmhdlyst(&mut self) -> SVSMHDLYST_W {
        SVSMHDLYST_W { w: self }
    }
    #[doc = "Bit 4 - SVS high side mode"]
    #[inline(always)]
    pub fn svshmd(&mut self) -> SVSHMD_W {
        SVSHMD_W { w: self }
    }
    #[doc = "Bit 6 - SVS and SVM high side event mask"]
    #[inline(always)]
    pub fn svsmhevm(&mut self) -> SVSMHEVM_W {
        SVSMHEVM_W { w: self }
    }
    #[doc = "Bit 7 - SVS and SVM high side auto control enable"]
    #[inline(always)]
    pub fn svsmhace(&mut self) -> SVSMHACE_W {
        SVSMHACE_W { w: self }
    }
    #[doc = "Bits 8:9 - SVS high side reset voltage level Bit: 0"]
    #[inline(always)]
    pub fn svshrvl(&mut self) -> SVSHRVL_W {
        SVSHRVL_W { w: self }
    }
    #[doc = "Bit 10 - SVS high side enable"]
    #[inline(always)]
    pub fn svshe(&mut self) -> SVSHE_W {
        SVSHE_W { w: self }
    }
    #[doc = "Bit 11 - SVS high side full performace mode"]
    #[inline(always)]
    pub fn svshfp(&mut self) -> SVSHFP_W {
        SVSHFP_W { w: self }
    }
    #[doc = "Bit 12 - SVM high side over-voltage enable"]
    #[inline(always)]
    pub fn svmhovpe(&mut self) -> SVMHOVPE_W {
        SVMHOVPE_W { w: self }
    }
    #[doc = "Bit 14 - SVM high side enable"]
    #[inline(always)]
    pub fn svmhe(&mut self) -> SVMHE_W {
        SVMHE_W { w: self }
    }
    #[doc = "Bit 15 - SVM high side full performace mode"]
    #[inline(always)]
    pub fn svmhfp(&mut self) -> SVMHFP_W {
        SVMHFP_W { w: self }
    }
}
