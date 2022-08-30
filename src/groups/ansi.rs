
use crate::{Context, Error};
use crate::util::ansi::{ansi,TextColors,BgColors,Style};

//ansi go brr
#[poise::command(prefix_command, track_edits, slash_command)]
pub(crate) async fn ansi_test(
    ctx:Context<'_>,
)-> Result<(), Error>{
    let mut out = String::new();
    let allcolors = vec![
        TextColors::Grey,
        TextColors::Red,
        TextColors::Green,
        TextColors::Yellow,
        TextColors::Blue,
        TextColors::Pink,
        TextColors::Cyan,
        TextColors::White,
        TextColors::None
    ];
    let allbg = vec![
        BgColors::DarkBlue,
        BgColors::Orange,
        BgColors::MarbleBlue,
        BgColors::Turqoise,
        BgColors::Grey,
        BgColors::Indigo,
        BgColors::LightGrey,
        BgColors::White,
        BgColors::None,
    ];
    let allstyles = vec![
        Style::Normal,
        Style::Bold,
        Style::Underline,
        Style::None,
    ];
    for color in &allcolors{
        out.push_str(format!(
            "{}[color]{:?} [bg]{:?}  ||  [normal] {}[underline] {}[bold]\n",
            ansi(color.clone(),BgColors::None,Style::None),
            color,
            BgColors::None,
            ansi(color.clone(),BgColors::None,Style::Underline),
            ansi(color.clone(),BgColors::None,Style::Bold)
        ).as_str());
        for bg in &allbg{
            out.push_str(format!(
                "{}[color]{:?} [bg]{:?}  ||  [normal] {}[underline] {}[bold]\n",
                ansi(color.clone(),bg.clone(),Style::None),
                color,
                bg,
                ansi(color.clone(),bg.clone(),Style::Underline),
                ansi(color.clone(),bg.clone(),Style::Bold)
            ).as_str());
        }
        out.push_str("[split]");
    }
    for i in out.split("[split]").into_iter(){
        ctx.say(format!("```ansi\n{}\n```",i).as_str()).await?;
    }
    
    Ok(())
}