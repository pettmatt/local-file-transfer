use serde::{Deserialize, Serialize};
use yewdux::prelude::*;
use std::fmt::Debug;

use crate::models::Device;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "session", storage_tab_sync)]
pub struct DeviceStore {
    pub device_list: Vec<Device>,
    pub selected_device_list: Vec<Device>
}

pub fn set_devices(device: Device, dispatch: Dispatch<DeviceStore>) {
    dispatch.reduce_mut(move |store| {
        store.selected_device_list.push(device);
    })
}

pub fn remove_device(remove_id: usize, dispatch: Dispatch<DeviceStore>) {
    dispatch.reduce_mut(move |store| {
        store.selected_device_list.retain(|device| device.id != remove_id);
    })
}