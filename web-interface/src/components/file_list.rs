use yew::prelude::*;
use yew::prelude::Properties;
use crate::models::Device;
use crate::models::File;

#[derive(Clone, Properties, PartialEq)]
pub struct FileListProps {
    pub devices: Vec<Device>,
    pub files: Vec<File>
}

#[function_component(FileList)]
pub fn file_list(props: &FileListProps) -> Html {
    let devices = &props.devices;
    let files = &props.files;

    let files_listed = files.iter().map(|file| html! {
        <>
        <span>{ format!("{}", file.name) } </span>
        <button key={ file.id }>
            { "X" }
        </button>
        </>
    }).collect::<Html>();

    html! {
        if devices.len() > 0 {
            <div class="file-container list">
                if files.len() > 0 {
                    { files_listed }
                }

                else {
                    <h2>{ "No files to transfer" }</h2>
                    <p>
                        <small>
                            { "Please drop a file here" }
                        </small>
                    </p>
                }
            </div>
        }
    }
}