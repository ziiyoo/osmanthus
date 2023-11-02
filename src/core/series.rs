use chrono::Duration;
use chrono::prelude::*;
use crate::bind::{ParseSeries, Param, Result, DateType, ResultDateText};
use crate::core::corpus::{get_offset_local_utc, has_symbol, unitize_month_name, unitize_month_text};
use crate::core::interfaces::{Parse};
use crate::utils::{eliminate_no_break_space, eliminate_symbol_normal, eliminate_symbol_safe, is_number, section_with_space, split_with_numeric, unitize_text, eliminate_empty_item, create_datetime, create_timestamp};

const LEN_SLIDER_SECTION: usize = 8;  // 年月日长度要求一定为8｜例如 20230115 20150630
const MAX_NUMBER_MONTH: u32 = 12;
const MAX_NUMBER_DAY: u32 = 31;
const MAX_NUMBER_HOURS: u32 = 24;
const MAX_NUMBER_MINUTES: u32 = 60;
const MAX_NUMBER_SECONDS: u32 = 60;
const MIN_NUMBER_YEAR: i32 = 1970;
const MAX_NUMBER_YEAR: i32 = 9999;


impl Parse for ParseSeries{
    fn parse(&mut self, text: &str) -> Result{
        let mut r = self.work(text);
        self.assemble(&mut r);
        return r
    }
}


impl ParseSeries{
    pub fn new(options: Option<Param>) -> ParseSeries{
        if let Some(param) = options{
            return ParseSeries{param}
        }
        return ParseSeries{..Default::default()}
    }

    fn work(&mut self, text: &str) -> Result {
        let mut data = Result { timezone: self.param.timezone.clone(), method: String::from("series"), ..Default::default() };
        // 循环 https://www.kingname.info/2022/03/09/this203506081102-is-gnelist/
        // 将连续有效字符放在一起 下一位不是数字时跳过 得到一组 [20220309, 203506081102]
        // 移动窗口 长度少于8意味着无法准确到天可以跳过
        // 得到多组数据
        // 年份推算 年份首位不可能是0、年份应该在1970～上限时间
        // 基于年份结果的日月分析
        // 去噪
        let mut item = self.eliminate(text);
        // 规整
        (item, _) = unitize_text(item.as_str());
        // 切分
        let mut section: Vec<String> = split_with_numeric(item);
        section = section_with_space(section);
        // 二次规整
        section = self.eliminate_noise_series(section);
        section = eliminate_empty_item(section);
        // 数值化
        let dates = self.numeric(section);
        // 按位分组
        let seeds = self.slider_window(&dates);
        // 正式解析
        if let Some(di) = self.reload_datetime_with_text_format(seeds){
            data.time = di;
            data.status = true;
        }
        return data
    }
    fn assemble(&self, item: &mut Result){
        // 无时区
        item.datetime.local.datetime = item.time;
        item.datetime.local.timestamp = create_timestamp(item.time);
        // TIMEZONE=LOCAL-DIFF
        item.datetime.timezone.datetime = item.time - Duration::seconds(get_offset_local_utc());
        item.datetime.timezone.timestamp = create_timestamp(item.datetime.timezone.datetime);
    }

    // 基于固有文本格式刷新时间对象
    fn reload_datetime_with_text_format(&self, seeds: Vec<String>) -> Option<NaiveDateTime>{
        let tail = " 00:00:00";
        for mut item in seeds{
            item.push_str(tail);
            match  NaiveDateTime::parse_from_str(item.as_str(), "%Y%m%d %H:%M:%S"){
                Err(_) => {}
                Ok(datetime) =>{
                    if self.validate(datetime){
                        return Some(datetime)
                    }
                }
            }
        }
        return None
    }

    /// 严格模式和限制模式的校验
    /// 校验日期数值合法性
    fn validate(&self, datetime: NaiveDateTime) -> bool{
        let mut threshold = MAX_NUMBER_YEAR;
        if self.param.strict{
            threshold = create_datetime(false, false).year();
        }
        if datetime.year() < MIN_NUMBER_YEAR || datetime.year() > threshold{
            return false
        }
        if datetime.month() > MAX_NUMBER_MONTH{
            return false
        }
        if  datetime.day() > MAX_NUMBER_DAY{
            return false
        }
        if datetime.hour() > MAX_NUMBER_HOURS{
            return false
        }
        if datetime.minute() > MAX_NUMBER_MINUTES{
            return false
        }
        if datetime.second() > MAX_NUMBER_SECONDS{
            return false
        }
        return true
    }

    fn eliminate(&self, text: &str) -> String{
        let mut item = eliminate_no_break_space(text);
        item = eliminate_symbol_safe(item.as_str());
        item = eliminate_no_break_space(item.as_str());
        return item.trim().to_string()
    }

    // 按位滑动窗口
    fn slider_window(&self, dates: &Vec<ResultDateText>) -> Vec<String>{
        let mut section:Vec<String> = Vec::new();
        for item in dates{
            let ceiling = item.text.chars().count();
            if item.text.chars().count() < LEN_SLIDER_SECTION{
                continue
            }
            for i in 0..ceiling{
                let current = i + LEN_SLIDER_SECTION;
                if current > ceiling{
                    continue
                }
                let sub = &item.text[i..current];
                section.push(sub.to_string());
            }
        }
        return section
    }

    /// 数值化
    fn numeric(&self, sec: Vec<String>) -> Vec<ResultDateText>{
        let mut sections: Vec<ResultDateText> = Vec::new();
        for (index, item) in sec.iter().enumerate(){
            if is_number(&item){
                sections.push(ResultDateText{index, text: item.to_string()});
                continue
            }
            if let Some(text) = unitize_month_text(&item){
                sections.push(ResultDateText{index, text});
            }
        }
        return sections
    }

    /// 消除无效字符
    /// 专门处理连续字符模式的无效字符
    /// 区别于常规模式对符号进行了限定
    fn eliminate_noise_series(&self, section:Vec<String>) -> Vec<String>{
        let mut data: Vec<String> = Vec::new();
        let mut items = Vec::new();
        for text in section{
            if is_number(text.as_str()){
                items.push(text.to_string());
                continue
            }
            let mut value:&str = "";
            let mut types = DateType::NONE;
            let text_for_search = eliminate_symbol_normal(text.as_str());
            if let Some(tv) = unitize_month_name(text_for_search.as_str()){
                value = tv;
                types = DateType::MONTH;
            }
            if has_symbol(&text){
                items.push(value.to_string());
                continue
            }
            if types != DateType::NONE {
                if let Some(text) = unitize_month_text(value){
                    items.push(text);
                    continue
                }
                items.push(value.to_string());
                continue
            }
            if items.len() == 0{
                continue
            }
            data.push(items.join(""));
            items = vec![];
        }
        return data
    }

}
