use std::io::Read;

use bancho_packets::{BanchoPacketRead, PayloadReader};
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Clone, Default)]
pub struct BanchoUserStats
{
    pub user_id:       i32,
    pub online_status: u8,
    pub description:   String,
    pub beatmap_md5:   String,
    pub mods:          u32,
    pub mode:          u8,
    pub beatmap_id:    i32,
    pub ranked_score:  i64,
    pub accuracy:      f32,
    pub playcount:     i32,
    pub total_score:   i64,
    pub rank:          i32,
    pub pp:            i16,
}

impl BanchoPacketRead<BanchoUserStats> for BanchoUserStats
{
    fn read(reader: &mut PayloadReader) -> Option<BanchoUserStats>
    {
        let mut payload = reader.payload();
        let user_id = payload.read_i32::<LittleEndian>().ok()?;

        let online_status = payload.read_u8().ok()?;

        let _ = payload.read_u8().ok()?;
        let desc_length = payload.read_u8().ok()?;
        let mut desc = vec![0u8; desc_length as _];
        payload.read_exact(&mut desc).ok()?;
        let description = String::from_utf8(desc).ok()?;

        let _ = payload.read_u8().ok()?;

        let map_length = payload.read_u8().ok()?;
        let mut map = vec![0u8; map_length as _];
        payload.read_exact(&mut map).ok()?;
        let beatmap_md5 = String::from_utf8(map).ok()?;

        let mods = payload.read_u32::<LittleEndian>().ok()?;
        let mode = payload.read_u8().ok()?;

        let beatmap_id = payload.read_i32::<LittleEndian>().ok()?;

        let ranked_score = payload.read_i64::<LittleEndian>().ok()?;
        let accuracy = payload.read_f32::<LittleEndian>().ok()?;
        let playcount = payload.read_i32::<LittleEndian>().ok()?;
        let total_score = payload.read_i64::<LittleEndian>().ok()?;
        let rank = payload.read_i32::<LittleEndian>().ok()?;
        let pp = payload.read_i16::<LittleEndian>().ok()?;

        Some(BanchoUserStats {
            user_id,
            online_status,
            description,
            beatmap_md5,
            mods,
            mode,
            beatmap_id,
            ranked_score,
            accuracy,
            playcount,
            total_score,
            rank,
            pp,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct BanchoUserPresence
{
    pub user_id:      i32,
    pub username:     String,
    pub utc_offset:   u8,
    pub country_code: u8,
    pub bancho_priv:  i32,
    pub longitude:    f32,
    pub latitude:     f32,
}

impl BanchoPacketRead<BanchoUserPresence> for BanchoUserPresence
{
    fn read(reader: &mut PayloadReader) -> Option<BanchoUserPresence>
    {
        let mut payload = reader.payload();

        let user_id = payload.read_i32::<LittleEndian>().ok()?;

        let _ = payload.read_u8().ok()?;
        let username_length = payload.read_u8().ok()?;
        let mut username = vec![0u8; username_length as _];
        payload.read_exact(&mut username).ok()?;
        let username = String::from_utf8(username).ok()?;

        let utc_offset = payload.read_u8().ok()?;
        let country_code = payload.read_u8().ok()?;
        let bancho_priv = payload.read_i32::<LittleEndian>().ok()?;
        let longitude = payload.read_f32::<LittleEndian>().ok()?;
        let latitude = payload.read_f32::<LittleEndian>().ok()?;

        Some(BanchoUserPresence {
            user_id,
            username,
            utc_offset,
            country_code,
            bancho_priv,
            longitude,
            latitude,
        })
    }
}
