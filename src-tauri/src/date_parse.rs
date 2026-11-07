//! 从文件名解析拍摄日期。与 `metadata` 的 EXIF 读写分离，便于单独测试与维护。

/// 从文件名解析拍摄日期，返回 EXIF 时间格式 "YYYY:MM:DD HH:MM:SS"。
/// ponytail: 启发式匹配常见命名（IMG_20240115_143022 / 2024-01-15 14.30.22 等），
/// 覆盖不了的长尾命名由"使用文件修改时间"兜底。
#[allow(clippy::doc_markdown)]
pub fn parse_date_from_filename(name: &str) -> Option<String> {
    let chars: Vec<char> = name.chars().collect();
    // 连续 8 位日期：20240115
    for i in 0..chars.len().saturating_sub(7) {
        if !chars[i..i + 8].iter().all(char::is_ascii_digit) {
            continue;
        }
        let s: String = chars[i..i + 8].iter().collect();
        if let Some(dt) = build_datetime(&s[0..4], &s[4..6], &s[6..8], &chars[i + 8..]) {
            return Some(dt);
        }
    }
    // 分隔日期：2024-01-15 / 2024.1.15
    for i in 0..chars.len().saturating_sub(9) {
        if !chars[i..i + 4].iter().all(char::is_ascii_digit) {
            continue;
        }
        let is_sep = |c: char| c == '-' || c == '.' || c == '_';
        if !is_sep(chars[i + 4]) || !is_sep(chars[i + 7]) {
            continue;
        }
        let (year, rest) = (&chars[i..i + 4], &chars[i + 5..]);
        let month: String = rest.iter().take_while(|c| c.is_ascii_digit()).collect();
        if month.is_empty() || month.len() > 2 || rest.get(month.len()).copied().is_none_or(|c| !is_sep(c)) {
            continue;
        }
        let day: String = rest[month.len() + 1..].iter().take_while(|c| c.is_ascii_digit()).collect();
        if day.is_empty() || day.len() > 2 {
            continue;
        }
        let y: String = year.iter().collect();
        let after = &chars[std::cmp::min(i + 5 + month.len() + 1 + day.len(), chars.len())..];
        if let Some(dt) = build_datetime(&y, &month, &day, after) {
            return Some(dt);
        }
    }
    None
}

fn build_datetime(year: &str, month: &str, day: &str, after: &[char]) -> Option<String> {
    if !(year.starts_with("19") || year.starts_with("20")) {
        return None;
    }
    let (year, month, day): (u32, u32, u32) = (year.parse().ok()?, month.parse().ok()?, day.parse().ok()?);
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let (hh, mm, ss) = parse_time_after(after).unwrap_or((0, 0, 0));
    Some(format!("{year:04}:{month:02}:{day:02} {hh:02}:{mm:02}:{ss:02}"))
}

/// 在日期后几个字符内寻找 6 位时间（hhmmss），允许分隔符。
fn parse_time_after(chars: &[char]) -> Option<(u32, u32, u32)> {
    let mut j = 0;
    while j < chars.len() && j <= 4 {
        if chars[j].is_ascii_digit() {
            let run: String = chars[j..].iter().take_while(|c| c.is_ascii_digit()).collect();
            if run.len() < 6 {
                return None;
            }
            let hh: u32 = run[0..2].parse().ok()?;
            let mm: u32 = run[2..4].parse().ok()?;
            let ss: u32 = run[4..6].parse().ok()?;
            if hh < 24 && mm < 60 && ss < 60 {
                return Some((hh, mm, ss));
            }
            return None;
        }
        j += 1;
    }
    None
}
