use serde::{Deserialize, Serialize};
use yewdux::prelude::*;
use std::fmt::Debug;

use crate::models::File;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "session", storage_tab_sync)]
pub struct FileStore {
    pub file_list: Vec<File>,
    pub received_file_list: Vec<File>,
    pub selected_file_list: Vec<File>
}

pub fn set_files(file: File, dispatch: Dispatch<FileStore>) {
    dispatch.reduce_mut(move |store| {
        store.selected_file_list.push(file);
    })
}

pub fn remove_selected_file(remove_id: usize, dispatch: Dispatch<FileStore>) {
    dispatch.reduce_mut(move |store| {
        store.selected_file_list.retain(|file| file.id != remove_id);
    })
}

pub fn remove_file(remove_id: usize, dispatch: Dispatch<FileStore>) {
    dispatch.reduce_mut(move |store| {
        store.file_list.retain(|file| file.id != remove_id);
    })
}

pub fn add_received_file(file: File, dispatch: Dispatch<FileStore>) {
    dispatch.reduce_mut(move |store| {
        store.received_file_list.push(file);
    })
}

pub fn remove_received_file(remove_id: usize, dispatch: Dispatch<FileStore>) {
    dispatch.reduce_mut(move |store| {
        store.file_list.retain(|file| file.id != remove_id);
    })
}