use std::collections::HashMap;

use serde::{Deserialize};

use super::asset::Asset;

#[derive(Debug, Deserialize)]
struct TextChannel {
    #[serde(rename = "_id")]
    id: String,
    server: String,
    name: String,
    description: Option<String>,
    nonce: Option<String>,
    icon: Option<Asset>,
    last_message: Option<String>,
    default_permissions: Option<i32>,
    role_permissions: HashMap<String, i32>,
    nsfw: bool,
}

#[derive(Debug, Deserialize)]
struct VoiceChannel {
    #[serde(rename = "_id")]
    id: String,
    server: String,
    nonce: Option<String>,
    name: String,
    description: Option<String>,
    icon: Option<Asset>,
    default_permissions: Option<i32>,
    role_permissions: HashMap<String, i32>,
    nsfw: bool
}

// figure out a solution for being able to edit both textchannel and voicechannel structs
#[derive(Debug, Deserialize)]
pub enum Channel {
    Text(TextChannel),
    Voice(VoiceChannel),
}


impl Channel {
    pub fn id(&self) -> &str {
        match self {
            Channel::Text(channel) => &channel.id,
            Channel::Voice(channel) => &channel.id,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Channel::Text(channel) => &channel.name,
            Channel::Voice(channel) => &channel.name,
        }
    }

    pub fn description(&self) -> Option<&str> {
        match self {
            Channel::Text(channel) => channel.description.as_ref().map(|s| s.as_str()),
            Channel::Voice(channel) => channel.description.as_ref().map(|s| s.as_str()),
        }
    }

    pub fn icon(&self) -> Option<&Asset> {
        match self {
            Channel::Text(channel) => channel.icon.as_ref(),
            Channel::Voice(channel) => channel.icon.as_ref(),
        }
    }

    pub fn last_message(&self) -> Option<&String> {
        match self {
            Channel::Text(channel) => channel.last_message.as_ref(),
            Channel::Voice(_) => None,
        }
    }

    pub fn default_permissions(&self) -> Option<i32> {
        match self {
            Channel::Text(channel) => channel.default_permissions,
            Channel::Voice(channel) => channel.default_permissions,
        }
    }

    pub fn role_permissions(&self) -> &HashMap<String, i32> {
        match self {
            Channel::Text(channel) => &channel.role_permissions,
            Channel::Voice(channel) => &channel.role_permissions,
        }
    }

    pub fn nsfw(&self) -> bool {
        match self {
            Channel::Text(channel) => channel.nsfw,
            Channel::Voice(channel) => channel.nsfw,
        }
    }

    pub fn server(&self) -> &str {
        match self {
            Channel::Text(channel) => &channel.server,
            Channel::Voice(channel) => &channel.server,
        }
    }

    pub fn is_text(&self) -> bool {
        match self {
            Channel::Text(_) => true,
            Channel::Voice(_) => false,
        }
    }

    pub fn is_voice(&self) -> bool {
        match self {
            Channel::Text(_) => false,
            Channel::Voice(_) => true,
        }
    }

    pub fn edit(&self, name: &str, description: Option<&str>, icon: Option<Asset>, nsfw: bool) -> Result<(), String> {
        todo!();
        match self {
            Channel::Text(channel) => {
                let mut channel = channel.clone();
                Ok(())
            },
            Channel::Voice(channel) => {
                let mut channel = channel.clone();
                Ok(())
            },
        }
    }

    pub fn delete(&self) -> Result<(), String> {
        todo!();
        match self {
            Channel::Text(channel) => {
                let mut channel = channel.clone();
                Ok(())
            },
            Channel::Voice(channel) => {
                let mut channel = channel.clone();
                Ok(())
            },
        }
    }
}
