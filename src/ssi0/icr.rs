#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICR {
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
pub struct _SSI_ICR_RORICW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_ICR_RORICW<'a> {
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
pub struct _SSI_ICR_RTICW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_ICR_RTICW<'a> {
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
pub struct _SSI_ICR_DMARXICW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_ICR_DMARXICW<'a> {
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
pub struct _SSI_ICR_DMATXICW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_ICR_DMATXICW<'a> {
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
pub struct _SSI_ICR_EOTICW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_ICR_EOTICW<'a> {
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
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SSI Receive Overrun Interrupt Clear"]
    #[inline(always)]
    pub fn ssi_icr_roric(&mut self) -> _SSI_ICR_RORICW {
        _SSI_ICR_RORICW { w: self }
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Interrupt Clear"]
    #[inline(always)]
    pub fn ssi_icr_rtic(&mut self) -> _SSI_ICR_RTICW {
        _SSI_ICR_RTICW { w: self }
    }
    #[doc = "Bit 4 - SSI Receive DMA Interrupt Clear"]
    #[inline(always)]
    pub fn ssi_icr_dmarxic(&mut self) -> _SSI_ICR_DMARXICW {
        _SSI_ICR_DMARXICW { w: self }
    }
    #[doc = "Bit 5 - SSI Transmit DMA Interrupt Clear"]
    #[inline(always)]
    pub fn ssi_icr_dmatxic(&mut self) -> _SSI_ICR_DMATXICW {
        _SSI_ICR_DMATXICW { w: self }
    }
    #[doc = "Bit 6 - End of Transmit Interrupt Clear"]
    #[inline(always)]
    pub fn ssi_icr_eotic(&mut self) -> _SSI_ICR_EOTICW {
        _SSI_ICR_EOTICW { w: self }
    }
}
