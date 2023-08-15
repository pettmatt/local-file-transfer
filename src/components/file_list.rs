use yew::prelude::*;
use yew::Properties;
use crate::models::Device;
use crate::models::File;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub devices: Vec<Device>
}

#[function_component(FileDetails)]
pub fn file_details(props: &Props) -> Html {
    // let devices: Vec<Device> = props.devices;

    // let files_listed = file_list.iter().map(|file| html! {
    //     <>
    //     <span>{ format!("{}", file.name) } </span>
    //     <button key={ file.id }>
    //         { "X" }
    //     </button>
    //     </>
    // }).collect::<Html>();

    html! {
        // if devices.len() > 0 {
            <div class="list-container">
                // if file_list.len() > 0 {
                //     { files_listed }
                // }

                // else {
                //     <h2>{ "No files to transfer" }</h2>
                //     <p>
                //         <small>
                //             { "Please drop a file here" }
                //         </small>
                //     </p>
                // }
            </div>
        // }
    }
}