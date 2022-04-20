#![allow(non_camel_case_types)]

#[derive(FromPrimitive, ToPrimitive)]
pub enum StyleIndex {
    border_bottom_width = 8,
    border_left_width = 11,
    border_right_width = 14,
    border_top_width = 17,
    font_size = 22,
    font_weight = 24,
    padding_bottom = 30,
    padding_left = 31,
    padding_right = 32,
    padding_top = 33,
    display = 39,
    float_ = 40,
    overflow_x = 41,
    overflow_y = 42,
    position = 43,
    color = 44,
    text_align = 46,
    text_indent = 47,
    z_index = 50,
}

struct Value<'a>(&'a str);

impl<'a> Into<i32> for Value<'a> {
    fn into(self) -> i32 {
        self.0.parse::<>().unwrap_or_default()
    }
}

impl<'a> Into<u32> for Value<'a> {
    fn into(self) -> u32 {
        self.0.parse::<>().unwrap_or_default()
    }
}
#[derive(FromPrimitive, ToPrimitive)]
pub enum Display {
    Inline = 0,
    None = 1,
    Block = 2,
    InlineBlock = 3,
    ListItem = 4,
    RunIn = 5,
    Compact = 6,
    Marker = 7,
    Table = 8,
    InlineTable = 9,
    TableRowGrouP = 10,
    TableHeaderGroup = 11,
    TableFooterGroup = 12,
    TableRow = 13,
    TableColumnGroup = 14,
    TableColumn = 15,
    TableCell = 16,
    TableCaption = 17,
    Inherit = 18,
}

impl<'a> Into<Display> for Value<'a> {
    fn into(self) -> Display {
        self.0.parse::<u32>().map(|x| num::FromPrimitive::from_u32(x)).ok().unwrap_or_default().unwrap_or_default()
    }
}

impl Default for Display {
    fn default() -> Self {
        Display::Inline
    }
}

#[derive(FromPrimitive, ToPrimitive)]
pub enum Float {
    None = 0,
    Left = 1,
    Right = 2,
    Inherit = 3,
}

impl<'a> Into<Float> for Value<'a> {
    fn into(self) -> Float {
        self.0.parse::<u32>().map(|x| num::FromPrimitive::from_u32(x)).ok().unwrap_or_default().unwrap_or_default()
    }
}

impl Default for Float {
    fn default() -> Self {
        Float::None
    }
}

#[derive(FromPrimitive, ToPrimitive)]
pub enum Overflow {
    Visible = 0,
    Hidden = 1,
    Scroll = 2,
    Auto = 3,
    NoDisplay = 4,
    NoContent = 5,
}

impl<'a> Into<Overflow> for Value<'a> {
    fn into(self) -> Overflow {
        self.0.parse::<u32>().map(|x| num::FromPrimitive::from_u32(x)).ok().unwrap_or_default().unwrap_or_default()
    }
}

impl Default for Overflow {
    fn default() -> Self {
        Overflow::Visible
    }
}

#[derive(FromPrimitive, ToPrimitive)]
pub enum Position {
    Static = 0,
    Absolute = 1,
    Fixed = 2,
    Relative = 3,
    Inherit = 4,
}

impl<'a> Into<Position> for Value<'a> {
    fn into(self) -> Position {
        self.0.parse::<u32>().map(|x| num::FromPrimitive::from_u32(x)).ok().unwrap_or_default().unwrap_or_default()
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::Static
    }
}

#[derive(FromPrimitive, ToPrimitive)]
pub enum TextAlign {
    Left = 0,
    Right = 1,
    Center = 2,
    Justify = 3,
    TextAlignInherit = 4,
}

impl<'a> Into<TextAlign> for Value<'a> {
    fn into(self) -> TextAlign {
        self.0.parse::<u32>().map(|x| num::FromPrimitive::from_u32(x)).ok().unwrap_or_default().unwrap_or_default()
    }
}

impl Default for TextAlign {
    fn default() -> Self {
        TextAlign::Left
    }
}

pub struct StyleInfo {
    border_bottom_width: i32,
    border_left_width: i32,
    border_right_width: i32,
    border_top_width: i32,
    font_size: i32,
    font_weight: i32,
    padding_bottom: i32,
    padding_left: i32,
    padding_right: i32,
    padding_top: i32,
    display: Display,
    float_: Float,
    overflow_x: Overflow,
    overflow_y: Overflow,
    position: Position,
    color: i32,
    text_align: TextAlign,
    text_indent: i32,
    z_index: i32,
}

impl Default for StyleInfo {
    fn default() -> Self {
        Self {
            border_bottom_width: Default::default(),
            border_left_width: Default::default(),
            border_right_width: Default::default(),
            border_top_width: Default::default(),
            font_size: Default::default(),
            font_weight: Default::default(),
            padding_bottom: Default::default(),
            padding_left: Default::default(),
            padding_right: Default::default(),
            padding_top: Default::default(),
            display: Default::default(),
            float_: Default::default(),
            overflow_x: Default::default(),
            overflow_y: Default::default(),
            position: Default::default(),
            color: Default::default(),
            text_align: Default::default(),
            text_indent: Default::default(),
            z_index: Default::default(),
        }
    }
}

macro_rules! parse_style {
    ($v: ident, $items: ident, $($name: ident), +) => {
        $(
            if $items.len() > StyleIndex::$name as usize {
                $v.$name = Value($items[StyleIndex::$name as usize]).into();
            }
        )+
    };
}

pub fn parse_style_info(s: &str, style: &mut StyleInfo) {
    let items = s.split(';').into_iter().collect::<Vec<_>>();
    parse_style!(style, items,
        border_bottom_width,
        border_left_width,
        border_right_width,
        border_top_width,
        font_size,
        font_weight,
        padding_bottom,
        padding_left,
        padding_right,
        padding_top,
        display,
        float_,
        overflow_x,
        overflow_y,
        position,
        color,
        text_align,
        text_indent,
        z_index
    );
}
