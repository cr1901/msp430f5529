#[doc = "Register `TB0EX0` reader"]
pub type R = crate::R<TB0EX0_SPEC>;
#[doc = "Register `TB0EX0` writer"]
pub type W = crate::W<TB0EX0_SPEC>;
#[doc = "Field `TBIDEX` reader - Timer0_B7 Input divider expansion Bit: 0"]
pub type TBIDEX_R = crate::FieldReader<TBIDEX_A>;
#[doc = "Timer0_B7 Input divider expansion Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for TBIDEX_A {
    type Ux = u8;
}
impl TBIDEX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TBIDEX_A {
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
    #[doc = "Timer0_B7 Input divider expansion : /1"]
    #[inline(always)]
    pub fn is_tbidex_0(&self) -> bool {
        *self == TBIDEX_A::TBIDEX_0
    }
    #[doc = "Timer0_B7 Input divider expansion : /2"]
    #[inline(always)]
    pub fn is_tbidex_1(&self) -> bool {
        *self == TBIDEX_A::TBIDEX_1
    }
    #[doc = "Timer0_B7 Input divider expansion : /3"]
    #[inline(always)]
    pub fn is_tbidex_2(&self) -> bool {
        *self == TBIDEX_A::TBIDEX_2
    }
    #[doc = "Timer0_B7 Input divider expansion : /4"]
    #[inline(always)]
    pub fn is_tbidex_3(&self) -> bool {
        *self == TBIDEX_A::TBIDEX_3
    }
    #[doc = "Timer0_B7 Input divider expansion : /5"]
    #[inline(always)]
    pub fn is_tbidex_4(&self) -> bool {
        *self == TBIDEX_A::TBIDEX_4
    }
    #[doc = "Timer0_B7 Input divider expansion : /6"]
    #[inline(always)]
    pub fn is_tbidex_5(&self) -> bool {
        *self == TBIDEX_A::TBIDEX_5
    }
    #[doc = "Timer0_B7 Input divider expansion : /7"]
    #[inline(always)]
    pub fn is_tbidex_6(&self) -> bool {
        *self == TBIDEX_A::TBIDEX_6
    }
    #[doc = "Timer0_B7 Input divider expansion : /8"]
    #[inline(always)]
    pub fn is_tbidex_7(&self) -> bool {
        *self == TBIDEX_A::TBIDEX_7
    }
}
#[doc = "Field `TBIDEX` writer - Timer0_B7 Input divider expansion Bit: 0"]
pub type TBIDEX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, TBIDEX_A>;
impl<'a, REG> TBIDEX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer0_B7 Input divider expansion : /1"]
    #[inline(always)]
    pub fn tbidex_0(self) -> &'a mut crate::W<REG> {
        self.variant(TBIDEX_A::TBIDEX_0)
    }
    #[doc = "Timer0_B7 Input divider expansion : /2"]
    #[inline(always)]
    pub fn tbidex_1(self) -> &'a mut crate::W<REG> {
        self.variant(TBIDEX_A::TBIDEX_1)
    }
    #[doc = "Timer0_B7 Input divider expansion : /3"]
    #[inline(always)]
    pub fn tbidex_2(self) -> &'a mut crate::W<REG> {
        self.variant(TBIDEX_A::TBIDEX_2)
    }
    #[doc = "Timer0_B7 Input divider expansion : /4"]
    #[inline(always)]
    pub fn tbidex_3(self) -> &'a mut crate::W<REG> {
        self.variant(TBIDEX_A::TBIDEX_3)
    }
    #[doc = "Timer0_B7 Input divider expansion : /5"]
    #[inline(always)]
    pub fn tbidex_4(self) -> &'a mut crate::W<REG> {
        self.variant(TBIDEX_A::TBIDEX_4)
    }
    #[doc = "Timer0_B7 Input divider expansion : /6"]
    #[inline(always)]
    pub fn tbidex_5(self) -> &'a mut crate::W<REG> {
        self.variant(TBIDEX_A::TBIDEX_5)
    }
    #[doc = "Timer0_B7 Input divider expansion : /7"]
    #[inline(always)]
    pub fn tbidex_6(self) -> &'a mut crate::W<REG> {
        self.variant(TBIDEX_A::TBIDEX_6)
    }
    #[doc = "Timer0_B7 Input divider expansion : /8"]
    #[inline(always)]
    pub fn tbidex_7(self) -> &'a mut crate::W<REG> {
        self.variant(TBIDEX_A::TBIDEX_7)
    }
}
impl R {
    #[doc = "Bits 0:2 - Timer0_B7 Input divider expansion Bit: 0"]
    #[inline(always)]
    pub fn tbidex(&self) -> TBIDEX_R {
        TBIDEX_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timer0_B7 Input divider expansion Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn tbidex(&mut self) -> TBIDEX_W<TB0EX0_SPEC> {
        TBIDEX_W::new(self, 0)
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
#[doc = "Timer0_B7 Expansion Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tb0ex0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tb0ex0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TB0EX0_SPEC;
impl crate::RegisterSpec for TB0EX0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tb0ex0::R`](R) reader structure"]
impl crate::Readable for TB0EX0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tb0ex0::W`](W) writer structure"]
impl crate::Writable for TB0EX0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TB0EX0 to value 0"]
impl crate::Resettable for TB0EX0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
