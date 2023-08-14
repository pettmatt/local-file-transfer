use yew::prelude::*;

use components::DeviceDetails;
use components::FileDetails;
pub use models::Device;
pub use models::File;
mod components;
mod models;

#[function_component(App)]
fn app() -> Html {
    let devices: Vec<Device> = vec![
        Device {
            id: 0,
            local_ip: "test 01".to_string()
        },
        Device {
            id: 1,
            local_ip: "test 02".to_string()
        }
    ];

    html! {
        <>
        <h1>{ "Hello World" }</h1>

        <DeviceDetails devices={ devices.clone() } />
        <FileDetails />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}