# osmanthus丨桂花算法

<p>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/Performence-High-blue">
  </a>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/CodeLanguage-Rust-origin">
  </a>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/Compatibility-Powerful-white">
  </a>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/Timezone-Auto-pink">
  </a>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/LanguageSupport-World-origin">
  </a>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/StrictMode-Support-yellow">
  </a>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/TimeOrder-Auto-brown">
  </a>
  <a href="https://github.com/ziiyoo/osmantuhs/blob/main/LICENSE" target="_blank">
    <img src="https://img.shields.io/badge/License-GPL-green">
  </a>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/OtherCodeLanguage-ComingSoon-grey">
  </a>
</p>

[LANGUAGE CN 中文文档点击这里](https://github.com/ziiyoo/osmanthus/blob/main/README-ZH.md)

Find And Automatically Format Time Text From The String.

### Features

- Fast 
- High Performance
- Light
- Auto TimeZone
- 100+ Language

That can be widely used in scenarios such as news sentiment analysis, bidding, and data cleansing.

This document describes the detailed usage of the **osmanthus**, its powerful parsing performance, incredible compatibility, global language and timezone support, online experience, support for other programming languages, test cases, and interesting creative stories.

It supports the parsing and auto-formatting of time text in the following four types.

1. [x] absolute｜such as `2013年july18 10:03下午`
2. [x] relative｜such as `3小时前`、`2 minutes ago`
3. [x] timestamp｜such as`1685025365`、`1663025361000`
4. [x] series｜such as`https://example.com/20210315/img/2035.png`

### Performance
<p>
  <a href="#" target="_blank">
        <img src="https://img.shields.io/badge/Performence-High-blue">
  </a>
  <a href="#" target="_blank">
        <img src="https://img.shields.io/badge/Compatibility-Powerful-white">
  </a>
</p>

A single parsing takes only **microseconds(µs)** and even **nanoseconds(ns)**, and has **excellent compatibility**. 

Even if there are messy noise symbols and irrelevant other text in the input string, it can accurately recognize and format the correct time text.

## Use In Rust
<p>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/Language-Rust-origin">
  </a>
</p>

The following is a list of several different types of time text parsing examples. For more examples, please refer to the sample code in **benches** 和 **examples**。

1、**Absolute Time Text**
```rust
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
```

2、**Relative Time Text**
```rust
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
        println!("relative parse result: {:?}, status: {}", datetime.format("%Y-%m-%d %H:%M:%S").to_string(), r.status);
    }
}
```

3、**Timestamp Time Text**
```rust
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
        println!("timestamp parse result: {:?}, status: {}", datetime.format("%Y-%m-%d %H:%M:%S").to_string(), r.status);
    }
}
```

4、**Series Time Text**
```rust
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
```

## Benchmark

The performance testing of osmanthus uses **Criterion**，the code at **benches**.

```bash
/// Machine Mac Stucio 
/// Chip: Apple M1 Max 
/// Memory:32GB
/// OS: MacOS 14.0

parse_timestamp benchmark result:
                        time:   [302.51 ns 302.98 ns 303.49 ns]
                        change: [+0.3496% +0.6413% +0.9291%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

parse_series benchmark result:
                        time:   [24.324 µs 24.363 µs 24.407 µs]
                        change: [-0.3387% +0.1293% +0.5512%] (p = 0.58 > 0.05)
                        No change in performance detected.

parse_relative benchmark result:
                        time:   [525.93 µs 529.13 µs 533.43 µs]
                        change: [+0.4510% +1.0907% +1.8495%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

parse_absolute benchmark result:
                        time:   [45.841 µs 45.966 µs 46.114 µs]
                        change: [+0.6914% +1.0410% +1.4468%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe

```

## Compatibility
The parsing tendency of the **osmanthus** is to identify and parse time text as extensively and accurately as possible, thus cleaning up texts that are mixed with various types of noise.

### Noise

The **osmanthus** is not daunted by noise, whether it's Chinese characters, letters, numbers, punctuation marks, or even other languages.

For specific compatibility cases, you can refer to the relevant code in the **benches** and **example** directories. We also welcome everyone to provide more test samples.

### Timezone
<p>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/Timezone-Auto-pink">
  </a>
</p>

Since global support is provided, time zones naturally need to be taken into consideration. 

Currently, the osmanthus supports automatic calculation and UTC time conversion for 390 different time zones, including commonly used ones such as CST, MST, BST, HAST, and more.

For a detailed list, please refer to the documentation.

[TIMEZONE LIST](https://github.com/FIMERIC/Fime/blob/main/TIMEZONE.md)

In the time zones listed above, the osmanthus will automatically recognize and calculate the correct time during processing, and provide the time zone and UTC time of the current operating environment in the parsing results, making it convenient for everyone to convert according to their own business and region.

### The order of time
<p>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/Order-Auto-brown">
  </a>
</p>

The time formats vary across different parts of the world, with common ones being year-month-day, day-month-year, and month-day-year. The algorithm will automatically convert the time text based on its content and the order of appearance. For example:

```bash
2013.05/12 -> 2013-05-12 00:00:00  // Correct order, parsed directly
2013.05/july 15:00 -> 2013-07-05 15:00:00 // Month is definite, Adjust order
05,06,2021 13:00 -> 2021-06-05 13:00  // Month is uncertain, but this format is usually day-month-year， Adjust order
05,13,2021 13:00 -> 2021-05-13 13:00  // Month cannot be greater than 12, Order is actually definite
```

### Mode Strict
<p>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/Strict-Support-yellow">
  </a>
</p>

In the context of news and public opinion analysis, there is a common requirement to identify the time of news publication. One important aspect to note is that the publication time of news cannot be later than the current local time; it must be earlier. For example, if the current time is "2023-10-10 10:00:05," the collected news articles' publication time will always be earlier than the current time. It cannot be scheduled for tomorrow.

In strict mode, the osmanthus will determine whether the time text is greater than the current time value. If it is greater, the algorithm will skip the current suspicious text and proceed to identify the next suspicious text.


## Language Support 100+
<p>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/Language-100+-origin">
  </a>
</p>

The author is an entrepreneur whose main job involves the collection and analysis of global news data. Therefore, the recognition and parsing of temporal text must meet the demands of globalization. Whether it's Mandarin Chinese, Russian, Japanese, German, French, English, Korean, Bengali, Vietnamese, or dozens of other languages worldwide, all are already supported.

However, due to my language proficiency, some more "localized" expressions may not be adequately covered, such as `недавно` in Russian or `gerade eben` in German.

If you provide more samples, we would greatly appreciate it

## Possible defects
**The excessive compatibility leads to a decrease in accuracy**, making it prone to misinterpretation. For example, the string`12 batch 2021.05. 13 2023page`  should be correctly parsed as  `2021-05-13 00:00:00`,but parse result maybe `2021-12-05 00:00:00`。

However, if other libraries are used for parsing, there is a high possibility of parsing failure or inability to parse any valid temporal text.


## Support Other Code Language

> todo ...

## Test

We have manually collected and organized time texts from various countries or regions across the five continents worldwide. Combined with artificially constructed examples, we have obtained over 700 time samples in dozens of variations. These samples cover different time zones, languages, and expressions of different eras. Below are just a few examples provided for reference.
```azure
"令和3年12月7日" - Epoch expression in Japan
"26 ก.ค. 2566 08:00 น." - Epoch expression in Thai
"2013-05-06T11:30:22+02:00" - Time zone expression based on UTC time offset
"September 17, 2012 at 10:09am PST" - Clear time zone expression
"29/10/2020 10h38 Pm" - Hour abbreviation
" 4 Αυγούστου 2023, 00:01 " - Different languages
"H_502_5@2010oct03 @H_502_5@2012/07/26.doc" - Long Text and Noise
"发布于 - /n6小時前," - Short Text and Noise
... ...
```

The complete test cases are **only made available to the development team**. If you are considering applying the osmanthus to your project but are concerned about its parsing capabilities due to the lack of provided test samples, you can either prepare sufficient samples for testing on your own or reach out to the creator for assistance with testing.

## Create story

<p>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/working-happy-blue">
  </a>
</p>

The osmanthus was extracted from our team's commercial projects.

Our team's main business is global news data collection. On one hand, we provide real-time data to sentiment analysis or data technology companies for analysis. On the other hand, we provide long-text training corpora to AI companies in the NLP field.

One crucial element in news analysis is the publication time. Conventional regular expressions and some third-party libraries are difficult to meet the global parsing requirements

#### Inspiration

In python, dateparser is arguably the most widely used library for time text formatting. We also used it for a while, but we found that its compatibility was not strong, and its parsing ability significantly decreased in the presence of certain noise interference.

Later, we considered utilizing deep learning for classification and computation. While the classification capability was close to our requirements, the formatting and compatibility were completely uncontrollable, which was quite absurd.

In such a scenario and circumstances, I made the decision to design and develop a new time text parsing program, and that's where the osmanthus came into being.

#### Design and Reference

The osmanthus referenced some text processing methods and approaches from dateparser both before and during its design process. These included pre-set regular expressions, text denoising, time zone extraction, and classification processing.

Please don't assume that this is mere copying. In fact, dateparser's processing methods were not sufficient. Otherwise, Osmanthus would have simply become a Rust version of dateparser without any improvement in parsing capabilities.

We have designed several new processing logics that significantly enhance compatibility and accuracy while maintaining parsing capabilities. The inclusion of features such as automatic time zone calculation, sliding window, automatic time order switching, and strict mode ensures that the parsing capabilities of the osmanthus are far ahead.
#### Why open source

* Currently, there is no open-source tool available on the market that can match its capabilities and compatibility. Creating it was to address the technical shortcomings in our business data, and open-sourcing it is to prevent others from facing the same challenges we did at that time.
* Open sourcing enables better advancement of the osmanthus by gathering more samples, receiving feedback, and continuously optimizing it for higher performance and accuracy.
* Open sourcing serves as a way to showcase our technical capabilities to the outside world

> working happy ... ...

