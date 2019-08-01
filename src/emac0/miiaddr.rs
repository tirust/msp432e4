#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MIIADDR {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_MIIADDR_MIIBR {
    bits: bool,
}
impl EMAC_MIIADDR_MIIBR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_MIIADDR_MIIBW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_MIIADDR_MIIBW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(1 << 0);
        self.w.bits |= ((value as u32) & 1) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_MIIADDR_MIIWR {
    bits: bool,
}
impl EMAC_MIIADDR_MIIWR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_MIIADDR_MIIWW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_MIIADDR_MIIWW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(1 << 1);
        self.w.bits |= ((value as u32) & 1) << 1;
        self.w
    }
}
#[doc = "Possible values of the field `EMAC_MIIADDR_CR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_MIIADDR_CRR {
    #[doc = "The frequency of the System Clock is 60 to 100 MHz providing a MDIO clock of SYSCLK/42"]
    EMAC_MIIADDR_CR_60_100,
    #[doc = "The frequency of the System Clock is 100 to 150 MHz providing a MDIO clock of SYSCLK/62"]
    EMAC_MIIADDR_CR_100_150,
    #[doc = "The frequency of the System Clock is 20-35 MHz providing a MDIO clock of System Clock/16"]
    EMAC_MIIADDR_CR_20_35,
    #[doc = "The frequency of the System Clock is 35 to 60 MHz providing a MDIO clock of System Clock/26"]
    EMAC_MIIADDR_CR_35_60,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl EMAC_MIIADDR_CRR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_MIIADDR_CRR::EMAC_MIIADDR_CR_60_100 => 0,
            EMAC_MIIADDR_CRR::EMAC_MIIADDR_CR_100_150 => 1,
            EMAC_MIIADDR_CRR::EMAC_MIIADDR_CR_20_35 => 2,
            EMAC_MIIADDR_CRR::EMAC_MIIADDR_CR_35_60 => 3,
            EMAC_MIIADDR_CRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_MIIADDR_CRR {
        match value {
            0 => EMAC_MIIADDR_CRR::EMAC_MIIADDR_CR_60_100,
            1 => EMAC_MIIADDR_CRR::EMAC_MIIADDR_CR_100_150,
            2 => EMAC_MIIADDR_CRR::EMAC_MIIADDR_CR_20_35,
            3 => EMAC_MIIADDR_CRR::EMAC_MIIADDR_CR_35_60,
            i => EMAC_MIIADDR_CRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_MIIADDR_CR_60_100`"]
    #[inline(always)]
    pub fn is_emac_miiaddr_cr_60_100(&self) -> bool {
        *self == EMAC_MIIADDR_CRR::EMAC_MIIADDR_CR_60_100
    }
    #[doc = "Checks if the value of the field is `EMAC_MIIADDR_CR_100_150`"]
    #[inline(always)]
    pub fn is_emac_miiaddr_cr_100_150(&self) -> bool {
        *self == EMAC_MIIADDR_CRR::EMAC_MIIADDR_CR_100_150
    }
    #[doc = "Checks if the value of the field is `EMAC_MIIADDR_CR_20_35`"]
    #[inline(always)]
    pub fn is_emac_miiaddr_cr_20_35(&self) -> bool {
        *self == EMAC_MIIADDR_CRR::EMAC_MIIADDR_CR_20_35
    }
    #[doc = "Checks if the value of the field is `EMAC_MIIADDR_CR_35_60`"]
    #[inline(always)]
    pub fn is_emac_miiaddr_cr_35_60(&self) -> bool {
        *self == EMAC_MIIADDR_CRR::EMAC_MIIADDR_CR_35_60
    }
}
#[doc = "Values that can be written to the field `EMAC_MIIADDR_CR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_MIIADDR_CRW {
    #[doc = "The frequency of the System Clock is 60 to 100 MHz providing a MDIO clock of SYSCLK/42"]
    EMAC_MIIADDR_CR_60_100,
    #[doc = "The frequency of the System Clock is 100 to 150 MHz providing a MDIO clock of SYSCLK/62"]
    EMAC_MIIADDR_CR_100_150,
    #[doc = "The frequency of the System Clock is 20-35 MHz providing a MDIO clock of System Clock/16"]
    EMAC_MIIADDR_CR_20_35,
    #[doc = "The frequency of the System Clock is 35 to 60 MHz providing a MDIO clock of System Clock/26"]
    EMAC_MIIADDR_CR_35_60,
}
impl EMAC_MIIADDR_CRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_MIIADDR_CRW::EMAC_MIIADDR_CR_60_100 => 0,
            EMAC_MIIADDR_CRW::EMAC_MIIADDR_CR_100_150 => 1,
            EMAC_MIIADDR_CRW::EMAC_MIIADDR_CR_20_35 => 2,
            EMAC_MIIADDR_CRW::EMAC_MIIADDR_CR_35_60 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_MIIADDR_CRW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_MIIADDR_CRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_MIIADDR_CRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The frequency of the System Clock is 60 to 100 MHz providing a MDIO clock of SYSCLK/42"]
    #[inline(always)]
    pub fn emac_miiaddr_cr_60_100(self) -> &'a mut W {
        self.variant(EMAC_MIIADDR_CRW::EMAC_MIIADDR_CR_60_100)
    }
    #[doc = "The frequency of the System Clock is 100 to 150 MHz providing a MDIO clock of SYSCLK/62"]
    #[inline(always)]
    pub fn emac_miiaddr_cr_100_150(self) -> &'a mut W {
        self.variant(EMAC_MIIADDR_CRW::EMAC_MIIADDR_CR_100_150)
    }
    #[doc = "The frequency of the System Clock is 20-35 MHz providing a MDIO clock of System Clock/16"]
    #[inline(always)]
    pub fn emac_miiaddr_cr_20_35(self) -> &'a mut W {
        self.variant(EMAC_MIIADDR_CRW::EMAC_MIIADDR_CR_20_35)
    }
    #[doc = "The frequency of the System Clock is 35 to 60 MHz providing a MDIO clock of System Clock/26"]
    #[inline(always)]
    pub fn emac_miiaddr_cr_35_60(self) -> &'a mut W {
        self.variant(EMAC_MIIADDR_CRW::EMAC_MIIADDR_CR_35_60)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 2);
        self.w.bits |= ((value as u32) & 15) << 2;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_MIIADDR_MIIR {
    bits: u8,
}
impl EMAC_MIIADDR_MIIR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_MIIADDR_MIIW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_MIIADDR_MIIW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(31 << 6);
        self.w.bits |= ((value as u32) & 31) << 6;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_MIIADDR_PLAR {
    bits: u8,
}
impl EMAC_MIIADDR_PLAR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_MIIADDR_PLAW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_MIIADDR_PLAW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(31 << 11);
        self.w.bits |= ((value as u32) & 31) << 11;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    pub fn emac_miiaddr_miib(&self) -> EMAC_MIIADDR_MIIBR {
        let bits = ((self.bits >> 0) & 1) != 0;
        EMAC_MIIADDR_MIIBR { bits }
    }
    #[doc = "Bit 1 - MII Write"]
    #[inline(always)]
    pub fn emac_miiaddr_miiw(&self) -> EMAC_MIIADDR_MIIWR {
        let bits = ((self.bits >> 1) & 1) != 0;
        EMAC_MIIADDR_MIIWR { bits }
    }
    #[doc = "Bits 2:5 - Clock Reference Frequency Selection"]
    #[inline(always)]
    pub fn emac_miiaddr_cr(&self) -> EMAC_MIIADDR_CRR {
        EMAC_MIIADDR_CRR::_from(((self.bits >> 2) & 15) as u8)
    }
    #[doc = "Bits 6:10 - MII Register"]
    #[inline(always)]
    pub fn emac_miiaddr_mii(&self) -> EMAC_MIIADDR_MIIR {
        let bits = ((self.bits >> 6) & 31) as u8;
        EMAC_MIIADDR_MIIR { bits }
    }
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline(always)]
    pub fn emac_miiaddr_pla(&self) -> EMAC_MIIADDR_PLAR {
        let bits = ((self.bits >> 11) & 31) as u8;
        EMAC_MIIADDR_PLAR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    pub fn emac_miiaddr_miib(&mut self) -> _EMAC_MIIADDR_MIIBW {
        _EMAC_MIIADDR_MIIBW { w: self }
    }
    #[doc = "Bit 1 - MII Write"]
    #[inline(always)]
    pub fn emac_miiaddr_miiw(&mut self) -> _EMAC_MIIADDR_MIIWW {
        _EMAC_MIIADDR_MIIWW { w: self }
    }
    #[doc = "Bits 2:5 - Clock Reference Frequency Selection"]
    #[inline(always)]
    pub fn emac_miiaddr_cr(&mut self) -> _EMAC_MIIADDR_CRW {
        _EMAC_MIIADDR_CRW { w: self }
    }
    #[doc = "Bits 6:10 - MII Register"]
    #[inline(always)]
    pub fn emac_miiaddr_mii(&mut self) -> _EMAC_MIIADDR_MIIW {
        _EMAC_MIIADDR_MIIW { w: self }
    }
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline(always)]
    pub fn emac_miiaddr_pla(&mut self) -> _EMAC_MIIADDR_PLAW {
        _EMAC_MIIADDR_PLAW { w: self }
    }
}
