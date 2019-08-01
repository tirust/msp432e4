#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SMIS {
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
pub struct I2C_SMIS_DATAMISR {
    bits: bool,
}
impl I2C_SMIS_DATAMISR {
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
pub struct _I2C_SMIS_DATAMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SMIS_DATAMISW<'a> {
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
pub struct I2C_SMIS_STARTMISR {
    bits: bool,
}
impl I2C_SMIS_STARTMISR {
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
pub struct _I2C_SMIS_STARTMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SMIS_STARTMISW<'a> {
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
pub struct I2C_SMIS_STOPMISR {
    bits: bool,
}
impl I2C_SMIS_STOPMISR {
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
pub struct _I2C_SMIS_STOPMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SMIS_STOPMISW<'a> {
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
pub struct I2C_SMIS_DMARXMISR {
    bits: bool,
}
impl I2C_SMIS_DMARXMISR {
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
pub struct _I2C_SMIS_DMARXMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SMIS_DMARXMISW<'a> {
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
pub struct I2C_SMIS_DMATXMISR {
    bits: bool,
}
impl I2C_SMIS_DMATXMISR {
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
pub struct _I2C_SMIS_DMATXMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SMIS_DMATXMISW<'a> {
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
pub struct I2C_SMIS_TXMISR {
    bits: bool,
}
impl I2C_SMIS_TXMISR {
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
pub struct _I2C_SMIS_TXMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SMIS_TXMISW<'a> {
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
pub struct I2C_SMIS_RXMISR {
    bits: bool,
}
impl I2C_SMIS_RXMISR {
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
pub struct _I2C_SMIS_RXMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SMIS_RXMISW<'a> {
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
pub struct I2C_SMIS_TXFEMISR {
    bits: bool,
}
impl I2C_SMIS_TXFEMISR {
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
pub struct _I2C_SMIS_TXFEMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SMIS_TXFEMISW<'a> {
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
pub struct I2C_SMIS_RXFFMISR {
    bits: bool,
}
impl I2C_SMIS_RXFFMISR {
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
pub struct _I2C_SMIS_RXFFMISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SMIS_RXFFMISW<'a> {
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
    #[doc = "Bit 0 - Data Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_datamis(&self) -> I2C_SMIS_DATAMISR {
        let bits = ((self.bits >> 0) & 1) != 0;
        I2C_SMIS_DATAMISR { bits }
    }
    #[doc = "Bit 1 - Start Condition Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_startmis(&self) -> I2C_SMIS_STARTMISR {
        let bits = ((self.bits >> 1) & 1) != 0;
        I2C_SMIS_STARTMISR { bits }
    }
    #[doc = "Bit 2 - Stop Condition Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_stopmis(&self) -> I2C_SMIS_STOPMISR {
        let bits = ((self.bits >> 2) & 1) != 0;
        I2C_SMIS_STOPMISR { bits }
    }
    #[doc = "Bit 3 - Receive DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_dmarxmis(&self) -> I2C_SMIS_DMARXMISR {
        let bits = ((self.bits >> 3) & 1) != 0;
        I2C_SMIS_DMARXMISR { bits }
    }
    #[doc = "Bit 4 - Transmit DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_dmatxmis(&self) -> I2C_SMIS_DMATXMISR {
        let bits = ((self.bits >> 4) & 1) != 0;
        I2C_SMIS_DMATXMISR { bits }
    }
    #[doc = "Bit 5 - Transmit FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_smis_txmis(&self) -> I2C_SMIS_TXMISR {
        let bits = ((self.bits >> 5) & 1) != 0;
        I2C_SMIS_TXMISR { bits }
    }
    #[doc = "Bit 6 - Receive FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_smis_rxmis(&self) -> I2C_SMIS_RXMISR {
        let bits = ((self.bits >> 6) & 1) != 0;
        I2C_SMIS_RXMISR { bits }
    }
    #[doc = "Bit 7 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_smis_txfemis(&self) -> I2C_SMIS_TXFEMISR {
        let bits = ((self.bits >> 7) & 1) != 0;
        I2C_SMIS_TXFEMISR { bits }
    }
    #[doc = "Bit 8 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_smis_rxffmis(&self) -> I2C_SMIS_RXFFMISR {
        let bits = ((self.bits >> 8) & 1) != 0;
        I2C_SMIS_RXFFMISR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Data Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_datamis(&mut self) -> _I2C_SMIS_DATAMISW {
        _I2C_SMIS_DATAMISW { w: self }
    }
    #[doc = "Bit 1 - Start Condition Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_startmis(&mut self) -> _I2C_SMIS_STARTMISW {
        _I2C_SMIS_STARTMISW { w: self }
    }
    #[doc = "Bit 2 - Stop Condition Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_stopmis(&mut self) -> _I2C_SMIS_STOPMISW {
        _I2C_SMIS_STOPMISW { w: self }
    }
    #[doc = "Bit 3 - Receive DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_dmarxmis(&mut self) -> _I2C_SMIS_DMARXMISW {
        _I2C_SMIS_DMARXMISW { w: self }
    }
    #[doc = "Bit 4 - Transmit DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_dmatxmis(&mut self) -> _I2C_SMIS_DMATXMISW {
        _I2C_SMIS_DMATXMISW { w: self }
    }
    #[doc = "Bit 5 - Transmit FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_smis_txmis(&mut self) -> _I2C_SMIS_TXMISW {
        _I2C_SMIS_TXMISW { w: self }
    }
    #[doc = "Bit 6 - Receive FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_smis_rxmis(&mut self) -> _I2C_SMIS_RXMISW {
        _I2C_SMIS_RXMISW { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_smis_txfemis(&mut self) -> _I2C_SMIS_TXFEMISW {
        _I2C_SMIS_TXFEMISW { w: self }
    }
    #[doc = "Bit 8 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_smis_rxffmis(&mut self) -> _I2C_SMIS_RXFFMISW {
        _I2C_SMIS_RXFFMISW { w: self }
    }
}
