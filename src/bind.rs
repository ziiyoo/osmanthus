use chrono::NaiveDateTime;

#[derive(Debug, PartialEq, Default, Clone)]
pub enum DateTimeLabel {
    #[default]
    Invalid, Numeric, Characters,
}

#[derive(Debug, Default)]
pub struct Result{
    pub status: bool,
    pub method: String,
    pub time: NaiveDateTime,
    pub datetime: DateTime,
    pub timezone: String,
}

#[derive(Debug, Default)]
pub struct Parse{
    pub param: Param
}

#[derive(Debug, Default)]
pub struct ParseTimestamp{
    pub param: Param,
    pub offset: i32,
    pub time: NaiveDateTime,
}

#[derive(Debug, Default)]
pub struct ParseSeries{
    pub param: Param
}

#[derive(Default, PartialEq, Debug)]
pub enum TIMEZONE {
    #[default]
    LOCAL,  // 当前时区
    UTC, EST, MST, PST, HAST,
    AKST, BST, IST, WET, CET,
    EET, MSK, JST, AEST, NZST,
    ICT, GMT, CST
}

#[derive(Debug, PartialEq)]
pub enum DateType{
    YEAR,
    MONTH,
    NONE,
}

#[derive(Debug, Default)]
pub struct ResultRelativeText{
    pub hit: bool,
    pub text: String,
    pub same: bool,
}

#[derive(Debug, Default)]
pub struct ResultDateText{
    pub index: usize,
    pub text: String,
}

#[derive(Debug, Default)]
pub struct TimeMark{
    pub year: DateTimeItem,
    pub month: DateTimeItem,
    pub day: DateTimeItem,
    pub time: DateTimeItem,
    pub timezone: DateTimeItem
}

#[derive(Debug, Default)]
pub struct DateTimeItem{
    pub label: DateTimeLabel,
    pub status: bool,
    pub value: u32,
}

#[derive(Debug)]
pub struct Token{
    pub text: String,
    pub label: DateTimeLabel,
}
#[derive(Debug, Default)]
pub struct ParseAbsolute{
    pub param: Param,
    pub offset: i32,
    pub time: NaiveDateTime,
    pub era: EraBasedCalendar,
    pub token: Vec<Token>,
}

/// 基于纪元年号的年份
/// Reiwa 日本令和
/// Thai 泰历
#[derive(Default, PartialEq, Debug)]
pub enum EraBasedCalendar {
    #[default]
    Non, Reiwa, Thai}

#[derive(Debug, Default)]
pub struct ParseRelative{
    pub param: Param
}

#[derive(Debug, Default)]
pub struct Param{
    // pub era: String, // 纪元年份
    pub timezone: String,  // 时区
    pub strict: bool  // 严格模式
}


#[derive(Debug, Default)]
pub struct DateTime{
    pub local: Item,
    pub timezone: Item,
}

#[derive(Debug, Default)]
pub struct Item{
    pub datetime: NaiveDateTime,
    pub timestamp: i64
}
