# bancho-test

Proof of Concept bancho client written in rust, implements ping, presence request and stat request functions.

This is not meant to be an usable product but rather an example of how someone might implement a standalone bancho client in rust.
### Example Output

```
created bancho client successfully!
login response: 24991886
BANCHO_USER_PRESENCE: Some(BanchoUserPresence { user_id: 24991886, username: "vernalis", utc_offset: 26, country_code: 67, bancho_priv: 172979713, longitude: 2.4709137e-16, latitude: 2.8738956e-38 })
BANCHO_USER_STATS: None
BANCHO_USER_PRESENCE: Some(BanchoUserPresence { user_id: -3, username: "BanchoBot", utc_offset: 24, country_code: 0, bancho_priv: 0, longitude: 0.0, latitude: 0.0 })
Non-payload
BANCHO_USER_PRESENCE: Some(BanchoUserPresence { user_id: 7562902, username: "mrekk", utc_offset: 34, country_code: 16, bancho_priv: 285710853, longitude: 5.2542253e-25, latitude: 6.3e-43 })
BANCHO_USER_PRESENCE: Some(BanchoUserPresence { user_id: 7562902, username: "mrekk", utc_offset: 34, country_code: 16, bancho_priv: 285710853, longitude: 5.2542253e-25, latitude: 6.3e-43 })
BANCHO_USER_PRESENCE: Some(BanchoUserPresence { user_id: 7562902, username: "mrekk", utc_offset: 34, country_code: 16, bancho_priv: 285710853, longitude: 5.2542253e-25, latitude: 6.3e-43 })
```
### Usage
 ```
 Usage: bancho-test.exe [-u <uid>]

a proof of concept bancho client

Options:
  -u, --uid         osu uid of the player to search for
  --help            display usage information
  ```

### Environment Variables

| Variable    | Description |
| -------- | ------- |
CLIENT_VERSION | Osu client version, can be found in your `osu!.<username>.cfg` under the name `LastVersion`
EXE_HASH | Osu executable hash, can be found in your `osu!.cfg` under the name of `h_osu!.exe`
HWID_STRING | Your osu hardware identifier string, obtaining this is left as an exercise for the reader
USER | Your osu username
PASSWORD | Your osu password
DEFAULT_UID | Fallback uid to search for if `-u/--uid` is not specified


