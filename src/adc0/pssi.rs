#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PSSI {
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
pub struct ADC_PSSI_SS0R {
    bits: bool,
}
impl ADC_PSSI_SS0R {
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
pub struct _ADC_PSSI_SS0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_PSSI_SS0W<'a> {
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
pub struct ADC_PSSI_SS1R {
    bits: bool,
}
impl ADC_PSSI_SS1R {
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
pub struct _ADC_PSSI_SS1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_PSSI_SS1W<'a> {
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
pub struct ADC_PSSI_SS2R {
    bits: bool,
}
impl ADC_PSSI_SS2R {
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
pub struct _ADC_PSSI_SS2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_PSSI_SS2W<'a> {
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
pub struct ADC_PSSI_SS3R {
    bits: bool,
}
impl ADC_PSSI_SS3R {
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
pub struct _ADC_PSSI_SS3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_PSSI_SS3W<'a> {
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
pub struct ADC_PSSI_SYNCWAITR {
    bits: bool,
}
impl ADC_PSSI_SYNCWAITR {
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
pub struct _ADC_PSSI_SYNCWAITW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_PSSI_SYNCWAITW<'a> {
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
        self.w.bits &= !(1 << 27);
        self.w.bits |= ((value as u32) & 1) << 27;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_PSSI_GSYNCR {
    bits: bool,
}
impl ADC_PSSI_GSYNCR {
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
pub struct _ADC_PSSI_GSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_PSSI_GSYNCW<'a> {
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
    #[doc = "Bit 0 - SS0 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss0(&self) -> ADC_PSSI_SS0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        ADC_PSSI_SS0R { bits }
    }
    #[doc = "Bit 1 - SS1 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss1(&self) -> ADC_PSSI_SS1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        ADC_PSSI_SS1R { bits }
    }
    #[doc = "Bit 2 - SS2 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss2(&self) -> ADC_PSSI_SS2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        ADC_PSSI_SS2R { bits }
    }
    #[doc = "Bit 3 - SS3 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss3(&self) -> ADC_PSSI_SS3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        ADC_PSSI_SS3R { bits }
    }
    #[doc = "Bit 27 - Synchronize Wait"]
    #[inline(always)]
    pub fn adc_pssi_syncwait(&self) -> ADC_PSSI_SYNCWAITR {
        let bits = ((self.bits >> 27) & 1) != 0;
        ADC_PSSI_SYNCWAITR { bits }
    }
    #[doc = "Bit 31 - Global Synchronize"]
    #[inline(always)]
    pub fn adc_pssi_gsync(&self) -> ADC_PSSI_GSYNCR {
        let bits = ((self.bits >> 31) & 1) != 0;
        ADC_PSSI_GSYNCR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SS0 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss0(&mut self) -> _ADC_PSSI_SS0W {
        _ADC_PSSI_SS0W { w: self }
    }
    #[doc = "Bit 1 - SS1 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss1(&mut self) -> _ADC_PSSI_SS1W {
        _ADC_PSSI_SS1W { w: self }
    }
    #[doc = "Bit 2 - SS2 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss2(&mut self) -> _ADC_PSSI_SS2W {
        _ADC_PSSI_SS2W { w: self }
    }
    #[doc = "Bit 3 - SS3 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss3(&mut self) -> _ADC_PSSI_SS3W {
        _ADC_PSSI_SS3W { w: self }
    }
    #[doc = "Bit 27 - Synchronize Wait"]
    #[inline(always)]
    pub fn adc_pssi_syncwait(&mut self) -> _ADC_PSSI_SYNCWAITW {
        _ADC_PSSI_SYNCWAITW { w: self }
    }
    #[doc = "Bit 31 - Global Synchronize"]
    #[inline(always)]
    pub fn adc_pssi_gsync(&mut self) -> _ADC_PSSI_GSYNCW {
        _ADC_PSSI_GSYNCW { w: self }
    }
}
