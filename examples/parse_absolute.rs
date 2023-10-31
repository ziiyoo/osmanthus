use osmanthus::parse_absolute;
use osmanthus::bind::Param;

fn main() {
    let samples = vec![
        "3/08/2023 | 11:51",  // 2023-08-03 11:51:00
        "aug 06 .2023 10h42",  // 2023-08-06 10:42:00"
        "2013年12月8号 pm 3:00",  // 2013-12-08 15:00:00
        "26 ก.ค. 2566 08:00 น.",  // 2023-07-26 08:00:00
        "2014年04月08日11时25分18秒 下午",  // 2014-04-08 23:25:18
        "2023-02-05 10:03:37 pm cst",
        "2023-07-30T14:12:51+02:00",
    ];
    for sample in samples{
        let r =parse_absolute(sample, Some(Param{strict: true, ..Default::default()}));
        let datetime = r.datetime.local.datetime;
        println!("series parse result: {:?}, status: {}", datetime.format("%Y-%m-%d %H:%M:%S").to_string(), r.status);
    }
}