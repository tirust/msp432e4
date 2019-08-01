#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SRIS {
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
pub struct I2C_SRIS_DATARISR {
    bits: bool,
}
impl I2C_SRIS_DATARISR {
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
pub struct _I2C_SRIS_DATARISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SRIS_DATARISW<'a> {
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
pub struct I2C_SRIS_STARTRISR {
    bits: bool,
}
impl I2C_SRIS_STARTRISR {
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
pub struct _I2C_SRIS_STARTRISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SRIS_STARTRISW<'a> {
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
pub struct I2C_SRIS_STOPRISR {
    bits: bool,
}
impl I2C_SRIS_STOPRISR {
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
pub struct _I2C_SRIS_STOPRISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SRIS_STOPRISW<'a> {
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
pub struct I2C_SRIS_DMARXRISR {
    bits: bool,
}
impl I2C_SRIS_DMARXRISR {
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
pub struct _I2C_SRIS_DMARXRISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SRIS_DMARXRISW<'a> {
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
pub struct I2C_SRIS_DMATXRISR {
    bits: bool,
}
impl I2C_SRIS_DMATXRISR {
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
pub struct _I2C_SRIS_DMATXRISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SRIS_DMATXRISW<'a> {
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
pub struct I2C_SRIS_TXRISR {
    bits: bool,
}
impl I2C_SRIS_TXRISR {
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
pub struct _I2C_SRIS_TXRISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SRIS_TXRISW<'a> {
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
pub struct I2C_SRIS_RXRISR {
    bits: bool,
}
impl I2C_SRIS_RXRISR {
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
pub struct _I2C_SRIS_RXRISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SRIS_RXRISW<'a> {
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
pub struct I2C_SRIS_TXFERISR {
    bits: bool,
}
impl I2C_SRIS_TXFERISR {
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
pub struct _I2C_SRIS_TXFERISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SRIS_TXFERISW<'a> {
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
pub struct I2C_SRIS_RXFFRISR {
    bits: bool,
}
impl I2C_SRIS_RXFFRISR {
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
pub struct _I2C_SRIS_RXFFRISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SRIS_RXFFRISW<'a> {
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
    #[doc = "Bit 0 - Data Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_dataris(&self) -> I2C_SRIS_DATARISR {
        let bits = ((self.bits >> 0) & 1) != 0;
        I2C_SRIS_DATARISR { bits }
    }
    #[doc = "Bit 1 - Start Condition Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_startris(&self) -> I2C_SRIS_STARTRISR {
        let bits = ((self.bits >> 1) & 1) != 0;
        I2C_SRIS_STARTRISR { bits }
    }
    #[doc = "Bit 2 - Stop Condition Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_stopris(&self) -> I2C_SRIS_STOPRISR {
        let bits = ((self.bits >> 2) & 1) != 0;
        I2C_SRIS_STOPRISR { bits }
    }
    #[doc = "Bit 3 - Receive DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_dmarxris(&self) -> I2C_SRIS_DMARXRISR {
        let bits = ((self.bits >> 3) & 1) != 0;
        I2C_SRIS_DMARXRISR { bits }
    }
    #[doc = "Bit 4 - Transmit DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_dmatxris(&self) -> I2C_SRIS_DMATXRISR {
        let bits = ((self.bits >> 4) & 1) != 0;
        I2C_SRIS_DMATXRISR { bits }
    }
    #[doc = "Bit 5 - Transmit Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_txris(&self) -> I2C_SRIS_TXRISR {
        let bits = ((self.bits >> 5) & 1) != 0;
        I2C_SRIS_TXRISR { bits }
    }
    #[doc = "Bit 6 - Receive FIFO Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_rxris(&self) -> I2C_SRIS_RXRISR {
        let bits = ((self.bits >> 6) & 1) != 0;
        I2C_SRIS_RXRISR { bits }
    }
    #[doc = "Bit 7 - Transmit FIFO Empty Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_txferis(&self) -> I2C_SRIS_TXFERISR {
        let bits = ((self.bits >> 7) & 1) != 0;
        I2C_SRIS_TXFERISR { bits }
    }
    #[doc = "Bit 8 - Receive FIFO Full Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_rxffris(&self) -> I2C_SRIS_RXFFRISR {
        let bits = ((self.bits >> 8) & 1) != 0;
        I2C_SRIS_RXFFRISR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Data Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_dataris(&mut self) -> _I2C_SRIS_DATARISW {
        _I2C_SRIS_DATARISW { w: self }
    }
    #[doc = "Bit 1 - Start Condition Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_startris(&mut self) -> _I2C_SRIS_STARTRISW {
        _I2C_SRIS_STARTRISW { w: self }
    }
    #[doc = "Bit 2 - Stop Condition Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_stopris(&mut self) -> _I2C_SRIS_STOPRISW {
        _I2C_SRIS_STOPRISW { w: self }
    }
    #[doc = "Bit 3 - Receive DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_dmarxris(&mut self) -> _I2C_SRIS_DMARXRISW {
        _I2C_SRIS_DMARXRISW { w: self }
    }
    #[doc = "Bit 4 - Transmit DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_dmatxris(&mut self) -> _I2C_SRIS_DMATXRISW {
        _I2C_SRIS_DMATXRISW { w: self }
    }
    #[doc = "Bit 5 - Transmit Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_txris(&mut self) -> _I2C_SRIS_TXRISW {
        _I2C_SRIS_TXRISW { w: self }
    }
    #[doc = "Bit 6 - Receive FIFO Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_rxris(&mut self) -> _I2C_SRIS_RXRISW {
        _I2C_SRIS_RXRISW { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO Empty Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_txferis(&mut self) -> _I2C_SRIS_TXFERISW {
        _I2C_SRIS_TXFERISW { w: self }
    }
    #[doc = "Bit 8 - Receive FIFO Full Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_rxffris(&mut self) -> _I2C_SRIS_RXFFRISW {
        _I2C_SRIS_RXFFRISW { w: self }
    }
}
