#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFOSTATUS {
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
pub struct I2C_FIFOSTATUS_TXFER {
    bits: bool,
}
impl I2C_FIFOSTATUS_TXFER {
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
pub struct _I2C_FIFOSTATUS_TXFEW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_FIFOSTATUS_TXFEW<'a> {
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
pub struct I2C_FIFOSTATUS_TXFFR {
    bits: bool,
}
impl I2C_FIFOSTATUS_TXFFR {
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
pub struct _I2C_FIFOSTATUS_TXFFW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_FIFOSTATUS_TXFFW<'a> {
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
pub struct I2C_FIFOSTATUS_TXBLWTRIGR {
    bits: bool,
}
impl I2C_FIFOSTATUS_TXBLWTRIGR {
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
pub struct _I2C_FIFOSTATUS_TXBLWTRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_FIFOSTATUS_TXBLWTRIGW<'a> {
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
pub struct I2C_FIFOSTATUS_RXFER {
    bits: bool,
}
impl I2C_FIFOSTATUS_RXFER {
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
pub struct _I2C_FIFOSTATUS_RXFEW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_FIFOSTATUS_RXFEW<'a> {
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
        self.w.bits &= !(1 << 16);
        self.w.bits |= ((value as u32) & 1) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct I2C_FIFOSTATUS_RXFFR {
    bits: bool,
}
impl I2C_FIFOSTATUS_RXFFR {
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
pub struct _I2C_FIFOSTATUS_RXFFW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_FIFOSTATUS_RXFFW<'a> {
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
        self.w.bits &= !(1 << 17);
        self.w.bits |= ((value as u32) & 1) << 17;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct I2C_FIFOSTATUS_RXABVTRIGR {
    bits: bool,
}
impl I2C_FIFOSTATUS_RXABVTRIGR {
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
pub struct _I2C_FIFOSTATUS_RXABVTRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_FIFOSTATUS_RXABVTRIGW<'a> {
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
        self.w.bits &= !(1 << 18);
        self.w.bits |= ((value as u32) & 1) << 18;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - TX FIFO Empty"]
    #[inline(always)]
    pub fn i2c_fifostatus_txfe(&self) -> I2C_FIFOSTATUS_TXFER {
        let bits = ((self.bits >> 0) & 1) != 0;
        I2C_FIFOSTATUS_TXFER { bits }
    }
    #[doc = "Bit 1 - TX FIFO Full"]
    #[inline(always)]
    pub fn i2c_fifostatus_txff(&self) -> I2C_FIFOSTATUS_TXFFR {
        let bits = ((self.bits >> 1) & 1) != 0;
        I2C_FIFOSTATUS_TXFFR { bits }
    }
    #[doc = "Bit 2 - TX FIFO Below Trigger Level"]
    #[inline(always)]
    pub fn i2c_fifostatus_txblwtrig(&self) -> I2C_FIFOSTATUS_TXBLWTRIGR {
        let bits = ((self.bits >> 2) & 1) != 0;
        I2C_FIFOSTATUS_TXBLWTRIGR { bits }
    }
    #[doc = "Bit 16 - RX FIFO Empty"]
    #[inline(always)]
    pub fn i2c_fifostatus_rxfe(&self) -> I2C_FIFOSTATUS_RXFER {
        let bits = ((self.bits >> 16) & 1) != 0;
        I2C_FIFOSTATUS_RXFER { bits }
    }
    #[doc = "Bit 17 - RX FIFO Full"]
    #[inline(always)]
    pub fn i2c_fifostatus_rxff(&self) -> I2C_FIFOSTATUS_RXFFR {
        let bits = ((self.bits >> 17) & 1) != 0;
        I2C_FIFOSTATUS_RXFFR { bits }
    }
    #[doc = "Bit 18 - RX FIFO Above Trigger Level"]
    #[inline(always)]
    pub fn i2c_fifostatus_rxabvtrig(&self) -> I2C_FIFOSTATUS_RXABVTRIGR {
        let bits = ((self.bits >> 18) & 1) != 0;
        I2C_FIFOSTATUS_RXABVTRIGR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - TX FIFO Empty"]
    #[inline(always)]
    pub fn i2c_fifostatus_txfe(&mut self) -> _I2C_FIFOSTATUS_TXFEW {
        _I2C_FIFOSTATUS_TXFEW { w: self }
    }
    #[doc = "Bit 1 - TX FIFO Full"]
    #[inline(always)]
    pub fn i2c_fifostatus_txff(&mut self) -> _I2C_FIFOSTATUS_TXFFW {
        _I2C_FIFOSTATUS_TXFFW { w: self }
    }
    #[doc = "Bit 2 - TX FIFO Below Trigger Level"]
    #[inline(always)]
    pub fn i2c_fifostatus_txblwtrig(&mut self) -> _I2C_FIFOSTATUS_TXBLWTRIGW {
        _I2C_FIFOSTATUS_TXBLWTRIGW { w: self }
    }
    #[doc = "Bit 16 - RX FIFO Empty"]
    #[inline(always)]
    pub fn i2c_fifostatus_rxfe(&mut self) -> _I2C_FIFOSTATUS_RXFEW {
        _I2C_FIFOSTATUS_RXFEW { w: self }
    }
    #[doc = "Bit 17 - RX FIFO Full"]
    #[inline(always)]
    pub fn i2c_fifostatus_rxff(&mut self) -> _I2C_FIFOSTATUS_RXFFW {
        _I2C_FIFOSTATUS_RXFFW { w: self }
    }
    #[doc = "Bit 18 - RX FIFO Above Trigger Level"]
    #[inline(always)]
    pub fn i2c_fifostatus_rxabvtrig(&mut self) -> _I2C_FIFOSTATUS_RXABVTRIGW {
        _I2C_FIFOSTATUS_RXABVTRIGW { w: self }
    }
}
