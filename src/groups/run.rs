use crate::{Context, Error};
use poise::command;
use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::{sync::Arc};

enum ExecutionStatus{
    Running,
    // Success,
    // Failure, //will use these later when embed building
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

struct Executor{
    client:Arc<Client>,
    data:ExecData,
    out:Option<RawExecResponse>,
    result:ExecutionStatus,
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


#[command(prefix_command)]
pub(crate) async fn run(
    ctx: Context<'_>,
    #[description = "run code"] block: poise::CodeBlock,
    #[lazy] stdin: Option<String>,
) -> Result<(), Error> {
    match block.language {
        Some(language) => {
            let mut executor = Executor{
                client: ctx.data().http_client.clone(),
                data: ExecData{
                    language: language,
                    files: vec![File{
                        content: block.code,
                        ..Default::default()
                    }],
                    stdin: match stdin {
                        Some(stdin) => stdin,
                        None => String::new(),
                    },
                    ..Default::default()
                },
                ..Default::default()
            };
            executor.run().await;
            match executor.out{
                Some(out)=>{
                    match out.run.output.trim(){
                        "" => {
                            ctx.say("**no output**").await?;
                        },
                        _ => {
                            ctx.say(format!("```{}\n{}```",out.language,out.run.output)).await?;
                        }
                    }
                },
                None=>{
                    ctx.say("no output recieved").await?;
                }
            }
        }
        None => {
            ctx.say("Please specify a language").await?;
        }
    }
    return Ok(())
}