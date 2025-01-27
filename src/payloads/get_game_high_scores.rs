// This file is auto generated by `cg` <https://github.com/teloxide/cg> (24572cd + local changes).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{TargetMessage, True};

impl_payload! {
    /// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. On success, returns an Array of [`GameHighScore`] objects.
    ///
    /// > This method will currently return scores for the target user, plus two of their closest neighbors on each side. Will also return the top three users if the user and his neighbors are not among them. Please note that this behavior is subject to change.
    ///
    /// [`GameHighScore`]: crate::types::GameHighScore
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub GetGameHighScores (GetGameHighScoresSetters) => True {
        required {
            /// User identifier
            pub user_id: i64,
            /// Target message
            #[serde(flatten)]
            pub target: TargetMessage [into],
        }
    }
}
