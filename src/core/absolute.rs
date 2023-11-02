use chrono::Duration;
use chrono::prelude::*;
use crate::bind::*;
use crate::core::interfaces::{Parse};
use crate::core::corpus::{unitize_date_text, search_meridian, unitize_month_numeric, get_offset_local_utc, unitize_spacial_express_time, search_era_japan, search_with_utc_pattern, search_dubious_date, unitize_timezone_with_text};
use crate::utils::{eliminate_noise, unitize_text, split_with_numeric,
                   section_with_space, search_offset_with_text, is_number,
                   create_datetime, create_timestamp, str_convert,
                   eliminate_symbol_point, tokenize, reorder_text_meridian};

const MAX_NUMBER_MONTH: u32 = 12;
const MAX_NUMBER_DAY: u32 = 31;
const MAX_NUMBER_HOURS: u32 = 24;
const MAX_NUMBER_MINUTES: u32 = 60;
const MAX_NUMBER_SECONDS: u32 = 60;
const MIN_NUMBER_YEAR: u32 = 1970;
const MAX_NUMBER_YEAR: u32 = 9999;
const MIN_NUMBER_YEAR_THAI: i32 = 2513;  // 泰历相对于公历多 543 年因此最小的泰历时间: 1970 + 543
const DIFF_NUMBER_YEAR_THAI: u32 = 543;
const MAX_LEN_TIME_TEXT: usize = 5;  // 11:02


impl Parse for ParseAbsolute{
    fn parse(&mut self, text: &str) -> Result{
        let item = self.pretreatment(text);
        let mut r = self.work(&item);
        self.assemble(&mut r);
        return r
    }
}


impl ParseAbsolute{
    pub fn new(options: Option<Param>) -> ParseAbsolute{
        if let Some(param) = options{
            return ParseAbsolute{param, ..Default::default()}
        }
        return ParseAbsolute{..Default::default()}
    }

    fn assemble(&self, item: &mut Result){
        self.attach_era(item);
        self.attach_timezone(item);
    }

    fn attach_era(&self, item: &mut Result){
        match self.era{
            EraBasedCalendar::Thai => {
                // 判断是否符合泰历年份
                if item.time.year() > MIN_NUMBER_YEAR_THAI{
                    if let Some(di) = item.time.with_year(item.time.year()-DIFF_NUMBER_YEAR_THAI as i32){
                        item.time = di;
                    }
                }
            }
            _=> {}
        }
    }


    /// 预处理
    fn pretreatment(&mut self, text: &str) -> String{
        // 时区和偏移量
        if self.param.timezone == "".to_string(){
            if let Some(offset) = self.search_timezone_with_offset(text){
                // 此处场景｜其他时区转换为协调时区需取反
                // 例如 2023-09-10 10:15:20+06:00 转为协调时区需要减去6
                // 例如 2023-09-10 10:15:20-07:00 转为协调时区需要加上7
                self.offset = offset;
                match offset.abs(){
                    0 => {self.param.timezone = "utc".to_string();}
                    _ => {self.param.timezone = "other".to_string();}
                }
            }
        }else{
            if let Some(n) = unitize_timezone_with_text(self.param.timezone.as_str()){
                self.offset = *n;
            }
        }
        // 特殊时间表达式的处理
        let mut temp: String = unitize_spacial_express_time(text);
        temp = search_era_japan(&temp);
        return temp
    }

    fn search_timezone_with_offset(&mut self, text: &str) -> Option<i32>{
        if let Some(offset) = self.search_offset_with_patterns(text){
            return Some(offset)
        }
        return None
    }

    fn search_offset_with_patterns(&self, text: &str) -> Option<i32>{
        for name in vec!["TZZeroOffset", "TZOffset", "TZZero"]{
            if let Some(offset) = self.find_timezone_with_captures(text, name){
                return Some(offset)
            }
        }
        if let Some(pattern) = search_with_utc_pattern("TZ"){
            for capture in pattern.captures_iter(text){
                if capture.len() == 0{continue}
                return Some(0)
            }
        }
        return None
    }

    fn find_timezone_with_captures(&self, text: &str, name: &str) -> Option<i32>{
        if let Some(pattern) = search_with_utc_pattern(name){
            for capture in pattern.captures_iter(text){
                if capture.len() == 0{continue}
                if let Some(value) = capture.get(capture.len()-1){
                    let offset = self.search_offset_with_text_format(value.as_str());
                    return Some(offset)
                }
            }
        }
        return None
    }


    /// 基于文本固有格式获取偏移量
    fn search_offset_with_text_format(&self, text: &str) -> i32{
        let mut offset = 0;
        if text.chars().count() < MAX_LEN_TIME_TEXT{
            return offset
        }
        let pure = text.replace(":", "").replace("0", "");
        if let Some(tail) = pure.get(1..){
            if let Some(number) = str_convert(tail){
                offset = number
            }
        }
        match pure.chars().next(){
            Some('+') => {
                return offset * 3600
            }
            Some('-') => {
                return offset * -3600
            },
            _ => {}
        }
        return offset
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
                // println!("local: {:?}", item.datetime.local.datetime);
                return
            }
            _ =>{
                // 指定时区非协调时区
                // 其他时区转换为协调时区
                // println!("附加时区前: {:?}", item.time);
                let temp = item.time - Duration::seconds(self.offset as i64);
                item.datetime.timezone.datetime = temp;
                item.datetime.timezone.timestamp = create_timestamp(temp);
                // println!("附加时区后｜协调时区时间: {:?}", temp);
                // LOCAL=UTC+DIFF
                item.datetime.local.datetime = item.datetime.timezone.datetime + Duration::seconds(get_offset_local_utc());
                item.datetime.local.timestamp = create_timestamp(item.datetime.local.datetime);
                // println!("协调时区转本地时区: {:?}", item.datetime.local.datetime);
                return
            }
        }
    }

    fn skip_unitize_date_text(&self, section:& Vec<String>, text: &str) -> bool{
        if text.trim() == "ago" && (
            section.contains(&"day".to_string())||
            section.contains(&"days".to_string())||
            section.contains(&"year".to_string())||
            section.contains(&"years".to_string())||
            section.contains(&"month".to_string())||
            section.contains(&"weeks".to_string())||
            section.contains(&"week".to_string())||
            section.contains(&"second".to_string())||
            section.contains(&"seconds".to_string())||
            section.contains(&"hour".to_string())||
            section.contains(&"hours".to_string())||
            section.contains(&"minute".to_string())||
            section.contains(&"minutes".to_string())
        ){return true}
        return false
    }

    /// 消除无效字符
    /// 既不是数字也不在有效字符范围内的用空字符串代替
    fn eliminate_noise_normal(&self,section:Vec<String>) -> Vec<String>{
        let mut res: Vec<String> = Vec::new();
        for text in &section{
            if is_number(text.as_str()){
                res.push(text.to_string());
                continue
            }
            if self.skip_unitize_date_text(&section, text){
                continue
            }
            if let Some(value) = unitize_date_text(text.as_str()){
                res.push(value);
                continue
            }
            res.push("".to_string());
        }
        return res
    }

    /// 元素重排
    /// 时分秒格式中存在的":"符号会影响元素分隔
    /// 重排消除这种影响
    fn reorder(&self, items: Vec<String>) -> String{
        let mut text = String::from("");
        if items.len() == 0{
            return text
        }
        for i in 1..items.len(){
            let current = items[i-1].as_str();
            let next = items[i].as_str();
            if current.is_empty(){
                continue
            }
            text.push_str(current);
            if current == ":" || next == ":"{
                continue
            }
            text.push_str(" ");
        }
        text.push_str(items[items.len()-1].as_str());
        return text
    }

    /// 字符转换成 TOKEN
    /// 方便后续处理
    fn tokenize(&self, item: &str) -> Vec<Token> {
        let mut token = tokenize(item);
        token = self.extract_effective_item(token);
        return token
    }

    /// 只取有效（后续可解析）的元素
    fn extract_effective_item(&self, token: Vec<Token>) -> Vec<Token>{
        let mut res: Vec<Token> = Vec::new();
        for v in token{
            if v.label == DateTimeLabel::Invalid{continue}
            res.push(v)
        }
        return res
    }

    /// 正式解析
    fn parse_token(&self) -> (NaiveDateTime, bool){
        let mut mark = TimeMark{..Default::default()};
        let mut param_utc = false;
        if self.param.timezone != "".to_string() || self.offset != 0{
            param_utc = true;
        }
        let mut month_force = false;
        let mut padding_order: Vec<String> = Vec::new();
        let mut datetime: NaiveDateTime = create_datetime(true, param_utc);
        for (index, item) in self.token.iter().enumerate(){
            match item.label{
                DateTimeLabel::Numeric =>{
                    // 时间类数据
                    if item.text.contains(":") && !mark.time.status && item.text.chars().count() > 2{
                        (datetime, mark.time.status) = self.parse_time_text(index, item, &self.token, datetime);
                    }
                    // 数字型日期类数据
                    let (number, datetime_type, force) = self.parse_number(item);
                    padding_order.push(self.get_order(&datetime_type));
                    datetime = self.reload_datetime(datetime, number, datetime_type, &mut mark, item.label.clone(), force);
                }
                DateTimeLabel::Characters => {
                    // 字符型日期类数据
                    let (number, datetime_type, force) = self.parse_alpha(item);
                    if datetime_type == DateType::MONTH && force{
                        month_force = force;
                    }
                    padding_order.push(self.get_order(&datetime_type));
                    datetime = self.reload_datetime(datetime, number, datetime_type, &mut mark, item.label.clone(), force);
                }
                _ => {}
            }
        }
        if mark.year.status && mark.month.status && mark.day.status && self.validate(datetime){
            let d = self.reload_datetime_with_force(month_force, datetime, padding_order);
            return (d, true)
        }
        return (datetime, false)
    }

    fn get_order(&self, datetime_type: &DateType) -> String{
        match datetime_type{
            DateType::MONTH => {
                return String::from("m")
            }
            DateType::YEAR => {
                return String::from("y")
            }
            _ => {}
        }
        return String::from(" ")
    }

    /// 基于是否强月份和填充顺序更新时间
    /// 主要是处理 06-07-2023 这类日月年时间格式
    /// 将其转化为 2023-07-06 年月日
    fn reload_datetime_with_force(&self, force: bool, datetime: NaiveDateTime, padding_order: Vec<String>) -> NaiveDateTime{
        let day = datetime.day();
        let month = datetime.month();
        let mut order: String = String::from("");
        for i in padding_order{
            if i.trim().is_empty(){
                continue
            }
            order.push_str(i.as_str());
        }
        if day <= MAX_NUMBER_MONTH && month <= MAX_NUMBER_MONTH && !force && order == "mmy"{
            if let Some(di) = datetime.with_month(day){
                if let Some(d) = di.with_day(month){
                    return d
                }
            }
        }
        return datetime
    }

    fn search_sibling_meridian(&self, index: usize, token: &Vec<Token>) -> Option<String>{
        let next = index + 1;
        // 尝试匹配午线,匹配到就覆盖
        if next < token.len(){
            let fake = token[next].text.as_str();
            if let Some(meridian) = search_meridian(fake){
                return Some(meridian)
            }
        }
        if index > 0{
            let prev = index - 1;
            let fake = token[prev].text.as_str();
            if let Some(meridian) = search_meridian(fake){
                return Some(meridian)
            }
        }
        return None
    }


    /// 严格模式和限制模式的校验
    /// 校验日期数值合法性
    fn validate(&self, datetime: NaiveDateTime) -> bool{
        if datetime.year() < MIN_NUMBER_YEAR as i32{
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
        if self.param.strict{
            let reference = create_datetime(false, false);
            let di = datetime + Duration::seconds(self.offset as i64);
            if reference < di && self.era == EraBasedCalendar::Non{
                return false
            }
        }
        return true
    }

    /// 解析字符型时间数据
    fn parse_time_text(&self, index: usize, item: &Token, token: &Vec<Token>, mut datetime: NaiveDateTime) -> (NaiveDateTime, bool){
        let mut status_mark_time: bool = false;
        let mut meridian = String::from("");  // 午线默认为空
        if let Some(v) = self.search_sibling_meridian(index, token){
            meridian = v;
        }
        let time = self.parse_time_with_text_format(&item.text, &meridian);
        if let Some(di) = datetime.with_hour(time.hour()){
            status_mark_time = true;
            datetime = di;
        }
        if let Some(di) = datetime.with_minute(time.minute()){
            datetime = di;
        }
        if let Some(di) = datetime.with_second(time.second()){
            datetime = di
        }
        return (datetime, status_mark_time)
    }

    /// 刷新时间对象
    fn reload_datetime(&self, datetime: NaiveDateTime, number: u32, datetime_type: DateType, mark: &mut TimeMark, label: DateTimeLabel, force: bool) -> NaiveDateTime{
        match datetime_type{
            DateType::YEAR => {
                if let Some(d) = datetime.with_year(number as i32){
                    if mark.year.status{
                        return datetime
                    }
                    mark.year.status = true;
                    mark.year.label = label;
                    mark.year.value = number;
                    return d
                }
            }
            DateType::MONTH => {
                // 月份｜值和类型确定的情况下可以强行设定
                if force{
                    if let Some(mut dm) = datetime.with_month(number){
                        if !mark.day.status {
                            if let Some(d) = dm.with_day(mark.month.value){
                                mark.day.value = mark.month.value;
                                mark.day.label = mark.month.label.clone();
                                mark.day.status = true;
                                dm = d;
                            }
                        }
                        mark.month.status = true;
                        mark.month.label = label;
                        mark.month.value = number;
                        return dm
                    }
                }
                // 月份｜类型相同且都为字符串类型的情况下可以强行设定
                if mark.month.label == DateTimeLabel::Characters && label == DateTimeLabel::Characters && number <= MAX_NUMBER_MONTH{
                    if let Some(d) = datetime.with_month(number){
                        mark.month.status = true;
                        mark.month.label = label;
                        mark.month.value = number;
                        return d
                    }
                }
                // 月份｜符合条件且月份信息没有锁定的情况下可以设定
                if number <= MAX_NUMBER_MONTH && !mark.month.status{
                    if let Some(d) = datetime.with_month(number) {
                        mark.month.status = true;
                        mark.month.label = label;
                        mark.month.value = number;
                        return d
                    }
                }
                // 天｜符合条件且天信息没有锁定的情况下可以设定
                if number <= MAX_NUMBER_DAY && !mark.day.status{
                    if let Some(d) = datetime.with_day(number){
                        mark.day.status = true;
                        mark.day.label = label;
                        mark.day.value = number;
                        return d
                    }
                }
            }
            _ => {}
        }
        return datetime
    }

    /// 基于固有格式解析时分秒和时区
    fn parse_time_with_text_format(&self, item: &str, meridian: &str) -> NaiveTime{
        let patterns = vec![
            String::from("%-I:%M:%S %P"),
            String::from("%-I:%M %P"),
            String::from("%I:%M %P"),
            String::from("%I:%M:%S %p"),
            String::from("%H:%M %p"),
            String::from("%H:%M:%S"),
            String::from("%H:%M")];
        let mut text = String::from(item);
        text.push_str(meridian);
        for pattern in &patterns{
            let naive = NaiveTime::parse_from_str(text.as_str(), pattern.as_str());
            match naive {
                Ok(di) => {
                    return di
                },
                _ => {}
            }
        }
        for pattern in &patterns{
            let naive = NaiveTime::parse_from_str(item, pattern.as_str());
            match naive {
                Ok(di) => {
                    return di
                },
                _ => {}
            }
        }
        return create_datetime(true, false).time()
    }

    /// 解析数字型日期类数据
    fn parse_number(&self, token : &Token) -> (u32, DateType, bool){
        /*
        判断冒号是否在其中
        数字类型，首先取长度
        然后转数字
        接着判断是否在1～31之间
        如果是，则是
        */
        match token.text.parse(){
            Ok(number) => {
                if number > 0 && number <= MAX_NUMBER_DAY{
                    return (number, DateType::MONTH, false)
                }
                if number < MIN_NUMBER_YEAR || number > MAX_NUMBER_YEAR && !self.param.strict{
                    return (0, DateType::NONE, false)
                }
                let datetime = create_datetime(false, false);
                if self.param.strict && self.era == EraBasedCalendar::Non{
                    // 严格模式下限定年份不超过当前年份
                    if number as i32 <= datetime.year(){
                        return (number, DateType::YEAR, true)
                    }
                }else{
                    return (number, DateType::YEAR, true)
                }
            }
            Err(_) => {
                return (0, DateType::NONE, false)
            }
        }
        return (0, DateType::NONE, false)
    }

    /// 解析字符型日期类数据
    fn parse_alpha(&self, token : &Token)-> (u32, DateType, bool){
        /*
        是否在星期中或者月份中
        是否在上午下午中
        如果在则转为数字
        然后返回
        */
        if let Some(month) = unitize_month_numeric(token.text.as_str()){
            return (*month, DateType::MONTH, true)
        }
        return (0, DateType::NONE, false)
    }

    fn mark_dubious_date(&mut self, text: &str) -> String{
        if let Some(item) = search_dubious_date(text){
            let token = self.tokenize(&item);
            self.token.extend(token);
            return text.replace(&item, "")
        }
        return text.to_string()
    }

    fn work(&mut self, text: &str) -> Result {
        let mut data = Result { timezone: self.param.timezone.clone(), method: String::from("absolute"), ..Default::default()};
        // 匹配可疑的日期文本｜同时提高后续解析时的优先级
        let text = self.mark_dubious_date(text);
        // 去噪
        let item = eliminate_noise(text.as_str());
        // 规整
        let (mut seed, era) = unitize_text(&item);
        self.era = era;
        seed = eliminate_symbol_point(seed.as_str());
        // 切分
        let seed= reorder_text_meridian(seed.as_str());
        let mut section: Vec<String> = split_with_numeric(seed);
        section = section_with_space(section);
        // 附加属性
        let (offset, timezone) = search_offset_with_text(&section);
        self.apply_offset_timezone(offset, timezone);
        // 二次规整
        section = self.eliminate_noise_normal(section);
        let entry = self.reorder(section);
        // 正式解析
        let token:Vec<Token> = self.tokenize(&entry);
        self.token.extend(token);
        let (datetime, status) = self.parse_token();
        data.time = datetime;
        data.status = status;
        return data
    }

    fn apply_offset_timezone(&mut self, offset: i32, timezone: String){
        if self.param.timezone == ""{
            self.offset = offset;
            self.param.timezone = timezone;
        }
    }
}
