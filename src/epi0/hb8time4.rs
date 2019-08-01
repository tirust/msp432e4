#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HB8TIME4 {
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
pub struct EPI_HB8TIME4_RDWSMR {
    bits: bool,
}
impl EPI_HB8TIME4_RDWSMR {
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
pub struct _EPI_HB8TIME4_RDWSMW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB8TIME4_RDWSMW<'a> {
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
pub struct EPI_HB8TIME4_WRWSMR {
    bits: bool,
}
impl EPI_HB8TIME4_WRWSMR {
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
pub struct _EPI_HB8TIME4_WRWSMW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB8TIME4_WRWSMW<'a> {
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
        self.w.bits &= !(1 << 4);
        self.w.bits |= ((value as u32) & 1) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_HB8TIME4_CAPWIDTHR {
    bits: u8,
}
impl EPI_HB8TIME4_CAPWIDTHR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB8TIME4_CAPWIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB8TIME4_CAPWIDTHW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 12);
        self.w.bits |= ((value as u32) & 3) << 12;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_HB8TIME4_IRDYDLYR {
    bits: u8,
}
impl EPI_HB8TIME4_IRDYDLYR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB8TIME4_IRDYDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB8TIME4_IRDYDLYW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 24);
        self.w.bits |= ((value as u32) & 3) << 24;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - CS3n Read Wait State Minus One"]
    #[inline(always)]
    pub fn epi_hb8time4_rdwsm(&self) -> EPI_HB8TIME4_RDWSMR {
        let bits = ((self.bits >> 0) & 1) != 0;
        EPI_HB8TIME4_RDWSMR { bits }
    }
    #[doc = "Bit 4 - CS3n Write Wait State Minus One"]
    #[inline(always)]
    pub fn epi_hb8time4_wrwsm(&self) -> EPI_HB8TIME4_WRWSMR {
        let bits = ((self.bits >> 4) & 1) != 0;
        EPI_HB8TIME4_WRWSMR { bits }
    }
    #[doc = "Bits 12:13 - CS3n Inter-transfer Capture Width"]
    #[inline(always)]
    pub fn epi_hb8time4_capwidth(&self) -> EPI_HB8TIME4_CAPWIDTHR {
        let bits = ((self.bits >> 12) & 3) as u8;
        EPI_HB8TIME4_CAPWIDTHR { bits }
    }
    #[doc = "Bits 24:25 - CS3n Input Ready Delay"]
    #[inline(always)]
    pub fn epi_hb8time4_irdydly(&self) -> EPI_HB8TIME4_IRDYDLYR {
        let bits = ((self.bits >> 24) & 3) as u8;
        EPI_HB8TIME4_IRDYDLYR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - CS3n Read Wait State Minus One"]
    #[inline(always)]
    pub fn epi_hb8time4_rdwsm(&mut self) -> _EPI_HB8TIME4_RDWSMW {
        _EPI_HB8TIME4_RDWSMW { w: self }
    }
    #[doc = "Bit 4 - CS3n Write Wait State Minus One"]
    #[inline(always)]
    pub fn epi_hb8time4_wrwsm(&mut self) -> _EPI_HB8TIME4_WRWSMW {
        _EPI_HB8TIME4_WRWSMW { w: self }
    }
    #[doc = "Bits 12:13 - CS3n Inter-transfer Capture Width"]
    #[inline(always)]
    pub fn epi_hb8time4_capwidth(&mut self) -> _EPI_HB8TIME4_CAPWIDTHW {
        _EPI_HB8TIME4_CAPWIDTHW { w: self }
    }
    #[doc = "Bits 24:25 - CS3n Input Ready Delay"]
    #[inline(always)]
    pub fn epi_hb8time4_irdydly(&mut self) -> _EPI_HB8TIME4_IRDYDLYW {
        _EPI_HB8TIME4_IRDYDLYW { w: self }
    }
}
