#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SIMR {
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
pub struct I2C_SIMR_DATAIMR {
    bits: bool,
}
impl I2C_SIMR_DATAIMR {
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
pub struct _I2C_SIMR_DATAIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SIMR_DATAIMW<'a> {
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
pub struct I2C_SIMR_STARTIMR {
    bits: bool,
}
impl I2C_SIMR_STARTIMR {
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
pub struct _I2C_SIMR_STARTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SIMR_STARTIMW<'a> {
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
pub struct I2C_SIMR_STOPIMR {
    bits: bool,
}
impl I2C_SIMR_STOPIMR {
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
pub struct _I2C_SIMR_STOPIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SIMR_STOPIMW<'a> {
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
pub struct I2C_SIMR_DMARXIMR {
    bits: bool,
}
impl I2C_SIMR_DMARXIMR {
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
pub struct _I2C_SIMR_DMARXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SIMR_DMARXIMW<'a> {
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
pub struct I2C_SIMR_DMATXIMR {
    bits: bool,
}
impl I2C_SIMR_DMATXIMR {
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
pub struct _I2C_SIMR_DMATXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SIMR_DMATXIMW<'a> {
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
pub struct I2C_SIMR_TXIMR {
    bits: bool,
}
impl I2C_SIMR_TXIMR {
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
pub struct _I2C_SIMR_TXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SIMR_TXIMW<'a> {
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
pub struct I2C_SIMR_RXIMR {
    bits: bool,
}
impl I2C_SIMR_RXIMR {
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
pub struct _I2C_SIMR_RXIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SIMR_RXIMW<'a> {
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
pub struct I2C_SIMR_TXFEIMR {
    bits: bool,
}
impl I2C_SIMR_TXFEIMR {
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
pub struct _I2C_SIMR_TXFEIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SIMR_TXFEIMW<'a> {
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
pub struct I2C_SIMR_RXFFIMR {
    bits: bool,
}
impl I2C_SIMR_RXFFIMR {
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
pub struct _I2C_SIMR_RXFFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SIMR_RXFFIMW<'a> {
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
    #[doc = "Bit 0 - Data Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_dataim(&self) -> I2C_SIMR_DATAIMR {
        let bits = ((self.bits >> 0) & 1) != 0;
        I2C_SIMR_DATAIMR { bits }
    }
    #[doc = "Bit 1 - Start Condition Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_startim(&self) -> I2C_SIMR_STARTIMR {
        let bits = ((self.bits >> 1) & 1) != 0;
        I2C_SIMR_STARTIMR { bits }
    }
    #[doc = "Bit 2 - Stop Condition Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_stopim(&self) -> I2C_SIMR_STOPIMR {
        let bits = ((self.bits >> 2) & 1) != 0;
        I2C_SIMR_STOPIMR { bits }
    }
    #[doc = "Bit 3 - Receive DMA Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_dmarxim(&self) -> I2C_SIMR_DMARXIMR {
        let bits = ((self.bits >> 3) & 1) != 0;
        I2C_SIMR_DMARXIMR { bits }
    }
    #[doc = "Bit 4 - Transmit DMA Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_dmatxim(&self) -> I2C_SIMR_DMATXIMR {
        let bits = ((self.bits >> 4) & 1) != 0;
        I2C_SIMR_DMATXIMR { bits }
    }
    #[doc = "Bit 5 - Transmit FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_txim(&self) -> I2C_SIMR_TXIMR {
        let bits = ((self.bits >> 5) & 1) != 0;
        I2C_SIMR_TXIMR { bits }
    }
    #[doc = "Bit 6 - Receive FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_rxim(&self) -> I2C_SIMR_RXIMR {
        let bits = ((self.bits >> 6) & 1) != 0;
        I2C_SIMR_RXIMR { bits }
    }
    #[doc = "Bit 7 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_txfeim(&self) -> I2C_SIMR_TXFEIMR {
        let bits = ((self.bits >> 7) & 1) != 0;
        I2C_SIMR_TXFEIMR { bits }
    }
    #[doc = "Bit 8 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_rxffim(&self) -> I2C_SIMR_RXFFIMR {
        let bits = ((self.bits >> 8) & 1) != 0;
        I2C_SIMR_RXFFIMR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Data Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_dataim(&mut self) -> _I2C_SIMR_DATAIMW {
        _I2C_SIMR_DATAIMW { w: self }
    }
    #[doc = "Bit 1 - Start Condition Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_startim(&mut self) -> _I2C_SIMR_STARTIMW {
        _I2C_SIMR_STARTIMW { w: self }
    }
    #[doc = "Bit 2 - Stop Condition Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_stopim(&mut self) -> _I2C_SIMR_STOPIMW {
        _I2C_SIMR_STOPIMW { w: self }
    }
    #[doc = "Bit 3 - Receive DMA Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_dmarxim(&mut self) -> _I2C_SIMR_DMARXIMW {
        _I2C_SIMR_DMARXIMW { w: self }
    }
    #[doc = "Bit 4 - Transmit DMA Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_dmatxim(&mut self) -> _I2C_SIMR_DMATXIMW {
        _I2C_SIMR_DMATXIMW { w: self }
    }
    #[doc = "Bit 5 - Transmit FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_txim(&mut self) -> _I2C_SIMR_TXIMW {
        _I2C_SIMR_TXIMW { w: self }
    }
    #[doc = "Bit 6 - Receive FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_rxim(&mut self) -> _I2C_SIMR_RXIMW {
        _I2C_SIMR_RXIMW { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_txfeim(&mut self) -> _I2C_SIMR_TXFEIMW {
        _I2C_SIMR_TXFEIMW { w: self }
    }
    #[doc = "Bit 8 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_rxffim(&mut self) -> _I2C_SIMR_RXFFIMW {
        _I2C_SIMR_RXFFIMW { w: self }
    }
}
