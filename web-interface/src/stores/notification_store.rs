use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "session", storage_tab_sync)]
pub struct NotificationStore {
    pub show: bool,
    pub message: String,
    pub message_type: String
}

pub fn set_notification_message(message: String, message_type: String, dispatch: Dispatch<NotificationStore>) {
    dispatch.reduce_mut(move |store| {
        store.message_type = message_type;
        store.message = message;
        store.show = true;
    })
}

pub fn hide_notification(dispatch: Dispatch<NotificationStore>) {
    dispatch.reduce_mut(move |store| {
        store.message_type = "".to_string();
        store.message = "".to_string();
        store.show = false;
    })
}