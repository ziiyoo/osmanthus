use chrono::{Duration, NaiveDateTime};
use crate::bind::{Param, Result, ParseTimestamp};
use crate::core::interfaces::{Parse};
use crate::utils::{is_timestamp, create_timestamp};
use crate::core::corpus::{get_offset_local_utc, unitize_timezone_with_text};

const LEN_TIMESTAMP_SECOND: usize = 10;
const LEN_TIMESTAMP_MILLISECOND: usize = 13;

impl Parse for ParseTimestamp{
    fn parse(&mut self, text: &str) -> Result{
        let mut r = self.work(text);
        self.assemble(&mut r);
        return r
    }
}


impl ParseTimestamp{
    pub fn new(options: Option<Param>) -> ParseTimestamp{
        if let Some(param) = options{
            let mut ins = ParseTimestamp{param, ..Default::default()};
            ins.set_offset_with_timezone();
            return ins
        }
        return ParseTimestamp{..Default::default()}
    }

    fn set_offset_with_timezone(&mut self){
        if let Some(offset) = unitize_timezone_with_text(self.param.timezone.as_str()){
            self.offset = *offset;
        }else{
            self.param.timezone = "".to_string();
        }
    }

    fn work(&self, text: &str) -> Result{
        let mut item = Result{timezone: self.param.timezone.clone(), method: String::from("timestamp"), ..Default::default()};
        if !is_timestamp(text){
            return item
        }
        let (timestamp, hit) = self.extract_timestamp(text);
        if !hit{
            return item
        }
        match text.chars().count(){
            LEN_TIMESTAMP_SECOND => {
                if let Some(v) = self.timestamp_convert(timestamp * 1000){
                    item.time = v;
                    item.status = true;
                }
            }
            LEN_TIMESTAMP_MILLISECOND => {
                if let Some(v) = self.timestamp_convert(timestamp){
                    item.time = v;
                    item.status = true;
                }
            }
            _ => {}
        }
        return item
    }

    fn timestamp_convert(&self, timestamp: i64) -> Option<NaiveDateTime>{
        if let Some(temp) = NaiveDateTime::from_timestamp_millis(timestamp){
            let di = temp + Duration::seconds(get_offset_local_utc());
            return Some(di)
        }
        return None
    }

    fn assemble(&self, item: &mut Result){
        self.attach_timezone(item);
    }

    /// 附加时区属性
    fn attach_timezone(&self, item: &mut Result){
        item.timezone = self.param.timezone.clone();
        match self.param.timezone.as_str(){
            "" => {
                // 无时区
                item.datetime.local.datetime = item.time;
                item.datetime.local.timestamp = create_timestamp(item.time);
                // TIMEZONE=LOCAL-DIFF
                item.datetime.timezone.datetime = item.time - Duration::seconds(get_offset_local_utc());
                item.datetime.timezone.timestamp = create_timestamp(item.datetime.timezone.datetime);
                return
            }
            "utc" => {
                // 指定时区为协调时区
                item.datetime.timezone.datetime = item.time;
                item.datetime.timezone.timestamp = create_timestamp(item.time);
                // LOCAL=UTC+DIFF
                item.datetime.local.datetime = item.datetime.timezone.datetime + Duration::seconds(get_offset_local_utc());
                item.datetime.local.timestamp = create_timestamp(item.datetime.local.datetime);
            }
            _ =>{
                // 指定时区非协调时区
                // 其他时区转换为协调时区
                // println!("item.time: {:?}, offset: {}", item.time, self.offset);
                let temp = item.time - Duration::seconds(self.offset as i64);
                item.datetime.timezone.datetime = temp;
                item.datetime.timezone.timestamp = create_timestamp(temp);
                // println!("temp: {:?}", temp);
                // LOCAL=UTC+DIFF
                item.datetime.local.datetime = item.datetime.timezone.datetime + Duration::seconds(get_offset_local_utc());
                item.datetime.local.timestamp = create_timestamp(item.datetime.local.datetime);
                // println!("local: {:?}", item.datetime.local.datetime);
                return
            }
        }
    }

    fn extract_timestamp(&self, text: &str) -> (i64, bool){
        let r = text.parse();
        return match r {
            Ok(number) => {(number, true)},
            Err(_) => {(0, false)}
        }
    }
}
