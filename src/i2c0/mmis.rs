#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MMIS {
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
pub struct I2C_MMIS_MISR {
    bits: bool,
}
impl I2C_MMIS_MISR {
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
pub struct _I2C_MMIS_MISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MMIS_MISW<'a> {
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
pub struct I2C_MMIS_CLKMISR {
    bits: bool,
}
impl I2C_MMIS_CLKMISR {
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
pub struct _I2C_MMIS_CLKMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MMIS_CLKMISW<'a> {
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
pub struct I2C_MMIS_DMARXMISR {
    bits: bool,
}
impl I2C_MMIS_DMARXMISR {
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
pub struct _I2C_MMIS_DMARXMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MMIS_DMARXMISW<'a> {
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
pub struct I2C_MMIS_DMATXMISR {
    bits: bool,
}
impl I2C_MMIS_DMATXMISR {
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
pub struct _I2C_MMIS_DMATXMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MMIS_DMATXMISW<'a> {
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
pub struct I2C_MMIS_NACKMISR {
    bits: bool,
}
impl I2C_MMIS_NACKMISR {
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
pub struct _I2C_MMIS_NACKMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MMIS_NACKMISW<'a> {
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
pub struct I2C_MMIS_STARTMISR {
    bits: bool,
}
impl I2C_MMIS_STARTMISR {
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
pub struct _I2C_MMIS_STARTMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MMIS_STARTMISW<'a> {
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
pub struct I2C_MMIS_STOPMISR {
    bits: bool,
}
impl I2C_MMIS_STOPMISR {
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
pub struct _I2C_MMIS_STOPMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MMIS_STOPMISW<'a> {
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
pub struct I2C_MMIS_ARBLOSTMISR {
    bits: bool,
}
impl I2C_MMIS_ARBLOSTMISR {
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
pub struct _I2C_MMIS_ARBLOSTMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MMIS_ARBLOSTMISW<'a> {
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
pub struct I2C_MMIS_TXMISR {
    bits: bool,
}
impl I2C_MMIS_TXMISR {
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
pub struct _I2C_MMIS_TXMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MMIS_TXMISW<'a> {
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
pub struct I2C_MMIS_RXMISR {
    bits: bool,
}
impl I2C_MMIS_RXMISR {
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
pub struct _I2C_MMIS_RXMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MMIS_RXMISW<'a> {
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
pub struct I2C_MMIS_TXFEMISR {
    bits: bool,
}
impl I2C_MMIS_TXFEMISR {
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
pub struct _I2C_MMIS_TXFEMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MMIS_TXFEMISW<'a> {
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
pub struct I2C_MMIS_RXFFMISR {
    bits: bool,
}
impl I2C_MMIS_RXFFMISR {
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
pub struct _I2C_MMIS_RXFFMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MMIS_RXFFMISW<'a> {
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
    #[doc = "Bit 0 - Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mmis_mis(&self) -> I2C_MMIS_MISR {
        let bits = ((self.bits >> 0) & 1) != 0;
        I2C_MMIS_MISR { bits }
    }
    #[doc = "Bit 1 - Clock Timeout Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mmis_clkmis(&self) -> I2C_MMIS_CLKMISR {
        let bits = ((self.bits >> 1) & 1) != 0;
        I2C_MMIS_CLKMISR { bits }
    }
    #[doc = "Bit 2 - Receive DMA Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mmis_dmarxmis(&self) -> I2C_MMIS_DMARXMISR {
        let bits = ((self.bits >> 2) & 1) != 0;
        I2C_MMIS_DMARXMISR { bits }
    }
    #[doc = "Bit 3 - Transmit DMA Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mmis_dmatxmis(&self) -> I2C_MMIS_DMATXMISR {
        let bits = ((self.bits >> 3) & 1) != 0;
        I2C_MMIS_DMATXMISR { bits }
    }
    #[doc = "Bit 4 - Address/Data NACK Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_nackmis(&self) -> I2C_MMIS_NACKMISR {
        let bits = ((self.bits >> 4) & 1) != 0;
        I2C_MMIS_NACKMISR { bits }
    }
    #[doc = "Bit 5 - START Detection Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_startmis(&self) -> I2C_MMIS_STARTMISR {
        let bits = ((self.bits >> 5) & 1) != 0;
        I2C_MMIS_STARTMISR { bits }
    }
    #[doc = "Bit 6 - STOP Detection Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_stopmis(&self) -> I2C_MMIS_STOPMISR {
        let bits = ((self.bits >> 6) & 1) != 0;
        I2C_MMIS_STOPMISR { bits }
    }
    #[doc = "Bit 7 - Arbitration Lost Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_arblostmis(&self) -> I2C_MMIS_ARBLOSTMISR {
        let bits = ((self.bits >> 7) & 1) != 0;
        I2C_MMIS_ARBLOSTMISR { bits }
    }
    #[doc = "Bit 8 - Transmit Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_txmis(&self) -> I2C_MMIS_TXMISR {
        let bits = ((self.bits >> 8) & 1) != 0;
        I2C_MMIS_TXMISR { bits }
    }
    #[doc = "Bit 9 - Receive FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_rxmis(&self) -> I2C_MMIS_RXMISR {
        let bits = ((self.bits >> 9) & 1) != 0;
        I2C_MMIS_RXMISR { bits }
    }
    #[doc = "Bit 10 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_txfemis(&self) -> I2C_MMIS_TXFEMISR {
        let bits = ((self.bits >> 10) & 1) != 0;
        I2C_MMIS_TXFEMISR { bits }
    }
    #[doc = "Bit 11 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_rxffmis(&self) -> I2C_MMIS_RXFFMISR {
        let bits = ((self.bits >> 11) & 1) != 0;
        I2C_MMIS_RXFFMISR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mmis_mis(&mut self) -> _I2C_MMIS_MISW {
        _I2C_MMIS_MISW { w: self }
    }
    #[doc = "Bit 1 - Clock Timeout Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mmis_clkmis(&mut self) -> _I2C_MMIS_CLKMISW {
        _I2C_MMIS_CLKMISW { w: self }
    }
    #[doc = "Bit 2 - Receive DMA Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mmis_dmarxmis(&mut self) -> _I2C_MMIS_DMARXMISW {
        _I2C_MMIS_DMARXMISW { w: self }
    }
    #[doc = "Bit 3 - Transmit DMA Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mmis_dmatxmis(&mut self) -> _I2C_MMIS_DMATXMISW {
        _I2C_MMIS_DMATXMISW { w: self }
    }
    #[doc = "Bit 4 - Address/Data NACK Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_nackmis(&mut self) -> _I2C_MMIS_NACKMISW {
        _I2C_MMIS_NACKMISW { w: self }
    }
    #[doc = "Bit 5 - START Detection Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_startmis(&mut self) -> _I2C_MMIS_STARTMISW {
        _I2C_MMIS_STARTMISW { w: self }
    }
    #[doc = "Bit 6 - STOP Detection Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_stopmis(&mut self) -> _I2C_MMIS_STOPMISW {
        _I2C_MMIS_STOPMISW { w: self }
    }
    #[doc = "Bit 7 - Arbitration Lost Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_arblostmis(&mut self) -> _I2C_MMIS_ARBLOSTMISW {
        _I2C_MMIS_ARBLOSTMISW { w: self }
    }
    #[doc = "Bit 8 - Transmit Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_txmis(&mut self) -> _I2C_MMIS_TXMISW {
        _I2C_MMIS_TXMISW { w: self }
    }
    #[doc = "Bit 9 - Receive FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_rxmis(&mut self) -> _I2C_MMIS_RXMISW {
        _I2C_MMIS_RXMISW { w: self }
    }
    #[doc = "Bit 10 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_txfemis(&mut self) -> _I2C_MMIS_TXFEMISW {
        _I2C_MMIS_TXFEMISW { w: self }
    }
    #[doc = "Bit 11 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_rxffmis(&mut self) -> _I2C_MMIS_RXFFMISW {
        _I2C_MMIS_RXFFMISW { w: self }
    }
}
