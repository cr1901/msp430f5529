#[doc = "Register `UCB1I2CSA` reader"]
pub type R = crate::R<UCB1I2CSA_SPEC>;
#[doc = "Register `UCB1I2CSA` writer"]
pub type W = crate::W<UCB1I2CSA_SPEC>;
#[doc = "Field `UCSA0` reader - I2C Slave Address 0"]
pub type UCSA0_R = crate::BitReader;
#[doc = "Field `UCSA0` writer - I2C Slave Address 0"]
pub type UCSA0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSA1` reader - I2C Slave Address 1"]
pub type UCSA1_R = crate::BitReader;
#[doc = "Field `UCSA1` writer - I2C Slave Address 1"]
pub type UCSA1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSA2` reader - I2C Slave Address 2"]
pub type UCSA2_R = crate::BitReader;
#[doc = "Field `UCSA2` writer - I2C Slave Address 2"]
pub type UCSA2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSA3` reader - I2C Slave Address 3"]
pub type UCSA3_R = crate::BitReader;
#[doc = "Field `UCSA3` writer - I2C Slave Address 3"]
pub type UCSA3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSA4` reader - I2C Slave Address 4"]
pub type UCSA4_R = crate::BitReader;
#[doc = "Field `UCSA4` writer - I2C Slave Address 4"]
pub type UCSA4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSA5` reader - I2C Slave Address 5"]
pub type UCSA5_R = crate::BitReader;
#[doc = "Field `UCSA5` writer - I2C Slave Address 5"]
pub type UCSA5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSA6` reader - I2C Slave Address 6"]
pub type UCSA6_R = crate::BitReader;
#[doc = "Field `UCSA6` writer - I2C Slave Address 6"]
pub type UCSA6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSA7` reader - I2C Slave Address 7"]
pub type UCSA7_R = crate::BitReader;
#[doc = "Field `UCSA7` writer - I2C Slave Address 7"]
pub type UCSA7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSA8` reader - I2C Slave Address 8"]
pub type UCSA8_R = crate::BitReader;
#[doc = "Field `UCSA8` writer - I2C Slave Address 8"]
pub type UCSA8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSA9` reader - I2C Slave Address 9"]
pub type UCSA9_R = crate::BitReader;
#[doc = "Field `UCSA9` writer - I2C Slave Address 9"]
pub type UCSA9_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C Slave Address 0"]
    #[inline(always)]
    pub fn ucsa0(&self) -> UCSA0_R {
        UCSA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Slave Address 1"]
    #[inline(always)]
    pub fn ucsa1(&self) -> UCSA1_R {
        UCSA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Slave Address 2"]
    #[inline(always)]
    pub fn ucsa2(&self) -> UCSA2_R {
        UCSA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C Slave Address 3"]
    #[inline(always)]
    pub fn ucsa3(&self) -> UCSA3_R {
        UCSA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Slave Address 4"]
    #[inline(always)]
    pub fn ucsa4(&self) -> UCSA4_R {
        UCSA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Slave Address 5"]
    #[inline(always)]
    pub fn ucsa5(&self) -> UCSA5_R {
        UCSA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Slave Address 6"]
    #[inline(always)]
    pub fn ucsa6(&self) -> UCSA6_R {
        UCSA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Slave Address 7"]
    #[inline(always)]
    pub fn ucsa7(&self) -> UCSA7_R {
        UCSA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Slave Address 8"]
    #[inline(always)]
    pub fn ucsa8(&self) -> UCSA8_R {
        UCSA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Slave Address 9"]
    #[inline(always)]
    pub fn ucsa9(&self) -> UCSA9_R {
        UCSA9_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Slave Address 0"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa0(&mut self) -> UCSA0_W<UCB1I2CSA_SPEC> {
        UCSA0_W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C Slave Address 1"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa1(&mut self) -> UCSA1_W<UCB1I2CSA_SPEC> {
        UCSA1_W::new(self, 1)
    }
    #[doc = "Bit 2 - I2C Slave Address 2"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa2(&mut self) -> UCSA2_W<UCB1I2CSA_SPEC> {
        UCSA2_W::new(self, 2)
    }
    #[doc = "Bit 3 - I2C Slave Address 3"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa3(&mut self) -> UCSA3_W<UCB1I2CSA_SPEC> {
        UCSA3_W::new(self, 3)
    }
    #[doc = "Bit 4 - I2C Slave Address 4"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa4(&mut self) -> UCSA4_W<UCB1I2CSA_SPEC> {
        UCSA4_W::new(self, 4)
    }
    #[doc = "Bit 5 - I2C Slave Address 5"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa5(&mut self) -> UCSA5_W<UCB1I2CSA_SPEC> {
        UCSA5_W::new(self, 5)
    }
    #[doc = "Bit 6 - I2C Slave Address 6"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa6(&mut self) -> UCSA6_W<UCB1I2CSA_SPEC> {
        UCSA6_W::new(self, 6)
    }
    #[doc = "Bit 7 - I2C Slave Address 7"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa7(&mut self) -> UCSA7_W<UCB1I2CSA_SPEC> {
        UCSA7_W::new(self, 7)
    }
    #[doc = "Bit 8 - I2C Slave Address 8"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa8(&mut self) -> UCSA8_W<UCB1I2CSA_SPEC> {
        UCSA8_W::new(self, 8)
    }
    #[doc = "Bit 9 - I2C Slave Address 9"]
    #[inline(always)]
    #[must_use]
    pub fn ucsa9(&mut self) -> UCSA9_W<UCB1I2CSA_SPEC> {
        UCSA9_W::new(self, 9)
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
#[doc = "USCI B1 I2C Slave Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb1i2csa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb1i2csa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB1I2CSA_SPEC;
impl crate::RegisterSpec for UCB1I2CSA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb1i2csa::R`](R) reader structure"]
impl crate::Readable for UCB1I2CSA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb1i2csa::W`](W) writer structure"]
impl crate::Writable for UCB1I2CSA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCB1I2CSA to value 0"]
impl crate::Resettable for UCB1I2CSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
