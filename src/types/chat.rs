use serde::{Deserialize, Serialize};

use crate::types::{ChatLocation, ChatPermissions, ChatPhoto, Message};

/// This object represents a chat.
///
/// [The official docs](https://core.telegram.org/bots/api#chat).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Chat {
    /// A unique identifier for this chat. This number may be greater than 32
    /// bits and some programming languages may have difficulty/silent defects
    /// in interpreting it. But it is smaller than 52 bits, so a signed 64 bit
    /// integer or double-precision float type are safe for storing this
    /// identifier.
    pub id: i64,

    #[serde(flatten)]
    pub kind: ChatKind,

    /// A chat photo. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub photo: Option<ChatPhoto>,

    /// The most recent pinned message (by sending date). Returned only in
    /// [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub pinned_message: Option<Box<Message>>,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatKind {
    Public(ChatPublic),
    Private(ChatPrivate),
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatPublic {
    /// A title, for supergroups, channels and group chats.
    pub title: Option<String>,

    #[serde(flatten)]
    pub kind: PublicChatKind,

    /// A description, for groups, supergroups and channel chats. Returned
    /// only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub description: Option<String>,

    /// A chat invite link, for groups, supergroups and channel chats. Each
    /// administrator in a chat generates their own invite links, so the
    /// bot must first generate the link using
    /// [`ExportChatInviteLink`]. Returned only in
    /// [`GetChat`].
    ///
    /// [`ExportChatInviteLink`]:
    /// crate::payloads::ExportChatInviteLink
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub invite_link: Option<String>,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatPrivate {
    /// A dummy field. Used to ensure that the `type` field is equal to
    /// `private`.
    #[serde(rename = "type")]
    #[serde(deserialize_with = "assert_private_field")]
    pub type_: (),

    /// A username, for private chats, supergroups and channels if
    /// available.
    pub username: Option<String>,

    /// A first name of the other party in a private chat.
    pub first_name: Option<String>,

    /// A last name of the other party in a private chat.
    pub last_name: Option<String>,

    /// Bio of the other party in a private chat. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub bio: Option<String>,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum PublicChatKind {
    Channel(PublicChatChannel),
    Group(PublicChatGroup),
    Supergroup(PublicChatSupergroup),
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PublicChatChannel {
    /// A username, for private chats, supergroups and channels if available.
    pub username: Option<String>,

    /// Unique identifier for the linked chat, i.e. the discussion group
    /// identifier for a channel and vice versa. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub linked_chat_id: Option<i64>,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PublicChatGroup {
    /// A default chat member permissions, for groups and supergroups. Returned
    /// only from [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub permissions: Option<ChatPermissions>,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicChatSupergroup {
    /// A username, for private chats, supergroups and channels if
    /// available.
    pub username: Option<String>,

    /// For supergroups, name of group sticker set. Returned only from
    /// [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub sticker_set_name: Option<String>,

    /// `true`, if the bot can change the group sticker set. Returned only
    /// from [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub can_set_sticker_set: Option<bool>,

    /// A default chat member permissions, for groups and supergroups.
    /// Returned only from [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub permissions: Option<ChatPermissions>,

    /// The minimum allowed delay between consecutive messages sent by each
    /// unpriviledged user. Returned only from [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub slow_mode_delay: Option<i32>,

    /// Unique identifier for the linked chat, i.e. the discussion group
    /// identifier for a channel and vice versa. Returned only in [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub linked_chat_id: Option<i64>,

    /// The location to which the supergroup is connected. Returned only in
    /// [`GetChat`].
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    pub location: Option<ChatLocation>,
}

struct PrivateChatKindVisitor;

impl<'de> serde::de::Visitor<'de> for PrivateChatKindVisitor {
    type Value = ();

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, r#"field equal to "private""#)
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        match v {
            "private" => Ok(()),
            _ => Err(E::invalid_value(
                serde::de::Unexpected::Str(v),
                &r#""private""#,
            )),
        }
    }
}

fn assert_private_field<'de, D>(des: D) -> Result<(), D::Error>
where
    D: serde::Deserializer<'de>,
{
    des.deserialize_str(PrivateChatKindVisitor)
}

impl Chat {
    pub fn is_private(&self) -> bool {
        matches!(self.kind, ChatKind::Private(_))
    }
    pub fn is_group(&self) -> bool {
        matches!(
            self.kind,
            ChatKind::Public(ChatPublic {
                kind: PublicChatKind::Group(_),
                ..
            })
        )
    }
    pub fn is_supergroup(&self) -> bool {
        matches!(
            self.kind,
            ChatKind::Public(ChatPublic {
                kind: PublicChatKind::Supergroup(_),
                ..
            })
        )
    }
    pub fn is_channel(&self) -> bool {
        matches!(
            self.kind,
            ChatKind::Public(ChatPublic {
                kind: PublicChatKind::Channel(_),
                ..
            })
        )
    }

    pub fn is_chat(&self) -> bool {
        self.is_private() || self.is_group() || self.is_supergroup()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;

    use crate::types::*;

    #[test]
    fn channel_de() {
        let expected = Chat {
            id: -1,
            kind: ChatKind::Public(ChatPublic {
                title: None,
                kind: PublicChatKind::Channel(PublicChatChannel {
                    username: Some("channelname".into()),
                    linked_chat_id: None,
                }),
                description: None,
                invite_link: None,
            }),
            photo: None,
            pinned_message: None,
        };
        let actual = from_str(r#"{"id":-1,"type":"channel","username":"channelname"}"#).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn private_chat_de() {
        assert_eq!(
            Chat {
                id: 0,
                kind: ChatKind::Private(ChatPrivate {
                    type_: (),
                    username: Some("username".into()),
                    first_name: Some("Anon".into()),
                    last_name: None,
                    bio: None,
                }),
                photo: None,
                pinned_message: None,
            },
            from_str(r#"{"id":0,"type":"private","username":"username","first_name":"Anon"}"#)
                .unwrap()
        );
    }

    #[test]
    fn private_chat_de_wrong_type_field() {
        assert!(from_str::<Chat>(r#"{"id":0,"type":"WRONG"}"#).is_err());
    }
}
