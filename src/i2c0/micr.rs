#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MICR {
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
#[doc = r"Proxy"]
pub struct _I2C_MICR_ICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MICR_ICW<'a> {
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
#[doc = r"Proxy"]
pub struct _I2C_MICR_CLKICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MICR_CLKICW<'a> {
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
#[doc = r"Proxy"]
pub struct _I2C_MICR_DMARXICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MICR_DMARXICW<'a> {
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
#[doc = r"Proxy"]
pub struct _I2C_MICR_DMATXICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MICR_DMATXICW<'a> {
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
#[doc = r"Proxy"]
pub struct _I2C_MICR_NACKICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MICR_NACKICW<'a> {
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
#[doc = r"Proxy"]
pub struct _I2C_MICR_STARTICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MICR_STARTICW<'a> {
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
#[doc = r"Proxy"]
pub struct _I2C_MICR_STOPICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MICR_STOPICW<'a> {
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
#[doc = r"Proxy"]
pub struct _I2C_MICR_ARBLOSTICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MICR_ARBLOSTICW<'a> {
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
#[doc = r"Proxy"]
pub struct _I2C_MICR_TXICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MICR_TXICW<'a> {
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
#[doc = r"Proxy"]
pub struct _I2C_MICR_RXICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MICR_RXICW<'a> {
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
#[doc = r"Proxy"]
pub struct _I2C_MICR_TXFEICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MICR_TXFEICW<'a> {
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
#[doc = r"Proxy"]
pub struct _I2C_MICR_RXFFICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MICR_RXFFICW<'a> {
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
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Master Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_ic(&mut self) -> _I2C_MICR_ICW {
        _I2C_MICR_ICW { w: self }
    }
    #[doc = "Bit 1 - Clock Timeout Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_clkic(&mut self) -> _I2C_MICR_CLKICW {
        _I2C_MICR_CLKICW { w: self }
    }
    #[doc = "Bit 2 - Receive DMA Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_dmarxic(&mut self) -> _I2C_MICR_DMARXICW {
        _I2C_MICR_DMARXICW { w: self }
    }
    #[doc = "Bit 3 - Transmit DMA Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_dmatxic(&mut self) -> _I2C_MICR_DMATXICW {
        _I2C_MICR_DMATXICW { w: self }
    }
    #[doc = "Bit 4 - Address/Data NACK Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_nackic(&mut self) -> _I2C_MICR_NACKICW {
        _I2C_MICR_NACKICW { w: self }
    }
    #[doc = "Bit 5 - START Detection Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_startic(&mut self) -> _I2C_MICR_STARTICW {
        _I2C_MICR_STARTICW { w: self }
    }
    #[doc = "Bit 6 - STOP Detection Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_stopic(&mut self) -> _I2C_MICR_STOPICW {
        _I2C_MICR_STOPICW { w: self }
    }
    #[doc = "Bit 7 - Arbitration Lost Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_arblostic(&mut self) -> _I2C_MICR_ARBLOSTICW {
        _I2C_MICR_ARBLOSTICW { w: self }
    }
    #[doc = "Bit 8 - Transmit FIFO Request Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_txic(&mut self) -> _I2C_MICR_TXICW {
        _I2C_MICR_TXICW { w: self }
    }
    #[doc = "Bit 9 - Receive FIFO Request Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_rxic(&mut self) -> _I2C_MICR_RXICW {
        _I2C_MICR_RXICW { w: self }
    }
    #[doc = "Bit 10 - Transmit FIFO Empty Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_txfeic(&mut self) -> _I2C_MICR_TXFEICW {
        _I2C_MICR_TXFEICW { w: self }
    }
    #[doc = "Bit 11 - Receive FIFO Full Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_rxffic(&mut self) -> _I2C_MICR_RXFFICW {
        _I2C_MICR_RXFFICW { w: self }
    }
}
