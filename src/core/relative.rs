use std::collections::HashMap;
use chrono::prelude::*;
use chrono::Duration;
use crate::bind::{ParseRelative, Param, Result};
use crate::core::interfaces::{Parse};
use crate::core::corpus::{get_offset_local_utc, unitize_relative_text};
use crate::utils::{create_datetime, create_timestamp, eliminate_noise, split_with_numeric, str_convert};

const MAX_SECTION_LEN_TRUSTED: usize = 5;
const MAX_TEXT_LEN_TRUSTED: usize = 12;
const MAX_NUMBER_MONTH: i64 = 12;
const MAX_NUMBER_DAY: i64 = 31;
const MAX_NUMBER_WEEK: i64 = 7;
const MAX_NUMBER_HOURS: i64 = 24;
const MAX_NUMBER_MINUTES: i64 = 60;
const MAX_NUMBER_SECONDS: i64 = 60;

impl Parse for ParseRelative{
    fn parse(&mut self, text: &str) -> Result{
        let mut r = self.work(text);
        self.assemble(&mut r);
        return r
    }
}


impl ParseRelative{
    pub fn new(options: Option<Param>) -> ParseRelative{
        if let Some(param) = options{
            return ParseRelative{param}
        }
        return ParseRelative{..Default::default()}
    }

    fn assemble(&self, item: &mut Result){
        // 无时区
        item.datetime.local.datetime = item.time;
        // println!("local datetime: {:?}", item.datetime.local.datetime);
        item.datetime.local.timestamp = create_timestamp(item.time);
        // TIMEZONE=LOCAL-DIFF
        item.datetime.timezone.datetime = item.time - Duration::seconds(get_offset_local_utc());
        item.datetime.timezone.timestamp = create_timestamp(item.datetime.timezone.datetime);
    }
    fn work(&self, text: &str) -> Result{
        let mut data = Result {method: String::from("relative"), ..Default::default() };
        // 去噪
        let mut item = eliminate_noise(text);
        // 规整
        item =  self.unitize_text(item.as_str());
        // 切分
        let section: Vec<String> = split_with_numeric(item);
        // 元素越多可信度越低
        if section.len() >= MAX_SECTION_LEN_TRUSTED{
            return data
        }
        let numeric = self.search_numeric(&section);
        for (index, item) in section.iter().enumerate() {
            if self.param.strict && item.chars().count() > MAX_TEXT_LEN_TRUSTED{
                continue
            }
            let result_relative_text = unitize_relative_text(item.as_str());
            if !result_relative_text.hit{continue}
            if result_relative_text.same && numeric.len() == 0{
                // 完全等于的情况下使用0时
                data.time = create_datetime(true, false);
                data.status = true;
                return data
            }
            for i in (0..index).rev(){
                if let Some(n) = numeric.get(&i){
                    if let Some(di) = self.reload_datetime_with_text(&result_relative_text.text, *n){
                        data.status = true;
                        data.time = di;
                        return data
                    }
                }
            }
        }
        return data
    }


    fn reload_datetime_with_text(&self,text: &str, number: i64) -> Option<NaiveDateTime>{
        let datetime = Local::now().naive_local();
        match text{
            "year ago" =>{
                let date = datetime.date();
                if let Some(effective) = NaiveDate::from_ymd_opt(date.year() - number as i32, date.month(), date.day()){
                    let item = NaiveDateTime::new(effective, datetime.time());
                    return Some(item)
                }
                return None
            }
            "month ago" =>{
                if number > MAX_NUMBER_MONTH{
                    return None
                }
                let date = datetime.date();
                if let Some(effective) = NaiveDate::from_ymd_opt(date.year(), date.month() - number  as u32, date.day()){
                    let item = NaiveDateTime::new(effective, datetime.time());
                    return Some(item)
                }
                return None
            }
            "week ago" =>{
                if number > MAX_NUMBER_WEEK{
                    return None
                }
                return Some(datetime - Duration::weeks(number))
            }
            "day ago" =>{
                if number > MAX_NUMBER_DAY{
                    return None
                }
                return Some(datetime - Duration::days(number))
            }
            "hour ago" =>{
                if number > MAX_NUMBER_HOURS{
                    return None
                }
                return Some(datetime - Duration::hours(number))
            }
            "minute ago" =>{
                if number > MAX_NUMBER_MINUTES{
                    return None
                }
                return Some(datetime - Duration::minutes(number))
            }
            "second ago" =>{
                if number > MAX_NUMBER_SECONDS*MAX_NUMBER_SECONDS{
                    return None
                }
                return Some(datetime - Duration::seconds(number))
            }
            _ => {}
        }
        return None
    }

    fn search_numeric(&self, section: &Vec<String>) -> HashMap<usize, i64>{
        let mut data:HashMap<usize, i64> = HashMap::new();
        for (index, item) in section.iter().enumerate() {
            if let Some(number) = str_convert(item.as_str()){
                if number <1 || number > (MAX_NUMBER_SECONDS*MAX_NUMBER_SECONDS) as i32{
                    continue
                }
                data.insert(index, number as i64);
            }
        }
        return data
    }

    fn unitize_text(&self, text: &str) -> String{
        let mut data: Vec<String> = Vec::new();
        let section: Vec<&str> = text.split(" ").collect();
        for text in section{
            let item = text.to_lowercase();
            data.push(item);
        }
        return data.join(" ")
    }
}
