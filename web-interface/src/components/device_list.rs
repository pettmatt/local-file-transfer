use yew::prelude::*;
use yew::prelude::Properties;
use crate::models::Device;
// use yewdux::prelude::use_store;
// use crate::stores::DeviceStore;

#[derive(Clone, Properties, PartialEq)]
pub struct DeviceListProps {
    pub devices: Vec<Device>
}

#[function_component(DeviceList)]
pub fn device_list(props: &DeviceListProps) -> Html {
    let devices = &props.devices;

    let devices_listed = devices.iter().map(|device| html! {
        <DeviceItem { device } />
    }).collect::<Html>();

    html! {
        <div class="device-container list">
            if devices.len() > 0 {
                { devices_listed }
            }

            else {
                <h2>{ "No devices found" }</h2>
                <p>
                    <small>
                        { "Please make sure you're connected to local network through wifi or ethernet cable" }
                    </small>
                </p>
            }
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct DeviceItemProps {
    pub device: Device
}

#[function_component(DeviceItem)]
fn device_item(props: &DeviceItemProps) -> Html {
    let device = &props.device;
    // let (store, dispatch) = use_store::<DeviceStore>();

    html! {
        <button key={ device.id }>
            { format!("{}", device.local_ip) }
        </button>
    }
}