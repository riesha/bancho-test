use std::{env, thread, time::Duration};

use anyhow::Result as anyResult;
use argh::FromArgs;
use bancho_test::bancho::BanchoClient;
use dotenv::dotenv;
#[derive(FromArgs, Debug)]
/// a proof of concept bancho client
struct Args
{
    /// osu uid of the player to search for
    #[argh(
        option,
        short = 'u',
        default = "env::var(\"DEFAULT_UID\").unwrap().parse::<i32>().unwrap()"
    )]
    uid: i32,
}

fn main() -> anyResult<()>
{
    dotenv().ok();

    let args: Args = argh::from_env();

    let bancho = BanchoClient::new(
        env::var("USER")?.as_str(),
        env::var("PASSWORD")?.as_str(),
        env::var("CLIENT_VERSION")?.as_str(),
        env::var("EXE_HASH")?.as_str(),
        env::var("HWID_STRING")?.as_str(),
    )?;

    loop
    {
        let res = bancho.send_presence_req(&[args.uid])?;
        bancho.display_response(res)?;
        thread::sleep(Duration::from_millis(500));
    }
    Ok(())
}
