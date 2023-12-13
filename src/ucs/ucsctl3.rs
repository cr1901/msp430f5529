#[doc = "Register `UCSCTL3` reader"]
pub type R = crate::R<UCSCTL3_SPEC>;
#[doc = "Register `UCSCTL3` writer"]
pub type W = crate::W<UCSCTL3_SPEC>;
#[doc = "Field `FLLREFDIV` reader - Reference Divider Bit : 0"]
pub type FLLREFDIV_R = crate::FieldReader<FLLREFDIV_A>;
#[doc = "Reference Divider Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLLREFDIV_A {
    #[doc = "0: Reference Divider: f(LFCLK)/1"]
    FLLREFDIV_0 = 0,
    #[doc = "1: Reference Divider: f(LFCLK)/2"]
    FLLREFDIV_1 = 1,
    #[doc = "2: Reference Divider: f(LFCLK)/4"]
    FLLREFDIV_2 = 2,
    #[doc = "3: Reference Divider: f(LFCLK)/8"]
    FLLREFDIV_3 = 3,
    #[doc = "4: Reference Divider: f(LFCLK)/12"]
    FLLREFDIV_4 = 4,
    #[doc = "5: Reference Divider: f(LFCLK)/16"]
    FLLREFDIV_5 = 5,
    #[doc = "6: Reference Divider: f(LFCLK)/16"]
    FLLREFDIV_6 = 6,
    #[doc = "7: Reference Divider: f(LFCLK)/16"]
    FLLREFDIV_7 = 7,
}
impl From<FLLREFDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: FLLREFDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FLLREFDIV_A {
    type Ux = u8;
}
impl FLLREFDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLLREFDIV_A {
        match self.bits {
            0 => FLLREFDIV_A::FLLREFDIV_0,
            1 => FLLREFDIV_A::FLLREFDIV_1,
            2 => FLLREFDIV_A::FLLREFDIV_2,
            3 => FLLREFDIV_A::FLLREFDIV_3,
            4 => FLLREFDIV_A::FLLREFDIV_4,
            5 => FLLREFDIV_A::FLLREFDIV_5,
            6 => FLLREFDIV_A::FLLREFDIV_6,
            7 => FLLREFDIV_A::FLLREFDIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Reference Divider: f(LFCLK)/1"]
    #[inline(always)]
    pub fn is_fllrefdiv_0(&self) -> bool {
        *self == FLLREFDIV_A::FLLREFDIV_0
    }
    #[doc = "Reference Divider: f(LFCLK)/2"]
    #[inline(always)]
    pub fn is_fllrefdiv_1(&self) -> bool {
        *self == FLLREFDIV_A::FLLREFDIV_1
    }
    #[doc = "Reference Divider: f(LFCLK)/4"]
    #[inline(always)]
    pub fn is_fllrefdiv_2(&self) -> bool {
        *self == FLLREFDIV_A::FLLREFDIV_2
    }
    #[doc = "Reference Divider: f(LFCLK)/8"]
    #[inline(always)]
    pub fn is_fllrefdiv_3(&self) -> bool {
        *self == FLLREFDIV_A::FLLREFDIV_3
    }
    #[doc = "Reference Divider: f(LFCLK)/12"]
    #[inline(always)]
    pub fn is_fllrefdiv_4(&self) -> bool {
        *self == FLLREFDIV_A::FLLREFDIV_4
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline(always)]
    pub fn is_fllrefdiv_5(&self) -> bool {
        *self == FLLREFDIV_A::FLLREFDIV_5
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline(always)]
    pub fn is_fllrefdiv_6(&self) -> bool {
        *self == FLLREFDIV_A::FLLREFDIV_6
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline(always)]
    pub fn is_fllrefdiv_7(&self) -> bool {
        *self == FLLREFDIV_A::FLLREFDIV_7
    }
}
#[doc = "Field `FLLREFDIV` writer - Reference Divider Bit : 0"]
pub type FLLREFDIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, FLLREFDIV_A>;
impl<'a, REG> FLLREFDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reference Divider: f(LFCLK)/1"]
    #[inline(always)]
    pub fn fllrefdiv_0(self) -> &'a mut crate::W<REG> {
        self.variant(FLLREFDIV_A::FLLREFDIV_0)
    }
    #[doc = "Reference Divider: f(LFCLK)/2"]
    #[inline(always)]
    pub fn fllrefdiv_1(self) -> &'a mut crate::W<REG> {
        self.variant(FLLREFDIV_A::FLLREFDIV_1)
    }
    #[doc = "Reference Divider: f(LFCLK)/4"]
    #[inline(always)]
    pub fn fllrefdiv_2(self) -> &'a mut crate::W<REG> {
        self.variant(FLLREFDIV_A::FLLREFDIV_2)
    }
    #[doc = "Reference Divider: f(LFCLK)/8"]
    #[inline(always)]
    pub fn fllrefdiv_3(self) -> &'a mut crate::W<REG> {
        self.variant(FLLREFDIV_A::FLLREFDIV_3)
    }
    #[doc = "Reference Divider: f(LFCLK)/12"]
    #[inline(always)]
    pub fn fllrefdiv_4(self) -> &'a mut crate::W<REG> {
        self.variant(FLLREFDIV_A::FLLREFDIV_4)
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline(always)]
    pub fn fllrefdiv_5(self) -> &'a mut crate::W<REG> {
        self.variant(FLLREFDIV_A::FLLREFDIV_5)
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline(always)]
    pub fn fllrefdiv_6(self) -> &'a mut crate::W<REG> {
        self.variant(FLLREFDIV_A::FLLREFDIV_6)
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline(always)]
    pub fn fllrefdiv_7(self) -> &'a mut crate::W<REG> {
        self.variant(FLLREFDIV_A::FLLREFDIV_7)
    }
}
#[doc = "Field `SELREF` reader - FLL Reference Clock Select Bit : 0"]
pub type SELREF_R = crate::FieldReader<SELREF_A>;
#[doc = "FLL Reference Clock Select Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELREF_A {
    #[doc = "0: FLL Reference Clock Select 0"]
    SELREF_0 = 0,
    #[doc = "1: FLL Reference Clock Select 1"]
    SELREF_1 = 1,
    #[doc = "2: FLL Reference Clock Select 2"]
    SELREF_2 = 2,
    #[doc = "3: FLL Reference Clock Select 3"]
    SELREF_3 = 3,
    #[doc = "4: FLL Reference Clock Select 4"]
    SELREF_4 = 4,
    #[doc = "5: FLL Reference Clock Select 5"]
    SELREF_5 = 5,
    #[doc = "6: FLL Reference Clock Select 6"]
    SELREF_6 = 6,
    #[doc = "7: FLL Reference Clock Select 7"]
    SELREF_7 = 7,
}
impl From<SELREF_A> for u8 {
    #[inline(always)]
    fn from(variant: SELREF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SELREF_A {
    type Ux = u8;
}
impl SELREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELREF_A {
        match self.bits {
            0 => SELREF_A::SELREF_0,
            1 => SELREF_A::SELREF_1,
            2 => SELREF_A::SELREF_2,
            3 => SELREF_A::SELREF_3,
            4 => SELREF_A::SELREF_4,
            5 => SELREF_A::SELREF_5,
            6 => SELREF_A::SELREF_6,
            7 => SELREF_A::SELREF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "FLL Reference Clock Select 0"]
    #[inline(always)]
    pub fn is_selref_0(&self) -> bool {
        *self == SELREF_A::SELREF_0
    }
    #[doc = "FLL Reference Clock Select 1"]
    #[inline(always)]
    pub fn is_selref_1(&self) -> bool {
        *self == SELREF_A::SELREF_1
    }
    #[doc = "FLL Reference Clock Select 2"]
    #[inline(always)]
    pub fn is_selref_2(&self) -> bool {
        *self == SELREF_A::SELREF_2
    }
    #[doc = "FLL Reference Clock Select 3"]
    #[inline(always)]
    pub fn is_selref_3(&self) -> bool {
        *self == SELREF_A::SELREF_3
    }
    #[doc = "FLL Reference Clock Select 4"]
    #[inline(always)]
    pub fn is_selref_4(&self) -> bool {
        *self == SELREF_A::SELREF_4
    }
    #[doc = "FLL Reference Clock Select 5"]
    #[inline(always)]
    pub fn is_selref_5(&self) -> bool {
        *self == SELREF_A::SELREF_5
    }
    #[doc = "FLL Reference Clock Select 6"]
    #[inline(always)]
    pub fn is_selref_6(&self) -> bool {
        *self == SELREF_A::SELREF_6
    }
    #[doc = "FLL Reference Clock Select 7"]
    #[inline(always)]
    pub fn is_selref_7(&self) -> bool {
        *self == SELREF_A::SELREF_7
    }
}
#[doc = "Field `SELREF` writer - FLL Reference Clock Select Bit : 0"]
pub type SELREF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SELREF_A>;
impl<'a, REG> SELREF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FLL Reference Clock Select 0"]
    #[inline(always)]
    pub fn selref_0(self) -> &'a mut crate::W<REG> {
        self.variant(SELREF_A::SELREF_0)
    }
    #[doc = "FLL Reference Clock Select 1"]
    #[inline(always)]
    pub fn selref_1(self) -> &'a mut crate::W<REG> {
        self.variant(SELREF_A::SELREF_1)
    }
    #[doc = "FLL Reference Clock Select 2"]
    #[inline(always)]
    pub fn selref_2(self) -> &'a mut crate::W<REG> {
        self.variant(SELREF_A::SELREF_2)
    }
    #[doc = "FLL Reference Clock Select 3"]
    #[inline(always)]
    pub fn selref_3(self) -> &'a mut crate::W<REG> {
        self.variant(SELREF_A::SELREF_3)
    }
    #[doc = "FLL Reference Clock Select 4"]
    #[inline(always)]
    pub fn selref_4(self) -> &'a mut crate::W<REG> {
        self.variant(SELREF_A::SELREF_4)
    }
    #[doc = "FLL Reference Clock Select 5"]
    #[inline(always)]
    pub fn selref_5(self) -> &'a mut crate::W<REG> {
        self.variant(SELREF_A::SELREF_5)
    }
    #[doc = "FLL Reference Clock Select 6"]
    #[inline(always)]
    pub fn selref_6(self) -> &'a mut crate::W<REG> {
        self.variant(SELREF_A::SELREF_6)
    }
    #[doc = "FLL Reference Clock Select 7"]
    #[inline(always)]
    pub fn selref_7(self) -> &'a mut crate::W<REG> {
        self.variant(SELREF_A::SELREF_7)
    }
}
impl R {
    #[doc = "Bits 0:2 - Reference Divider Bit : 0"]
    #[inline(always)]
    pub fn fllrefdiv(&self) -> FLLREFDIV_R {
        FLLREFDIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - FLL Reference Clock Select Bit : 0"]
    #[inline(always)]
    pub fn selref(&self) -> SELREF_R {
        SELREF_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reference Divider Bit : 0"]
    #[inline(always)]
    #[must_use]
    pub fn fllrefdiv(&mut self) -> FLLREFDIV_W<UCSCTL3_SPEC> {
        FLLREFDIV_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - FLL Reference Clock Select Bit : 0"]
    #[inline(always)]
    #[must_use]
    pub fn selref(&mut self) -> SELREF_W<UCSCTL3_SPEC> {
        SELREF_W::new(self, 4)
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
#[doc = "UCS Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsctl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsctl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSCTL3_SPEC;
impl crate::RegisterSpec for UCSCTL3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucsctl3::R`](R) reader structure"]
impl crate::Readable for UCSCTL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsctl3::W`](W) writer structure"]
impl crate::Writable for UCSCTL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSCTL3 to value 0"]
impl crate::Resettable for UCSCTL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
