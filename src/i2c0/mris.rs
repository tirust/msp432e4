#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MRIS {
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
pub struct I2C_MRIS_RISR {
    bits: bool,
}
impl I2C_MRIS_RISR {
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
pub struct _I2C_MRIS_RISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MRIS_RISW<'a> {
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
pub struct I2C_MRIS_CLKRISR {
    bits: bool,
}
impl I2C_MRIS_CLKRISR {
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
pub struct _I2C_MRIS_CLKRISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MRIS_CLKRISW<'a> {
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
pub struct I2C_MRIS_DMARXRISR {
    bits: bool,
}
impl I2C_MRIS_DMARXRISR {
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
pub struct _I2C_MRIS_DMARXRISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MRIS_DMARXRISW<'a> {
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
pub struct I2C_MRIS_DMATXRISR {
    bits: bool,
}
impl I2C_MRIS_DMATXRISR {
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
pub struct _I2C_MRIS_DMATXRISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MRIS_DMATXRISW<'a> {
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
pub struct I2C_MRIS_NACKRISR {
    bits: bool,
}
impl I2C_MRIS_NACKRISR {
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
pub struct _I2C_MRIS_NACKRISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MRIS_NACKRISW<'a> {
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
pub struct I2C_MRIS_STARTRISR {
    bits: bool,
}
impl I2C_MRIS_STARTRISR {
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
pub struct _I2C_MRIS_STARTRISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MRIS_STARTRISW<'a> {
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
pub struct I2C_MRIS_STOPRISR {
    bits: bool,
}
impl I2C_MRIS_STOPRISR {
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
pub struct _I2C_MRIS_STOPRISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MRIS_STOPRISW<'a> {
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
pub struct I2C_MRIS_ARBLOSTRISR {
    bits: bool,
}
impl I2C_MRIS_ARBLOSTRISR {
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
pub struct _I2C_MRIS_ARBLOSTRISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MRIS_ARBLOSTRISW<'a> {
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
pub struct I2C_MRIS_TXRISR {
    bits: bool,
}
impl I2C_MRIS_TXRISR {
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
pub struct _I2C_MRIS_TXRISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MRIS_TXRISW<'a> {
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
pub struct I2C_MRIS_RXRISR {
    bits: bool,
}
impl I2C_MRIS_RXRISR {
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
pub struct _I2C_MRIS_RXRISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MRIS_RXRISW<'a> {
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
pub struct I2C_MRIS_TXFERISR {
    bits: bool,
}
impl I2C_MRIS_TXFERISR {
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
pub struct _I2C_MRIS_TXFERISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MRIS_TXFERISW<'a> {
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
pub struct I2C_MRIS_RXFFRISR {
    bits: bool,
}
impl I2C_MRIS_RXFFRISR {
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
pub struct _I2C_MRIS_RXFFRISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MRIS_RXFFRISW<'a> {
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
    #[doc = "Bit 0 - Master Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_ris(&self) -> I2C_MRIS_RISR {
        let bits = ((self.bits >> 0) & 1) != 0;
        I2C_MRIS_RISR { bits }
    }
    #[doc = "Bit 1 - Clock Timeout Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_clkris(&self) -> I2C_MRIS_CLKRISR {
        let bits = ((self.bits >> 1) & 1) != 0;
        I2C_MRIS_CLKRISR { bits }
    }
    #[doc = "Bit 2 - Receive DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_dmarxris(&self) -> I2C_MRIS_DMARXRISR {
        let bits = ((self.bits >> 2) & 1) != 0;
        I2C_MRIS_DMARXRISR { bits }
    }
    #[doc = "Bit 3 - Transmit DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_dmatxris(&self) -> I2C_MRIS_DMATXRISR {
        let bits = ((self.bits >> 3) & 1) != 0;
        I2C_MRIS_DMATXRISR { bits }
    }
    #[doc = "Bit 4 - Address/Data NACK Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_nackris(&self) -> I2C_MRIS_NACKRISR {
        let bits = ((self.bits >> 4) & 1) != 0;
        I2C_MRIS_NACKRISR { bits }
    }
    #[doc = "Bit 5 - START Detection Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_startris(&self) -> I2C_MRIS_STARTRISR {
        let bits = ((self.bits >> 5) & 1) != 0;
        I2C_MRIS_STARTRISR { bits }
    }
    #[doc = "Bit 6 - STOP Detection Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_stopris(&self) -> I2C_MRIS_STOPRISR {
        let bits = ((self.bits >> 6) & 1) != 0;
        I2C_MRIS_STOPRISR { bits }
    }
    #[doc = "Bit 7 - Arbitration Lost Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_arblostris(&self) -> I2C_MRIS_ARBLOSTRISR {
        let bits = ((self.bits >> 7) & 1) != 0;
        I2C_MRIS_ARBLOSTRISR { bits }
    }
    #[doc = "Bit 8 - Transmit Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_txris(&self) -> I2C_MRIS_TXRISR {
        let bits = ((self.bits >> 8) & 1) != 0;
        I2C_MRIS_TXRISR { bits }
    }
    #[doc = "Bit 9 - Receive FIFO Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_rxris(&self) -> I2C_MRIS_RXRISR {
        let bits = ((self.bits >> 9) & 1) != 0;
        I2C_MRIS_RXRISR { bits }
    }
    #[doc = "Bit 10 - Transmit FIFO Empty Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_txferis(&self) -> I2C_MRIS_TXFERISR {
        let bits = ((self.bits >> 10) & 1) != 0;
        I2C_MRIS_TXFERISR { bits }
    }
    #[doc = "Bit 11 - Receive FIFO Full Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_rxffris(&self) -> I2C_MRIS_RXFFRISR {
        let bits = ((self.bits >> 11) & 1) != 0;
        I2C_MRIS_RXFFRISR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Master Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_ris(&mut self) -> _I2C_MRIS_RISW {
        _I2C_MRIS_RISW { w: self }
    }
    #[doc = "Bit 1 - Clock Timeout Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_clkris(&mut self) -> _I2C_MRIS_CLKRISW {
        _I2C_MRIS_CLKRISW { w: self }
    }
    #[doc = "Bit 2 - Receive DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_dmarxris(&mut self) -> _I2C_MRIS_DMARXRISW {
        _I2C_MRIS_DMARXRISW { w: self }
    }
    #[doc = "Bit 3 - Transmit DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_dmatxris(&mut self) -> _I2C_MRIS_DMATXRISW {
        _I2C_MRIS_DMATXRISW { w: self }
    }
    #[doc = "Bit 4 - Address/Data NACK Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_nackris(&mut self) -> _I2C_MRIS_NACKRISW {
        _I2C_MRIS_NACKRISW { w: self }
    }
    #[doc = "Bit 5 - START Detection Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_startris(&mut self) -> _I2C_MRIS_STARTRISW {
        _I2C_MRIS_STARTRISW { w: self }
    }
    #[doc = "Bit 6 - STOP Detection Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_stopris(&mut self) -> _I2C_MRIS_STOPRISW {
        _I2C_MRIS_STOPRISW { w: self }
    }
    #[doc = "Bit 7 - Arbitration Lost Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_arblostris(&mut self) -> _I2C_MRIS_ARBLOSTRISW {
        _I2C_MRIS_ARBLOSTRISW { w: self }
    }
    #[doc = "Bit 8 - Transmit Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_txris(&mut self) -> _I2C_MRIS_TXRISW {
        _I2C_MRIS_TXRISW { w: self }
    }
    #[doc = "Bit 9 - Receive FIFO Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_rxris(&mut self) -> _I2C_MRIS_RXRISW {
        _I2C_MRIS_RXRISW { w: self }
    }
    #[doc = "Bit 10 - Transmit FIFO Empty Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_txferis(&mut self) -> _I2C_MRIS_TXFERISW {
        _I2C_MRIS_TXFERISW { w: self }
    }
    #[doc = "Bit 11 - Receive FIFO Full Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_rxffris(&mut self) -> _I2C_MRIS_RXFFRISW {
        _I2C_MRIS_RXFFRISW { w: self }
    }
}
