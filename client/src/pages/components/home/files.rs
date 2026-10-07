use leptos::prelude::*;
use crate::{components::FileItem, pages::components::home::FileFilterSection};

#[derive(Clone)]
struct File{
    name: String
}
#[component]
pub fn Files() -> impl IntoView {
    
    let (files, _set_files) = signal(
        vec![
            File {
                name: "Document".to_string()
            },
            File {
                name: "Document".to_string()
            },
            File {
                name: "Document".to_string()
            },
            File {
                name: "Document".to_string()
            },
            File {
                name: "Document".to_string()
            },
            File {
                name: "Document".to_string()
            },
        ]
    );
    view! {
        <div class="files">
            <div class="files-header">
                <p class="inter-normal-black">"All files"</p>
            </div>
            <FileFilterSection/>
            <div class="files-content">
                <For 
                    each = move || files.get()
                    key = |file| file.name.clone()
                    let(child)
                >
                    <FileItem label=child.name/>
                </For>    
            </div>
        </div>   
    }
}