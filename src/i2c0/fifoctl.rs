#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFOCTL {
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
pub struct I2C_FIFOCTL_TXTRIGR {
    bits: u8,
}
impl I2C_FIFOCTL_TXTRIGR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _I2C_FIFOCTL_TXTRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_FIFOCTL_TXTRIGW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 0);
        self.w.bits |= ((value as u32) & 7) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct I2C_FIFOCTL_DMATXENAR {
    bits: bool,
}
impl I2C_FIFOCTL_DMATXENAR {
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
pub struct _I2C_FIFOCTL_DMATXENAW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_FIFOCTL_DMATXENAW<'a> {
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
        self.w.bits &= !(1 << 13);
        self.w.bits |= ((value as u32) & 1) << 13;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct I2C_FIFOCTL_TXFLUSHR {
    bits: bool,
}
impl I2C_FIFOCTL_TXFLUSHR {
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
pub struct _I2C_FIFOCTL_TXFLUSHW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_FIFOCTL_TXFLUSHW<'a> {
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
        self.w.bits &= !(1 << 14);
        self.w.bits |= ((value as u32) & 1) << 14;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct I2C_FIFOCTL_TXASGNMTR {
    bits: bool,
}
impl I2C_FIFOCTL_TXASGNMTR {
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
pub struct _I2C_FIFOCTL_TXASGNMTW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_FIFOCTL_TXASGNMTW<'a> {
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
        self.w.bits &= !(1 << 15);
        self.w.bits |= ((value as u32) & 1) << 15;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct I2C_FIFOCTL_RXTRIGR {
    bits: u8,
}
impl I2C_FIFOCTL_RXTRIGR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _I2C_FIFOCTL_RXTRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_FIFOCTL_RXTRIGW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 16);
        self.w.bits |= ((value as u32) & 7) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct I2C_FIFOCTL_DMARXENAR {
    bits: bool,
}
impl I2C_FIFOCTL_DMARXENAR {
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
pub struct _I2C_FIFOCTL_DMARXENAW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_FIFOCTL_DMARXENAW<'a> {
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
        self.w.bits &= !(1 << 29);
        self.w.bits |= ((value as u32) & 1) << 29;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct I2C_FIFOCTL_RXFLUSHR {
    bits: bool,
}
impl I2C_FIFOCTL_RXFLUSHR {
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
pub struct _I2C_FIFOCTL_RXFLUSHW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_FIFOCTL_RXFLUSHW<'a> {
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
        self.w.bits &= !(1 << 30);
        self.w.bits |= ((value as u32) & 1) << 30;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct I2C_FIFOCTL_RXASGNMTR {
    bits: bool,
}
impl I2C_FIFOCTL_RXASGNMTR {
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
pub struct _I2C_FIFOCTL_RXASGNMTW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_FIFOCTL_RXASGNMTW<'a> {
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
        self.w.bits &= !(1 << 31);
        self.w.bits |= ((value as u32) & 1) << 31;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - TX FIFO Trigger"]
    #[inline(always)]
    pub fn i2c_fifoctl_txtrig(&self) -> I2C_FIFOCTL_TXTRIGR {
        let bits = ((self.bits >> 0) & 7) as u8;
        I2C_FIFOCTL_TXTRIGR { bits }
    }
    #[doc = "Bit 13 - DMA TX Channel Enable"]
    #[inline(always)]
    pub fn i2c_fifoctl_dmatxena(&self) -> I2C_FIFOCTL_DMATXENAR {
        let bits = ((self.bits >> 13) & 1) != 0;
        I2C_FIFOCTL_DMATXENAR { bits }
    }
    #[doc = "Bit 14 - TX FIFO Flush"]
    #[inline(always)]
    pub fn i2c_fifoctl_txflush(&self) -> I2C_FIFOCTL_TXFLUSHR {
        let bits = ((self.bits >> 14) & 1) != 0;
        I2C_FIFOCTL_TXFLUSHR { bits }
    }
    #[doc = "Bit 15 - TX Control Assignment"]
    #[inline(always)]
    pub fn i2c_fifoctl_txasgnmt(&self) -> I2C_FIFOCTL_TXASGNMTR {
        let bits = ((self.bits >> 15) & 1) != 0;
        I2C_FIFOCTL_TXASGNMTR { bits }
    }
    #[doc = "Bits 16:18 - RX FIFO Trigger"]
    #[inline(always)]
    pub fn i2c_fifoctl_rxtrig(&self) -> I2C_FIFOCTL_RXTRIGR {
        let bits = ((self.bits >> 16) & 7) as u8;
        I2C_FIFOCTL_RXTRIGR { bits }
    }
    #[doc = "Bit 29 - DMA RX Channel Enable"]
    #[inline(always)]
    pub fn i2c_fifoctl_dmarxena(&self) -> I2C_FIFOCTL_DMARXENAR {
        let bits = ((self.bits >> 29) & 1) != 0;
        I2C_FIFOCTL_DMARXENAR { bits }
    }
    #[doc = "Bit 30 - RX FIFO Flush"]
    #[inline(always)]
    pub fn i2c_fifoctl_rxflush(&self) -> I2C_FIFOCTL_RXFLUSHR {
        let bits = ((self.bits >> 30) & 1) != 0;
        I2C_FIFOCTL_RXFLUSHR { bits }
    }
    #[doc = "Bit 31 - RX Control Assignment"]
    #[inline(always)]
    pub fn i2c_fifoctl_rxasgnmt(&self) -> I2C_FIFOCTL_RXASGNMTR {
        let bits = ((self.bits >> 31) & 1) != 0;
        I2C_FIFOCTL_RXASGNMTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - TX FIFO Trigger"]
    #[inline(always)]
    pub fn i2c_fifoctl_txtrig(&mut self) -> _I2C_FIFOCTL_TXTRIGW {
        _I2C_FIFOCTL_TXTRIGW { w: self }
    }
    #[doc = "Bit 13 - DMA TX Channel Enable"]
    #[inline(always)]
    pub fn i2c_fifoctl_dmatxena(&mut self) -> _I2C_FIFOCTL_DMATXENAW {
        _I2C_FIFOCTL_DMATXENAW { w: self }
    }
    #[doc = "Bit 14 - TX FIFO Flush"]
    #[inline(always)]
    pub fn i2c_fifoctl_txflush(&mut self) -> _I2C_FIFOCTL_TXFLUSHW {
        _I2C_FIFOCTL_TXFLUSHW { w: self }
    }
    #[doc = "Bit 15 - TX Control Assignment"]
    #[inline(always)]
    pub fn i2c_fifoctl_txasgnmt(&mut self) -> _I2C_FIFOCTL_TXASGNMTW {
        _I2C_FIFOCTL_TXASGNMTW { w: self }
    }
    #[doc = "Bits 16:18 - RX FIFO Trigger"]
    #[inline(always)]
    pub fn i2c_fifoctl_rxtrig(&mut self) -> _I2C_FIFOCTL_RXTRIGW {
        _I2C_FIFOCTL_RXTRIGW { w: self }
    }
    #[doc = "Bit 29 - DMA RX Channel Enable"]
    #[inline(always)]
    pub fn i2c_fifoctl_dmarxena(&mut self) -> _I2C_FIFOCTL_DMARXENAW {
        _I2C_FIFOCTL_DMARXENAW { w: self }
    }
    #[doc = "Bit 30 - RX FIFO Flush"]
    #[inline(always)]
    pub fn i2c_fifoctl_rxflush(&mut self) -> _I2C_FIFOCTL_RXFLUSHW {
        _I2C_FIFOCTL_RXFLUSHW { w: self }
    }
    #[doc = "Bit 31 - RX Control Assignment"]
    #[inline(always)]
    pub fn i2c_fifoctl_rxasgnmt(&mut self) -> _I2C_FIFOCTL_RXASGNMTW {
        _I2C_FIFOCTL_RXASGNMTW { w: self }
    }
}
