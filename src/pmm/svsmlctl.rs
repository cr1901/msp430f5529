#[doc = "Reader of register SVSMLCTL"]
pub type R = crate::R<u16, super::SVSMLCTL>;
#[doc = "Writer for register SVSMLCTL"]
pub type W = crate::W<u16, super::SVSMLCTL>;
#[doc = "Register SVSMLCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::SVSMLCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SVS and SVM low side Reset Release Voltage Level Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SVSMLRRL_A {
    #[doc = "0: SVS and SVM low side Reset Release Voltage Level 0"]
    SVSMLRRL_0 = 0,
    #[doc = "1: SVS and SVM low side Reset Release Voltage Level 1"]
    SVSMLRRL_1 = 1,
    #[doc = "2: SVS and SVM low side Reset Release Voltage Level 2"]
    SVSMLRRL_2 = 2,
    #[doc = "3: SVS and SVM low side Reset Release Voltage Level 3"]
    SVSMLRRL_3 = 3,
    #[doc = "4: SVS and SVM low side Reset Release Voltage Level 4"]
    SVSMLRRL_4 = 4,
    #[doc = "5: SVS and SVM low side Reset Release Voltage Level 5"]
    SVSMLRRL_5 = 5,
    #[doc = "6: SVS and SVM low side Reset Release Voltage Level 6"]
    SVSMLRRL_6 = 6,
    #[doc = "7: SVS and SVM low side Reset Release Voltage Level 7"]
    SVSMLRRL_7 = 7,
}
impl From<SVSMLRRL_A> for u8 {
    #[inline(always)]
    fn from(variant: SVSMLRRL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SVSMLRRL`"]
pub type SVSMLRRL_R = crate::R<u8, SVSMLRRL_A>;
impl SVSMLRRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSMLRRL_A {
        match self.bits {
            0 => SVSMLRRL_A::SVSMLRRL_0,
            1 => SVSMLRRL_A::SVSMLRRL_1,
            2 => SVSMLRRL_A::SVSMLRRL_2,
            3 => SVSMLRRL_A::SVSMLRRL_3,
            4 => SVSMLRRL_A::SVSMLRRL_4,
            5 => SVSMLRRL_A::SVSMLRRL_5,
            6 => SVSMLRRL_A::SVSMLRRL_6,
            7 => SVSMLRRL_A::SVSMLRRL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SVSMLRRL_0`"]
    #[inline(always)]
    pub fn is_svsmlrrl_0(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_0
    }
    #[doc = "Checks if the value of the field is `SVSMLRRL_1`"]
    #[inline(always)]
    pub fn is_svsmlrrl_1(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_1
    }
    #[doc = "Checks if the value of the field is `SVSMLRRL_2`"]
    #[inline(always)]
    pub fn is_svsmlrrl_2(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_2
    }
    #[doc = "Checks if the value of the field is `SVSMLRRL_3`"]
    #[inline(always)]
    pub fn is_svsmlrrl_3(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_3
    }
    #[doc = "Checks if the value of the field is `SVSMLRRL_4`"]
    #[inline(always)]
    pub fn is_svsmlrrl_4(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_4
    }
    #[doc = "Checks if the value of the field is `SVSMLRRL_5`"]
    #[inline(always)]
    pub fn is_svsmlrrl_5(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_5
    }
    #[doc = "Checks if the value of the field is `SVSMLRRL_6`"]
    #[inline(always)]
    pub fn is_svsmlrrl_6(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_6
    }
    #[doc = "Checks if the value of the field is `SVSMLRRL_7`"]
    #[inline(always)]
    pub fn is_svsmlrrl_7(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_7
    }
}
#[doc = "Write proxy for field `SVSMLRRL`"]
pub struct SVSMLRRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSMLRRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVSMLRRL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 0"]
    #[inline(always)]
    pub fn svsmlrrl_0(self) -> &'a mut W {
        self.variant(SVSMLRRL_A::SVSMLRRL_0)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 1"]
    #[inline(always)]
    pub fn svsmlrrl_1(self) -> &'a mut W {
        self.variant(SVSMLRRL_A::SVSMLRRL_1)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 2"]
    #[inline(always)]
    pub fn svsmlrrl_2(self) -> &'a mut W {
        self.variant(SVSMLRRL_A::SVSMLRRL_2)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 3"]
    #[inline(always)]
    pub fn svsmlrrl_3(self) -> &'a mut W {
        self.variant(SVSMLRRL_A::SVSMLRRL_3)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 4"]
    #[inline(always)]
    pub fn svsmlrrl_4(self) -> &'a mut W {
        self.variant(SVSMLRRL_A::SVSMLRRL_4)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 5"]
    #[inline(always)]
    pub fn svsmlrrl_5(self) -> &'a mut W {
        self.variant(SVSMLRRL_A::SVSMLRRL_5)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 6"]
    #[inline(always)]
    pub fn svsmlrrl_6(self) -> &'a mut W {
        self.variant(SVSMLRRL_A::SVSMLRRL_6)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 7"]
    #[inline(always)]
    pub fn svsmlrrl_7(self) -> &'a mut W {
        self.variant(SVSMLRRL_A::SVSMLRRL_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `SVSMLDLYST`"]
pub type SVSMLDLYST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSMLDLYST`"]
pub struct SVSMLDLYST_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSMLDLYST_W<'a> {
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
#[doc = "Reader of field `SVSLMD`"]
pub type SVSLMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSLMD`"]
pub struct SVSLMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSLMD_W<'a> {
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
#[doc = "Reader of field `SVSMLEVM`"]
pub type SVSMLEVM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSMLEVM`"]
pub struct SVSMLEVM_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSMLEVM_W<'a> {
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
#[doc = "Reader of field `SVSMLACE`"]
pub type SVSMLACE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSMLACE`"]
pub struct SVSMLACE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSMLACE_W<'a> {
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
#[doc = "SVS low side reset voltage level Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SVSLRVL_A {
    #[doc = "0: SVS low side Reset Release Voltage Level 0"]
    SVSLRVL_0 = 0,
    #[doc = "1: SVS low side Reset Release Voltage Level 1"]
    SVSLRVL_1 = 1,
    #[doc = "2: SVS low side Reset Release Voltage Level 2"]
    SVSLRVL_2 = 2,
    #[doc = "3: SVS low side Reset Release Voltage Level 3"]
    SVSLRVL_3 = 3,
}
impl From<SVSLRVL_A> for u8 {
    #[inline(always)]
    fn from(variant: SVSLRVL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SVSLRVL`"]
pub type SVSLRVL_R = crate::R<u8, SVSLRVL_A>;
impl SVSLRVL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSLRVL_A {
        match self.bits {
            0 => SVSLRVL_A::SVSLRVL_0,
            1 => SVSLRVL_A::SVSLRVL_1,
            2 => SVSLRVL_A::SVSLRVL_2,
            3 => SVSLRVL_A::SVSLRVL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SVSLRVL_0`"]
    #[inline(always)]
    pub fn is_svslrvl_0(&self) -> bool {
        *self == SVSLRVL_A::SVSLRVL_0
    }
    #[doc = "Checks if the value of the field is `SVSLRVL_1`"]
    #[inline(always)]
    pub fn is_svslrvl_1(&self) -> bool {
        *self == SVSLRVL_A::SVSLRVL_1
    }
    #[doc = "Checks if the value of the field is `SVSLRVL_2`"]
    #[inline(always)]
    pub fn is_svslrvl_2(&self) -> bool {
        *self == SVSLRVL_A::SVSLRVL_2
    }
    #[doc = "Checks if the value of the field is `SVSLRVL_3`"]
    #[inline(always)]
    pub fn is_svslrvl_3(&self) -> bool {
        *self == SVSLRVL_A::SVSLRVL_3
    }
}
#[doc = "Write proxy for field `SVSLRVL`"]
pub struct SVSLRVL_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSLRVL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVSLRVL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SVS low side Reset Release Voltage Level 0"]
    #[inline(always)]
    pub fn svslrvl_0(self) -> &'a mut W {
        self.variant(SVSLRVL_A::SVSLRVL_0)
    }
    #[doc = "SVS low side Reset Release Voltage Level 1"]
    #[inline(always)]
    pub fn svslrvl_1(self) -> &'a mut W {
        self.variant(SVSLRVL_A::SVSLRVL_1)
    }
    #[doc = "SVS low side Reset Release Voltage Level 2"]
    #[inline(always)]
    pub fn svslrvl_2(self) -> &'a mut W {
        self.variant(SVSLRVL_A::SVSLRVL_2)
    }
    #[doc = "SVS low side Reset Release Voltage Level 3"]
    #[inline(always)]
    pub fn svslrvl_3(self) -> &'a mut W {
        self.variant(SVSLRVL_A::SVSLRVL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `SVSLE`"]
pub type SVSLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSLE`"]
pub struct SVSLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSLE_W<'a> {
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
#[doc = "Reader of field `SVSLFP`"]
pub type SVSLFP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSLFP`"]
pub struct SVSLFP_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSLFP_W<'a> {
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
#[doc = "Reader of field `SVMLOVPE`"]
pub type SVMLOVPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMLOVPE`"]
pub struct SVMLOVPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMLOVPE_W<'a> {
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
#[doc = "Reader of field `SVMLE`"]
pub type SVMLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMLE`"]
pub struct SVMLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMLE_W<'a> {
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
#[doc = "Reader of field `SVMLFP`"]
pub type SVMLFP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVMLFP`"]
pub struct SVMLFP_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMLFP_W<'a> {
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
    #[doc = "Bits 0:2 - SVS and SVM low side Reset Release Voltage Level Bit: 0"]
    #[inline(always)]
    pub fn svsmlrrl(&self) -> SVSMLRRL_R {
        SVSMLRRL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - SVS and SVM low side delay status"]
    #[inline(always)]
    pub fn svsmldlyst(&self) -> SVSMLDLYST_R {
        SVSMLDLYST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SVS low side mode"]
    #[inline(always)]
    pub fn svslmd(&self) -> SVSLMD_R {
        SVSLMD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SVS and SVM low side event mask"]
    #[inline(always)]
    pub fn svsmlevm(&self) -> SVSMLEVM_R {
        SVSMLEVM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SVS and SVM low side auto control enable"]
    #[inline(always)]
    pub fn svsmlace(&self) -> SVSMLACE_R {
        SVSMLACE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - SVS low side reset voltage level Bit: 0"]
    #[inline(always)]
    pub fn svslrvl(&self) -> SVSLRVL_R {
        SVSLRVL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - SVS low side enable"]
    #[inline(always)]
    pub fn svsle(&self) -> SVSLE_R {
        SVSLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SVS low side full performace mode"]
    #[inline(always)]
    pub fn svslfp(&self) -> SVSLFP_R {
        SVSLFP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SVM low side over-voltage enable"]
    #[inline(always)]
    pub fn svmlovpe(&self) -> SVMLOVPE_R {
        SVMLOVPE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SVM low side enable"]
    #[inline(always)]
    pub fn svmle(&self) -> SVMLE_R {
        SVMLE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SVM low side full performace mode"]
    #[inline(always)]
    pub fn svmlfp(&self) -> SVMLFP_R {
        SVMLFP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SVS and SVM low side Reset Release Voltage Level Bit: 0"]
    #[inline(always)]
    pub fn svsmlrrl(&mut self) -> SVSMLRRL_W {
        SVSMLRRL_W { w: self }
    }
    #[doc = "Bit 3 - SVS and SVM low side delay status"]
    #[inline(always)]
    pub fn svsmldlyst(&mut self) -> SVSMLDLYST_W {
        SVSMLDLYST_W { w: self }
    }
    #[doc = "Bit 4 - SVS low side mode"]
    #[inline(always)]
    pub fn svslmd(&mut self) -> SVSLMD_W {
        SVSLMD_W { w: self }
    }
    #[doc = "Bit 6 - SVS and SVM low side event mask"]
    #[inline(always)]
    pub fn svsmlevm(&mut self) -> SVSMLEVM_W {
        SVSMLEVM_W { w: self }
    }
    #[doc = "Bit 7 - SVS and SVM low side auto control enable"]
    #[inline(always)]
    pub fn svsmlace(&mut self) -> SVSMLACE_W {
        SVSMLACE_W { w: self }
    }
    #[doc = "Bits 8:9 - SVS low side reset voltage level Bit: 0"]
    #[inline(always)]
    pub fn svslrvl(&mut self) -> SVSLRVL_W {
        SVSLRVL_W { w: self }
    }
    #[doc = "Bit 10 - SVS low side enable"]
    #[inline(always)]
    pub fn svsle(&mut self) -> SVSLE_W {
        SVSLE_W { w: self }
    }
    #[doc = "Bit 11 - SVS low side full performace mode"]
    #[inline(always)]
    pub fn svslfp(&mut self) -> SVSLFP_W {
        SVSLFP_W { w: self }
    }
    #[doc = "Bit 12 - SVM low side over-voltage enable"]
    #[inline(always)]
    pub fn svmlovpe(&mut self) -> SVMLOVPE_W {
        SVMLOVPE_W { w: self }
    }
    #[doc = "Bit 14 - SVM low side enable"]
    #[inline(always)]
    pub fn svmle(&mut self) -> SVMLE_W {
        SVMLE_W { w: self }
    }
    #[doc = "Bit 15 - SVM low side full performace mode"]
    #[inline(always)]
    pub fn svmlfp(&mut self) -> SVMLFP_W {
        SVMLFP_W { w: self }
    }
}
