#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAT {
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
pub struct EPI_STAT_ACTIVER {
    bits: bool,
}
impl EPI_STAT_ACTIVER {
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
pub struct _EPI_STAT_ACTIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_STAT_ACTIVEW<'a> {
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
pub struct EPI_STAT_NBRBUSYR {
    bits: bool,
}
impl EPI_STAT_NBRBUSYR {
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
pub struct _EPI_STAT_NBRBUSYW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_STAT_NBRBUSYW<'a> {
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
pub struct EPI_STAT_WBUSYR {
    bits: bool,
}
impl EPI_STAT_WBUSYR {
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
pub struct _EPI_STAT_WBUSYW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_STAT_WBUSYW<'a> {
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
        self.w.bits &= !(1 << 5);
        self.w.bits |= ((value as u32) & 1) << 5;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_STAT_INITSEQR {
    bits: bool,
}
impl EPI_STAT_INITSEQR {
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
pub struct _EPI_STAT_INITSEQW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_STAT_INITSEQW<'a> {
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
        self.w.bits &= !(1 << 6);
        self.w.bits |= ((value as u32) & 1) << 6;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_STAT_XFEMPTYR {
    bits: bool,
}
impl EPI_STAT_XFEMPTYR {
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
pub struct _EPI_STAT_XFEMPTYW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_STAT_XFEMPTYW<'a> {
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
        self.w.bits &= !(1 << 7);
        self.w.bits |= ((value as u32) & 1) << 7;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_STAT_XFFULLR {
    bits: bool,
}
impl EPI_STAT_XFFULLR {
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
pub struct _EPI_STAT_XFFULLW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_STAT_XFFULLW<'a> {
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
        self.w.bits &= !(1 << 8);
        self.w.bits |= ((value as u32) & 1) << 8;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Register Active"]
    #[inline(always)]
    pub fn epi_stat_active(&self) -> EPI_STAT_ACTIVER {
        let bits = ((self.bits >> 0) & 1) != 0;
        EPI_STAT_ACTIVER { bits }
    }
    #[doc = "Bit 4 - Non-Blocking Read Busy"]
    #[inline(always)]
    pub fn epi_stat_nbrbusy(&self) -> EPI_STAT_NBRBUSYR {
        let bits = ((self.bits >> 4) & 1) != 0;
        EPI_STAT_NBRBUSYR { bits }
    }
    #[doc = "Bit 5 - Write Busy"]
    #[inline(always)]
    pub fn epi_stat_wbusy(&self) -> EPI_STAT_WBUSYR {
        let bits = ((self.bits >> 5) & 1) != 0;
        EPI_STAT_WBUSYR { bits }
    }
    #[doc = "Bit 6 - Initialization Sequence"]
    #[inline(always)]
    pub fn epi_stat_initseq(&self) -> EPI_STAT_INITSEQR {
        let bits = ((self.bits >> 6) & 1) != 0;
        EPI_STAT_INITSEQR { bits }
    }
    #[doc = "Bit 7 - External FIFO Empty"]
    #[inline(always)]
    pub fn epi_stat_xfempty(&self) -> EPI_STAT_XFEMPTYR {
        let bits = ((self.bits >> 7) & 1) != 0;
        EPI_STAT_XFEMPTYR { bits }
    }
    #[doc = "Bit 8 - External FIFO Full"]
    #[inline(always)]
    pub fn epi_stat_xffull(&self) -> EPI_STAT_XFFULLR {
        let bits = ((self.bits >> 8) & 1) != 0;
        EPI_STAT_XFFULLR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Register Active"]
    #[inline(always)]
    pub fn epi_stat_active(&mut self) -> _EPI_STAT_ACTIVEW {
        _EPI_STAT_ACTIVEW { w: self }
    }
    #[doc = "Bit 4 - Non-Blocking Read Busy"]
    #[inline(always)]
    pub fn epi_stat_nbrbusy(&mut self) -> _EPI_STAT_NBRBUSYW {
        _EPI_STAT_NBRBUSYW { w: self }
    }
    #[doc = "Bit 5 - Write Busy"]
    #[inline(always)]
    pub fn epi_stat_wbusy(&mut self) -> _EPI_STAT_WBUSYW {
        _EPI_STAT_WBUSYW { w: self }
    }
    #[doc = "Bit 6 - Initialization Sequence"]
    #[inline(always)]
    pub fn epi_stat_initseq(&mut self) -> _EPI_STAT_INITSEQW {
        _EPI_STAT_INITSEQW { w: self }
    }
    #[doc = "Bit 7 - External FIFO Empty"]
    #[inline(always)]
    pub fn epi_stat_xfempty(&mut self) -> _EPI_STAT_XFEMPTYW {
        _EPI_STAT_XFEMPTYW { w: self }
    }
    #[doc = "Bit 8 - External FIFO Full"]
    #[inline(always)]
    pub fn epi_stat_xffull(&mut self) -> _EPI_STAT_XFFULLW {
        _EPI_STAT_XFFULLW { w: self }
    }
}
