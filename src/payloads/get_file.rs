// This file is auto generated by `cg` <https://github.com/teloxide/cg> (24572cd + local changes).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::File;

impl_payload! {
    /// Use this method to get basic info about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a [`File`] object is returned. The file can then be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`, where `<file_path>` is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling [`GetFile`] again.
    ///
    /// [`File`]: crate::types::File
    /// [`GetFile`]: crate::payloads::GetFile
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub GetFile (GetFileSetters) => File {
        required {
            /// File identifier to get info about
            pub file_id: String [into],
        }
    }
}
