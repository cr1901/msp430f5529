#[doc = "Register `UCSCTL4` reader"]
pub type R = crate::R<UCSCTL4_SPEC>;
#[doc = "Register `UCSCTL4` writer"]
pub type W = crate::W<UCSCTL4_SPEC>;
#[doc = "Field `SELM` reader - MCLK Source Select Bit: 0"]
pub type SELM_R = crate::FieldReader<SELM_A>;
#[doc = "MCLK Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for SELM_A {
    type Ux = u8;
}
impl SELM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELM_A {
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
    #[doc = "MCLK Source Select 0"]
    #[inline(always)]
    pub fn is_selm_0(&self) -> bool {
        *self == SELM_A::SELM_0
    }
    #[doc = "MCLK Source Select 1"]
    #[inline(always)]
    pub fn is_selm_1(&self) -> bool {
        *self == SELM_A::SELM_1
    }
    #[doc = "MCLK Source Select 2"]
    #[inline(always)]
    pub fn is_selm_2(&self) -> bool {
        *self == SELM_A::SELM_2
    }
    #[doc = "MCLK Source Select 3"]
    #[inline(always)]
    pub fn is_selm_3(&self) -> bool {
        *self == SELM_A::SELM_3
    }
    #[doc = "MCLK Source Select 4"]
    #[inline(always)]
    pub fn is_selm_4(&self) -> bool {
        *self == SELM_A::SELM_4
    }
    #[doc = "MCLK Source Select 5"]
    #[inline(always)]
    pub fn is_selm_5(&self) -> bool {
        *self == SELM_A::SELM_5
    }
    #[doc = "MCLK Source Select 6"]
    #[inline(always)]
    pub fn is_selm_6(&self) -> bool {
        *self == SELM_A::SELM_6
    }
    #[doc = "MCLK Source Select 7"]
    #[inline(always)]
    pub fn is_selm_7(&self) -> bool {
        *self == SELM_A::SELM_7
    }
}
#[doc = "Field `SELM` writer - MCLK Source Select Bit: 0"]
pub type SELM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SELM_A>;
impl<'a, REG> SELM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCLK Source Select 0"]
    #[inline(always)]
    pub fn selm_0(self) -> &'a mut crate::W<REG> {
        self.variant(SELM_A::SELM_0)
    }
    #[doc = "MCLK Source Select 1"]
    #[inline(always)]
    pub fn selm_1(self) -> &'a mut crate::W<REG> {
        self.variant(SELM_A::SELM_1)
    }
    #[doc = "MCLK Source Select 2"]
    #[inline(always)]
    pub fn selm_2(self) -> &'a mut crate::W<REG> {
        self.variant(SELM_A::SELM_2)
    }
    #[doc = "MCLK Source Select 3"]
    #[inline(always)]
    pub fn selm_3(self) -> &'a mut crate::W<REG> {
        self.variant(SELM_A::SELM_3)
    }
    #[doc = "MCLK Source Select 4"]
    #[inline(always)]
    pub fn selm_4(self) -> &'a mut crate::W<REG> {
        self.variant(SELM_A::SELM_4)
    }
    #[doc = "MCLK Source Select 5"]
    #[inline(always)]
    pub fn selm_5(self) -> &'a mut crate::W<REG> {
        self.variant(SELM_A::SELM_5)
    }
    #[doc = "MCLK Source Select 6"]
    #[inline(always)]
    pub fn selm_6(self) -> &'a mut crate::W<REG> {
        self.variant(SELM_A::SELM_6)
    }
    #[doc = "MCLK Source Select 7"]
    #[inline(always)]
    pub fn selm_7(self) -> &'a mut crate::W<REG> {
        self.variant(SELM_A::SELM_7)
    }
}
#[doc = "Field `SELS` reader - SMCLK Source Select Bit: 0"]
pub type SELS_R = crate::FieldReader<SELS_A>;
#[doc = "SMCLK Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for SELS_A {
    type Ux = u8;
}
impl SELS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELS_A {
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
    #[doc = "SMCLK Source Select 0"]
    #[inline(always)]
    pub fn is_sels_0(&self) -> bool {
        *self == SELS_A::SELS_0
    }
    #[doc = "SMCLK Source Select 1"]
    #[inline(always)]
    pub fn is_sels_1(&self) -> bool {
        *self == SELS_A::SELS_1
    }
    #[doc = "SMCLK Source Select 2"]
    #[inline(always)]
    pub fn is_sels_2(&self) -> bool {
        *self == SELS_A::SELS_2
    }
    #[doc = "SMCLK Source Select 3"]
    #[inline(always)]
    pub fn is_sels_3(&self) -> bool {
        *self == SELS_A::SELS_3
    }
    #[doc = "SMCLK Source Select 4"]
    #[inline(always)]
    pub fn is_sels_4(&self) -> bool {
        *self == SELS_A::SELS_4
    }
    #[doc = "SMCLK Source Select 5"]
    #[inline(always)]
    pub fn is_sels_5(&self) -> bool {
        *self == SELS_A::SELS_5
    }
    #[doc = "SMCLK Source Select 6"]
    #[inline(always)]
    pub fn is_sels_6(&self) -> bool {
        *self == SELS_A::SELS_6
    }
    #[doc = "SMCLK Source Select 7"]
    #[inline(always)]
    pub fn is_sels_7(&self) -> bool {
        *self == SELS_A::SELS_7
    }
}
#[doc = "Field `SELS` writer - SMCLK Source Select Bit: 0"]
pub type SELS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SELS_A>;
impl<'a, REG> SELS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SMCLK Source Select 0"]
    #[inline(always)]
    pub fn sels_0(self) -> &'a mut crate::W<REG> {
        self.variant(SELS_A::SELS_0)
    }
    #[doc = "SMCLK Source Select 1"]
    #[inline(always)]
    pub fn sels_1(self) -> &'a mut crate::W<REG> {
        self.variant(SELS_A::SELS_1)
    }
    #[doc = "SMCLK Source Select 2"]
    #[inline(always)]
    pub fn sels_2(self) -> &'a mut crate::W<REG> {
        self.variant(SELS_A::SELS_2)
    }
    #[doc = "SMCLK Source Select 3"]
    #[inline(always)]
    pub fn sels_3(self) -> &'a mut crate::W<REG> {
        self.variant(SELS_A::SELS_3)
    }
    #[doc = "SMCLK Source Select 4"]
    #[inline(always)]
    pub fn sels_4(self) -> &'a mut crate::W<REG> {
        self.variant(SELS_A::SELS_4)
    }
    #[doc = "SMCLK Source Select 5"]
    #[inline(always)]
    pub fn sels_5(self) -> &'a mut crate::W<REG> {
        self.variant(SELS_A::SELS_5)
    }
    #[doc = "SMCLK Source Select 6"]
    #[inline(always)]
    pub fn sels_6(self) -> &'a mut crate::W<REG> {
        self.variant(SELS_A::SELS_6)
    }
    #[doc = "SMCLK Source Select 7"]
    #[inline(always)]
    pub fn sels_7(self) -> &'a mut crate::W<REG> {
        self.variant(SELS_A::SELS_7)
    }
}
#[doc = "Field `SELA` reader - ACLK Source Select Bit: 0"]
pub type SELA_R = crate::FieldReader<SELA_A>;
#[doc = "ACLK Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for SELA_A {
    type Ux = u8;
}
impl SELA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELA_A {
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
    #[doc = "ACLK Source Select 0"]
    #[inline(always)]
    pub fn is_sela_0(&self) -> bool {
        *self == SELA_A::SELA_0
    }
    #[doc = "ACLK Source Select 1"]
    #[inline(always)]
    pub fn is_sela_1(&self) -> bool {
        *self == SELA_A::SELA_1
    }
    #[doc = "ACLK Source Select 2"]
    #[inline(always)]
    pub fn is_sela_2(&self) -> bool {
        *self == SELA_A::SELA_2
    }
    #[doc = "ACLK Source Select 3"]
    #[inline(always)]
    pub fn is_sela_3(&self) -> bool {
        *self == SELA_A::SELA_3
    }
    #[doc = "ACLK Source Select 4"]
    #[inline(always)]
    pub fn is_sela_4(&self) -> bool {
        *self == SELA_A::SELA_4
    }
    #[doc = "ACLK Source Select 5"]
    #[inline(always)]
    pub fn is_sela_5(&self) -> bool {
        *self == SELA_A::SELA_5
    }
    #[doc = "ACLK Source Select 6"]
    #[inline(always)]
    pub fn is_sela_6(&self) -> bool {
        *self == SELA_A::SELA_6
    }
    #[doc = "ACLK Source Select 7"]
    #[inline(always)]
    pub fn is_sela_7(&self) -> bool {
        *self == SELA_A::SELA_7
    }
}
#[doc = "Field `SELA` writer - ACLK Source Select Bit: 0"]
pub type SELA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SELA_A>;
impl<'a, REG> SELA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ACLK Source Select 0"]
    #[inline(always)]
    pub fn sela_0(self) -> &'a mut crate::W<REG> {
        self.variant(SELA_A::SELA_0)
    }
    #[doc = "ACLK Source Select 1"]
    #[inline(always)]
    pub fn sela_1(self) -> &'a mut crate::W<REG> {
        self.variant(SELA_A::SELA_1)
    }
    #[doc = "ACLK Source Select 2"]
    #[inline(always)]
    pub fn sela_2(self) -> &'a mut crate::W<REG> {
        self.variant(SELA_A::SELA_2)
    }
    #[doc = "ACLK Source Select 3"]
    #[inline(always)]
    pub fn sela_3(self) -> &'a mut crate::W<REG> {
        self.variant(SELA_A::SELA_3)
    }
    #[doc = "ACLK Source Select 4"]
    #[inline(always)]
    pub fn sela_4(self) -> &'a mut crate::W<REG> {
        self.variant(SELA_A::SELA_4)
    }
    #[doc = "ACLK Source Select 5"]
    #[inline(always)]
    pub fn sela_5(self) -> &'a mut crate::W<REG> {
        self.variant(SELA_A::SELA_5)
    }
    #[doc = "ACLK Source Select 6"]
    #[inline(always)]
    pub fn sela_6(self) -> &'a mut crate::W<REG> {
        self.variant(SELA_A::SELA_6)
    }
    #[doc = "ACLK Source Select 7"]
    #[inline(always)]
    pub fn sela_7(self) -> &'a mut crate::W<REG> {
        self.variant(SELA_A::SELA_7)
    }
}
impl R {
    #[doc = "Bits 0:2 - MCLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn selm(&self) -> SELM_R {
        SELM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - SMCLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn sels(&self) -> SELS_R {
        SELS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - ACLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn sela(&self) -> SELA_R {
        SELA_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCLK Source Select Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn selm(&mut self) -> SELM_W<UCSCTL4_SPEC> {
        SELM_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SMCLK Source Select Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn sels(&mut self) -> SELS_W<UCSCTL4_SPEC> {
        SELS_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - ACLK Source Select Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn sela(&mut self) -> SELA_W<UCSCTL4_SPEC> {
        SELA_W::new(self, 8)
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
#[doc = "UCS Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsctl4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsctl4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSCTL4_SPEC;
impl crate::RegisterSpec for UCSCTL4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucsctl4::R`](R) reader structure"]
impl crate::Readable for UCSCTL4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsctl4::W`](W) writer structure"]
impl crate::Writable for UCSCTL4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSCTL4 to value 0"]
impl crate::Resettable for UCSCTL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
