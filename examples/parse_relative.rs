use osmanthus::parse_relative;
use osmanthus::bind::Param;

fn main() {
    let samples = vec![
        "发布于 - /n6小時前,",  // 6 hours ago
        "（ 시간: 3분 전）", // 3 minute ago
        "- about / 2 minutes ago", // 2 minutes ago
        "30天前 来源：新华网", // 30 days ago
        "publish 5 second ago." // 5 second ago.
    ];
    for sample in samples{
        let r =parse_relative(sample, Some(Param{strict: true, ..Default::default()}));
        let datetime = r.datetime.local.datetime;
        println!("relative time text parse result: {:?}, status: {}", datetime.format("%Y-%m-%d %H:%M:%S").to_string(), r.status);
    }
}