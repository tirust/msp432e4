#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SSCTL0 {
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
pub struct ADC_SSCTL0_D0R {
    bits: bool,
}
impl ADC_SSCTL0_D0R {
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
pub struct _ADC_SSCTL0_D0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_D0W<'a> {
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
pub struct ADC_SSCTL0_END0R {
    bits: bool,
}
impl ADC_SSCTL0_END0R {
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
pub struct _ADC_SSCTL0_END0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_END0W<'a> {
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
pub struct ADC_SSCTL0_IE0R {
    bits: bool,
}
impl ADC_SSCTL0_IE0R {
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
pub struct _ADC_SSCTL0_IE0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_IE0W<'a> {
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
pub struct ADC_SSCTL0_TS0R {
    bits: bool,
}
impl ADC_SSCTL0_TS0R {
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
pub struct _ADC_SSCTL0_TS0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_TS0W<'a> {
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
pub struct ADC_SSCTL0_D1R {
    bits: bool,
}
impl ADC_SSCTL0_D1R {
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
pub struct _ADC_SSCTL0_D1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_D1W<'a> {
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
pub struct ADC_SSCTL0_END1R {
    bits: bool,
}
impl ADC_SSCTL0_END1R {
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
pub struct _ADC_SSCTL0_END1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_END1W<'a> {
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
pub struct ADC_SSCTL0_IE1R {
    bits: bool,
}
impl ADC_SSCTL0_IE1R {
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
pub struct _ADC_SSCTL0_IE1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_IE1W<'a> {
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
pub struct ADC_SSCTL0_TS1R {
    bits: bool,
}
impl ADC_SSCTL0_TS1R {
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
pub struct _ADC_SSCTL0_TS1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_TS1W<'a> {
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
pub struct ADC_SSCTL0_D2R {
    bits: bool,
}
impl ADC_SSCTL0_D2R {
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
pub struct _ADC_SSCTL0_D2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_D2W<'a> {
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
pub struct ADC_SSCTL0_END2R {
    bits: bool,
}
impl ADC_SSCTL0_END2R {
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
pub struct _ADC_SSCTL0_END2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_END2W<'a> {
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
pub struct ADC_SSCTL0_IE2R {
    bits: bool,
}
impl ADC_SSCTL0_IE2R {
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
pub struct _ADC_SSCTL0_IE2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_IE2W<'a> {
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
pub struct ADC_SSCTL0_TS2R {
    bits: bool,
}
impl ADC_SSCTL0_TS2R {
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
pub struct _ADC_SSCTL0_TS2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_TS2W<'a> {
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
#[doc = r"Value of the field"]
pub struct ADC_SSCTL0_D3R {
    bits: bool,
}
impl ADC_SSCTL0_D3R {
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
pub struct _ADC_SSCTL0_D3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_D3W<'a> {
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
        self.w.bits &= !(1 << 12);
        self.w.bits |= ((value as u32) & 1) << 12;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSCTL0_END3R {
    bits: bool,
}
impl ADC_SSCTL0_END3R {
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
pub struct _ADC_SSCTL0_END3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_END3W<'a> {
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
pub struct ADC_SSCTL0_IE3R {
    bits: bool,
}
impl ADC_SSCTL0_IE3R {
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
pub struct _ADC_SSCTL0_IE3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_IE3W<'a> {
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
pub struct ADC_SSCTL0_TS3R {
    bits: bool,
}
impl ADC_SSCTL0_TS3R {
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
pub struct _ADC_SSCTL0_TS3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_TS3W<'a> {
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
pub struct ADC_SSCTL0_D4R {
    bits: bool,
}
impl ADC_SSCTL0_D4R {
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
pub struct _ADC_SSCTL0_D4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_D4W<'a> {
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
pub struct ADC_SSCTL0_END4R {
    bits: bool,
}
impl ADC_SSCTL0_END4R {
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
pub struct _ADC_SSCTL0_END4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_END4W<'a> {
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
pub struct ADC_SSCTL0_IE4R {
    bits: bool,
}
impl ADC_SSCTL0_IE4R {
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
pub struct _ADC_SSCTL0_IE4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_IE4W<'a> {
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
#[doc = r"Value of the field"]
pub struct ADC_SSCTL0_TS4R {
    bits: bool,
}
impl ADC_SSCTL0_TS4R {
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
pub struct _ADC_SSCTL0_TS4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_TS4W<'a> {
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
#[doc = r"Value of the field"]
pub struct ADC_SSCTL0_D5R {
    bits: bool,
}
impl ADC_SSCTL0_D5R {
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
pub struct _ADC_SSCTL0_D5W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_D5W<'a> {
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
#[doc = r"Value of the field"]
pub struct ADC_SSCTL0_END5R {
    bits: bool,
}
impl ADC_SSCTL0_END5R {
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
pub struct _ADC_SSCTL0_END5W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_END5W<'a> {
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
#[doc = r"Value of the field"]
pub struct ADC_SSCTL0_IE5R {
    bits: bool,
}
impl ADC_SSCTL0_IE5R {
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
pub struct _ADC_SSCTL0_IE5W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_IE5W<'a> {
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
#[doc = r"Value of the field"]
pub struct ADC_SSCTL0_TS5R {
    bits: bool,
}
impl ADC_SSCTL0_TS5R {
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
pub struct _ADC_SSCTL0_TS5W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_TS5W<'a> {
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
#[doc = r"Value of the field"]
pub struct ADC_SSCTL0_D6R {
    bits: bool,
}
impl ADC_SSCTL0_D6R {
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
pub struct _ADC_SSCTL0_D6W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_D6W<'a> {
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
        self.w.bits &= !(1 << 24);
        self.w.bits |= ((value as u32) & 1) << 24;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSCTL0_END6R {
    bits: bool,
}
impl ADC_SSCTL0_END6R {
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
pub struct _ADC_SSCTL0_END6W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_END6W<'a> {
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
        self.w.bits &= !(1 << 25);
        self.w.bits |= ((value as u32) & 1) << 25;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSCTL0_IE6R {
    bits: bool,
}
impl ADC_SSCTL0_IE6R {
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
pub struct _ADC_SSCTL0_IE6W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_IE6W<'a> {
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
        self.w.bits &= !(1 << 26);
        self.w.bits |= ((value as u32) & 1) << 26;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSCTL0_TS6R {
    bits: bool,
}
impl ADC_SSCTL0_TS6R {
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
pub struct _ADC_SSCTL0_TS6W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_TS6W<'a> {
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
pub struct ADC_SSCTL0_D7R {
    bits: bool,
}
impl ADC_SSCTL0_D7R {
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
pub struct _ADC_SSCTL0_D7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_D7W<'a> {
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
        self.w.bits &= !(1 << 28);
        self.w.bits |= ((value as u32) & 1) << 28;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSCTL0_END7R {
    bits: bool,
}
impl ADC_SSCTL0_END7R {
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
pub struct _ADC_SSCTL0_END7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_END7W<'a> {
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
pub struct ADC_SSCTL0_IE7R {
    bits: bool,
}
impl ADC_SSCTL0_IE7R {
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
pub struct _ADC_SSCTL0_IE7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_IE7W<'a> {
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
pub struct ADC_SSCTL0_TS7R {
    bits: bool,
}
impl ADC_SSCTL0_TS7R {
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
pub struct _ADC_SSCTL0_TS7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSCTL0_TS7W<'a> {
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
    #[doc = "Bit 0 - 1st Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d0(&self) -> ADC_SSCTL0_D0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        ADC_SSCTL0_D0R { bits }
    }
    #[doc = "Bit 1 - 1st Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end0(&self) -> ADC_SSCTL0_END0R {
        let bits = ((self.bits >> 1) & 1) != 0;
        ADC_SSCTL0_END0R { bits }
    }
    #[doc = "Bit 2 - 1st Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie0(&self) -> ADC_SSCTL0_IE0R {
        let bits = ((self.bits >> 2) & 1) != 0;
        ADC_SSCTL0_IE0R { bits }
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts0(&self) -> ADC_SSCTL0_TS0R {
        let bits = ((self.bits >> 3) & 1) != 0;
        ADC_SSCTL0_TS0R { bits }
    }
    #[doc = "Bit 4 - 2nd Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d1(&self) -> ADC_SSCTL0_D1R {
        let bits = ((self.bits >> 4) & 1) != 0;
        ADC_SSCTL0_D1R { bits }
    }
    #[doc = "Bit 5 - 2nd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end1(&self) -> ADC_SSCTL0_END1R {
        let bits = ((self.bits >> 5) & 1) != 0;
        ADC_SSCTL0_END1R { bits }
    }
    #[doc = "Bit 6 - 2nd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie1(&self) -> ADC_SSCTL0_IE1R {
        let bits = ((self.bits >> 6) & 1) != 0;
        ADC_SSCTL0_IE1R { bits }
    }
    #[doc = "Bit 7 - 2nd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts1(&self) -> ADC_SSCTL0_TS1R {
        let bits = ((self.bits >> 7) & 1) != 0;
        ADC_SSCTL0_TS1R { bits }
    }
    #[doc = "Bit 8 - 3rd Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d2(&self) -> ADC_SSCTL0_D2R {
        let bits = ((self.bits >> 8) & 1) != 0;
        ADC_SSCTL0_D2R { bits }
    }
    #[doc = "Bit 9 - 3rd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end2(&self) -> ADC_SSCTL0_END2R {
        let bits = ((self.bits >> 9) & 1) != 0;
        ADC_SSCTL0_END2R { bits }
    }
    #[doc = "Bit 10 - 3rd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie2(&self) -> ADC_SSCTL0_IE2R {
        let bits = ((self.bits >> 10) & 1) != 0;
        ADC_SSCTL0_IE2R { bits }
    }
    #[doc = "Bit 11 - 3rd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts2(&self) -> ADC_SSCTL0_TS2R {
        let bits = ((self.bits >> 11) & 1) != 0;
        ADC_SSCTL0_TS2R { bits }
    }
    #[doc = "Bit 12 - 4th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d3(&self) -> ADC_SSCTL0_D3R {
        let bits = ((self.bits >> 12) & 1) != 0;
        ADC_SSCTL0_D3R { bits }
    }
    #[doc = "Bit 13 - 4th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end3(&self) -> ADC_SSCTL0_END3R {
        let bits = ((self.bits >> 13) & 1) != 0;
        ADC_SSCTL0_END3R { bits }
    }
    #[doc = "Bit 14 - 4th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie3(&self) -> ADC_SSCTL0_IE3R {
        let bits = ((self.bits >> 14) & 1) != 0;
        ADC_SSCTL0_IE3R { bits }
    }
    #[doc = "Bit 15 - 4th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts3(&self) -> ADC_SSCTL0_TS3R {
        let bits = ((self.bits >> 15) & 1) != 0;
        ADC_SSCTL0_TS3R { bits }
    }
    #[doc = "Bit 16 - 5th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d4(&self) -> ADC_SSCTL0_D4R {
        let bits = ((self.bits >> 16) & 1) != 0;
        ADC_SSCTL0_D4R { bits }
    }
    #[doc = "Bit 17 - 5th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end4(&self) -> ADC_SSCTL0_END4R {
        let bits = ((self.bits >> 17) & 1) != 0;
        ADC_SSCTL0_END4R { bits }
    }
    #[doc = "Bit 18 - 5th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie4(&self) -> ADC_SSCTL0_IE4R {
        let bits = ((self.bits >> 18) & 1) != 0;
        ADC_SSCTL0_IE4R { bits }
    }
    #[doc = "Bit 19 - 5th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts4(&self) -> ADC_SSCTL0_TS4R {
        let bits = ((self.bits >> 19) & 1) != 0;
        ADC_SSCTL0_TS4R { bits }
    }
    #[doc = "Bit 20 - 6th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d5(&self) -> ADC_SSCTL0_D5R {
        let bits = ((self.bits >> 20) & 1) != 0;
        ADC_SSCTL0_D5R { bits }
    }
    #[doc = "Bit 21 - 6th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end5(&self) -> ADC_SSCTL0_END5R {
        let bits = ((self.bits >> 21) & 1) != 0;
        ADC_SSCTL0_END5R { bits }
    }
    #[doc = "Bit 22 - 6th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie5(&self) -> ADC_SSCTL0_IE5R {
        let bits = ((self.bits >> 22) & 1) != 0;
        ADC_SSCTL0_IE5R { bits }
    }
    #[doc = "Bit 23 - 6th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts5(&self) -> ADC_SSCTL0_TS5R {
        let bits = ((self.bits >> 23) & 1) != 0;
        ADC_SSCTL0_TS5R { bits }
    }
    #[doc = "Bit 24 - 7th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d6(&self) -> ADC_SSCTL0_D6R {
        let bits = ((self.bits >> 24) & 1) != 0;
        ADC_SSCTL0_D6R { bits }
    }
    #[doc = "Bit 25 - 7th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end6(&self) -> ADC_SSCTL0_END6R {
        let bits = ((self.bits >> 25) & 1) != 0;
        ADC_SSCTL0_END6R { bits }
    }
    #[doc = "Bit 26 - 7th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie6(&self) -> ADC_SSCTL0_IE6R {
        let bits = ((self.bits >> 26) & 1) != 0;
        ADC_SSCTL0_IE6R { bits }
    }
    #[doc = "Bit 27 - 7th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts6(&self) -> ADC_SSCTL0_TS6R {
        let bits = ((self.bits >> 27) & 1) != 0;
        ADC_SSCTL0_TS6R { bits }
    }
    #[doc = "Bit 28 - 8th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d7(&self) -> ADC_SSCTL0_D7R {
        let bits = ((self.bits >> 28) & 1) != 0;
        ADC_SSCTL0_D7R { bits }
    }
    #[doc = "Bit 29 - 8th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end7(&self) -> ADC_SSCTL0_END7R {
        let bits = ((self.bits >> 29) & 1) != 0;
        ADC_SSCTL0_END7R { bits }
    }
    #[doc = "Bit 30 - 8th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie7(&self) -> ADC_SSCTL0_IE7R {
        let bits = ((self.bits >> 30) & 1) != 0;
        ADC_SSCTL0_IE7R { bits }
    }
    #[doc = "Bit 31 - 8th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts7(&self) -> ADC_SSCTL0_TS7R {
        let bits = ((self.bits >> 31) & 1) != 0;
        ADC_SSCTL0_TS7R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - 1st Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d0(&mut self) -> _ADC_SSCTL0_D0W {
        _ADC_SSCTL0_D0W { w: self }
    }
    #[doc = "Bit 1 - 1st Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end0(&mut self) -> _ADC_SSCTL0_END0W {
        _ADC_SSCTL0_END0W { w: self }
    }
    #[doc = "Bit 2 - 1st Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie0(&mut self) -> _ADC_SSCTL0_IE0W {
        _ADC_SSCTL0_IE0W { w: self }
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts0(&mut self) -> _ADC_SSCTL0_TS0W {
        _ADC_SSCTL0_TS0W { w: self }
    }
    #[doc = "Bit 4 - 2nd Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d1(&mut self) -> _ADC_SSCTL0_D1W {
        _ADC_SSCTL0_D1W { w: self }
    }
    #[doc = "Bit 5 - 2nd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end1(&mut self) -> _ADC_SSCTL0_END1W {
        _ADC_SSCTL0_END1W { w: self }
    }
    #[doc = "Bit 6 - 2nd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie1(&mut self) -> _ADC_SSCTL0_IE1W {
        _ADC_SSCTL0_IE1W { w: self }
    }
    #[doc = "Bit 7 - 2nd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts1(&mut self) -> _ADC_SSCTL0_TS1W {
        _ADC_SSCTL0_TS1W { w: self }
    }
    #[doc = "Bit 8 - 3rd Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d2(&mut self) -> _ADC_SSCTL0_D2W {
        _ADC_SSCTL0_D2W { w: self }
    }
    #[doc = "Bit 9 - 3rd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end2(&mut self) -> _ADC_SSCTL0_END2W {
        _ADC_SSCTL0_END2W { w: self }
    }
    #[doc = "Bit 10 - 3rd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie2(&mut self) -> _ADC_SSCTL0_IE2W {
        _ADC_SSCTL0_IE2W { w: self }
    }
    #[doc = "Bit 11 - 3rd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts2(&mut self) -> _ADC_SSCTL0_TS2W {
        _ADC_SSCTL0_TS2W { w: self }
    }
    #[doc = "Bit 12 - 4th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d3(&mut self) -> _ADC_SSCTL0_D3W {
        _ADC_SSCTL0_D3W { w: self }
    }
    #[doc = "Bit 13 - 4th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end3(&mut self) -> _ADC_SSCTL0_END3W {
        _ADC_SSCTL0_END3W { w: self }
    }
    #[doc = "Bit 14 - 4th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie3(&mut self) -> _ADC_SSCTL0_IE3W {
        _ADC_SSCTL0_IE3W { w: self }
    }
    #[doc = "Bit 15 - 4th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts3(&mut self) -> _ADC_SSCTL0_TS3W {
        _ADC_SSCTL0_TS3W { w: self }
    }
    #[doc = "Bit 16 - 5th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d4(&mut self) -> _ADC_SSCTL0_D4W {
        _ADC_SSCTL0_D4W { w: self }
    }
    #[doc = "Bit 17 - 5th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end4(&mut self) -> _ADC_SSCTL0_END4W {
        _ADC_SSCTL0_END4W { w: self }
    }
    #[doc = "Bit 18 - 5th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie4(&mut self) -> _ADC_SSCTL0_IE4W {
        _ADC_SSCTL0_IE4W { w: self }
    }
    #[doc = "Bit 19 - 5th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts4(&mut self) -> _ADC_SSCTL0_TS4W {
        _ADC_SSCTL0_TS4W { w: self }
    }
    #[doc = "Bit 20 - 6th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d5(&mut self) -> _ADC_SSCTL0_D5W {
        _ADC_SSCTL0_D5W { w: self }
    }
    #[doc = "Bit 21 - 6th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end5(&mut self) -> _ADC_SSCTL0_END5W {
        _ADC_SSCTL0_END5W { w: self }
    }
    #[doc = "Bit 22 - 6th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie5(&mut self) -> _ADC_SSCTL0_IE5W {
        _ADC_SSCTL0_IE5W { w: self }
    }
    #[doc = "Bit 23 - 6th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts5(&mut self) -> _ADC_SSCTL0_TS5W {
        _ADC_SSCTL0_TS5W { w: self }
    }
    #[doc = "Bit 24 - 7th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d6(&mut self) -> _ADC_SSCTL0_D6W {
        _ADC_SSCTL0_D6W { w: self }
    }
    #[doc = "Bit 25 - 7th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end6(&mut self) -> _ADC_SSCTL0_END6W {
        _ADC_SSCTL0_END6W { w: self }
    }
    #[doc = "Bit 26 - 7th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie6(&mut self) -> _ADC_SSCTL0_IE6W {
        _ADC_SSCTL0_IE6W { w: self }
    }
    #[doc = "Bit 27 - 7th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts6(&mut self) -> _ADC_SSCTL0_TS6W {
        _ADC_SSCTL0_TS6W { w: self }
    }
    #[doc = "Bit 28 - 8th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d7(&mut self) -> _ADC_SSCTL0_D7W {
        _ADC_SSCTL0_D7W { w: self }
    }
    #[doc = "Bit 29 - 8th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end7(&mut self) -> _ADC_SSCTL0_END7W {
        _ADC_SSCTL0_END7W { w: self }
    }
    #[doc = "Bit 30 - 8th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie7(&mut self) -> _ADC_SSCTL0_IE7W {
        _ADC_SSCTL0_IE7W { w: self }
    }
    #[doc = "Bit 31 - 8th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts7(&mut self) -> _ADC_SSCTL0_TS7W {
        _ADC_SSCTL0_TS7W { w: self }
    }
}
