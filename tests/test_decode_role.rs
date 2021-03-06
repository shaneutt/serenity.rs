extern crate serde_json;
extern crate serenity;

use serde_json::Value;
use serenity::model::{Guild, Role};

#[test]
fn decode_negative_one_role_position() {
    let json = r#"{
        "position": -1,
        "permissions": 37215297,
        "name": "@everyone",
        "mentionable": false,
        "managed": false,
        "id": "444",
        "hoist": false,
        "color": 0
    }"#;

    let value: Value = serde_json::from_str(json).unwrap();

    assert!(Role::decode(value).is_ok());
}


#[test]
fn decode_guild_with_n1_role_position() {
    let json = r#"{
    "voice_states": [],
    "verification_level": 0,
    "unavailable": false,
    "splash": null,
    "roles": [
      {
        "position": -1,
        "permissions": 37215297,
        "name": "@everyone",
        "mentionable": false,
        "managed": false,
        "id": "1",
        "hoist": false,
        "color": 0
      },
      {
        "position": 1,
        "permissions": 66583679,
        "name": "role",
        "mentionable": false,
        "managed": false,
        "id": "1",
        "hoist": true,
        "color": 7419530
      },
      {
        "position": 2,
        "permissions": 536345727,
        "name": "role 2",
        "mentionable": false,
        "managed": false,
        "id": "17",
        "hoist": true,
        "color": 2123412
      },
      {
        "position": 3,
        "permissions": 66583679,
        "name": "role 3",
        "mentionable": false,
        "managed": false,
        "id": "166",
        "hoist": true,
        "color": 3447003
      },
      {
        "position": 1,
        "permissions": 37215297,
        "name": "aaaaaa",
        "mentionable": true,
        "managed": false,
        "id": "88",
        "hoist": false,
        "color": 15277667
      },
      {
        "position": 1,
        "permissions": 35840,
        "name": "aaaabsadfasda",
        "mentionable": false,
        "managed": true,
        "id": "643534543",
        "hoist": false,
        "color": 0
      }
    ],
    "region": "us-central",
    "presences": [
      {
        "user": {
          "id": "2342342"
        },
        "status": "online",
        "game": null
      },
      {
        "user": {
          "id": "1233432"
        },
        "status": "online",
        "game": null
      },
      {
        "user": {
          "id": "35353534"
        },
        "status": "online",
        "game": {
          "url": "",
          "type": 0,
          "name": "aaaaaa"
        }
      },
      {
        "user": {
          "id": "12314324"
        },
        "status": "online",
        "game": null
      }
    ],
    "owner_id": "7",
    "name": "guild name",
    "mfa_level": 0,
    "members": [
      {
        "user": {
          "username": "aaa",
          "id": "92781184873947136",
          "discriminator": "6291",
          "avatar": "asdasdadada"
        },
        "roles": [
          "164155714355462146"
        ],
        "nick": "asdasdadas",
        "mute": false,
        "joined_at": "fake",
        "deaf": false
      },
      {
        "user": {
          "username": "aaaaaa",
          "id": "161972119494852608",
          "discriminator": "7653",
          "avatar": "ffffff"
        },
        "roles": [
          "2342432423432"
        ],
        "mute": false,
        "joined_at": "fake",
        "deaf": false
      },
      {
        "user": {
          "username": "aaaaa",
          "id": "167333834952540160",
          "discriminator": "0857",
          "bot": true,
          "avatar": "ffffff"
        },
        "roles": [
          "34534543543"
        ],
        "mute": false,
        "joined_at": "fake",
        "deaf": false
      },
      {
        "user": {
          "username": "aaaaaaa",
          "id": "171403455745884160",
          "discriminator": "0075",
          "avatar": "ffffff"
        },
        "roles": [
          "56465464"
        ],
        "mute": false,
        "joined_at": "fake",
        "deaf": false
      },
      {
        "user": {
          "username": "asdasdsadas",
          "id": "12312312",
          "discriminator": "7181",
          "bot": true,
          "avatar": "ffffff"
        },
        "roles": [
          "12313212321"
        ],
        "nick": null,
        "mute": false,
        "joined_at": "fake",
        "deaf": false
      },
      {
        "user": {
          "username": "aaaaa",
          "id": "1231231231",
          "discriminator": "2138",
          "bot": true,
          "avatar": "fake"
        },
        "roles": [
          "1231231312"
        ],
        "nick": null,
        "mute": false,
        "joined_at": "fake",
        "deaf": false
      }
    ],
    "member_count": 6,
    "large": false,
    "joined_at": "fake",
    "id": "12321321312321",
    "icon": "fake icon",
    "features": [],
    "emojis": [],
    "default_message_notifications": 0,
    "channels": [
      {
        "type": 0,
        "topic": "",
        "position": 0,
        "permission_overwrites": [
          {
            "type": "role",
            "id": "123131231321",
            "deny": 0,
            "allow": 0
          }
        ],
        "name": "asdadsa",
        "last_message_id": "5676576575",
        "id": "3453543543"
      },
      {
        "user_limit": 0,
        "type": 2,
        "position": 0,
        "permission_overwrites": [],
        "name": "adssadasda",
        "id": "56464564645",
        "bitrate": 63841
      },
      {
        "user_limit": 0,
        "type": 2,
        "position": 1,
        "permission_overwrites": [
          {
            "type": "role",
            "id": "23423432423",
            "deny": 2097152,
            "allow": 0
          }
        ],
        "name": "AFK",
        "id": "23432434242",
        "bitrate": 64000
      },
      {
        "type": 0,
        "topic": null,
        "position": 1,
        "permission_overwrites": [],
        "name": "asdasdasdsa",
        "last_message_id": "234324324242",
        "id": "2342343243242"
      },
      {
        "user_limit": 0,
        "type": 2,
        "position": 2,
        "permission_overwrites": [],
        "name": "asdadsa",
        "id": "32134242342",
        "bitrate": 96000
      }
    ],
    "afk_timeout": 900,
    "afk_channel_id": "23432423423"
  }"#;

    let value: Value = serde_json::from_str(json).unwrap();

    assert!(Guild::decode(value).is_ok());
}
