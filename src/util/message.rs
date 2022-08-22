use poise::CreateReply;
use poise::serenity_prelude::builder::CreateEmbed;


#[derive(Debug)]
struct EmbedData{
    title: String,
    description: String,
    color: u32,
    fields: Vec<(String, String, bool)>,
    pages: Vec<(String, String)>,
    footer: String,
    thumbnail: String,
    image: String,
    highlighting: bool,
}


struct Builder{
    content: Option<String>,
    embed_data: Option<EmbedData>,
    embed: Option<CreateEmbed>,
    ephemeral: bool,
}

impl Default for Builder{
    fn default() -> Self {
        Self {
            content: Some(String::from("<PlaceHolder>")),
            embed_data: None,
            embed: None,
            ephemeral: false,
        }
    }
}

impl Builder{
    fn set_embed(&mut self, embed: CreateEmbed){
        self.embed = Some(embed);
    }
    fn set_content(&mut self, content: String){
        self.content = Some(content);
    }
    fn set_ephemeral(&mut self, ephemeral: bool){
        self.ephemeral = ephemeral;
    }
    fn build_embed(&mut self){
        let embed = CreateEmbed::default()
        .title(self.embed_data.as_ref().unwrap().title.as_str())
        .description(self.embed_data.as_ref().unwrap().description.as_str())
        .color(self.embed_data.as_ref().unwrap().color)
        .fields(self.embed_data.as_ref().unwrap().fields);
        
            
        self.embed = Some(embed.clone());
    }
}





fn main(){
    CreateReply{
        content: Some(String::from("")),
        embeds: vec![],
        attachments: vec![],
        ephemeral: false,
        allowed_mentions: None,
        ..Default::default()
    };
    ()
}