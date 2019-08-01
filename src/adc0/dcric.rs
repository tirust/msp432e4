#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCRIC {
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
pub struct _ADC_DCRIC_DCINT0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCRIC_DCINT0W<'a> {
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
pub struct _ADC_DCRIC_DCINT1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCRIC_DCINT1W<'a> {
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
pub struct _ADC_DCRIC_DCINT2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCRIC_DCINT2W<'a> {
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
pub struct _ADC_DCRIC_DCINT3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCRIC_DCINT3W<'a> {
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
pub struct _ADC_DCRIC_DCINT4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCRIC_DCINT4W<'a> {
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
pub struct _ADC_DCRIC_DCINT5W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCRIC_DCINT5W<'a> {
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
pub struct _ADC_DCRIC_DCINT6W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCRIC_DCINT6W<'a> {
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
pub struct _ADC_DCRIC_DCINT7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCRIC_DCINT7W<'a> {
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
pub struct _ADC_DCRIC_DCTRIG0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCRIC_DCTRIG0W<'a> {
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
#[doc = r"Proxy"]
pub struct _ADC_DCRIC_DCTRIG1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCRIC_DCTRIG1W<'a> {
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
#[doc = r"Proxy"]
pub struct _ADC_DCRIC_DCTRIG2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCRIC_DCTRIG2W<'a> {
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
#[doc = r"Proxy"]
pub struct _ADC_DCRIC_DCTRIG3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCRIC_DCTRIG3W<'a> {
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
        self.w.bits &= !(1 << 19);
        self.w.bits |= ((value as u32) & 1) << 19;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _ADC_DCRIC_DCTRIG4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCRIC_DCTRIG4W<'a> {
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
        self.w.bits &= !(1 << 20);
        self.w.bits |= ((value as u32) & 1) << 20;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _ADC_DCRIC_DCTRIG5W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCRIC_DCTRIG5W<'a> {
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
        self.w.bits &= !(1 << 21);
        self.w.bits |= ((value as u32) & 1) << 21;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _ADC_DCRIC_DCTRIG6W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCRIC_DCTRIG6W<'a> {
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
        self.w.bits &= !(1 << 22);
        self.w.bits |= ((value as u32) & 1) << 22;
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _ADC_DCRIC_DCTRIG7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCRIC_DCTRIG7W<'a> {
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
        self.w.bits &= !(1 << 23);
        self.w.bits |= ((value as u32) & 1) << 23;
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
    #[doc = "Bit 0 - Digital Comparator Interrupt 0"]
    #[inline(always)]
    pub fn adc_dcric_dcint0(&mut self) -> _ADC_DCRIC_DCINT0W {
        _ADC_DCRIC_DCINT0W { w: self }
    }
    #[doc = "Bit 1 - Digital Comparator Interrupt 1"]
    #[inline(always)]
    pub fn adc_dcric_dcint1(&mut self) -> _ADC_DCRIC_DCINT1W {
        _ADC_DCRIC_DCINT1W { w: self }
    }
    #[doc = "Bit 2 - Digital Comparator Interrupt 2"]
    #[inline(always)]
    pub fn adc_dcric_dcint2(&mut self) -> _ADC_DCRIC_DCINT2W {
        _ADC_DCRIC_DCINT2W { w: self }
    }
    #[doc = "Bit 3 - Digital Comparator Interrupt 3"]
    #[inline(always)]
    pub fn adc_dcric_dcint3(&mut self) -> _ADC_DCRIC_DCINT3W {
        _ADC_DCRIC_DCINT3W { w: self }
    }
    #[doc = "Bit 4 - Digital Comparator Interrupt 4"]
    #[inline(always)]
    pub fn adc_dcric_dcint4(&mut self) -> _ADC_DCRIC_DCINT4W {
        _ADC_DCRIC_DCINT4W { w: self }
    }
    #[doc = "Bit 5 - Digital Comparator Interrupt 5"]
    #[inline(always)]
    pub fn adc_dcric_dcint5(&mut self) -> _ADC_DCRIC_DCINT5W {
        _ADC_DCRIC_DCINT5W { w: self }
    }
    #[doc = "Bit 6 - Digital Comparator Interrupt 6"]
    #[inline(always)]
    pub fn adc_dcric_dcint6(&mut self) -> _ADC_DCRIC_DCINT6W {
        _ADC_DCRIC_DCINT6W { w: self }
    }
    #[doc = "Bit 7 - Digital Comparator Interrupt 7"]
    #[inline(always)]
    pub fn adc_dcric_dcint7(&mut self) -> _ADC_DCRIC_DCINT7W {
        _ADC_DCRIC_DCINT7W { w: self }
    }
    #[doc = "Bit 16 - Digital Comparator Trigger 0"]
    #[inline(always)]
    pub fn adc_dcric_dctrig0(&mut self) -> _ADC_DCRIC_DCTRIG0W {
        _ADC_DCRIC_DCTRIG0W { w: self }
    }
    #[doc = "Bit 17 - Digital Comparator Trigger 1"]
    #[inline(always)]
    pub fn adc_dcric_dctrig1(&mut self) -> _ADC_DCRIC_DCTRIG1W {
        _ADC_DCRIC_DCTRIG1W { w: self }
    }
    #[doc = "Bit 18 - Digital Comparator Trigger 2"]
    #[inline(always)]
    pub fn adc_dcric_dctrig2(&mut self) -> _ADC_DCRIC_DCTRIG2W {
        _ADC_DCRIC_DCTRIG2W { w: self }
    }
    #[doc = "Bit 19 - Digital Comparator Trigger 3"]
    #[inline(always)]
    pub fn adc_dcric_dctrig3(&mut self) -> _ADC_DCRIC_DCTRIG3W {
        _ADC_DCRIC_DCTRIG3W { w: self }
    }
    #[doc = "Bit 20 - Digital Comparator Trigger 4"]
    #[inline(always)]
    pub fn adc_dcric_dctrig4(&mut self) -> _ADC_DCRIC_DCTRIG4W {
        _ADC_DCRIC_DCTRIG4W { w: self }
    }
    #[doc = "Bit 21 - Digital Comparator Trigger 5"]
    #[inline(always)]
    pub fn adc_dcric_dctrig5(&mut self) -> _ADC_DCRIC_DCTRIG5W {
        _ADC_DCRIC_DCTRIG5W { w: self }
    }
    #[doc = "Bit 22 - Digital Comparator Trigger 6"]
    #[inline(always)]
    pub fn adc_dcric_dctrig6(&mut self) -> _ADC_DCRIC_DCTRIG6W {
        _ADC_DCRIC_DCTRIG6W { w: self }
    }
    #[doc = "Bit 23 - Digital Comparator Trigger 7"]
    #[inline(always)]
    pub fn adc_dcric_dctrig7(&mut self) -> _ADC_DCRIC_DCTRIG7W {
        _ADC_DCRIC_DCTRIG7W { w: self }
    }
}
