use leptos::{prelude::*};
use serde::Deserialize;
use crate::{components::FileItem, pages::components::home::FileFilterSection};

#[derive(Debug, Deserialize, Clone)]
struct GetFilesResponse {
    files: Vec<File>
}

#[derive(Clone, Debug, Deserialize)]
pub struct File{
    id: String,
    name: String,
    size: i64,
}
#[component]
pub fn Files() -> impl IntoView {

    let (files_signal, set_files_signal) = signal(
        Vec::new()
    );

    let (page, set_page) = signal(1);

    let files = LocalResource::new(move || get_files(page.get()));

    let async_files = move || 

        Suspend::new(async move {
            let files = files.await;

            if let Ok(files) = files { 
                set_files_signal.set(files);

                view! {
                    <For 
                        each = move || files_signal.get()
                        key = |file| file.name.clone()
                        let(child)
                    >
                        <FileItem label=child.name/>
                    </For> 
                }.into_any()
            } else {
                view! {
                    <p>"Error loading files"</p>
                }.into_any()
            }

        });
    
    
    
    view! {
        
            <div class="files">
                <div class="files-header">
                    <p class="inter-normal-black">"All files"</p>
                </div>
                <FileFilterSection/>
                <div class="files-content">
                    <Suspense
                        fallback= move || view! { <p>"Loading..."</p>}
                    >
                        {async_files} 
                    </Suspense>   
                </div>
            </div>   
    }
}


async fn get_files(page: i32) -> Result<Vec<File>, String> {

    let response = reqwest::Client::new()
        .get(format!("http://localhost:8000/files?page={}", page))
        .send()
        .await
        .map_err(|e| {
            format!("{}", e)
        })?;

    let files : GetFilesResponse = serde_json::from_str(
        &response
        .text()
        .await
        .map_err(|e| {
            format!("{}", e)
        })?
    )
    .map_err(|e| {
        format!("{}", e)
    })?;

    Ok(files.files)
}