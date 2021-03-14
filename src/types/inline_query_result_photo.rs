use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent, MessageEntity, ParseMode};

/// Represents a link to a photo.
///
/// By default, this photo will be sent by the user with optional caption.
/// Alternatively, you can use `input_message_content` to send a message with
/// the specified content instead of the photo.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultphoto).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultPhoto {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid URL of the photo. Photo must be in **jpeg** format. Photo size
    /// must not exceed 5MB.
    pub photo_url: String,

    /// URL of the thumbnail for the photo.
    pub thumb_url: String,

    /// Width of the photo.
    pub photo_width: Option<i32>,

    /// Height of the photo.
    pub photo_height: Option<i32>,

    /// Title for the result.
    pub title: Option<String>,

    /// Short description of the result.
    pub description: Option<String>,

    /// Caption of the photo to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// List of special entities that appear in the caption, which can be
    /// specified instead of `parse_mode`.
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the photo.
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultPhoto {
    pub fn new<S1, S2, S3>(id: S1, photo_url: S2, thumb_url: S3) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        Self {
            id: id.into(),
            photo_url: photo_url.into(),
            thumb_url: thumb_url.into(),
            photo_width: None,
            photo_height: None,
            title: None,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.id = val.into();
        self
    }

    pub fn photo_url<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.photo_url = val.into();
        self
    }

    pub fn thumb_url<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.thumb_url = val.into();
        self
    }

    pub fn photo_width<S>(mut self, val: i32) -> Self {
        self.photo_width = Some(val);
        self
    }

    pub fn photo_height<S>(mut self, val: i32) -> Self {
        self.photo_height = Some(val);
        self
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = Some(val.into());
        self
    }

    pub fn description<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.description = Some(val.into());
        self
    }

    pub fn caption<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub fn parse_mode<S>(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn caption_entities<C>(mut self, val: C) -> Self
    where
        C: IntoIterator<Item = MessageEntity>,
    {
        self.caption_entities = Some(val.into_iter().collect());
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }

    pub fn input_message_content(mut self, val: InputMessageContent) -> Self {
        self.input_message_content = Some(val);
        self
    }
}
