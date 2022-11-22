/*
quetta, ronna, yotta,
zetta, exa, peta,
tera, giga, mega,
kilo, hecto, deca,
deci, centi, milli,
micro, nano, pico,
femto, atto, zepto,
yocto, ronto, quecto,
*/
pub trait Si {
    fn quecto(&self) -> &'static str;
    fn ronto(&self) -> &'static str;
    fn yocto(&self) -> &'static str;
    fn zepto(&self) -> &'static str;
    fn atto(&self) -> &'static str;
    fn femto(&self) -> &'static str;
    fn pico(&self) -> &'static str;
    fn nano(&self) -> &'static str;
    fn micro(&self) -> &'static str;
    fn milli(&self) -> &'static str;
    fn centi(&self) -> &'static str;
    fn deci(&self) -> &'static str;
    fn deca(&self) -> &'static str;
    fn hecto(&self) -> &'static str;
    fn kilo(&self) -> &'static str;
    fn mega(&self) -> &'static str;
    fn giga(&self) -> &'static str;
    fn tera(&self) -> &'static str;
    fn peta(&self) -> &'static str;
    fn exa(&self) -> &'static str;
    fn zetta(&self) -> &'static str;
    fn yotta(&self) -> &'static str;
    fn ronna(&self) -> &'static str;
    fn quetta(&self) -> &'static str;
}

// ************************
// * TEST TEST  TEST TEST *
// ************************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true)
    }

    #[test]
    #[should_panic]
    fn it_fails() {
        assert!(false)
    }
}
