use yew::prelude::*;
use crate::models::Device;

#[function_component(DeviceDetails)]
pub fn device_details_in_network(devices: &Vec<&Device>) -> Html {
    let mut select_list: Vec<Device> = Vec::new();

    fn handle_select(device_id: usize) {
        select_list.push(&devices[device_id]);
    }

    let devices_listed = devices.iter().map(|device| html! {
        <button onclick={ handle_select(device.id) } key={ device.id }>
            { format!("{}", device.local_ip) }
        </button>
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