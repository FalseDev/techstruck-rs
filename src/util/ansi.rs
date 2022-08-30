#[derive(Debug,Clone)]
pub enum TextColors{
    Grey,
    Red,
    Green,
    Yellow,
    Blue,
    Pink,
    Cyan,
    White,
    None
}
#[derive(Debug,Clone)]
pub enum BgColors{
    DarkBlue,
    Orange,
    MarbleBlue,
    Turqoise,
    Grey,
    Indigo,
    LightGrey,
    White,
    None,
}
#[derive(Debug,Clone)]
pub enum Style{
    Normal,
    Bold,
    Underline,
    None,
}

pub fn ansi(color:TextColors,bg:BgColors,style:Style)->String{
    let mut ansi = String::new();
    ansi.push_str("[0m[");
    match style{
        Style::Bold=>{
            ansi.push_str("1");
        }
        Style::Underline=>{
            ansi.push_str("4");
        }
        _=>{
            ansi.push_str("0");
        }
    }
    match color{
        TextColors::Grey => {
            ansi.push_str(";30");
        },
        TextColors::Red => {
            ansi.push_str(";31");
        },
        TextColors::Green => {
            ansi.push_str(";32");
        },
        TextColors::Yellow => {
            ansi.push_str(";33");
        },
        TextColors::Blue => {
            ansi.push_str(";34");
        },
        TextColors::Pink => {
            ansi.push_str(";35");
        },
        TextColors::Cyan => {
            ansi.push_str(";36");
        },
        TextColors::White => {
            ansi.push_str(";37");
        }
        _=>{}
    }
    
    match bg{
        BgColors::DarkBlue=>{
            ansi.push_str(";40");
        }
        BgColors::Orange=>{
            ansi.push_str(";41");
        }
        BgColors::MarbleBlue=>{
            ansi.push_str(";42");
        }
        BgColors::Turqoise=>{
            ansi.push_str(";43");
        }
        BgColors::Grey=>{
            ansi.push_str(";44");
        }
        BgColors::Indigo=>{
            ansi.push_str(";45");
        }
        BgColors::LightGrey=>{
            ansi.push_str(";46");
        }
        BgColors::White=>{
            ansi.push_str(";47");
        }
        _=>{}
    }
    ansi.push_str("m");
    ansi
}