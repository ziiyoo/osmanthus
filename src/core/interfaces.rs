use crate::bind::Result;

pub(crate) trait Parse{
    fn parse(&mut self, text: &str) -> Result;
}