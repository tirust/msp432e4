#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FR {
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
pub struct UART_FR_CTSR {
    bits: bool,
}
impl UART_FR_CTSR {
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
pub struct _UART_FR_CTSW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_FR_CTSW<'a> {
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
pub struct UART_FR_DSRR {
    bits: bool,
}
impl UART_FR_DSRR {
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
pub struct _UART_FR_DSRW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_FR_DSRW<'a> {
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
#[doc = r"Value of the field"]
pub struct UART_FR_DCDR {
    bits: bool,
}
impl UART_FR_DCDR {
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
pub struct _UART_FR_DCDW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_FR_DCDW<'a> {
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
        self.w.bits &= !(1 << 2);
        self.w.bits |= ((value as u32) & 1) << 2;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UART_FR_BUSYR {
    bits: bool,
}
impl UART_FR_BUSYR {
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
pub struct _UART_FR_BUSYW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_FR_BUSYW<'a> {
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
        self.w.bits &= !(1 << 3);
        self.w.bits |= ((value as u32) & 1) << 3;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct UART_FR_RXFER {
    bits: bool,
}
impl UART_FR_RXFER {
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
pub struct _UART_FR_RXFEW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_FR_RXFEW<'a> {
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
pub struct UART_FR_TXFFR {
    bits: bool,
}
impl UART_FR_TXFFR {
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
pub struct _UART_FR_TXFFW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_FR_TXFFW<'a> {
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
pub struct UART_FR_RXFFR {
    bits: bool,
}
impl UART_FR_RXFFR {
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
pub struct _UART_FR_RXFFW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_FR_RXFFW<'a> {
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
pub struct UART_FR_TXFER {
    bits: bool,
}
impl UART_FR_TXFER {
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
pub struct _UART_FR_TXFEW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_FR_TXFEW<'a> {
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
pub struct UART_FR_RIR {
    bits: bool,
}
impl UART_FR_RIR {
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
pub struct _UART_FR_RIW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_FR_RIW<'a> {
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
    #[doc = "Bit 0 - Clear To Send"]
    #[inline(always)]
    pub fn uart_fr_cts(&self) -> UART_FR_CTSR {
        let bits = ((self.bits >> 0) & 1) != 0;
        UART_FR_CTSR { bits }
    }
    #[doc = "Bit 1 - Data Set Ready"]
    #[inline(always)]
    pub fn uart_fr_dsr(&self) -> UART_FR_DSRR {
        let bits = ((self.bits >> 1) & 1) != 0;
        UART_FR_DSRR { bits }
    }
    #[doc = "Bit 2 - Data Carrier Detect"]
    #[inline(always)]
    pub fn uart_fr_dcd(&self) -> UART_FR_DCDR {
        let bits = ((self.bits >> 2) & 1) != 0;
        UART_FR_DCDR { bits }
    }
    #[doc = "Bit 3 - UART Busy"]
    #[inline(always)]
    pub fn uart_fr_busy(&self) -> UART_FR_BUSYR {
        let bits = ((self.bits >> 3) & 1) != 0;
        UART_FR_BUSYR { bits }
    }
    #[doc = "Bit 4 - UART Receive FIFO Empty"]
    #[inline(always)]
    pub fn uart_fr_rxfe(&self) -> UART_FR_RXFER {
        let bits = ((self.bits >> 4) & 1) != 0;
        UART_FR_RXFER { bits }
    }
    #[doc = "Bit 5 - UART Transmit FIFO Full"]
    #[inline(always)]
    pub fn uart_fr_txff(&self) -> UART_FR_TXFFR {
        let bits = ((self.bits >> 5) & 1) != 0;
        UART_FR_TXFFR { bits }
    }
    #[doc = "Bit 6 - UART Receive FIFO Full"]
    #[inline(always)]
    pub fn uart_fr_rxff(&self) -> UART_FR_RXFFR {
        let bits = ((self.bits >> 6) & 1) != 0;
        UART_FR_RXFFR { bits }
    }
    #[doc = "Bit 7 - UART Transmit FIFO Empty"]
    #[inline(always)]
    pub fn uart_fr_txfe(&self) -> UART_FR_TXFER {
        let bits = ((self.bits >> 7) & 1) != 0;
        UART_FR_TXFER { bits }
    }
    #[doc = "Bit 8 - Ring Indicator"]
    #[inline(always)]
    pub fn uart_fr_ri(&self) -> UART_FR_RIR {
        let bits = ((self.bits >> 8) & 1) != 0;
        UART_FR_RIR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Clear To Send"]
    #[inline(always)]
    pub fn uart_fr_cts(&mut self) -> _UART_FR_CTSW {
        _UART_FR_CTSW { w: self }
    }
    #[doc = "Bit 1 - Data Set Ready"]
    #[inline(always)]
    pub fn uart_fr_dsr(&mut self) -> _UART_FR_DSRW {
        _UART_FR_DSRW { w: self }
    }
    #[doc = "Bit 2 - Data Carrier Detect"]
    #[inline(always)]
    pub fn uart_fr_dcd(&mut self) -> _UART_FR_DCDW {
        _UART_FR_DCDW { w: self }
    }
    #[doc = "Bit 3 - UART Busy"]
    #[inline(always)]
    pub fn uart_fr_busy(&mut self) -> _UART_FR_BUSYW {
        _UART_FR_BUSYW { w: self }
    }
    #[doc = "Bit 4 - UART Receive FIFO Empty"]
    #[inline(always)]
    pub fn uart_fr_rxfe(&mut self) -> _UART_FR_RXFEW {
        _UART_FR_RXFEW { w: self }
    }
    #[doc = "Bit 5 - UART Transmit FIFO Full"]
    #[inline(always)]
    pub fn uart_fr_txff(&mut self) -> _UART_FR_TXFFW {
        _UART_FR_TXFFW { w: self }
    }
    #[doc = "Bit 6 - UART Receive FIFO Full"]
    #[inline(always)]
    pub fn uart_fr_rxff(&mut self) -> _UART_FR_RXFFW {
        _UART_FR_RXFFW { w: self }
    }
    #[doc = "Bit 7 - UART Transmit FIFO Empty"]
    #[inline(always)]
    pub fn uart_fr_txfe(&mut self) -> _UART_FR_TXFEW {
        _UART_FR_TXFEW { w: self }
    }
    #[doc = "Bit 8 - Ring Indicator"]
    #[inline(always)]
    pub fn uart_fr_ri(&mut self) -> _UART_FR_RIW {
        _UART_FR_RIW { w: self }
    }
}
