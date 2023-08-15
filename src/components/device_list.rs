use yew::prelude::*;
use yew::prelude::Properties;
use yewdux::prelude::use_store;
use crate::models::Device;
// use crate::stores::DeviceStore;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub devices: Vec<Device>
}

#[function_component(DeviceDetails)]
pub fn device_details(props: &Props) -> Html {
    let devices: &Vec<Device> = &props.devices;

    let devices_listed = devices.iter().map(|device| html! {
        // <DeviceItem device={device} />
        <h3 key={ device.id }>{ "Device item" }</h3>
    }).collect::<Html>();

    html! {
        <div class="list-container">
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

#[function_component(DeviceItem)]
fn device_item(device: &Device) -> Html {
    // let (store, dispatch) = use_store::<DeviceStore>();
    // println!("DEVICE STORE {:?}", store);

    html! {
        <button key={ device.id }>
            { format!("{}", device.local_ip) }
        </button>
    }
}