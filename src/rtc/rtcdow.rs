#[doc = "Register `RTCDOW` reader"]
pub type R = crate::R<RTCDOW_SPEC>;
#[doc = "Register `RTCDOW` writer"]
pub type W = crate::W<RTCDOW_SPEC>;
#[doc = "Field `DOW0` reader - Real Time Clock DOW Bit: 0"]
pub type DOW0_R = crate::BitReader;
#[doc = "Field `DOW0` writer - Real Time Clock DOW Bit: 0"]
pub type DOW0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOW1` reader - Real Time Clock DOW Bit: 1"]
pub type DOW1_R = crate::BitReader;
#[doc = "Field `DOW1` writer - Real Time Clock DOW Bit: 1"]
pub type DOW1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOW2` reader - Real Time Clock DOW Bit: 2"]
pub type DOW2_R = crate::BitReader;
#[doc = "Field `DOW2` writer - Real Time Clock DOW Bit: 2"]
pub type DOW2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOW3` reader - Real Time Clock DOW Bit: 3"]
pub type DOW3_R = crate::BitReader;
#[doc = "Field `DOW3` writer - Real Time Clock DOW Bit: 3"]
pub type DOW3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOW4` reader - Real Time Clock DOW Bit: 4"]
pub type DOW4_R = crate::BitReader;
#[doc = "Field `DOW4` writer - Real Time Clock DOW Bit: 4"]
pub type DOW4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOW5` reader - Real Time Clock DOW Bit: 5"]
pub type DOW5_R = crate::BitReader;
#[doc = "Field `DOW5` writer - Real Time Clock DOW Bit: 5"]
pub type DOW5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOW6` reader - Real Time Clock DOW Bit: 6"]
pub type DOW6_R = crate::BitReader;
#[doc = "Field `DOW6` writer - Real Time Clock DOW Bit: 6"]
pub type DOW6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Real Time Clock DOW Bit: 0"]
    #[inline(always)]
    pub fn dow0(&self) -> DOW0_R {
        DOW0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Real Time Clock DOW Bit: 1"]
    #[inline(always)]
    pub fn dow1(&self) -> DOW1_R {
        DOW1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real Time Clock DOW Bit: 2"]
    #[inline(always)]
    pub fn dow2(&self) -> DOW2_R {
        DOW2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Real Time Clock DOW Bit: 3"]
    #[inline(always)]
    pub fn dow3(&self) -> DOW3_R {
        DOW3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Real Time Clock DOW Bit: 4"]
    #[inline(always)]
    pub fn dow4(&self) -> DOW4_R {
        DOW4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Real Time Clock DOW Bit: 5"]
    #[inline(always)]
    pub fn dow5(&self) -> DOW5_R {
        DOW5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Real Time Clock DOW Bit: 6"]
    #[inline(always)]
    pub fn dow6(&self) -> DOW6_R {
        DOW6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock DOW Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn dow0(&mut self) -> DOW0_W<RTCDOW_SPEC> {
        DOW0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Real Time Clock DOW Bit: 1"]
    #[inline(always)]
    #[must_use]
    pub fn dow1(&mut self) -> DOW1_W<RTCDOW_SPEC> {
        DOW1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Real Time Clock DOW Bit: 2"]
    #[inline(always)]
    #[must_use]
    pub fn dow2(&mut self) -> DOW2_W<RTCDOW_SPEC> {
        DOW2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Real Time Clock DOW Bit: 3"]
    #[inline(always)]
    #[must_use]
    pub fn dow3(&mut self) -> DOW3_W<RTCDOW_SPEC> {
        DOW3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Real Time Clock DOW Bit: 4"]
    #[inline(always)]
    #[must_use]
    pub fn dow4(&mut self) -> DOW4_W<RTCDOW_SPEC> {
        DOW4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Real Time Clock DOW Bit: 5"]
    #[inline(always)]
    #[must_use]
    pub fn dow5(&mut self) -> DOW5_W<RTCDOW_SPEC> {
        DOW5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Real Time Clock DOW Bit: 6"]
    #[inline(always)]
    #[must_use]
    pub fn dow6(&mut self) -> DOW6_W<RTCDOW_SPEC> {
        DOW6_W::new(self, 6)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Real Time Clock Day of week\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcdow::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcdow::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCDOW_SPEC;
impl crate::RegisterSpec for RTCDOW_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rtcdow::R`](R) reader structure"]
impl crate::Readable for RTCDOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtcdow::W`](W) writer structure"]
impl crate::Writable for RTCDOW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCDOW to value 0"]
impl crate::Resettable for RTCDOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
