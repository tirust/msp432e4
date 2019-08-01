#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MIMR {
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
pub struct I2C_MIMR_IMR {
    bits: bool,
}
impl I2C_MIMR_IMR {
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
pub struct _I2C_MIMR_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MIMR_IMW<'a> {
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
pub struct I2C_MIMR_CLKIMR {
    bits: bool,
}
impl I2C_MIMR_CLKIMR {
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
pub struct _I2C_MIMR_CLKIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MIMR_CLKIMW<'a> {
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
pub struct I2C_MIMR_DMARXIMR {
    bits: bool,
}
impl I2C_MIMR_DMARXIMR {
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
pub struct _I2C_MIMR_DMARXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MIMR_DMARXIMW<'a> {
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
pub struct I2C_MIMR_DMATXIMR {
    bits: bool,
}
impl I2C_MIMR_DMATXIMR {
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
pub struct _I2C_MIMR_DMATXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MIMR_DMATXIMW<'a> {
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
pub struct I2C_MIMR_NACKIMR {
    bits: bool,
}
impl I2C_MIMR_NACKIMR {
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
pub struct _I2C_MIMR_NACKIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MIMR_NACKIMW<'a> {
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
pub struct I2C_MIMR_STARTIMR {
    bits: bool,
}
impl I2C_MIMR_STARTIMR {
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
pub struct _I2C_MIMR_STARTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MIMR_STARTIMW<'a> {
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
pub struct I2C_MIMR_STOPIMR {
    bits: bool,
}
impl I2C_MIMR_STOPIMR {
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
pub struct _I2C_MIMR_STOPIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MIMR_STOPIMW<'a> {
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
pub struct I2C_MIMR_ARBLOSTIMR {
    bits: bool,
}
impl I2C_MIMR_ARBLOSTIMR {
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
pub struct _I2C_MIMR_ARBLOSTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MIMR_ARBLOSTIMW<'a> {
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
pub struct I2C_MIMR_TXIMR {
    bits: bool,
}
impl I2C_MIMR_TXIMR {
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
pub struct _I2C_MIMR_TXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MIMR_TXIMW<'a> {
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
#[doc = r"Value of the field"]
pub struct I2C_MIMR_RXIMR {
    bits: bool,
}
impl I2C_MIMR_RXIMR {
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
pub struct _I2C_MIMR_RXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MIMR_RXIMW<'a> {
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
        self.w.bits &= !(1 << 9);
        self.w.bits |= ((value as u32) & 1) << 9;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct I2C_MIMR_TXFEIMR {
    bits: bool,
}
impl I2C_MIMR_TXFEIMR {
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
pub struct _I2C_MIMR_TXFEIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MIMR_TXFEIMW<'a> {
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
        self.w.bits &= !(1 << 10);
        self.w.bits |= ((value as u32) & 1) << 10;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct I2C_MIMR_RXFFIMR {
    bits: bool,
}
impl I2C_MIMR_RXFFIMR {
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
pub struct _I2C_MIMR_RXFFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MIMR_RXFFIMW<'a> {
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
        self.w.bits &= !(1 << 11);
        self.w.bits |= ((value as u32) & 1) << 11;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Master Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_im(&self) -> I2C_MIMR_IMR {
        let bits = ((self.bits >> 0) & 1) != 0;
        I2C_MIMR_IMR { bits }
    }
    #[doc = "Bit 1 - Clock Timeout Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_clkim(&self) -> I2C_MIMR_CLKIMR {
        let bits = ((self.bits >> 1) & 1) != 0;
        I2C_MIMR_CLKIMR { bits }
    }
    #[doc = "Bit 2 - Receive DMA Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_dmarxim(&self) -> I2C_MIMR_DMARXIMR {
        let bits = ((self.bits >> 2) & 1) != 0;
        I2C_MIMR_DMARXIMR { bits }
    }
    #[doc = "Bit 3 - Transmit DMA Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_dmatxim(&self) -> I2C_MIMR_DMATXIMR {
        let bits = ((self.bits >> 3) & 1) != 0;
        I2C_MIMR_DMATXIMR { bits }
    }
    #[doc = "Bit 4 - Address/Data NACK Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_nackim(&self) -> I2C_MIMR_NACKIMR {
        let bits = ((self.bits >> 4) & 1) != 0;
        I2C_MIMR_NACKIMR { bits }
    }
    #[doc = "Bit 5 - START Detection Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_startim(&self) -> I2C_MIMR_STARTIMR {
        let bits = ((self.bits >> 5) & 1) != 0;
        I2C_MIMR_STARTIMR { bits }
    }
    #[doc = "Bit 6 - STOP Detection Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_stopim(&self) -> I2C_MIMR_STOPIMR {
        let bits = ((self.bits >> 6) & 1) != 0;
        I2C_MIMR_STOPIMR { bits }
    }
    #[doc = "Bit 7 - Arbitration Lost Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_arblostim(&self) -> I2C_MIMR_ARBLOSTIMR {
        let bits = ((self.bits >> 7) & 1) != 0;
        I2C_MIMR_ARBLOSTIMR { bits }
    }
    #[doc = "Bit 8 - Transmit FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_txim(&self) -> I2C_MIMR_TXIMR {
        let bits = ((self.bits >> 8) & 1) != 0;
        I2C_MIMR_TXIMR { bits }
    }
    #[doc = "Bit 9 - Receive FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_rxim(&self) -> I2C_MIMR_RXIMR {
        let bits = ((self.bits >> 9) & 1) != 0;
        I2C_MIMR_RXIMR { bits }
    }
    #[doc = "Bit 10 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_txfeim(&self) -> I2C_MIMR_TXFEIMR {
        let bits = ((self.bits >> 10) & 1) != 0;
        I2C_MIMR_TXFEIMR { bits }
    }
    #[doc = "Bit 11 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_rxffim(&self) -> I2C_MIMR_RXFFIMR {
        let bits = ((self.bits >> 11) & 1) != 0;
        I2C_MIMR_RXFFIMR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Master Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_im(&mut self) -> _I2C_MIMR_IMW {
        _I2C_MIMR_IMW { w: self }
    }
    #[doc = "Bit 1 - Clock Timeout Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_clkim(&mut self) -> _I2C_MIMR_CLKIMW {
        _I2C_MIMR_CLKIMW { w: self }
    }
    #[doc = "Bit 2 - Receive DMA Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_dmarxim(&mut self) -> _I2C_MIMR_DMARXIMW {
        _I2C_MIMR_DMARXIMW { w: self }
    }
    #[doc = "Bit 3 - Transmit DMA Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_dmatxim(&mut self) -> _I2C_MIMR_DMATXIMW {
        _I2C_MIMR_DMATXIMW { w: self }
    }
    #[doc = "Bit 4 - Address/Data NACK Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_nackim(&mut self) -> _I2C_MIMR_NACKIMW {
        _I2C_MIMR_NACKIMW { w: self }
    }
    #[doc = "Bit 5 - START Detection Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_startim(&mut self) -> _I2C_MIMR_STARTIMW {
        _I2C_MIMR_STARTIMW { w: self }
    }
    #[doc = "Bit 6 - STOP Detection Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_stopim(&mut self) -> _I2C_MIMR_STOPIMW {
        _I2C_MIMR_STOPIMW { w: self }
    }
    #[doc = "Bit 7 - Arbitration Lost Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_arblostim(&mut self) -> _I2C_MIMR_ARBLOSTIMW {
        _I2C_MIMR_ARBLOSTIMW { w: self }
    }
    #[doc = "Bit 8 - Transmit FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_txim(&mut self) -> _I2C_MIMR_TXIMW {
        _I2C_MIMR_TXIMW { w: self }
    }
    #[doc = "Bit 9 - Receive FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_rxim(&mut self) -> _I2C_MIMR_RXIMW {
        _I2C_MIMR_RXIMW { w: self }
    }
    #[doc = "Bit 10 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_txfeim(&mut self) -> _I2C_MIMR_TXFEIMW {
        _I2C_MIMR_TXFEIMW { w: self }
    }
    #[doc = "Bit 11 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_rxffim(&mut self) -> _I2C_MIMR_RXFFIMW {
        _I2C_MIMR_RXFFIMW { w: self }
    }
}
