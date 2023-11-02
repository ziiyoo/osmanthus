use osmanthus::parse;
use osmanthus::bind::Param;

fn main() {
    let samples = vec![
        "https://www.kingname.info/2022/JULY309/this20350205-is-gnelist/",  // series, 2022-07-30 00:00:00"
        "3/08/2023 | 11:51",  // absolute, 2023-08-03 11:51:00
        "发布于 - /n6小時前,",  // relative, 6 hours ago
        "/202211/W02022110720101102590.jpg", // series, 2022-11-07 00:00:00
        "1677380340" // timestamp, 2023-02-26 10:59:00
    ];
    for sample in samples{
        let r =parse(sample, Some(Param{strict: true, ..Default::default()}));
        let datetime = r.datetime.local.datetime;
        println!("auto parse result: {:?}, status: {}", datetime.format("%Y-%m-%d %H:%M:%S").to_string(), r.status);
    }
}