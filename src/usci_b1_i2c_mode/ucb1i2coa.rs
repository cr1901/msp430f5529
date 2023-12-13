#[doc = "Register `UCB1I2COA` reader"]
pub type R = crate::R<UCB1I2COA_SPEC>;
#[doc = "Register `UCB1I2COA` writer"]
pub type W = crate::W<UCB1I2COA_SPEC>;
#[doc = "Field `UCOA0` reader - I2C Own Address 0"]
pub type UCOA0_R = crate::BitReader;
#[doc = "Field `UCOA0` writer - I2C Own Address 0"]
pub type UCOA0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOA1` reader - I2C Own Address 1"]
pub type UCOA1_R = crate::BitReader;
#[doc = "Field `UCOA1` writer - I2C Own Address 1"]
pub type UCOA1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOA2` reader - I2C Own Address 2"]
pub type UCOA2_R = crate::BitReader;
#[doc = "Field `UCOA2` writer - I2C Own Address 2"]
pub type UCOA2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOA3` reader - I2C Own Address 3"]
pub type UCOA3_R = crate::BitReader;
#[doc = "Field `UCOA3` writer - I2C Own Address 3"]
pub type UCOA3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOA4` reader - I2C Own Address 4"]
pub type UCOA4_R = crate::BitReader;
#[doc = "Field `UCOA4` writer - I2C Own Address 4"]
pub type UCOA4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOA5` reader - I2C Own Address 5"]
pub type UCOA5_R = crate::BitReader;
#[doc = "Field `UCOA5` writer - I2C Own Address 5"]
pub type UCOA5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOA6` reader - I2C Own Address 6"]
pub type UCOA6_R = crate::BitReader;
#[doc = "Field `UCOA6` writer - I2C Own Address 6"]
pub type UCOA6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOA7` reader - I2C Own Address 7"]
pub type UCOA7_R = crate::BitReader;
#[doc = "Field `UCOA7` writer - I2C Own Address 7"]
pub type UCOA7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOA8` reader - I2C Own Address 8"]
pub type UCOA8_R = crate::BitReader;
#[doc = "Field `UCOA8` writer - I2C Own Address 8"]
pub type UCOA8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCOA9` reader - I2C Own Address 9"]
pub type UCOA9_R = crate::BitReader;
#[doc = "Field `UCOA9` writer - I2C Own Address 9"]
pub type UCOA9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCGCEN` reader - I2C General Call enable"]
pub type UCGCEN_R = crate::BitReader;
#[doc = "Field `UCGCEN` writer - I2C General Call enable"]
pub type UCGCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C Own Address 0"]
    #[inline(always)]
    pub fn ucoa0(&self) -> UCOA0_R {
        UCOA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Own Address 1"]
    #[inline(always)]
    pub fn ucoa1(&self) -> UCOA1_R {
        UCOA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Own Address 2"]
    #[inline(always)]
    pub fn ucoa2(&self) -> UCOA2_R {
        UCOA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C Own Address 3"]
    #[inline(always)]
    pub fn ucoa3(&self) -> UCOA3_R {
        UCOA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Own Address 4"]
    #[inline(always)]
    pub fn ucoa4(&self) -> UCOA4_R {
        UCOA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Own Address 5"]
    #[inline(always)]
    pub fn ucoa5(&self) -> UCOA5_R {
        UCOA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Own Address 6"]
    #[inline(always)]
    pub fn ucoa6(&self) -> UCOA6_R {
        UCOA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Own Address 7"]
    #[inline(always)]
    pub fn ucoa7(&self) -> UCOA7_R {
        UCOA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Own Address 8"]
    #[inline(always)]
    pub fn ucoa8(&self) -> UCOA8_R {
        UCOA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Own Address 9"]
    #[inline(always)]
    pub fn ucoa9(&self) -> UCOA9_R {
        UCOA9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C General Call enable"]
    #[inline(always)]
    pub fn ucgcen(&self) -> UCGCEN_R {
        UCGCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Own Address 0"]
    #[inline(always)]
    #[must_use]
    pub fn ucoa0(&mut self) -> UCOA0_W<UCB1I2COA_SPEC> {
        UCOA0_W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C Own Address 1"]
    #[inline(always)]
    #[must_use]
    pub fn ucoa1(&mut self) -> UCOA1_W<UCB1I2COA_SPEC> {
        UCOA1_W::new(self, 1)
    }
    #[doc = "Bit 2 - I2C Own Address 2"]
    #[inline(always)]
    #[must_use]
    pub fn ucoa2(&mut self) -> UCOA2_W<UCB1I2COA_SPEC> {
        UCOA2_W::new(self, 2)
    }
    #[doc = "Bit 3 - I2C Own Address 3"]
    #[inline(always)]
    #[must_use]
    pub fn ucoa3(&mut self) -> UCOA3_W<UCB1I2COA_SPEC> {
        UCOA3_W::new(self, 3)
    }
    #[doc = "Bit 4 - I2C Own Address 4"]
    #[inline(always)]
    #[must_use]
    pub fn ucoa4(&mut self) -> UCOA4_W<UCB1I2COA_SPEC> {
        UCOA4_W::new(self, 4)
    }
    #[doc = "Bit 5 - I2C Own Address 5"]
    #[inline(always)]
    #[must_use]
    pub fn ucoa5(&mut self) -> UCOA5_W<UCB1I2COA_SPEC> {
        UCOA5_W::new(self, 5)
    }
    #[doc = "Bit 6 - I2C Own Address 6"]
    #[inline(always)]
    #[must_use]
    pub fn ucoa6(&mut self) -> UCOA6_W<UCB1I2COA_SPEC> {
        UCOA6_W::new(self, 6)
    }
    #[doc = "Bit 7 - I2C Own Address 7"]
    #[inline(always)]
    #[must_use]
    pub fn ucoa7(&mut self) -> UCOA7_W<UCB1I2COA_SPEC> {
        UCOA7_W::new(self, 7)
    }
    #[doc = "Bit 8 - I2C Own Address 8"]
    #[inline(always)]
    #[must_use]
    pub fn ucoa8(&mut self) -> UCOA8_W<UCB1I2COA_SPEC> {
        UCOA8_W::new(self, 8)
    }
    #[doc = "Bit 9 - I2C Own Address 9"]
    #[inline(always)]
    #[must_use]
    pub fn ucoa9(&mut self) -> UCOA9_W<UCB1I2COA_SPEC> {
        UCOA9_W::new(self, 9)
    }
    #[doc = "Bit 15 - I2C General Call enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucgcen(&mut self) -> UCGCEN_W<UCB1I2COA_SPEC> {
        UCGCEN_W::new(self, 15)
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
#[doc = "USCI B1 I2C Own Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1i2coa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1i2coa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB1I2COA_SPEC;
impl crate::RegisterSpec for UCB1I2COA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb1i2coa::R`](R) reader structure"]
impl crate::Readable for UCB1I2COA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb1i2coa::W`](W) writer structure"]
impl crate::Writable for UCB1I2COA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCB1I2COA to value 0"]
impl crate::Resettable for UCB1I2COA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
