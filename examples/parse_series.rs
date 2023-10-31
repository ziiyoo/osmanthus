use osmanthus::parse_series;
use osmanthus::bind::Param;

fn main() {
    let samples = vec![
        "https://www.kingname.info/2022/JULY309/this20350205-is-gnelist/",  // 2022-07-30 00:00:00"
        "H_502_5@2010oct03 @H_502_5@2012/07/26.doc",  // 2010-10-03 00:00:00
        "https://new.qq.com/rain/a/k09381120221126A03W2R00",  // 2022-11-26 00:00:00
        "/202211/W02022110720101102590.jpg", // 2022-11-07 00:00:00
        "http://cjrb.cjn.cn/html/2023-01/16/content_250826.htm" // 2023-01-16 00:00:00
    ];
    for sample in samples{
        let r =parse_series(sample, Some(Param{strict: true, ..Default::default()}));
        let datetime = r.datetime.local.datetime;
        println!("series parse result: {:?}, status: {}", datetime.format("%Y-%m-%d %H:%M:%S").to_string(), r.status);
    }
}