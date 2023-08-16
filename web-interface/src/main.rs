use yew::prelude::*;
use crate::components::device_list::DeviceList;
use crate::components::file_list::FileList;
pub use models::Device;
pub use models::File;
mod components;
mod models;
mod stores;

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

        <DeviceList devices={ devices.clone() } />
        <FileList devices={ devices.clone() } files={ vec![] } />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}