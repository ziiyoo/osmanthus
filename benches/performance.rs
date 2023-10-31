#![allow(unused)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use osmanthus::{parse_absolute, parse_relative, parse_timestamp, parse_series};
use osmanthus::bind::Param;
pub fn criterion_benchmark_parse_series(c: &mut Criterion) {
    let samples = vec![
        "https://www.kingname.info/2022/JULY309/this20350205-is-gnelist/",
        "H_502_5@2010oct03 @H_502_5@2012/07/26.doc",
        "https://new.qq.com/rain/a/k09381120221126A03W2R00",
        "/202211/W02022110720101102590.jpg",
        "http://cjrb.cjn.cn/html/2023-01/16/content_250826.htm"
    ];
    c.bench_function(
        "parse_series benchmark result:", |b| {
            b.iter(||{
                for item in &samples{
                    parse_series(item, Some(Param{strict: true, ..Default::default()}));
                }
            });
        }
    );
}

pub fn criterion_benchmark_parse_relative(c: &mut Criterion) {
    let samples = vec![
        "发布于 - /n6小時前,",
        "（ 시간: 3분 전）",
        "- about / 2 minutes ago",
        "30天前 来源：新华网",
        "publish 5 days ago."
    ];
    c.bench_function(
        "parse_relative benchmark result:", |b| {
            b.iter(||{
                for item in &samples{
                    parse_relative(item, Some(Param{strict: true, ..Default::default()}));
                }
            });
        }
    );
}

pub fn criterion_benchmark_parse_absolute(c: &mut Criterion) {
    let samples = vec![
        "11/08/2023 | 11:51",
        "aug 06 .2023 10h42",
        "2013年12月8号 下午 3:00",
        "2023-07-30T14:12:51+02:00",
        "26 ก.ค. 2566 08:00 น.",
        "2014年04月08日11时25分18秒 下午",
        "2023-02-05 10:03:37 pm cst",
    ];
    c.bench_function(
        "parse_absolute benchmark result:", |b| {
            b.iter(||{
                for item in &samples{
                    parse_absolute(item, Some(Param{strict: true, ..Default::default()}));
                }
            });
        }
    );
}

pub fn criterion_benchmark_parse_timestamp(c: &mut Criterion) {
    let samples = vec![
        "1677380340",
        "1677380340236982058745",
        "16773803abc",
        "你好，中国",
    ];
    c.bench_function(
        "parse_timestamp benchmark result:", |b| {
            b.iter(||{
                for item in &samples{
                    parse_timestamp(item, Some(Param{strict: true, ..Default::default()}));
                }
            });
        }
    );
}

criterion_group!(benches, criterion_benchmark_parse_timestamp, criterion_benchmark_parse_series, criterion_benchmark_parse_relative,
    criterion_benchmark_parse_absolute);
criterion_main!(benches);