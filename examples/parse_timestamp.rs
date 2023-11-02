use osmanthus::parse_timestamp;
use osmanthus::bind::Param;

fn main() {
    let samples = vec![
        "1677380340",  // success
        "1677380340236982058745",  // parse fail
        "16773803abc",   // parse fail
        "你好，中国",   // parse fail
    ];
    for sample in samples{
        let r =parse_timestamp(sample, Some(Param{strict: true, ..Default::default()}));
        let datetime = r.datetime.local.datetime;
        println!("timestamp time text parse result: {:?}, status: {}", datetime.format("%Y-%m-%d %H:%M:%S").to_string(), r.status);
    }
}