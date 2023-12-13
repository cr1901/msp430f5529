#[doc = "Register `TA0EX0` reader"]
pub type R = crate::R<TA0EX0_SPEC>;
#[doc = "Register `TA0EX0` writer"]
pub type W = crate::W<TA0EX0_SPEC>;
#[doc = "Field `TAIDEX` reader - Timer A Input divider expansion Bit: 0"]
pub type TAIDEX_R = crate::FieldReader<TAIDEX_A>;
#[doc = "Timer A Input divider expansion Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAIDEX_A {
    #[doc = "0: Timer A Input divider expansion : /1"]
    TAIDEX_0 = 0,
    #[doc = "1: Timer A Input divider expansion : /2"]
    TAIDEX_1 = 1,
    #[doc = "2: Timer A Input divider expansion : /3"]
    TAIDEX_2 = 2,
    #[doc = "3: Timer A Input divider expansion : /4"]
    TAIDEX_3 = 3,
    #[doc = "4: Timer A Input divider expansion : /5"]
    TAIDEX_4 = 4,
    #[doc = "5: Timer A Input divider expansion : /6"]
    TAIDEX_5 = 5,
    #[doc = "6: Timer A Input divider expansion : /7"]
    TAIDEX_6 = 6,
    #[doc = "7: Timer A Input divider expansion : /8"]
    TAIDEX_7 = 7,
}
impl From<TAIDEX_A> for u8 {
    #[inline(always)]
    fn from(variant: TAIDEX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TAIDEX_A {
    type Ux = u8;
}
impl TAIDEX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAIDEX_A {
        match self.bits {
            0 => TAIDEX_A::TAIDEX_0,
            1 => TAIDEX_A::TAIDEX_1,
            2 => TAIDEX_A::TAIDEX_2,
            3 => TAIDEX_A::TAIDEX_3,
            4 => TAIDEX_A::TAIDEX_4,
            5 => TAIDEX_A::TAIDEX_5,
            6 => TAIDEX_A::TAIDEX_6,
            7 => TAIDEX_A::TAIDEX_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer A Input divider expansion : /1"]
    #[inline(always)]
    pub fn is_taidex_0(&self) -> bool {
        *self == TAIDEX_A::TAIDEX_0
    }
    #[doc = "Timer A Input divider expansion : /2"]
    #[inline(always)]
    pub fn is_taidex_1(&self) -> bool {
        *self == TAIDEX_A::TAIDEX_1
    }
    #[doc = "Timer A Input divider expansion : /3"]
    #[inline(always)]
    pub fn is_taidex_2(&self) -> bool {
        *self == TAIDEX_A::TAIDEX_2
    }
    #[doc = "Timer A Input divider expansion : /4"]
    #[inline(always)]
    pub fn is_taidex_3(&self) -> bool {
        *self == TAIDEX_A::TAIDEX_3
    }
    #[doc = "Timer A Input divider expansion : /5"]
    #[inline(always)]
    pub fn is_taidex_4(&self) -> bool {
        *self == TAIDEX_A::TAIDEX_4
    }
    #[doc = "Timer A Input divider expansion : /6"]
    #[inline(always)]
    pub fn is_taidex_5(&self) -> bool {
        *self == TAIDEX_A::TAIDEX_5
    }
    #[doc = "Timer A Input divider expansion : /7"]
    #[inline(always)]
    pub fn is_taidex_6(&self) -> bool {
        *self == TAIDEX_A::TAIDEX_6
    }
    #[doc = "Timer A Input divider expansion : /8"]
    #[inline(always)]
    pub fn is_taidex_7(&self) -> bool {
        *self == TAIDEX_A::TAIDEX_7
    }
}
#[doc = "Field `TAIDEX` writer - Timer A Input divider expansion Bit: 0"]
pub type TAIDEX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, TAIDEX_A>;
impl<'a, REG> TAIDEX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer A Input divider expansion : /1"]
    #[inline(always)]
    pub fn taidex_0(self) -> &'a mut crate::W<REG> {
        self.variant(TAIDEX_A::TAIDEX_0)
    }
    #[doc = "Timer A Input divider expansion : /2"]
    #[inline(always)]
    pub fn taidex_1(self) -> &'a mut crate::W<REG> {
        self.variant(TAIDEX_A::TAIDEX_1)
    }
    #[doc = "Timer A Input divider expansion : /3"]
    #[inline(always)]
    pub fn taidex_2(self) -> &'a mut crate::W<REG> {
        self.variant(TAIDEX_A::TAIDEX_2)
    }
    #[doc = "Timer A Input divider expansion : /4"]
    #[inline(always)]
    pub fn taidex_3(self) -> &'a mut crate::W<REG> {
        self.variant(TAIDEX_A::TAIDEX_3)
    }
    #[doc = "Timer A Input divider expansion : /5"]
    #[inline(always)]
    pub fn taidex_4(self) -> &'a mut crate::W<REG> {
        self.variant(TAIDEX_A::TAIDEX_4)
    }
    #[doc = "Timer A Input divider expansion : /6"]
    #[inline(always)]
    pub fn taidex_5(self) -> &'a mut crate::W<REG> {
        self.variant(TAIDEX_A::TAIDEX_5)
    }
    #[doc = "Timer A Input divider expansion : /7"]
    #[inline(always)]
    pub fn taidex_6(self) -> &'a mut crate::W<REG> {
        self.variant(TAIDEX_A::TAIDEX_6)
    }
    #[doc = "Timer A Input divider expansion : /8"]
    #[inline(always)]
    pub fn taidex_7(self) -> &'a mut crate::W<REG> {
        self.variant(TAIDEX_A::TAIDEX_7)
    }
}
impl R {
    #[doc = "Bits 0:2 - Timer A Input divider expansion Bit: 0"]
    #[inline(always)]
    pub fn taidex(&self) -> TAIDEX_R {
        TAIDEX_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timer A Input divider expansion Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn taidex(&mut self) -> TAIDEX_W<TA0EX0_SPEC> {
        TAIDEX_W::new(self, 0)
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
#[doc = "Timer0_A5 Expansion Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta0ex0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta0ex0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TA0EX0_SPEC;
impl crate::RegisterSpec for TA0EX0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta0ex0::R`](R) reader structure"]
impl crate::Readable for TA0EX0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ta0ex0::W`](W) writer structure"]
impl crate::Writable for TA0EX0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TA0EX0 to value 0"]
impl crate::Resettable for TA0EX0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
