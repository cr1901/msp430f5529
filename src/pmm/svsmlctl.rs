#[doc = "Register `SVSMLCTL` reader"]
pub type R = crate::R<SVSMLCTL_SPEC>;
#[doc = "Register `SVSMLCTL` writer"]
pub type W = crate::W<SVSMLCTL_SPEC>;
#[doc = "Field `SVSMLRRL` reader - SVS and SVM low side Reset Release Voltage Level Bit: 0"]
pub type SVSMLRRL_R = crate::FieldReader<SVSMLRRL_A>;
#[doc = "SVS and SVM low side Reset Release Voltage Level Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for SVSMLRRL_A {
    type Ux = u8;
}
impl SVSMLRRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SVSMLRRL_A {
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
    #[doc = "SVS and SVM low side Reset Release Voltage Level 0"]
    #[inline(always)]
    pub fn is_svsmlrrl_0(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_0
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 1"]
    #[inline(always)]
    pub fn is_svsmlrrl_1(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_1
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 2"]
    #[inline(always)]
    pub fn is_svsmlrrl_2(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_2
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 3"]
    #[inline(always)]
    pub fn is_svsmlrrl_3(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_3
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 4"]
    #[inline(always)]
    pub fn is_svsmlrrl_4(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_4
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 5"]
    #[inline(always)]
    pub fn is_svsmlrrl_5(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_5
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 6"]
    #[inline(always)]
    pub fn is_svsmlrrl_6(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_6
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 7"]
    #[inline(always)]
    pub fn is_svsmlrrl_7(&self) -> bool {
        *self == SVSMLRRL_A::SVSMLRRL_7
    }
}
#[doc = "Field `SVSMLRRL` writer - SVS and SVM low side Reset Release Voltage Level Bit: 0"]
pub type SVSMLRRL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SVSMLRRL_A>;
impl<'a, REG> SVSMLRRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SVS and SVM low side Reset Release Voltage Level 0"]
    #[inline(always)]
    pub fn svsmlrrl_0(self) -> &'a mut crate::W<REG> {
        self.variant(SVSMLRRL_A::SVSMLRRL_0)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 1"]
    #[inline(always)]
    pub fn svsmlrrl_1(self) -> &'a mut crate::W<REG> {
        self.variant(SVSMLRRL_A::SVSMLRRL_1)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 2"]
    #[inline(always)]
    pub fn svsmlrrl_2(self) -> &'a mut crate::W<REG> {
        self.variant(SVSMLRRL_A::SVSMLRRL_2)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 3"]
    #[inline(always)]
    pub fn svsmlrrl_3(self) -> &'a mut crate::W<REG> {
        self.variant(SVSMLRRL_A::SVSMLRRL_3)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 4"]
    #[inline(always)]
    pub fn svsmlrrl_4(self) -> &'a mut crate::W<REG> {
        self.variant(SVSMLRRL_A::SVSMLRRL_4)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 5"]
    #[inline(always)]
    pub fn svsmlrrl_5(self) -> &'a mut crate::W<REG> {
        self.variant(SVSMLRRL_A::SVSMLRRL_5)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 6"]
    #[inline(always)]
    pub fn svsmlrrl_6(self) -> &'a mut crate::W<REG> {
        self.variant(SVSMLRRL_A::SVSMLRRL_6)
    }
    #[doc = "SVS and SVM low side Reset Release Voltage Level 7"]
    #[inline(always)]
    pub fn svsmlrrl_7(self) -> &'a mut crate::W<REG> {
        self.variant(SVSMLRRL_A::SVSMLRRL_7)
    }
}
#[doc = "Field `SVSMLDLYST` reader - SVS and SVM low side delay status"]
pub type SVSMLDLYST_R = crate::BitReader;
#[doc = "Field `SVSMLDLYST` writer - SVS and SVM low side delay status"]
pub type SVSMLDLYST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVSLMD` reader - SVS low side mode"]
pub type SVSLMD_R = crate::BitReader;
#[doc = "Field `SVSLMD` writer - SVS low side mode"]
pub type SVSLMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVSMLEVM` reader - SVS and SVM low side event mask"]
pub type SVSMLEVM_R = crate::BitReader;
#[doc = "Field `SVSMLEVM` writer - SVS and SVM low side event mask"]
pub type SVSMLEVM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVSMLACE` reader - SVS and SVM low side auto control enable"]
pub type SVSMLACE_R = crate::BitReader;
#[doc = "Field `SVSMLACE` writer - SVS and SVM low side auto control enable"]
pub type SVSMLACE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVSLRVL` reader - SVS low side reset voltage level Bit: 0"]
pub type SVSLRVL_R = crate::FieldReader<SVSLRVL_A>;
#[doc = "SVS low side reset voltage level Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for SVSLRVL_A {
    type Ux = u8;
}
impl SVSLRVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SVSLRVL_A {
        match self.bits {
            0 => SVSLRVL_A::SVSLRVL_0,
            1 => SVSLRVL_A::SVSLRVL_1,
            2 => SVSLRVL_A::SVSLRVL_2,
            3 => SVSLRVL_A::SVSLRVL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "SVS low side Reset Release Voltage Level 0"]
    #[inline(always)]
    pub fn is_svslrvl_0(&self) -> bool {
        *self == SVSLRVL_A::SVSLRVL_0
    }
    #[doc = "SVS low side Reset Release Voltage Level 1"]
    #[inline(always)]
    pub fn is_svslrvl_1(&self) -> bool {
        *self == SVSLRVL_A::SVSLRVL_1
    }
    #[doc = "SVS low side Reset Release Voltage Level 2"]
    #[inline(always)]
    pub fn is_svslrvl_2(&self) -> bool {
        *self == SVSLRVL_A::SVSLRVL_2
    }
    #[doc = "SVS low side Reset Release Voltage Level 3"]
    #[inline(always)]
    pub fn is_svslrvl_3(&self) -> bool {
        *self == SVSLRVL_A::SVSLRVL_3
    }
}
#[doc = "Field `SVSLRVL` writer - SVS low side reset voltage level Bit: 0"]
pub type SVSLRVL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SVSLRVL_A>;
impl<'a, REG> SVSLRVL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SVS low side Reset Release Voltage Level 0"]
    #[inline(always)]
    pub fn svslrvl_0(self) -> &'a mut crate::W<REG> {
        self.variant(SVSLRVL_A::SVSLRVL_0)
    }
    #[doc = "SVS low side Reset Release Voltage Level 1"]
    #[inline(always)]
    pub fn svslrvl_1(self) -> &'a mut crate::W<REG> {
        self.variant(SVSLRVL_A::SVSLRVL_1)
    }
    #[doc = "SVS low side Reset Release Voltage Level 2"]
    #[inline(always)]
    pub fn svslrvl_2(self) -> &'a mut crate::W<REG> {
        self.variant(SVSLRVL_A::SVSLRVL_2)
    }
    #[doc = "SVS low side Reset Release Voltage Level 3"]
    #[inline(always)]
    pub fn svslrvl_3(self) -> &'a mut crate::W<REG> {
        self.variant(SVSLRVL_A::SVSLRVL_3)
    }
}
#[doc = "Field `SVSLE` reader - SVS low side enable"]
pub type SVSLE_R = crate::BitReader;
#[doc = "Field `SVSLE` writer - SVS low side enable"]
pub type SVSLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVSLFP` reader - SVS low side full performace mode"]
pub type SVSLFP_R = crate::BitReader;
#[doc = "Field `SVSLFP` writer - SVS low side full performace mode"]
pub type SVSLFP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVMLOVPE` reader - SVM low side over-voltage enable"]
pub type SVMLOVPE_R = crate::BitReader;
#[doc = "Field `SVMLOVPE` writer - SVM low side over-voltage enable"]
pub type SVMLOVPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVMLE` reader - SVM low side enable"]
pub type SVMLE_R = crate::BitReader;
#[doc = "Field `SVMLE` writer - SVM low side enable"]
pub type SVMLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVMLFP` reader - SVM low side full performace mode"]
pub type SVMLFP_R = crate::BitReader;
#[doc = "Field `SVMLFP` writer - SVM low side full performace mode"]
pub type SVMLFP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - SVS and SVM low side Reset Release Voltage Level Bit: 0"]
    #[inline(always)]
    pub fn svsmlrrl(&self) -> SVSMLRRL_R {
        SVSMLRRL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - SVS and SVM low side delay status"]
    #[inline(always)]
    pub fn svsmldlyst(&self) -> SVSMLDLYST_R {
        SVSMLDLYST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SVS low side mode"]
    #[inline(always)]
    pub fn svslmd(&self) -> SVSLMD_R {
        SVSLMD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SVS and SVM low side event mask"]
    #[inline(always)]
    pub fn svsmlevm(&self) -> SVSMLEVM_R {
        SVSMLEVM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SVS and SVM low side auto control enable"]
    #[inline(always)]
    pub fn svsmlace(&self) -> SVSMLACE_R {
        SVSMLACE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - SVS low side reset voltage level Bit: 0"]
    #[inline(always)]
    pub fn svslrvl(&self) -> SVSLRVL_R {
        SVSLRVL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - SVS low side enable"]
    #[inline(always)]
    pub fn svsle(&self) -> SVSLE_R {
        SVSLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SVS low side full performace mode"]
    #[inline(always)]
    pub fn svslfp(&self) -> SVSLFP_R {
        SVSLFP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SVM low side over-voltage enable"]
    #[inline(always)]
    pub fn svmlovpe(&self) -> SVMLOVPE_R {
        SVMLOVPE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - SVM low side enable"]
    #[inline(always)]
    pub fn svmle(&self) -> SVMLE_R {
        SVMLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SVM low side full performace mode"]
    #[inline(always)]
    pub fn svmlfp(&self) -> SVMLFP_R {
        SVMLFP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SVS and SVM low side Reset Release Voltage Level Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn svsmlrrl(&mut self) -> SVSMLRRL_W<SVSMLCTL_SPEC> {
        SVSMLRRL_W::new(self, 0)
    }
    #[doc = "Bit 3 - SVS and SVM low side delay status"]
    #[inline(always)]
    #[must_use]
    pub fn svsmldlyst(&mut self) -> SVSMLDLYST_W<SVSMLCTL_SPEC> {
        SVSMLDLYST_W::new(self, 3)
    }
    #[doc = "Bit 4 - SVS low side mode"]
    #[inline(always)]
    #[must_use]
    pub fn svslmd(&mut self) -> SVSLMD_W<SVSMLCTL_SPEC> {
        SVSLMD_W::new(self, 4)
    }
    #[doc = "Bit 6 - SVS and SVM low side event mask"]
    #[inline(always)]
    #[must_use]
    pub fn svsmlevm(&mut self) -> SVSMLEVM_W<SVSMLCTL_SPEC> {
        SVSMLEVM_W::new(self, 6)
    }
    #[doc = "Bit 7 - SVS and SVM low side auto control enable"]
    #[inline(always)]
    #[must_use]
    pub fn svsmlace(&mut self) -> SVSMLACE_W<SVSMLCTL_SPEC> {
        SVSMLACE_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - SVS low side reset voltage level Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn svslrvl(&mut self) -> SVSLRVL_W<SVSMLCTL_SPEC> {
        SVSLRVL_W::new(self, 8)
    }
    #[doc = "Bit 10 - SVS low side enable"]
    #[inline(always)]
    #[must_use]
    pub fn svsle(&mut self) -> SVSLE_W<SVSMLCTL_SPEC> {
        SVSLE_W::new(self, 10)
    }
    #[doc = "Bit 11 - SVS low side full performace mode"]
    #[inline(always)]
    #[must_use]
    pub fn svslfp(&mut self) -> SVSLFP_W<SVSMLCTL_SPEC> {
        SVSLFP_W::new(self, 11)
    }
    #[doc = "Bit 12 - SVM low side over-voltage enable"]
    #[inline(always)]
    #[must_use]
    pub fn svmlovpe(&mut self) -> SVMLOVPE_W<SVSMLCTL_SPEC> {
        SVMLOVPE_W::new(self, 12)
    }
    #[doc = "Bit 14 - SVM low side enable"]
    #[inline(always)]
    #[must_use]
    pub fn svmle(&mut self) -> SVMLE_W<SVSMLCTL_SPEC> {
        SVMLE_W::new(self, 14)
    }
    #[doc = "Bit 15 - SVM low side full performace mode"]
    #[inline(always)]
    #[must_use]
    pub fn svmlfp(&mut self) -> SVMLFP_W<SVSMLCTL_SPEC> {
        SVMLFP_W::new(self, 15)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SVS and SVM low side control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`svsmlctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`svsmlctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SVSMLCTL_SPEC;
impl crate::RegisterSpec for SVSMLCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`svsmlctl::R`](R) reader structure"]
impl crate::Readable for SVSMLCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`svsmlctl::W`](W) writer structure"]
impl crate::Writable for SVSMLCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SVSMLCTL to value 0"]
impl crate::Resettable for SVSMLCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
