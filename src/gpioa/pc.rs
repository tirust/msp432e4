#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PC {
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
#[doc = "Possible values of the field `GPIO_PC_EDM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_PC_EDM0R {
    #[doc = "Drive values of 2, 4 and 8 mA are maintained. GPIO n Drive Select (GPIODRnR) registers function as normal"]
    GPIO_PC_EDM0_DISABLE,
    #[doc = "An additional 6 mA option is provided"]
    GPIO_PC_EDM0_6MA,
    #[doc = "A 2 mA driver is always enabled; setting the corresponding GPIODR4R register bit adds 2 mA and setting the corresponding GPIODR8R of GPIODR12R register bit adds an additional 4 mA"]
    GPIO_PC_EDM0_PLUS2MA,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl GPIO_PC_EDM0R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO_PC_EDM0R::GPIO_PC_EDM0_DISABLE => 0,
            GPIO_PC_EDM0R::GPIO_PC_EDM0_6MA => 1,
            GPIO_PC_EDM0R::GPIO_PC_EDM0_PLUS2MA => 3,
            GPIO_PC_EDM0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> GPIO_PC_EDM0R {
        match value {
            0 => GPIO_PC_EDM0R::GPIO_PC_EDM0_DISABLE,
            1 => GPIO_PC_EDM0R::GPIO_PC_EDM0_6MA,
            3 => GPIO_PC_EDM0R::GPIO_PC_EDM0_PLUS2MA,
            i => GPIO_PC_EDM0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_PC_EDM0_DISABLE`"]
    #[inline(always)]
    pub fn is_gpio_pc_edm0_disable(&self) -> bool {
        *self == GPIO_PC_EDM0R::GPIO_PC_EDM0_DISABLE
    }
    #[doc = "Checks if the value of the field is `GPIO_PC_EDM0_6MA`"]
    #[inline(always)]
    pub fn is_gpio_pc_edm0_6ma(&self) -> bool {
        *self == GPIO_PC_EDM0R::GPIO_PC_EDM0_6MA
    }
    #[doc = "Checks if the value of the field is `GPIO_PC_EDM0_PLUS2MA`"]
    #[inline(always)]
    pub fn is_gpio_pc_edm0_plus2ma(&self) -> bool {
        *self == GPIO_PC_EDM0R::GPIO_PC_EDM0_PLUS2MA
    }
}
#[doc = "Values that can be written to the field `GPIO_PC_EDM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_PC_EDM0W {
    #[doc = "Drive values of 2, 4 and 8 mA are maintained. GPIO n Drive Select (GPIODRnR) registers function as normal"]
    GPIO_PC_EDM0_DISABLE,
    #[doc = "An additional 6 mA option is provided"]
    GPIO_PC_EDM0_6MA,
    #[doc = "A 2 mA driver is always enabled; setting the corresponding GPIODR4R register bit adds 2 mA and setting the corresponding GPIODR8R of GPIODR12R register bit adds an additional 4 mA"]
    GPIO_PC_EDM0_PLUS2MA,
}
impl GPIO_PC_EDM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO_PC_EDM0W::GPIO_PC_EDM0_DISABLE => 0,
            GPIO_PC_EDM0W::GPIO_PC_EDM0_6MA => 1,
            GPIO_PC_EDM0W::GPIO_PC_EDM0_PLUS2MA => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _GPIO_PC_EDM0W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_PC_EDM0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_PC_EDM0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Drive values of 2, 4 and 8 mA are maintained. GPIO n Drive Select (GPIODRnR) registers function as normal"]
    #[inline(always)]
    pub fn gpio_pc_edm0_disable(self) -> &'a mut W {
        self.variant(GPIO_PC_EDM0W::GPIO_PC_EDM0_DISABLE)
    }
    #[doc = "An additional 6 mA option is provided"]
    #[inline(always)]
    pub fn gpio_pc_edm0_6ma(self) -> &'a mut W {
        self.variant(GPIO_PC_EDM0W::GPIO_PC_EDM0_6MA)
    }
    #[doc = "A 2 mA driver is always enabled; setting the corresponding GPIODR4R register bit adds 2 mA and setting the corresponding GPIODR8R of GPIODR12R register bit adds an additional 4 mA"]
    #[inline(always)]
    pub fn gpio_pc_edm0_plus2ma(self) -> &'a mut W {
        self.variant(GPIO_PC_EDM0W::GPIO_PC_EDM0_PLUS2MA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct GPIO_PC_EDM1R {
    bits: u8,
}
impl GPIO_PC_EDM1R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _GPIO_PC_EDM1W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_PC_EDM1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 2);
        self.w.bits |= ((value as u32) & 3) << 2;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct GPIO_PC_EDM2R {
    bits: u8,
}
impl GPIO_PC_EDM2R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _GPIO_PC_EDM2W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_PC_EDM2W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct GPIO_PC_EDM3R {
    bits: u8,
}
impl GPIO_PC_EDM3R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _GPIO_PC_EDM3W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_PC_EDM3W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 6);
        self.w.bits |= ((value as u32) & 3) << 6;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct GPIO_PC_EDM4R {
    bits: u8,
}
impl GPIO_PC_EDM4R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _GPIO_PC_EDM4W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_PC_EDM4W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 8);
        self.w.bits |= ((value as u32) & 3) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct GPIO_PC_EDM5R {
    bits: u8,
}
impl GPIO_PC_EDM5R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _GPIO_PC_EDM5W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_PC_EDM5W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 10);
        self.w.bits |= ((value as u32) & 3) << 10;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct GPIO_PC_EDM6R {
    bits: u8,
}
impl GPIO_PC_EDM6R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _GPIO_PC_EDM6W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_PC_EDM6W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 12);
        self.w.bits |= ((value as u32) & 3) << 12;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct GPIO_PC_EDM7R {
    bits: u8,
}
impl GPIO_PC_EDM7R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _GPIO_PC_EDM7W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_PC_EDM7W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 14);
        self.w.bits |= ((value as u32) & 3) << 14;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Extended Drive Mode Bit 0"]
    #[inline(always)]
    pub fn gpio_pc_edm0(&self) -> GPIO_PC_EDM0R {
        GPIO_PC_EDM0R::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bits 2:3 - Extended Drive Mode Bit 1"]
    #[inline(always)]
    pub fn gpio_pc_edm1(&self) -> GPIO_PC_EDM1R {
        let bits = ((self.bits >> 2) & 3) as u8;
        GPIO_PC_EDM1R { bits }
    }
    #[doc = "Bits 4:5 - Extended Drive Mode Bit 2"]
    #[inline(always)]
    pub fn gpio_pc_edm2(&self) -> GPIO_PC_EDM2R {
        let bits = ((self.bits >> 4) & 3) as u8;
        GPIO_PC_EDM2R { bits }
    }
    #[doc = "Bits 6:7 - Extended Drive Mode Bit 3"]
    #[inline(always)]
    pub fn gpio_pc_edm3(&self) -> GPIO_PC_EDM3R {
        let bits = ((self.bits >> 6) & 3) as u8;
        GPIO_PC_EDM3R { bits }
    }
    #[doc = "Bits 8:9 - Extended Drive Mode Bit 4"]
    #[inline(always)]
    pub fn gpio_pc_edm4(&self) -> GPIO_PC_EDM4R {
        let bits = ((self.bits >> 8) & 3) as u8;
        GPIO_PC_EDM4R { bits }
    }
    #[doc = "Bits 10:11 - Extended Drive Mode Bit 5"]
    #[inline(always)]
    pub fn gpio_pc_edm5(&self) -> GPIO_PC_EDM5R {
        let bits = ((self.bits >> 10) & 3) as u8;
        GPIO_PC_EDM5R { bits }
    }
    #[doc = "Bits 12:13 - Extended Drive Mode Bit 6"]
    #[inline(always)]
    pub fn gpio_pc_edm6(&self) -> GPIO_PC_EDM6R {
        let bits = ((self.bits >> 12) & 3) as u8;
        GPIO_PC_EDM6R { bits }
    }
    #[doc = "Bits 14:15 - Extended Drive Mode Bit 7"]
    #[inline(always)]
    pub fn gpio_pc_edm7(&self) -> GPIO_PC_EDM7R {
        let bits = ((self.bits >> 14) & 3) as u8;
        GPIO_PC_EDM7R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Extended Drive Mode Bit 0"]
    #[inline(always)]
    pub fn gpio_pc_edm0(&mut self) -> _GPIO_PC_EDM0W {
        _GPIO_PC_EDM0W { w: self }
    }
    #[doc = "Bits 2:3 - Extended Drive Mode Bit 1"]
    #[inline(always)]
    pub fn gpio_pc_edm1(&mut self) -> _GPIO_PC_EDM1W {
        _GPIO_PC_EDM1W { w: self }
    }
    #[doc = "Bits 4:5 - Extended Drive Mode Bit 2"]
    #[inline(always)]
    pub fn gpio_pc_edm2(&mut self) -> _GPIO_PC_EDM2W {
        _GPIO_PC_EDM2W { w: self }
    }
    #[doc = "Bits 6:7 - Extended Drive Mode Bit 3"]
    #[inline(always)]
    pub fn gpio_pc_edm3(&mut self) -> _GPIO_PC_EDM3W {
        _GPIO_PC_EDM3W { w: self }
    }
    #[doc = "Bits 8:9 - Extended Drive Mode Bit 4"]
    #[inline(always)]
    pub fn gpio_pc_edm4(&mut self) -> _GPIO_PC_EDM4W {
        _GPIO_PC_EDM4W { w: self }
    }
    #[doc = "Bits 10:11 - Extended Drive Mode Bit 5"]
    #[inline(always)]
    pub fn gpio_pc_edm5(&mut self) -> _GPIO_PC_EDM5W {
        _GPIO_PC_EDM5W { w: self }
    }
    #[doc = "Bits 12:13 - Extended Drive Mode Bit 6"]
    #[inline(always)]
    pub fn gpio_pc_edm6(&mut self) -> _GPIO_PC_EDM6W {
        _GPIO_PC_EDM6W { w: self }
    }
    #[doc = "Bits 14:15 - Extended Drive Mode Bit 7"]
    #[inline(always)]
    pub fn gpio_pc_edm7(&mut self) -> _GPIO_PC_EDM7W {
        _GPIO_PC_EDM7W { w: self }
    }
}
