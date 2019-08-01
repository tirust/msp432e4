#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IRQENABLE {
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
pub struct DES_IRQENABLE_M_CONTEX_INR {
    bits: bool,
}
impl DES_IRQENABLE_M_CONTEX_INR {
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
pub struct _DES_IRQENABLE_M_CONTEX_INW<'a> {
    w: &'a mut W,
}
impl<'a> _DES_IRQENABLE_M_CONTEX_INW<'a> {
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
pub struct DES_IRQENABLE_M_DATA_INR {
    bits: bool,
}
impl DES_IRQENABLE_M_DATA_INR {
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
pub struct _DES_IRQENABLE_M_DATA_INW<'a> {
    w: &'a mut W,
}
impl<'a> _DES_IRQENABLE_M_DATA_INW<'a> {
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
pub struct DES_IRQENABLE_M_DATA_OUTR {
    bits: bool,
}
impl DES_IRQENABLE_M_DATA_OUTR {
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
pub struct _DES_IRQENABLE_M_DATA_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _DES_IRQENABLE_M_DATA_OUTW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - If this bit is set to 1 the context interrupt is enabled"]
    #[inline(always)]
    pub fn des_irqenable_m_contex_in(&self) -> DES_IRQENABLE_M_CONTEX_INR {
        let bits = ((self.bits >> 0) & 1) != 0;
        DES_IRQENABLE_M_CONTEX_INR { bits }
    }
    #[doc = "Bit 1 - If this bit is set to 1 the data input interrupt is enabled"]
    #[inline(always)]
    pub fn des_irqenable_m_data_in(&self) -> DES_IRQENABLE_M_DATA_INR {
        let bits = ((self.bits >> 1) & 1) != 0;
        DES_IRQENABLE_M_DATA_INR { bits }
    }
    #[doc = "Bit 2 - If this bit is set to 1 the data output interrupt is enabled"]
    #[inline(always)]
    pub fn des_irqenable_m_data_out(&self) -> DES_IRQENABLE_M_DATA_OUTR {
        let bits = ((self.bits >> 2) & 1) != 0;
        DES_IRQENABLE_M_DATA_OUTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - If this bit is set to 1 the context interrupt is enabled"]
    #[inline(always)]
    pub fn des_irqenable_m_contex_in(&mut self) -> _DES_IRQENABLE_M_CONTEX_INW {
        _DES_IRQENABLE_M_CONTEX_INW { w: self }
    }
    #[doc = "Bit 1 - If this bit is set to 1 the data input interrupt is enabled"]
    #[inline(always)]
    pub fn des_irqenable_m_data_in(&mut self) -> _DES_IRQENABLE_M_DATA_INW {
        _DES_IRQENABLE_M_DATA_INW { w: self }
    }
    #[doc = "Bit 2 - If this bit is set to 1 the data output interrupt is enabled"]
    #[inline(always)]
    pub fn des_irqenable_m_data_out(&mut self) -> _DES_IRQENABLE_M_DATA_OUTW {
        _DES_IRQENABLE_M_DATA_OUTW { w: self }
    }
}
