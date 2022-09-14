use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use time::OffsetDateTime;

use super::Context;
use crate::{
    http::Request,
    model::{
        embed::Embed,
        message:*
        permission::Permissions,
        Id, member::ThreadMember,
        user::User,
        message::*
    },
};

#[doc = discord_url!(
"https://discord.com/developers/docs/resources/channel#create-message"
)]
/// TODO: 'FILES MUST BE ATTACHED USING A MULTIPART/FORM-DATA BODY AS DESXCRIBED IN UPLOADING FILES'
///  At least one of content, embeds, sticker_ids, components, or files[n] is required.
pub struct MessageSay {
    content: Option<String>,
    nonce:  Option<String>, // tee hee funny nonce
    tts: Option<bool>,
    embeds: Vec<Embed>,
    allowed_mentions: Option<AllowedMentions>,
    message_reference: Option<MessageReference>,
    components: Vec<Message>,
    sticker_ids: Vec<Id>,
    // This stuff should be handled via Attachment
    //files: Vec<String>, // TODO: FILES
    //payload_json: Option<String>, // uploading files stuff
    attachments: Vec<Attachment>,
    flags: Option<u8>,
}

impl MessageSay {
    pub fn validate(&self) -> bool {
        // eek
        self.content.is_some()
            || !self.embeds.is_empty()
            || !self.sticker_ids.is_empty()
            || !self.components.is_empty()
    }
    pub fn content(self, text: &str) -> Self {
        Self {
            content: Some(text.to_string()),
            ..self
        }
    }
    pub fn nonce(self, nonce: &str) -> Self {
        Self {
            nonce: Some(nonce.to_string()),
            ..self
        }
    }
    pub fn tts(self, tts: bool) -> Self {
        Self {
            tts: Some(tts),
            ..self
        }
    }
    pub fn embeds(self, embeds: Vec<Embed>) -> Self {
        Self {
            embeds,
            ..self
        }
    }
    pub fn allowed_mentions(self, mentions: AllowedMentions) -> Self {
        Self {
            allowed_mentions: Some(mentions),
            ..self
        }
    }
    pub fn message_ref(self, reference: MessageReference) -> Self {
        Self {
            message_reference: Some(reference),
            ..self
        }
    }
    pub fn components(self, components: Vec<Message>) -> Self {
        Self {
            components,
            ..self
        }
    }
    pub fn stickers(self, sticker_ids: Vec<Id>) -> Self {
        Self {
            sticker_ids,
            ..self
        }
    }


}

#[doc = discord_url!(
"https://discord.com/developers/docs/resources/channel#create-message"
)]
/// TODO: 'FILES MUST BE ATTACHED USING A MULTIPART/FORM-DATA BODY AS DESXCRIBED IN UPLOADING FILES'
///  At least one of content, embeds, sticker_ids, components, or files[n] is required.
pub struct AttachmentSay {
    attachments: Vec<Attachment>,
    message: Option<MessageSay>
}

impl AttachmentSay {
    pub fn validate(&self) -> bool {
        // eek
        !self.attachments.is_empty()
    }
    pub fn attach(self) -> Self {
        todo!()
    }


}
