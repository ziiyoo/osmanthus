use crate::core::interfaces::Parse;
pub mod core;
pub mod bind;
pub mod utils;

use crate::bind::{Result, Param, ParseTimestamp, ParseSeries, ParseRelative, ParseAbsolute};
use crate::core::parse_auto::parse_auto;

pub fn parse_timestamp(text: &str, options: Option<Param>) -> Result{
    let mut ins = ParseTimestamp::new(options);
    let r = ins.parse(text);
    return r
}

pub fn parse_series(text: &str, options: Option<Param>) -> Result{
    let mut ins = ParseSeries::new(options);
    let r = ins.parse(text);
    return r
}

pub fn parse_relative(text: &str, options: Option<Param>) -> Result{
    let mut ins = ParseRelative::new(options);
    let r = ins.parse(text);
    return r
}

pub fn parse_absolute(text: &str, options: Option<Param>) -> Result{
    let mut ins = ParseAbsolute::new(options);
    let r = ins.parse(text);
    return r
}

pub fn parse(text: &str, options: Option<Param>) -> Result{
    return parse_auto(text, options)
}