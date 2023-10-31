# osmanthus 桂花算法

<p>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/性能-强的不像话-blue">
  </a>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/Language-Rust-origin">
  </a>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/兼容性-简直离谱-white">
  </a>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/时区-自动计算-pink">
  </a>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/多语种-全球多个地区-origin">
  </a>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/全球化-支持-orange">
  </a>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/严格模式-支持-yellow">
  </a>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/时间顺序-自动切换-brown">
  </a>
  <a href="https://github.com/ziiyoo/osmantuhs/blob/main/LICENSE" target="_blank">
    <img src="https://img.shields.io/badge/License-GPL-green">
  </a>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/其他编程语言-即将支持-grey">
  </a>
</p>

[LANGUAGE EN CLICK HERE](https://github.com/ziiyoo/osmantuhs)

**osmanthus** 中文名称为桂花，看名字你就知道它有多香

它是一款**超高性能**且支持**多语种**的**时间文本自动格式化**工具库，可广泛用于新闻舆情、招投标、数据清洗等情景。


这份文档描述了**桂花算法**详细的使用方法、强悍的解析性能、离谱的兼容性、全球化的语种和时区支持、在线体验、其他编程语言支持、测试用例和有趣的创造故事。

它支持以下 4 种类型的时间文本解析与自动格式化
1. [x] 绝对时间｜例如`2013年july18 10:03下午`
2. [x] 相对时间｜例如`3小时前`、`2 minutes ago`
3. [x] 时间戳｜例如`1685025365`、`1663025361000`
4. [x] 连续文字｜例如`https://seaxii.com/20210315/img/2035.png`

### 性能与兼容性
<p>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/性能-强的不像话-blue">
  </a>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/兼容性-简直离谱-white">
  </a>
</p>

单次解析耗时仅为**微秒-µs**甚至是**纳秒-ns**，并且具有**极其优秀的兼容性**，就算输入字符串中存在杂乱的噪声符号和不相关的其他文字，它也能够准确识别和格式化正确的时间文本。

> 解析耗时取自下方性能测试结果

## 在线体验

> 待补充 ...

## 使用方法 In Rust
<p>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/Language-Rust-origin">
  </a>
</p>

以下列出几种不同类型的时间文本解析示例，更多示例可参考 **benches** 和 **examples** 中的示例代码。

1、**绝对时间 Absolute**
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

2、**相对时间 Relative**
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

3、**时间戳 Timestamp**
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

4、**连续时间 Series**
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

## 性能

osmanthus 的性能测试使用的是**Criterion**，具体的测试代码可到 **benches** 查看

```bash
/// 测试机 Mac Stucio 
/// 芯片 Apple M1 Max 
/// 内存 32GB
/// 系统 MacOS 14.0

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


通过性能测试对比可以得出，**桂花算法**的解析速度遥遥领先。


## 兼容性
**桂花算法**的解析倾向是**尽可能多**、**尽可能准**地去识别和解析时间文本，因此对那些夹杂着各式各样噪声的文本进行了清洗。

### 噪声处理

无论噪声是汉字、字母、数字、标点符号或者说是其它语言，都难不倒桂花算法。

具体的兼容性案例可查看 **benches** 和 **example** 相关代码，同时也欢迎大家提供更多的测试样本。

### 时区
<p>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/时区-自动计算-pink">
  </a>
</p>

既然支持全球化，那么时区自然也是要考虑的。目前桂花算法支持了 390 种不同时区的自动计算和 UTC 时间转换，包含常见的例如 `CST`、`MST`、`BST`、`HAST`等

详细名单请查看 [TIMEZONE LIST 时区支持名单](https://github.com/ziiyoo/osmantuhs/blob/main/TIMEZONE.md)

在以上支持名单中的时区，桂花算法在处理时会自动识别并计算出正确的时间，在解析结果中提供当前运行环境所属时区的时间和 UTC 时间，方便大家根据自己的业务和地区进行转换。


### 时间顺序
<p>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/时间顺序-自动切换-brown">
  </a>
</p>

全球各个地方的时间顺序是不一致的，常见的有**年月日**、**日月年**、**月日年**，算法会根据时间文本的内容和出现顺序自动转换，例如

```bash
2013.05/12 -> 2013-05-12 00:00:00  // 顺序正确直接解析
2013.05/july 15:00 -> 2013-07-05 15:00:00 // 月份是确定的｜调整顺序
05,06,2021 13:00 -> 2021-06-05 13:00  // 月份不确定但是这种写法通常是日月年｜调整顺序
05,13,2021 13:00 -> 2021-05-13 13:00  // 月份不可能大于 12｜顺序其实是确定的
```

### 严格模式
<p>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/严格模式-支持-yellow">
  </a>
</p>

在新闻舆情场景中，有个普遍的需求是识别新闻发布的时间。这里有个需要注意的地方，新闻发布的时间不可能比当地的当前时间更靠后，一定是靠前的。即假设当前时间是 "2023-10-10- 10:00:05"，那采集到的新闻的发布时间一定比当前时间早，不可能是明天才发布的。

严格模式下，桂花算法会判定时间文本是否比当前时间的值更大，如果更大则跳过当前可疑文本，去识别下一段可疑文本。


## 全球化多语种
<p>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/多语种-全球多个地区-origin">
  </a>
</p>

作者是一名创业者，主要工作是全球新闻数据的采集和解析，因此时间文本的识别和解析必须满足**全球化**的需求。无论是汉语、俄语、日语、德语、法语、英语、韩语、孟加拉语、越南语还是全球其他几十种语种，均已支持

不过受限于本人语言水平，一些比较"本地化"的表述方法覆盖能力不强，例如俄语的"不久前"、德语的"刚才"等文字。

**欢迎大家提供更多、更准确、更地道的测试样本。**

## 可能存在的缺陷
1. **兼容性太强导致准确度下降**，即容易错判，例如`12 batch 2021.05. 13 2023page`这段字符串，正确的解析结果应该是`2021-05-13 00:00:00`,但实际上解析出来是`2021-12-05 00:00:00`。不过如果用其他库进行解析，很有可能解析失败、解析不到任何有效时间文本。
2. 待补充

## 其他编程语言支持

> 待支持...

## 测试与用例

我们人工收集整理了全球五大洲各个国家或地区的时间文本，加上人为构造，得到了**几十种**、共 **700** 多个时间样本。涵盖了**不同时区**、**不同语种**、**不同纪元表达**等，一下仅仅给出**小部分样本**作为参考
```azure
"令和3年12月7日" - 日本的纪元表达
"26 ก.ค. 2566 08:00 น." - 泰国的泰历纪元表达
"2013-05-06T11:30:22+02:00" - 基于 UTC 时间偏移量的时区表达
"September 17, 2012 at 10:09am PST" - 明确的时区表达
"29/10/2020 10h38 Pm" - 小时缩写法
" 4 Αυγούστου 2023, 00:01 " - 不同语种
"H_502_5@2010oct03 @H_502_5@2012/07/26.doc" - 长文本与噪声
"发布于 - /n6小時前," - 短文本与噪声
... ...
```

完整的测试用例**只开放给开发组**。如果你考虑将 桂花算法应用到你的工程中，但因为这里没提供测试样本而担忧它的解析能力，可以**自行准备足够的样本进行测试**，也可以联系创作者协助测试。


## 创造故事

<p>
  <a href="#" target="_blank">
    <img src="https://img.shields.io/badge/持续进步-冲鸭-blue">
  </a>
</p>

桂花算法是从我们团队商业项目中抽离出来的。

我们团队主线业务是全球新闻数据采集，一方面提供实时数据给一些舆情或者数据科技公司用于分析，一方面为 NLP 方向的 Ai 公司提供长文本训练语料。

新闻解析中很重要的一个元素就是发布时间，常规则正则表达式和一些第三方库难以满足全球化的解析需求。

#### 浅浅尝试

在 Python 领域，dateparser 几乎是使用范围最广的时间文本格式化工具库。我们也用过一段时间，但在使用中发现它的兼容性不强，在一定的噪声干扰下解析能力直线下降。

后来考虑基于深度学习做分类和计算，分类能力比较接近需求，但是格式化和兼容性完全不受控制，非常离谱。

在这样的场景和情况下，我决定设计开发一款新的时间文本解析程序，这就是桂花算法。

#### 设计与参考

桂花算法在设计前和设计的过程中参考了 dateparser 的一些文本处理方式和方法，比如预设正则、文本降噪、时区提取、分类处理等。

**不要以为这是抄的**，实际上 dateparser 的处理方法远远不够，要不然 **osmanthus** 不仅变成了 Rust 版本的 dateparser，解析能力上也不会有进步。

我们设计了几种全新的处理逻辑，在保持解析能力的情况下大幅度增强了兼容性和正确性。自动计算时区时间、滑动窗口、自动切换时间顺序、严格模式等设计让**桂花算法**的解析能力遥遥领先

#### 为什么开源

* 目前市面上没有能力和兼容性与之相当的开源工具，创造它是为了解决我们商业数据上的技术短板，开源则是为了让其他人不要像我们当时一样挠头。
* 开源可以更好地促进桂花算法发展，收集更多的样本、得到更多的反馈、持续优化以得到更高的性能和更高的准确率。
* 开源是向外界展示我们技术能力的一个方式。

> 持续进步，冲鸭 ... ...
