use crate::bind::{Param, Result};
use crate::{parse_absolute, parse_relative, parse_series, parse_timestamp};

pub fn parse_auto(text: &str, options: Option<Param>) -> Result{
    let item = Result{timezone: "none".to_string(), method: String::from("none"), ..Default::default()};
    let result_parse_timestamp = parse_timestamp(text, clone_options(&options));
    if result_parse_timestamp.status{
        return result_parse_timestamp
    }
    let result_parse_relative = parse_relative(text, clone_options(&options));
    if result_parse_relative.status{
        return result_parse_relative
    }
    let result_parse_absolute = parse_absolute(text, clone_options(&options));
    if result_parse_absolute.status{
        return result_parse_absolute
    }
    let result_parse_series = parse_series(text, clone_options(&options));
    if result_parse_series.status{
        return result_parse_series
    }
    return item
}

fn clone_options(options: &Option<Param>) -> Option<Param>{
    if let Some(opt) = options{
        let param = Param{
            era: opt.era.clone(),
            timezone: opt.timezone.clone(),
            strict: opt.strict.clone()
        };
        return Some(param)
    }
    return None
}
