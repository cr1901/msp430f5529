#[doc = "Register `UCSCTL5` reader"]
pub type R = crate::R<UCSCTL5_SPEC>;
#[doc = "Register `UCSCTL5` writer"]
pub type W = crate::W<UCSCTL5_SPEC>;
#[doc = "Field `DIVM` reader - MCLK Divider Bit: 0"]
pub type DIVM_R = crate::FieldReader<DIVM_A>;
#[doc = "MCLK Divider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVM_A {
    #[doc = "0: MCLK Source Divider 0"]
    DIVM_0 = 0,
    #[doc = "1: MCLK Source Divider 1"]
    DIVM_1 = 1,
    #[doc = "2: MCLK Source Divider 2"]
    DIVM_2 = 2,
    #[doc = "3: MCLK Source Divider 3"]
    DIVM_3 = 3,
    #[doc = "4: MCLK Source Divider 4"]
    DIVM_4 = 4,
    #[doc = "5: MCLK Source Divider 5"]
    DIVM_5 = 5,
    #[doc = "6: MCLK Source Divider 6"]
    DIVM_6 = 6,
    #[doc = "7: MCLK Source Divider 7"]
    DIVM_7 = 7,
}
impl From<DIVM_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIVM_A {
    type Ux = u8;
}
impl DIVM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIVM_A {
        match self.bits {
            0 => DIVM_A::DIVM_0,
            1 => DIVM_A::DIVM_1,
            2 => DIVM_A::DIVM_2,
            3 => DIVM_A::DIVM_3,
            4 => DIVM_A::DIVM_4,
            5 => DIVM_A::DIVM_5,
            6 => DIVM_A::DIVM_6,
            7 => DIVM_A::DIVM_7,
            _ => unreachable!(),
        }
    }
    #[doc = "MCLK Source Divider 0"]
    #[inline(always)]
    pub fn is_divm_0(&self) -> bool {
        *self == DIVM_A::DIVM_0
    }
    #[doc = "MCLK Source Divider 1"]
    #[inline(always)]
    pub fn is_divm_1(&self) -> bool {
        *self == DIVM_A::DIVM_1
    }
    #[doc = "MCLK Source Divider 2"]
    #[inline(always)]
    pub fn is_divm_2(&self) -> bool {
        *self == DIVM_A::DIVM_2
    }
    #[doc = "MCLK Source Divider 3"]
    #[inline(always)]
    pub fn is_divm_3(&self) -> bool {
        *self == DIVM_A::DIVM_3
    }
    #[doc = "MCLK Source Divider 4"]
    #[inline(always)]
    pub fn is_divm_4(&self) -> bool {
        *self == DIVM_A::DIVM_4
    }
    #[doc = "MCLK Source Divider 5"]
    #[inline(always)]
    pub fn is_divm_5(&self) -> bool {
        *self == DIVM_A::DIVM_5
    }
    #[doc = "MCLK Source Divider 6"]
    #[inline(always)]
    pub fn is_divm_6(&self) -> bool {
        *self == DIVM_A::DIVM_6
    }
    #[doc = "MCLK Source Divider 7"]
    #[inline(always)]
    pub fn is_divm_7(&self) -> bool {
        *self == DIVM_A::DIVM_7
    }
}
#[doc = "Field `DIVM` writer - MCLK Divider Bit: 0"]
pub type DIVM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, DIVM_A>;
impl<'a, REG> DIVM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCLK Source Divider 0"]
    #[inline(always)]
    pub fn divm_0(self) -> &'a mut crate::W<REG> {
        self.variant(DIVM_A::DIVM_0)
    }
    #[doc = "MCLK Source Divider 1"]
    #[inline(always)]
    pub fn divm_1(self) -> &'a mut crate::W<REG> {
        self.variant(DIVM_A::DIVM_1)
    }
    #[doc = "MCLK Source Divider 2"]
    #[inline(always)]
    pub fn divm_2(self) -> &'a mut crate::W<REG> {
        self.variant(DIVM_A::DIVM_2)
    }
    #[doc = "MCLK Source Divider 3"]
    #[inline(always)]
    pub fn divm_3(self) -> &'a mut crate::W<REG> {
        self.variant(DIVM_A::DIVM_3)
    }
    #[doc = "MCLK Source Divider 4"]
    #[inline(always)]
    pub fn divm_4(self) -> &'a mut crate::W<REG> {
        self.variant(DIVM_A::DIVM_4)
    }
    #[doc = "MCLK Source Divider 5"]
    #[inline(always)]
    pub fn divm_5(self) -> &'a mut crate::W<REG> {
        self.variant(DIVM_A::DIVM_5)
    }
    #[doc = "MCLK Source Divider 6"]
    #[inline(always)]
    pub fn divm_6(self) -> &'a mut crate::W<REG> {
        self.variant(DIVM_A::DIVM_6)
    }
    #[doc = "MCLK Source Divider 7"]
    #[inline(always)]
    pub fn divm_7(self) -> &'a mut crate::W<REG> {
        self.variant(DIVM_A::DIVM_7)
    }
}
#[doc = "Field `DIVS` reader - SMCLK Divider Bit: 0"]
pub type DIVS_R = crate::FieldReader<DIVS_A>;
#[doc = "SMCLK Divider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVS_A {
    #[doc = "0: SMCLK Source Divider 0"]
    DIVS_0 = 0,
    #[doc = "1: SMCLK Source Divider 1"]
    DIVS_1 = 1,
    #[doc = "2: SMCLK Source Divider 2"]
    DIVS_2 = 2,
    #[doc = "3: SMCLK Source Divider 3"]
    DIVS_3 = 3,
    #[doc = "4: SMCLK Source Divider 4"]
    DIVS_4 = 4,
    #[doc = "5: SMCLK Source Divider 5"]
    DIVS_5 = 5,
    #[doc = "6: SMCLK Source Divider 6"]
    DIVS_6 = 6,
    #[doc = "7: SMCLK Source Divider 7"]
    DIVS_7 = 7,
}
impl From<DIVS_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIVS_A {
    type Ux = u8;
}
impl DIVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIVS_A {
        match self.bits {
            0 => DIVS_A::DIVS_0,
            1 => DIVS_A::DIVS_1,
            2 => DIVS_A::DIVS_2,
            3 => DIVS_A::DIVS_3,
            4 => DIVS_A::DIVS_4,
            5 => DIVS_A::DIVS_5,
            6 => DIVS_A::DIVS_6,
            7 => DIVS_A::DIVS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "SMCLK Source Divider 0"]
    #[inline(always)]
    pub fn is_divs_0(&self) -> bool {
        *self == DIVS_A::DIVS_0
    }
    #[doc = "SMCLK Source Divider 1"]
    #[inline(always)]
    pub fn is_divs_1(&self) -> bool {
        *self == DIVS_A::DIVS_1
    }
    #[doc = "SMCLK Source Divider 2"]
    #[inline(always)]
    pub fn is_divs_2(&self) -> bool {
        *self == DIVS_A::DIVS_2
    }
    #[doc = "SMCLK Source Divider 3"]
    #[inline(always)]
    pub fn is_divs_3(&self) -> bool {
        *self == DIVS_A::DIVS_3
    }
    #[doc = "SMCLK Source Divider 4"]
    #[inline(always)]
    pub fn is_divs_4(&self) -> bool {
        *self == DIVS_A::DIVS_4
    }
    #[doc = "SMCLK Source Divider 5"]
    #[inline(always)]
    pub fn is_divs_5(&self) -> bool {
        *self == DIVS_A::DIVS_5
    }
    #[doc = "SMCLK Source Divider 6"]
    #[inline(always)]
    pub fn is_divs_6(&self) -> bool {
        *self == DIVS_A::DIVS_6
    }
    #[doc = "SMCLK Source Divider 7"]
    #[inline(always)]
    pub fn is_divs_7(&self) -> bool {
        *self == DIVS_A::DIVS_7
    }
}
#[doc = "Field `DIVS` writer - SMCLK Divider Bit: 0"]
pub type DIVS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, DIVS_A>;
impl<'a, REG> DIVS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SMCLK Source Divider 0"]
    #[inline(always)]
    pub fn divs_0(self) -> &'a mut crate::W<REG> {
        self.variant(DIVS_A::DIVS_0)
    }
    #[doc = "SMCLK Source Divider 1"]
    #[inline(always)]
    pub fn divs_1(self) -> &'a mut crate::W<REG> {
        self.variant(DIVS_A::DIVS_1)
    }
    #[doc = "SMCLK Source Divider 2"]
    #[inline(always)]
    pub fn divs_2(self) -> &'a mut crate::W<REG> {
        self.variant(DIVS_A::DIVS_2)
    }
    #[doc = "SMCLK Source Divider 3"]
    #[inline(always)]
    pub fn divs_3(self) -> &'a mut crate::W<REG> {
        self.variant(DIVS_A::DIVS_3)
    }
    #[doc = "SMCLK Source Divider 4"]
    #[inline(always)]
    pub fn divs_4(self) -> &'a mut crate::W<REG> {
        self.variant(DIVS_A::DIVS_4)
    }
    #[doc = "SMCLK Source Divider 5"]
    #[inline(always)]
    pub fn divs_5(self) -> &'a mut crate::W<REG> {
        self.variant(DIVS_A::DIVS_5)
    }
    #[doc = "SMCLK Source Divider 6"]
    #[inline(always)]
    pub fn divs_6(self) -> &'a mut crate::W<REG> {
        self.variant(DIVS_A::DIVS_6)
    }
    #[doc = "SMCLK Source Divider 7"]
    #[inline(always)]
    pub fn divs_7(self) -> &'a mut crate::W<REG> {
        self.variant(DIVS_A::DIVS_7)
    }
}
#[doc = "Field `DIVA` reader - ACLK Divider Bit: 0"]
pub type DIVA_R = crate::FieldReader<DIVA_A>;
#[doc = "ACLK Divider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVA_A {
    #[doc = "0: ACLK Source Divider 0"]
    DIVA_0 = 0,
    #[doc = "1: ACLK Source Divider 1"]
    DIVA_1 = 1,
    #[doc = "2: ACLK Source Divider 2"]
    DIVA_2 = 2,
    #[doc = "3: ACLK Source Divider 3"]
    DIVA_3 = 3,
    #[doc = "4: ACLK Source Divider 4"]
    DIVA_4 = 4,
    #[doc = "5: ACLK Source Divider 5"]
    DIVA_5 = 5,
    #[doc = "6: ACLK Source Divider 6"]
    DIVA_6 = 6,
    #[doc = "7: ACLK Source Divider 7"]
    DIVA_7 = 7,
}
impl From<DIVA_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIVA_A {
    type Ux = u8;
}
impl DIVA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIVA_A {
        match self.bits {
            0 => DIVA_A::DIVA_0,
            1 => DIVA_A::DIVA_1,
            2 => DIVA_A::DIVA_2,
            3 => DIVA_A::DIVA_3,
            4 => DIVA_A::DIVA_4,
            5 => DIVA_A::DIVA_5,
            6 => DIVA_A::DIVA_6,
            7 => DIVA_A::DIVA_7,
            _ => unreachable!(),
        }
    }
    #[doc = "ACLK Source Divider 0"]
    #[inline(always)]
    pub fn is_diva_0(&self) -> bool {
        *self == DIVA_A::DIVA_0
    }
    #[doc = "ACLK Source Divider 1"]
    #[inline(always)]
    pub fn is_diva_1(&self) -> bool {
        *self == DIVA_A::DIVA_1
    }
    #[doc = "ACLK Source Divider 2"]
    #[inline(always)]
    pub fn is_diva_2(&self) -> bool {
        *self == DIVA_A::DIVA_2
    }
    #[doc = "ACLK Source Divider 3"]
    #[inline(always)]
    pub fn is_diva_3(&self) -> bool {
        *self == DIVA_A::DIVA_3
    }
    #[doc = "ACLK Source Divider 4"]
    #[inline(always)]
    pub fn is_diva_4(&self) -> bool {
        *self == DIVA_A::DIVA_4
    }
    #[doc = "ACLK Source Divider 5"]
    #[inline(always)]
    pub fn is_diva_5(&self) -> bool {
        *self == DIVA_A::DIVA_5
    }
    #[doc = "ACLK Source Divider 6"]
    #[inline(always)]
    pub fn is_diva_6(&self) -> bool {
        *self == DIVA_A::DIVA_6
    }
    #[doc = "ACLK Source Divider 7"]
    #[inline(always)]
    pub fn is_diva_7(&self) -> bool {
        *self == DIVA_A::DIVA_7
    }
}
#[doc = "Field `DIVA` writer - ACLK Divider Bit: 0"]
pub type DIVA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, DIVA_A>;
impl<'a, REG> DIVA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ACLK Source Divider 0"]
    #[inline(always)]
    pub fn diva_0(self) -> &'a mut crate::W<REG> {
        self.variant(DIVA_A::DIVA_0)
    }
    #[doc = "ACLK Source Divider 1"]
    #[inline(always)]
    pub fn diva_1(self) -> &'a mut crate::W<REG> {
        self.variant(DIVA_A::DIVA_1)
    }
    #[doc = "ACLK Source Divider 2"]
    #[inline(always)]
    pub fn diva_2(self) -> &'a mut crate::W<REG> {
        self.variant(DIVA_A::DIVA_2)
    }
    #[doc = "ACLK Source Divider 3"]
    #[inline(always)]
    pub fn diva_3(self) -> &'a mut crate::W<REG> {
        self.variant(DIVA_A::DIVA_3)
    }
    #[doc = "ACLK Source Divider 4"]
    #[inline(always)]
    pub fn diva_4(self) -> &'a mut crate::W<REG> {
        self.variant(DIVA_A::DIVA_4)
    }
    #[doc = "ACLK Source Divider 5"]
    #[inline(always)]
    pub fn diva_5(self) -> &'a mut crate::W<REG> {
        self.variant(DIVA_A::DIVA_5)
    }
    #[doc = "ACLK Source Divider 6"]
    #[inline(always)]
    pub fn diva_6(self) -> &'a mut crate::W<REG> {
        self.variant(DIVA_A::DIVA_6)
    }
    #[doc = "ACLK Source Divider 7"]
    #[inline(always)]
    pub fn diva_7(self) -> &'a mut crate::W<REG> {
        self.variant(DIVA_A::DIVA_7)
    }
}
#[doc = "Field `DIVPA` reader - ACLK from Pin Divider Bit: 0"]
pub type DIVPA_R = crate::FieldReader<DIVPA_A>;
#[doc = "ACLK from Pin Divider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVPA_A {
    #[doc = "0: ACLK from Pin Source Divider 0"]
    DIVPA_0 = 0,
    #[doc = "1: ACLK from Pin Source Divider 1"]
    DIVPA_1 = 1,
    #[doc = "2: ACLK from Pin Source Divider 2"]
    DIVPA_2 = 2,
    #[doc = "3: ACLK from Pin Source Divider 3"]
    DIVPA_3 = 3,
    #[doc = "4: ACLK from Pin Source Divider 4"]
    DIVPA_4 = 4,
    #[doc = "5: ACLK from Pin Source Divider 5"]
    DIVPA_5 = 5,
    #[doc = "6: ACLK from Pin Source Divider 6"]
    DIVPA_6 = 6,
    #[doc = "7: ACLK from Pin Source Divider 7"]
    DIVPA_7 = 7,
}
impl From<DIVPA_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVPA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIVPA_A {
    type Ux = u8;
}
impl DIVPA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIVPA_A {
        match self.bits {
            0 => DIVPA_A::DIVPA_0,
            1 => DIVPA_A::DIVPA_1,
            2 => DIVPA_A::DIVPA_2,
            3 => DIVPA_A::DIVPA_3,
            4 => DIVPA_A::DIVPA_4,
            5 => DIVPA_A::DIVPA_5,
            6 => DIVPA_A::DIVPA_6,
            7 => DIVPA_A::DIVPA_7,
            _ => unreachable!(),
        }
    }
    #[doc = "ACLK from Pin Source Divider 0"]
    #[inline(always)]
    pub fn is_divpa_0(&self) -> bool {
        *self == DIVPA_A::DIVPA_0
    }
    #[doc = "ACLK from Pin Source Divider 1"]
    #[inline(always)]
    pub fn is_divpa_1(&self) -> bool {
        *self == DIVPA_A::DIVPA_1
    }
    #[doc = "ACLK from Pin Source Divider 2"]
    #[inline(always)]
    pub fn is_divpa_2(&self) -> bool {
        *self == DIVPA_A::DIVPA_2
    }
    #[doc = "ACLK from Pin Source Divider 3"]
    #[inline(always)]
    pub fn is_divpa_3(&self) -> bool {
        *self == DIVPA_A::DIVPA_3
    }
    #[doc = "ACLK from Pin Source Divider 4"]
    #[inline(always)]
    pub fn is_divpa_4(&self) -> bool {
        *self == DIVPA_A::DIVPA_4
    }
    #[doc = "ACLK from Pin Source Divider 5"]
    #[inline(always)]
    pub fn is_divpa_5(&self) -> bool {
        *self == DIVPA_A::DIVPA_5
    }
    #[doc = "ACLK from Pin Source Divider 6"]
    #[inline(always)]
    pub fn is_divpa_6(&self) -> bool {
        *self == DIVPA_A::DIVPA_6
    }
    #[doc = "ACLK from Pin Source Divider 7"]
    #[inline(always)]
    pub fn is_divpa_7(&self) -> bool {
        *self == DIVPA_A::DIVPA_7
    }
}
#[doc = "Field `DIVPA` writer - ACLK from Pin Divider Bit: 0"]
pub type DIVPA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, DIVPA_A>;
impl<'a, REG> DIVPA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ACLK from Pin Source Divider 0"]
    #[inline(always)]
    pub fn divpa_0(self) -> &'a mut crate::W<REG> {
        self.variant(DIVPA_A::DIVPA_0)
    }
    #[doc = "ACLK from Pin Source Divider 1"]
    #[inline(always)]
    pub fn divpa_1(self) -> &'a mut crate::W<REG> {
        self.variant(DIVPA_A::DIVPA_1)
    }
    #[doc = "ACLK from Pin Source Divider 2"]
    #[inline(always)]
    pub fn divpa_2(self) -> &'a mut crate::W<REG> {
        self.variant(DIVPA_A::DIVPA_2)
    }
    #[doc = "ACLK from Pin Source Divider 3"]
    #[inline(always)]
    pub fn divpa_3(self) -> &'a mut crate::W<REG> {
        self.variant(DIVPA_A::DIVPA_3)
    }
    #[doc = "ACLK from Pin Source Divider 4"]
    #[inline(always)]
    pub fn divpa_4(self) -> &'a mut crate::W<REG> {
        self.variant(DIVPA_A::DIVPA_4)
    }
    #[doc = "ACLK from Pin Source Divider 5"]
    #[inline(always)]
    pub fn divpa_5(self) -> &'a mut crate::W<REG> {
        self.variant(DIVPA_A::DIVPA_5)
    }
    #[doc = "ACLK from Pin Source Divider 6"]
    #[inline(always)]
    pub fn divpa_6(self) -> &'a mut crate::W<REG> {
        self.variant(DIVPA_A::DIVPA_6)
    }
    #[doc = "ACLK from Pin Source Divider 7"]
    #[inline(always)]
    pub fn divpa_7(self) -> &'a mut crate::W<REG> {
        self.variant(DIVPA_A::DIVPA_7)
    }
}
impl R {
    #[doc = "Bits 0:2 - MCLK Divider Bit: 0"]
    #[inline(always)]
    pub fn divm(&self) -> DIVM_R {
        DIVM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - SMCLK Divider Bit: 0"]
    #[inline(always)]
    pub fn divs(&self) -> DIVS_R {
        DIVS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - ACLK Divider Bit: 0"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - ACLK from Pin Divider Bit: 0"]
    #[inline(always)]
    pub fn divpa(&self) -> DIVPA_R {
        DIVPA_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCLK Divider Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn divm(&mut self) -> DIVM_W<UCSCTL5_SPEC> {
        DIVM_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SMCLK Divider Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn divs(&mut self) -> DIVS_W<UCSCTL5_SPEC> {
        DIVS_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - ACLK Divider Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn diva(&mut self) -> DIVA_W<UCSCTL5_SPEC> {
        DIVA_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - ACLK from Pin Divider Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn divpa(&mut self) -> DIVPA_W<UCSCTL5_SPEC> {
        DIVPA_W::new(self, 12)
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
#[doc = "UCS Control Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsctl5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsctl5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSCTL5_SPEC;
impl crate::RegisterSpec for UCSCTL5_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucsctl5::R`](R) reader structure"]
impl crate::Readable for UCSCTL5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsctl5::W`](W) writer structure"]
impl crate::Writable for UCSCTL5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSCTL5 to value 0"]
impl crate::Resettable for UCSCTL5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
