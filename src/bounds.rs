use crate::ElementId;

#[derive(Copy, Clone, Debug)]
pub enum HorizAlign {
    LeftOffset(i32),
    //LeftOffsetProportional(f32),
    //LeftAlign,
    Center,
    //RightAlign,
    //RightOffsetProportional(f32),
    RightOffset(i32)
}

#[derive(Copy, Clone, Debug)]
pub enum VertAlign {
    TopOffset(i32),
    //TopOffsetProportional(f32),
    //TopAlign,
    Center,
    //BottomAlign,
    //BottomOffsetProportional(f32),
    BottomOffset(i32)
}

#[derive(Copy, Clone, Debug)]
pub enum Position {
    Absolute((i32, i32)),
    Relative((i32, i32)),
    //RelativeProportional((f32, f32)),
    Align(HorizAlign, VertAlign)
}

#[derive(Copy, Clone, Debug)]
pub enum Size {
    Absolute((u32, u32)), //direct value of width and height in pixels
    Relative((i32, i32)), //how many fewer or more pixels than another element
    //RelativeProportional((f32, f32)), //proportion of other element's width and height
    //AspectWidth(f32, u32), //height calculated from aspect ratio and absolute width
    //AspectHeight(f32, u32), //width calculated from aspect ratio and absolute height
    //AspectFill(f32), //fill up parent bounds while maintaining aspect ratio
    //FillPad((u32, u32), (u32, u32)), //fill up parent bounds with absolute padding on (left, right), (top, bottom)
    //FillPadProportion((f32, f32), (f32, f32)), //fil up parent bounds with proportional padding on (left, right), (top, bottom)
    Fill //completely fill parent bounds (inherit parent bounds)
}

#[derive(Copy, Clone, Debug)]
pub struct Bounds {
    position: Position,
    size: Size
}

impl Bounds {
    pub fn new(pos: Position, s: Size) -> Bounds {
        Bounds {
            position: pos,
            size: s
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn size(&self) -> Size {
        self.size
    }
}
