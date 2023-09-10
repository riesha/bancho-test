use anyhow::*;
use bancho_packets::{
    client::{Ping, UserPresenceRequest, UserStatsRequest},
    PacketReader, PayloadReader,
};
use reqwest::{blocking::Response, header::USER_AGENT};

use crate::model::{BanchoUserPresence, BanchoUserStats};
const BANCHO_HOST: &str = "https://c.ppy.sh/";
pub struct BanchoClient
{
    pub client:      reqwest::blocking::Client,
    pub cho_token:   String,
    pub username:    String,
    pub password:    String,
    pub client_ver:  String,
    pub exe_hash:    String,
    pub hwid_string: String,
}
impl BanchoClient
{
    pub fn new(
        username: &str, password: &str, client_ver: &str, exe_hash: &str, hwid_string: &str,
    ) -> Result<BanchoClient>
    {
        let password = format!("{:x}", md5::compute(password));

        let client = reqwest::blocking::Client::builder()
            .cookie_store(true)
            .build()?;

        let req_txt =
            format!("{username}\n{password}\n{client_ver}|2|0|{exe_hash}:{hwid_string}:|0\n");

        let res = client
            .get(BANCHO_HOST)
            .body(req_txt)
            .header("osu-version", client_ver)
            .header("Accept-Encoding", "gzip")
            .header(USER_AGENT, "osu!")
            .header("Connection", "Keep-Alive")
            .send()?;

        if res.error_for_status_ref().is_err()
        {
            dbg!(&res.status(), &res.text()?);
            panic!("initial bancho req was error");
        }

        let cho_token = &res
            .headers()
            .get("cho-token")
            .and_then(|x| x.to_str().ok())
            .ok_or(anyhow!("couldnt get cho token"))?;

        println!("created bancho client successfully!");
        let cho = BanchoClient {
            client,
            cho_token: cho_token.to_string(),
            username: username.into(),
            password,
            client_ver: client_ver.into(),
            exe_hash: exe_hash.into(),
            hwid_string: hwid_string.into(),
        };

        cho.display_response(res)?;
        Ok(cho)
    }

    pub fn display_response(&self, res: Response) -> Result<()>
    {
        let bytes = res.bytes()?;
        let reader = PacketReader::new(&bytes);

        for packet in reader
        {
            match packet.payload
            {
                Some(payload) =>
                {
                    let mut payload_reader = PayloadReader::new(payload);
                    match packet.id
                    {
                        bancho_packets::PacketId::BANCHO_USER_PRESENCE =>
                        {
                            let message = payload_reader.read::<BanchoUserPresence>();
                            println!("{:?}: {:?}", packet.id, message);
                        }
                        bancho_packets::PacketId::BANCHO_USER_STATS =>
                        {
                            let message = payload_reader.read::<BanchoUserStats>();
                            println!("{:?}: {:?}", packet.id, message);
                        }
                        bancho_packets::PacketId::BANCHO_USER_LOGOUT =>
                        {
                            // let id = payload_reader.read::<i32>().unwrap();
                            // println!("user [{id}] is offline");
                        }
                        bancho_packets::PacketId::BANCHO_USER_LOGIN_REPLY =>
                        {
                            let id = payload_reader.read::<i32>().unwrap();
                            println!("login response: {id}");
                        }
                        _ =>
                        {}
                    }
                }
                None => println!("Non-payload"),
            }
        }

        Ok(())
    }

    pub fn send_packet(&self, packet: Vec<u8>) -> Result<Response>
    {
        let res = self
            .client
            .post(BANCHO_HOST)
            .body(packet)
            .header("osu-token", self.cho_token.clone())
            .header("Accept-Encoding", "gzip")
            .header(USER_AGENT, "osu!")
            .header("Connection", "Keep-Alive")
            .send()?;

        if res.error_for_status_ref().is_err()
        {
            dbg!(&res.status());
            dbg!(&res.text()?);
            panic!("error sending packet");
        }

        Ok(res)
    }

    pub fn send_presence_req(&self, uids: &[i32]) -> Result<Response>
    {
        Ok(self.send_packet(UserPresenceRequest::pack(uids))?)
    }

    pub fn send_stats_req(&self, uids: &[i32]) -> Result<Response>
    {
        Ok(self.send_packet(UserStatsRequest::pack(uids))?)
    }

    pub fn send_ping(&self) -> Result<Response> { Ok(self.send_packet(Ping::pack())?) }
}
