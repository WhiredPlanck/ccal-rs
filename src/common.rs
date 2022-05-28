// Jie Qi 节气
pub const SOLAR_TERM_LONG: [&str; 24] = [
    "Beginning of Spring", // 立春
    "Rain Water", // 雨水
    "Awakening of Insects", // 惊蛰
    "Spring Equinox", // 春分
    "Pure Brightness", // 清明
    "Grain Rain", // 谷雨
    "Begining of Summer", // 立夏
    "Grain Buds", // 小满
    "Grain in Ear", // 芒种
    "Summer Solstice", // 夏至
    "Minor Heat", // 小暑
    "Major Heat", // 大暑
    "Begining of Autumn", // 立秋
    "End of Heat", // 处暑
    "White Dew", // 白露
    "Autumn Equinox", // 秋分
    "Cold Dew", // 寒露
    "Frost's Descent", // 霜降
    "Begining of Winter", // 立冬
    "Minor Snow", // 小雪
    "Major Snow", // 大雪
    "Winter Solstice", // 冬至
    "Minor Cold", // 小寒
    "Major Cold", // 大寒
];

// 月份（长值）
pub const MONTH_LONG: [&str; 12] = [
    "January", "February", "March", "April", "May", "June", "July", "August",
    "September", "October", "November", "December",
];

// 星期（长值）
pub const DAY_LONG: [&str; 7] = [
    "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday",
    "Saturday",
];

// 星期（短值）
pub const DAY_SHORT: [&str; 7] = [
    "Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat",
];

// Tian Gan 天干
pub const HEAVENLY_STEMS: [&str; 10] = [
    "Jia", "Yi", "Bing", "Ding", "Wu", "Ji", "Geng", "Xin", "Ren", "Gui",
];

// Di Zhi 地支
pub const EARTHLY_BRANCHES: [&str; 12] = [
    "Zi", "Chou", "Yin", "Mou", "Chen", "Si", "Wu", "Wei", "Shen", "You", "Xu",
    "Hai",
];

pub fn is_leap_year(year: u32) -> bool {
    match (year % 4, year % 100, year % 400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        (_, _, _) => false,
    }
}
