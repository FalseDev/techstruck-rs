use std::sync::Arc;

use crate::{
    common::{Context, Data, Error},
    lua::types::ToLuaData,
};
use mlua::Lua;
use poise::command;
use tokio::{runtime::Builder, sync::mpsc, task::LocalSet};

type Task = (super::types::Context, Vec<u8>);

pub(crate) fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![lua()]
}

#[command(prefix_command, owners_only)]
pub(crate) async fn lua(ctx: Context<'_>, #[rest] filename: String) -> Result<(), Error> {
    let code = std::fs::read(format!("/home/vscode/luau-exp/{}.lua", filename))?;
    let ctx = Arc::new(ctx).to_user_data();

    let spawner = LocalSpawner::new();

    spawner.spawn((ctx, code));

    Ok(())
}

#[derive(Clone)]
struct LocalSpawner {
    send: mpsc::UnboundedSender<Task>,
}

impl LocalSpawner {
    pub fn new() -> Self {
        let (send, mut recv) = mpsc::unbounded_channel::<Task>();

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        std::thread::spawn(move || {
            let local = LocalSet::new();

            local.spawn_local(async move {
                while let Some(task) = recv.recv().await {
                    tokio::task::spawn_local(async move {
                        let lua = Lua::new();
                        lua.sandbox(true).unwrap();
                        lua.globals().set("ctx", task.0)?;
                        let code = lua.load(&task.1);
                        code.exec_async().await?;
                        Ok::<(), anyhow::Error>(())
                    });
                }
            });

            rt.block_on(local);
        });

        Self { send }
    }

    pub fn spawn(&self, task: Task) {
        self.send
            .send(task)
            .expect("Thread with LocalSet has shut down.");
    }
}
