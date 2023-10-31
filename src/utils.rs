use chrono::prelude::*;
use crate::core::corpus::{replace_with_pattern, unitize_date_text, is_thai_month, get_offset_local_utc,
                          match_with_pattern, unitize_timezone_with_text};
use crate::bind::{EraBasedCalendar, DateTimeLabel, Token};


const CHAR_NUMERIC: &str = "0123456789:";
const CHAR_CHARACTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn is_timestamp(text: &str) -> bool{
    if is_number(text) && (text.chars().count() == 10||text.chars().count() == 13) {
        return true
    }
    return false
}

pub fn is_number(text: &str) -> bool{
    let r:Result<i64, _> = text.parse();
    match r {
        Ok(_) => true,
        Err(_) => {false}
    }
}

pub fn create_datetime(zero: bool, utc: bool) -> NaiveDateTime{
    if zero{
        return create_datetime_zero();
    }
    return create_datetime_current(utc);
}


/// 基于初始时间生成
pub fn create_datetime_zero() -> NaiveDateTime{
    let date = NaiveDateTime::parse_from_str("1970-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").expect("");
    return date
}


/// 基于当前时间生成
pub fn create_datetime_current(utc: bool) ->NaiveDateTime{
    let mut date: NaiveDate = Local::now().date_naive();
    if utc{
        date = Utc::now().date_naive();
    }
    let time = NaiveTime::from_hms_opt(0, 0, 0);
    let datetime = NaiveDateTime::new(date, time.unwrap());
    return datetime
}

/// 去噪
pub fn eliminate_noise(text: &str) -> String{
    let mut item = text.trim().replace("/n", " ");
    item = eliminate_no_break_space(item.as_str());
    item = eliminate_symbol_normal(&item);
    item = eliminate_space(&item);
    return item.trim().to_string()
}

/// 消除NBSP
pub fn eliminate_no_break_space(text: &str) -> String{
    return replace_with_pattern("no_break_space", text)
}

///
pub fn eliminate_symbol_point(text: &str) -> String{
    return replace_with_pattern("symbol_point", text)
}

pub fn eliminate_symbol_normal(text: &str) -> String{
    return replace_with_pattern("symbol_normal", text)
}

pub fn eliminate_symbol_safe(text: &str) -> String{
    return replace_with_pattern("symbol_safe", text)
}

pub fn eliminate_space(text: &str) -> String{
    return replace_with_pattern("space", text)
}

/// 规整文本统一表述
/// 10月 -> october
/// 公元2002年 10월 13 5:50 PM Cst - "公元2002年", "october", "13", "5:50", "pm", "cst"
///  return "公元2002年 october 13 5:50 pm cst"
pub fn unitize_text(text: &str) -> (String, EraBasedCalendar){
    let mut era = EraBasedCalendar::Non;
    let mut data: Vec<String> = Vec::new();
    let section: Vec<&str> = text.split(" ").collect();
    for text in section{
        let item = text.to_lowercase();
        if let Some(value) = unitize_date_text(&item){
            // 标记泰历
            if is_thai_month(&item){
                era = EraBasedCalendar::Thai;
            }
            data.push(value);
            continue
        }
        data.push(item);
    }
    return (data.join(" "), era)
}

/// 消除空元素
pub fn eliminate_empty_item(section: Vec<String>) -> Vec<String>{
    let mut seeds: Vec<String> = Vec::new();
    for item in section{
        if !item.is_empty(){
            seeds.push(item);
        }
    }
    return seeds
}


/// 基于数字将字符串分为多段
pub fn split_with_numeric(text: String) -> Vec<String> {
    return match_with_pattern("split_with_numeric", text)
}

pub fn create_timestamp(datetime: NaiveDateTime) -> i64{
    let timestamp =  datetime.timestamp() - get_offset_local_utc();
    return timestamp * 1000
}

/// 基于空格切分
pub fn section_with_space(sec:Vec<String>) -> Vec<String>{
    let mut sections: Vec<String> = Vec::new();
    for s in sec{
        for (_, v) in s.split(" ").enumerate(){
            sections.push(v.to_string())
        }
    }
    return sections
}

// 基于文本匹配时区和偏移量
pub fn search_offset_with_text(section: &Vec<String>) ->(i32, String){
    let mut number = 0;
    let mut timezone = String::from("");
    for v in section{
        if let Some(n) = unitize_timezone_with_text(v){
            number = *n;
            timezone = v.to_string();
        }
    }
    return (number, timezone)
}


/// 字符串转数字类型
pub fn str_convert(text: &str) -> Option<i32>{
    let r:Result<i32, _> = text.parse();
    return match r {
        Ok(v) => Some(v),
        Err(_) => {
            None
        },
    }
}


fn get_datetime_label(ch: &char) -> DateTimeLabel{
    return match ch{
        ch if CHAR_NUMERIC.contains(*ch) => DateTimeLabel::Numeric,
        ch if CHAR_CHARACTERS.contains(*ch) => DateTimeLabel::Characters,
        _ => DateTimeLabel::Invalid
    }
}

pub fn tokenize(text: &str) -> Vec<Token>{
    let mut items = Vec::new();
    let mut item = String::new();
    let mut previous_label = DateTimeLabel::Invalid;
    for ch in text.chars() {
        let label = get_datetime_label(&ch);
        if label != previous_label {
            if !item.is_empty() {
                items.push(Token {text: std::mem::take(&mut item),label: previous_label});
            }
            item.clear();
            previous_label = label;
        }
        item.push(ch);
    }
    if !item.is_empty() {
        items.push(Token{text: item, label: previous_label});
    }
    return items
}

pub fn reorder_text_meridian(text: &str) -> String{
    let mut words: Vec<&str> = text.split_whitespace().collect();
    let mut item =  String::from("");
    let mut text_time = String::from("");
    let mut text_meridian = String::from("");
    let mut position_time: i32 = -1;
    let mut position_meridian: i32 = -1;
    let mut distance:i32 = 0;
    for (index, word) in words.iter().enumerate() {
        if position_time > -1 && position_meridian > -1  && distance == 1 {
            break
        }
        if word == &"am" || word == &"pm"{
            text_meridian = word.to_string();
            position_meridian = index as i32;
            distance = (position_time - position_meridian).abs();
        }
        if !word.contains(":"){
            continue
        }
        match NaiveTime::parse_from_str(word, "%H:%M:%S"){
            Ok(_)=>{
                text_time = word.to_string();
                position_time = index as i32;
                distance = (position_time - position_meridian).abs();
                continue
            }
            Err(_) => {}
        }
        match NaiveTime::parse_from_str(word, "%H:%M"){
            Ok(_)=>{
                position_time = index as i32;
                text_time = word.to_string();
                distance = (position_time - position_meridian).abs();
                continue
            }
            Err(_) => {}
        }
    }
    if position_time < 0{return text.to_string()}
    if position_time > -1 && position_meridian > -1  && distance == 1 {
        item.push_str(text_time.as_str());
        item.push_str(" ");
        item.push_str(text_meridian.as_str());
        words[position_time as usize] = item.as_str();
        words[position_meridian as usize] = "";
        return words.join(" ")
    }
    item = text_time;
    if position_meridian != -1{
        words[position_meridian as usize] = "";
    }
    words[position_time as usize] = item.as_str();
    return words.join(" ")
}