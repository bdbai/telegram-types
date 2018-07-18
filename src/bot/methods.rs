//! Request parameters types of Telegram bot methods.
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::default::Default;
use std::error::Error;
use std::fmt;
use super::types;
use super::types::{ChatId, ForceReply, InlineKeyboardMarkup, MessageId,
                   ParseMode, ReplyKeyboardMarkup, ReplyKeyboardRemove, UpdateId, UserId};


/// Chat integer identifier or username
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
pub enum ChatTarget {
    Id(ChatId),
    Username(String),
}


/// Use this method to receive incoming updates using long
/// polling ([wiki](https://en.wikipedia.org/wiki/Push_technology#Long_polling)).
/// An Array of [`Update`](types::Update) objects is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GetUpdates {
    pub offset: Option<UpdateId>,
    pub limit: Option<i32>,
    pub timeout: Option<i32>,
    pub allowed_updates: Option<Vec<String>>,
}


impl GetUpdates {
    pub fn new() -> GetUpdates {
        Default::default()
    }

    pub fn offset(&mut self, x: UpdateId) {
        self.offset = Some(x)
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct ApiError {
    error_code: i32,
    description: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ERROR {}] {}", self.error_code, self.description)
    }
}


impl Error for ApiError {
    fn description(&self) -> &str {
        self.description.as_ref()
    }
}



/// Use this method to specify a url and receive incoming updates via an outgoing webhook.
/// Whenever there is an update for the bot, we will send an HTTPS POST request to the specified
/// url, containing a JSON-serialized [`Update`](types::Update). In case of an unsuccessful request, we will give up
/// after a reasonable amount of attempts. Returns True on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetWebhook {
    pub url: String,
    // certificate
    pub max_connections: Option<i32>,
    pub allowed_updates: Option<Vec<String>>,
}


impl SetWebhook {
    pub fn new(url: String) -> SetWebhook {
        SetWebhook {
            url,
            max_connections: None,
            allowed_updates: None,
        }
    }
}


/// Kinds of reply markup.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboard(InlineKeyboardMarkup),
    ReplyKeyboard(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

/// Send text messages. On success, the sent [`Message`](types::Message) is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendMessage {
    pub chat_id: ChatTarget,
    pub text: String,
    pub parse_mode: Option<ParseMode>,
    pub disable_web_page_preview: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<MessageId>,
    pub reply_markup: Option<ReplyMarkup>,
}


impl SendMessage {
    pub fn new(chat_id: ChatTarget, text: String) -> SendMessage {
        SendMessage {
            chat_id,
            text,
            parse_mode: None,
            disable_web_page_preview: Some(false),
            reply_to_message_id: None,
            disable_notification: Some(false),
            reply_markup: None,
        }
    }

    pub fn reply(chat_id: ChatTarget, text: String, message_id: MessageId) -> SendMessage {
        let message = Self::new(chat_id, text);
        SendMessage {
            reply_to_message_id: Some(message_id),
            ..message
        }
    }
}


/// Use this method to forward messages of any kind. On success, the sent `Message` is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ForwardMessage {
    pub chat_id: ChatTarget,
    pub from_chat_id: ChatTarget,
    pub message_id: MessageId,
}

/// To get a list of profile pictures for a user. Returns a [`UserProfilePhotos`](types::UserProfilePhotos) object.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetUserProfilePhotos {
    pub user_id: UserId,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}


/// Use this method to get up to date information about the chat (current name of the user
/// for one-on-one conversations, current username of a user, group or channel, etc.).
///
/// Returns a [`Chat`] object on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChat {
    pub chat_id: ChatTarget,
}

/// Use this method to get the number of members in a chat. Returns `Int` on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChatMembersCount {
    pub chat_id: ChatTarget,
}

/// Use this method to get a list of administrators in a chat. On success, returns an Array
/// of `ChatMember` objects that contains information about all chat administrators except
/// other bots. If the chat is a group or a supergroup and no administrators were appointed,
/// only the creator will be returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChatAdministrators {
    pub chat_id: ChatTarget,
}

/// Use this method to get information about a member of a chat. Returns a `ChatMember`
/// object on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetChatMember {
    pub chat_id: ChatTarget,
    pub user_id: UserId,
}


/// Use this method to edit text and game messages sent by the bot or via the bot (for inline bots).
/// On success, if edited message is sent by the bot, the edited [`Message`](types::Message) is
/// returned, otherwise True is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EditMessageText {
    pub chat_id: Option<ChatTarget>,
    pub message_id: Option<MessageId>,
    pub inline_message_id: Option<String>,
    pub text: String,
    pub parse_mode: Option<ParseMode>,
    pub disable_web_page_preview: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to edit captions of messages sent by the bot or via the bot (for inline bots).
/// On success, if edited message is sent by the bot, the edited [`Message`](types::Message) is
/// returned, otherwise True is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EditMessageCaption {
    pub chat_id: Option<ChatTarget>,
    pub message_id: Option<MessageId>,
    pub inline_message_id: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}


/// Use this method to edit only the reply markup of messages sent by the bot or via the bot (for
/// inline bots). On success, if edited message is sent by the bot, the edited [`Message`](types::Message)
/// is returned, otherwise True is returned.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EditMessageReplyMarkup {
    pub chat_id: Option<ChatTarget>,
    pub message_id: Option<MessageId>,
    pub inline_message_id: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}


/// Use this method to delete a message, including service messages, with the following limitations:
///
/// - A message can only be deleted if it was sent less than 48 hours ago.
/// - Bots can delete outgoing messages in groups and supergroups.
/// - Bots granted can_post_messages permissions can delete outgoing messages in channels.
/// - If the bot is an administrator of a group, it can delete any message there.
/// - If the bot has can_delete_messages permission in a supergroup or a channel, it can delete any message there.
///
/// Returns True on success.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeleteMessage {
    pub chat_id: ChatTarget,
    pub message_id: MessageId,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GetMe;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteWebhook;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetWebhookInfo;

/// Telegram methods.
pub trait Method: Serialize {
    /// Method name in the Telegram Bot API url.
    const NAME: &'static str;
    /// Method return type.
    type Item: DeserializeOwned;

    /// Get method url.
    fn url(token: String) -> String {
        format!("https://api.telegram.org/bot{}/{}", token, Self::NAME)
    }
}

macro_rules! impl_method {
    ($Type: ty, $name: expr, $Item: ty) => {
        impl Method for $Type {
            const NAME: &'static str = $name;
            type Item = $Item;
        }
    };

    ($Type: ty, $name: expr) => { impl_method!($Type, $name, bool); };
}


impl_method!(GetUpdates, "getUpdates", Vec<types::Update>);
impl_method!(GetMe, "getMe", types::User);
impl_method!(SetWebhook, "setWebhook");
impl_method!(DeleteWebhook, "deleteWebhook");
impl_method!(GetWebhookInfo, "getWebhookInfo", types::WebhookInfo);
impl_method!(GetChat, "getChat", types::Chat);
impl_method!(SendMessage, "sendMessage", types::Message);
impl_method!(ForwardMessage, "forwardMessage", types::Message);
impl_method!(EditMessageText, "editMessageText", types::Message);
impl_method!(DeleteMessage, "deleteMessage");
impl_method!(EditMessageCaption, "editMessageCaption");


#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TelegramResult<T> // WTF! JUST WORK!
{
    pub ok: bool,
    pub description: Option<String>,
    pub err_code: Option<i32>,
    pub result: Option<T>,
}

pub type UpdateList = TelegramResult<Vec<types::Update>>;
