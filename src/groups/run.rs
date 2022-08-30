use crate::{Context, Error};
use poise::command;
use lazy_static::lazy_static;
use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use regex::Regex;
use crate::util::ansi::{ansi, TextColors, BgColors, Style};

#[derive(Debug)]
enum ExecutionStatus{
    Running,
    Success,
    Failure, //will use these later when embed building
}

lazy_static!{
    static ref RE_CODEBLOCK:Regex = Regex::new(
        r"(\w*)\s*(?:```)(\w*)?([\s\S]*)(?:```$)"
    ).unwrap();
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct File{
    name: String,
    content: String,
    encoding: String,
}

impl Default for File{
    fn default() -> Self{
        File{
            name: String::from("never.gonna.give.you.up"),
            content: String::new(),
            encoding: String::from("utf8"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecResult {
    pub stdout: String,
    pub stderr: String,
    pub output: String,
    pub code: Option<isize>,
    pub signal: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ExecData{
    pub language: String,
    pub version: String,
    pub files: Vec<File>,
    pub stdin: String,
    pub args: Vec<String>,
    pub compile_timeout: isize,
    pub run_timeout: isize,
    pub compile_memory_limit: isize,
    pub run_memory_limit: isize,
}

impl Default for ExecData{
    fn default() -> Self {
        Self {
            language: String::new(),
            version: String::from("*"),
            files: vec![],
            stdin: String::new(),
            args: vec![],
            compile_timeout: 10000,
            run_timeout: 3000,
            compile_memory_limit: -1,
            run_memory_limit: -1,
        }
    }
} 
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RawExecResponse {
    pub language: String,
    pub version: String,
    pub run: ExecResult,
    pub compile: Option<ExecResult>,
}

#[derive(Debug)]
struct Executor{
    client:Arc<Client>,
    data:ExecData,
    out:Option<RawExecResponse>,
    result:ExecutionStatus,//use later
    url:String,
}

impl Default for Executor{
    fn default() -> Self{
        Executor{
            client:Arc::new(Client::new()),
            data:ExecData::default(),
            out:None,
            result:ExecutionStatus::Running,
            url:String::from("https://emkc.org/api/v2/piston"),
        }
    }
}

impl Executor{

    async fn run(&mut self){
        let dat = self.client
        .post(format!("{}/execute", self.url))
        .json::<ExecData>(&self.data)
        .send()
        .await
        .unwrap();
        match dat.status(){
            reqwest::StatusCode::OK => {
                self.out = Some(dat.json::<RawExecResponse>().await.unwrap());
            }
            _ => {
                println!("error at Executor.run > {}: {}", dat.status(), dat.text().await.unwrap());
            }
        }
    }
}

#[derive(Debug)]
struct ParsedArgs{
    version:String,
    language:Option<String>,
    files:Vec<File>,
    stdin:String,
    args:Vec<String>,
}
impl ParsedArgs{
    fn set_version(&mut self, version:String){
        self.version = version;
    }
    fn set_language(&mut self, language:String){
        self.language = Some(language);
    }
    fn add_file(&mut self, files:File){
        self.files.push(files);
    }
    fn set_stdin(&mut self, stdin:String){
        self.stdin = stdin;
    }
    fn add_arg(&mut self, args:String){
        self.args.push(args);
    }
}

fn parse_args(arg:String)->ParsedArgs{
    let mut parsed = ParsedArgs{
        version:String::from("*"),
        language:None,
        files:vec![],
        stdin:String::from(""),
        args:vec![],
    };
    let flag_re = regex::Regex::new(r"--|=").unwrap();
    // [version,9.2 ,lang,"rust rust" ,brr]
    let mut raw_args = flag_re.split(&arg).peekable();
    loop{
        match raw_args.next(){
            Some(part) => {
                match part.trim(){
                    "version"=>{
                        raw_args.next().map(|next|{
                            parsed.set_version(next.trim().to_string());
                        });
                    }
                    "language"=>{
                        raw_args.next().map(|next|{
                            parsed.set_language(next.trim().to_string());
                        });
                    }
                    "stdin"=>{
                        raw_args.next().map(|next|{
                            parsed.set_stdin(next.trim().to_string());
                        });
                    }
                    "args"=>{
                        raw_args.next().map(|next|{
                            let mut args = next.split(",");
                            loop{
                                match args.next(){
                                    Some(arg)=>{
                                        parsed.add_arg(arg.trim().to_string());
                                    }
                                    None=>{
                                        break;
                                    }
                                }
                            }
                        });
                    }
                    "debug"=>{
                        println!("{:?}", parsed);
                    }
                    _ => {
                        if part.starts_with("file-"){
                            let filename = part.split("-").nth(1).unwrap().split(" ").nth(0).unwrap().trim();
                            let code = RE_CODEBLOCK.captures(&part.trim()).and_then(|block|{
                                block.get(3).map(|code| code.as_str())
                            }).unwrap();
                            parsed.add_file(File{
                                name: filename.to_string(),
                                content: String::from(code),
                                encoding: String::from("utf8"),
                            });
                        }
                    }
                }
            },
            None => {
                break;
            }
        }
    }
    parsed
}

fn check_len(out:&String)->bool{
    if out.len() > 1120 || out.lines().count() > 20{
        return false;
    }else{
        return true;
    }
}


#[command(prefix_command)]
pub(crate) async fn run(
    ctx: Context<'_>,
    #[description = "run code"] block: poise::CodeBlock,
    #[rest] rawargs: Option<String>,
) -> Result<(), Error> {
    match block.language {
        Some(mut language) => {
            // sort arguments
            let args = parse_args(rawargs.unwrap_or(String::from("")));
            if !args.language.is_none(){
                language = args.language.unwrap();
            }
            let mut files = vec![File{
                content: block.code,
                ..Default::default()
            }];
            files.extend(args.files);
            // build executor
            let mut executor = Executor{
                client: ctx.data().http_client.clone(),
                data: ExecData{
                    language: language,
                    version: args.version,
                    files: files,
                    args: args.args,
                    stdin: args.stdin,
                    ..Default::default()
                },
                ..Default::default()
            };
            executor.run().await;
            match executor.out{
                Some(out)=>{
                    match out.run.output.trim(){
                        "" => {
                            ctx.send(|f| f
                                .embed(|e| e
                                    .title(format!("**{}**",out.language))
                                    .url("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
                                    .description(format!(
                                        "```ansi\n{}no output\n```",
                                        ansi(TextColors::Pink,BgColors::None,Style::Bold)

                                    ))
                                )
                            ).await?;
                        },
                        _ => {
                            match check_len(&out.run.output){
                                true => {
                                    let resp = ctx.send(|f| f
                                        .embed(|e| e
                                            .title(format!(
                                                "**{}**",
                                                out.language
                                            ))
                                            .url("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
                                            .description(format!("```{}\n{}\n```",out.language,out.run.output))
                                        )
                                        .components(|c| c
                                            .create_action_row(|r| r
                                                .create_button(|b| b
                                                    .custom_id("run.status")
                                                    .label(match out.run.code{
                                                        Some(0)=>"code run",
                                                        _=>"code errored"
                                                    })
                                                    .style(match out.run.code{
                                                        Some(0)=>poise::serenity_prelude::ButtonStyle::Success,
                                                        _=>poise::serenity_prelude::ButtonStyle::Danger
                                                    })
                                                )
                                            ))
                                    ).await?;
                                    resp.message().await?.await_component_interactions().await?
                                },
                                false => {
                                    ctx.say("**output too long**").await?;
                                }
                            }
                        }
                    }
                },
                None=>{
                    ctx.send(|f| f
                        .content("**🛑 Error while getting response from api**")
                        .embed(|f| f
                            .title("** **")
                            .description(format!("```rs\n{:#?}\n```",executor))
                            .url("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
                        )
                        .ephemeral(true)
                    ).await?;
                }
            }
        }
        None => {
            ctx.send(|f| f
                .embed(|e| e
                    .description(format!(
                        "```ansi\n{}please specify a language\n```",
                        ansi(TextColors::Red,BgColors::None,Style::Bold)
                    )
                ))
            ).await?;
        }
        
    }
    return Ok(())
}