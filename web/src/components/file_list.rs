use yew::prelude::*;
use crate::models::Device;
use crate::models::File;

#[function_component(FileDetails)]
pub fn file_details(devices: &Vec<&Device>) -> Html {
    let mut file_list: Vec<File> = Vec::new();

    fn handle_remove(file_id: usize) {
        file_list.retain(|&file| file.id != file_id);
    }

    let files_listed = file_list.iter().map(|file| html! {
        <>
        <span>{ format!("{}", file.name) } </span>
        <button onclick={ handle_remove(file.id) } key={ file.id }>
            { "X" }
        </button>
        </>
    }).collect::<Html>();

    html! {
        if devices.len() > 0 {
            <div class="list-container">
                if files_listed.len() > 0 {
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