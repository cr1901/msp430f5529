#[doc = "Register `SVSMHCTL` reader"]
pub type R = crate::R<SVSMHCTL_SPEC>;
#[doc = "Register `SVSMHCTL` writer"]
pub type W = crate::W<SVSMHCTL_SPEC>;
#[doc = "Field `SVSMHRRL` reader - SVS and SVM high side Reset Release Voltage Level Bit: 0"]
pub type SVSMHRRL_R = crate::FieldReader<SVSMHRRL_A>;
#[doc = "SVS and SVM high side Reset Release Voltage Level Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for SVSMHRRL_A {
    type Ux = u8;
}
impl SVSMHRRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SVSMHRRL_A {
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
    #[doc = "SVS and SVM high side Reset Release Voltage Level 0"]
    #[inline(always)]
    pub fn is_svsmhrrl_0(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_0
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 1"]
    #[inline(always)]
    pub fn is_svsmhrrl_1(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_1
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 2"]
    #[inline(always)]
    pub fn is_svsmhrrl_2(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_2
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 3"]
    #[inline(always)]
    pub fn is_svsmhrrl_3(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_3
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 4"]
    #[inline(always)]
    pub fn is_svsmhrrl_4(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_4
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 5"]
    #[inline(always)]
    pub fn is_svsmhrrl_5(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_5
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 6"]
    #[inline(always)]
    pub fn is_svsmhrrl_6(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_6
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 7"]
    #[inline(always)]
    pub fn is_svsmhrrl_7(&self) -> bool {
        *self == SVSMHRRL_A::SVSMHRRL_7
    }
}
#[doc = "Field `SVSMHRRL` writer - SVS and SVM high side Reset Release Voltage Level Bit: 0"]
pub type SVSMHRRL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SVSMHRRL_A>;
impl<'a, REG> SVSMHRRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SVS and SVM high side Reset Release Voltage Level 0"]
    #[inline(always)]
    pub fn svsmhrrl_0(self) -> &'a mut crate::W<REG> {
        self.variant(SVSMHRRL_A::SVSMHRRL_0)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 1"]
    #[inline(always)]
    pub fn svsmhrrl_1(self) -> &'a mut crate::W<REG> {
        self.variant(SVSMHRRL_A::SVSMHRRL_1)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 2"]
    #[inline(always)]
    pub fn svsmhrrl_2(self) -> &'a mut crate::W<REG> {
        self.variant(SVSMHRRL_A::SVSMHRRL_2)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 3"]
    #[inline(always)]
    pub fn svsmhrrl_3(self) -> &'a mut crate::W<REG> {
        self.variant(SVSMHRRL_A::SVSMHRRL_3)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 4"]
    #[inline(always)]
    pub fn svsmhrrl_4(self) -> &'a mut crate::W<REG> {
        self.variant(SVSMHRRL_A::SVSMHRRL_4)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 5"]
    #[inline(always)]
    pub fn svsmhrrl_5(self) -> &'a mut crate::W<REG> {
        self.variant(SVSMHRRL_A::SVSMHRRL_5)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 6"]
    #[inline(always)]
    pub fn svsmhrrl_6(self) -> &'a mut crate::W<REG> {
        self.variant(SVSMHRRL_A::SVSMHRRL_6)
    }
    #[doc = "SVS and SVM high side Reset Release Voltage Level 7"]
    #[inline(always)]
    pub fn svsmhrrl_7(self) -> &'a mut crate::W<REG> {
        self.variant(SVSMHRRL_A::SVSMHRRL_7)
    }
}
#[doc = "Field `SVSMHDLYST` reader - SVS and SVM high side delay status"]
pub type SVSMHDLYST_R = crate::BitReader;
#[doc = "Field `SVSMHDLYST` writer - SVS and SVM high side delay status"]
pub type SVSMHDLYST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVSHMD` reader - SVS high side mode"]
pub type SVSHMD_R = crate::BitReader;
#[doc = "Field `SVSHMD` writer - SVS high side mode"]
pub type SVSHMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVSMHEVM` reader - SVS and SVM high side event mask"]
pub type SVSMHEVM_R = crate::BitReader;
#[doc = "Field `SVSMHEVM` writer - SVS and SVM high side event mask"]
pub type SVSMHEVM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVSMHACE` reader - SVS and SVM high side auto control enable"]
pub type SVSMHACE_R = crate::BitReader;
#[doc = "Field `SVSMHACE` writer - SVS and SVM high side auto control enable"]
pub type SVSMHACE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVSHRVL` reader - SVS high side reset voltage level Bit: 0"]
pub type SVSHRVL_R = crate::FieldReader<SVSHRVL_A>;
#[doc = "SVS high side reset voltage level Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for SVSHRVL_A {
    type Ux = u8;
}
impl SVSHRVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SVSHRVL_A {
        match self.bits {
            0 => SVSHRVL_A::SVSHRVL_0,
            1 => SVSHRVL_A::SVSHRVL_1,
            2 => SVSHRVL_A::SVSHRVL_2,
            3 => SVSHRVL_A::SVSHRVL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "SVS high side Reset Release Voltage Level 0"]
    #[inline(always)]
    pub fn is_svshrvl_0(&self) -> bool {
        *self == SVSHRVL_A::SVSHRVL_0
    }
    #[doc = "SVS high side Reset Release Voltage Level 1"]
    #[inline(always)]
    pub fn is_svshrvl_1(&self) -> bool {
        *self == SVSHRVL_A::SVSHRVL_1
    }
    #[doc = "SVS high side Reset Release Voltage Level 2"]
    #[inline(always)]
    pub fn is_svshrvl_2(&self) -> bool {
        *self == SVSHRVL_A::SVSHRVL_2
    }
    #[doc = "SVS high side Reset Release Voltage Level 3"]
    #[inline(always)]
    pub fn is_svshrvl_3(&self) -> bool {
        *self == SVSHRVL_A::SVSHRVL_3
    }
}
#[doc = "Field `SVSHRVL` writer - SVS high side reset voltage level Bit: 0"]
pub type SVSHRVL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SVSHRVL_A>;
impl<'a, REG> SVSHRVL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SVS high side Reset Release Voltage Level 0"]
    #[inline(always)]
    pub fn svshrvl_0(self) -> &'a mut crate::W<REG> {
        self.variant(SVSHRVL_A::SVSHRVL_0)
    }
    #[doc = "SVS high side Reset Release Voltage Level 1"]
    #[inline(always)]
    pub fn svshrvl_1(self) -> &'a mut crate::W<REG> {
        self.variant(SVSHRVL_A::SVSHRVL_1)
    }
    #[doc = "SVS high side Reset Release Voltage Level 2"]
    #[inline(always)]
    pub fn svshrvl_2(self) -> &'a mut crate::W<REG> {
        self.variant(SVSHRVL_A::SVSHRVL_2)
    }
    #[doc = "SVS high side Reset Release Voltage Level 3"]
    #[inline(always)]
    pub fn svshrvl_3(self) -> &'a mut crate::W<REG> {
        self.variant(SVSHRVL_A::SVSHRVL_3)
    }
}
#[doc = "Field `SVSHE` reader - SVS high side enable"]
pub type SVSHE_R = crate::BitReader;
#[doc = "Field `SVSHE` writer - SVS high side enable"]
pub type SVSHE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVSHFP` reader - SVS high side full performace mode"]
pub type SVSHFP_R = crate::BitReader;
#[doc = "Field `SVSHFP` writer - SVS high side full performace mode"]
pub type SVSHFP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVMHOVPE` reader - SVM high side over-voltage enable"]
pub type SVMHOVPE_R = crate::BitReader;
#[doc = "Field `SVMHOVPE` writer - SVM high side over-voltage enable"]
pub type SVMHOVPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVMHE` reader - SVM high side enable"]
pub type SVMHE_R = crate::BitReader;
#[doc = "Field `SVMHE` writer - SVM high side enable"]
pub type SVMHE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVMHFP` reader - SVM high side full performace mode"]
pub type SVMHFP_R = crate::BitReader;
#[doc = "Field `SVMHFP` writer - SVM high side full performace mode"]
pub type SVMHFP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - SVS and SVM high side Reset Release Voltage Level Bit: 0"]
    #[inline(always)]
    pub fn svsmhrrl(&self) -> SVSMHRRL_R {
        SVSMHRRL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - SVS and SVM high side delay status"]
    #[inline(always)]
    pub fn svsmhdlyst(&self) -> SVSMHDLYST_R {
        SVSMHDLYST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SVS high side mode"]
    #[inline(always)]
    pub fn svshmd(&self) -> SVSHMD_R {
        SVSHMD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SVS and SVM high side event mask"]
    #[inline(always)]
    pub fn svsmhevm(&self) -> SVSMHEVM_R {
        SVSMHEVM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SVS and SVM high side auto control enable"]
    #[inline(always)]
    pub fn svsmhace(&self) -> SVSMHACE_R {
        SVSMHACE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - SVS high side reset voltage level Bit: 0"]
    #[inline(always)]
    pub fn svshrvl(&self) -> SVSHRVL_R {
        SVSHRVL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - SVS high side enable"]
    #[inline(always)]
    pub fn svshe(&self) -> SVSHE_R {
        SVSHE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SVS high side full performace mode"]
    #[inline(always)]
    pub fn svshfp(&self) -> SVSHFP_R {
        SVSHFP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SVM high side over-voltage enable"]
    #[inline(always)]
    pub fn svmhovpe(&self) -> SVMHOVPE_R {
        SVMHOVPE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - SVM high side enable"]
    #[inline(always)]
    pub fn svmhe(&self) -> SVMHE_R {
        SVMHE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SVM high side full performace mode"]
    #[inline(always)]
    pub fn svmhfp(&self) -> SVMHFP_R {
        SVMHFP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SVS and SVM high side Reset Release Voltage Level Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn svsmhrrl(&mut self) -> SVSMHRRL_W<SVSMHCTL_SPEC> {
        SVSMHRRL_W::new(self, 0)
    }
    #[doc = "Bit 3 - SVS and SVM high side delay status"]
    #[inline(always)]
    #[must_use]
    pub fn svsmhdlyst(&mut self) -> SVSMHDLYST_W<SVSMHCTL_SPEC> {
        SVSMHDLYST_W::new(self, 3)
    }
    #[doc = "Bit 4 - SVS high side mode"]
    #[inline(always)]
    #[must_use]
    pub fn svshmd(&mut self) -> SVSHMD_W<SVSMHCTL_SPEC> {
        SVSHMD_W::new(self, 4)
    }
    #[doc = "Bit 6 - SVS and SVM high side event mask"]
    #[inline(always)]
    #[must_use]
    pub fn svsmhevm(&mut self) -> SVSMHEVM_W<SVSMHCTL_SPEC> {
        SVSMHEVM_W::new(self, 6)
    }
    #[doc = "Bit 7 - SVS and SVM high side auto control enable"]
    #[inline(always)]
    #[must_use]
    pub fn svsmhace(&mut self) -> SVSMHACE_W<SVSMHCTL_SPEC> {
        SVSMHACE_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - SVS high side reset voltage level Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn svshrvl(&mut self) -> SVSHRVL_W<SVSMHCTL_SPEC> {
        SVSHRVL_W::new(self, 8)
    }
    #[doc = "Bit 10 - SVS high side enable"]
    #[inline(always)]
    #[must_use]
    pub fn svshe(&mut self) -> SVSHE_W<SVSMHCTL_SPEC> {
        SVSHE_W::new(self, 10)
    }
    #[doc = "Bit 11 - SVS high side full performace mode"]
    #[inline(always)]
    #[must_use]
    pub fn svshfp(&mut self) -> SVSHFP_W<SVSMHCTL_SPEC> {
        SVSHFP_W::new(self, 11)
    }
    #[doc = "Bit 12 - SVM high side over-voltage enable"]
    #[inline(always)]
    #[must_use]
    pub fn svmhovpe(&mut self) -> SVMHOVPE_W<SVSMHCTL_SPEC> {
        SVMHOVPE_W::new(self, 12)
    }
    #[doc = "Bit 14 - SVM high side enable"]
    #[inline(always)]
    #[must_use]
    pub fn svmhe(&mut self) -> SVMHE_W<SVSMHCTL_SPEC> {
        SVMHE_W::new(self, 14)
    }
    #[doc = "Bit 15 - SVM high side full performace mode"]
    #[inline(always)]
    #[must_use]
    pub fn svmhfp(&mut self) -> SVMHFP_W<SVSMHCTL_SPEC> {
        SVMHFP_W::new(self, 15)
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
#[doc = "SVS and SVM high side control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`svsmhctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`svsmhctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SVSMHCTL_SPEC;
impl crate::RegisterSpec for SVSMHCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`svsmhctl::R`](R) reader structure"]
impl crate::Readable for SVSMHCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`svsmhctl::W`](W) writer structure"]
impl crate::Writable for SVSMHCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SVSMHCTL to value 0"]
impl crate::Resettable for SVSMHCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
