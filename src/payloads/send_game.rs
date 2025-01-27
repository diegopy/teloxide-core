// This file is auto generated by `cg` <https://github.com/teloxide/cg> (24572cd + local changes).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{Message, ReplyMarkup};

impl_payload! {
    /// Use this method to send a game. On success, the sent [`Message`] is returned.
    ///
    /// [`Message`]: crate::types::Message
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SendGame (SendGameSetters) => Message {
        required {
            /// Unique identifier for the target chat
            pub chat_id: u32,
            /// Short name of the game, serves as the unique identifier for the game. Set up your games via Botfather.
            pub game_short_name: String [into],
        }
        optional {
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// If the message is a reply, ID of the original message
            pub reply_to_message_id: i32,
            /// Pass _True_, if the message should be sent even if the specified replied-to message is not found
            pub allow_sending_without_reply: bool,
            /// A JSON-serialized object for an [inline keyboard]. If empty, one 'Play game_title' button will be shown. If not empty, the first button must launch the game.
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            pub reply_markup: ReplyMarkup [into],
        }
    }
}
