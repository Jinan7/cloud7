use leptos::prelude::*;
use reactive_stores::Store;


#[derive(Store, Clone)]
pub struct UploadPayload {
    pub key: uuid::Uuid,
    pub file_name: String,
    pub percentage: u8,
}

#[derive(Store, Clone)]
pub struct Uploads {
    #[store(key: uuid::Uuid = |row| row.key.clone())]
    pub rows: Vec<UploadPayload> 
}


#[component]
pub fn Context(
    children: Children
) -> impl IntoView {
    
    let uploads = Store::new(Uploads { rows: Vec::new() });

    provide_context(uploads);
    view! {
        || {children()} 
    }
}

pub fn use_upload_context() -> Store<Uploads>{
    use_context::<Store<Uploads>>().expect("couldnt find Store<context::Uploads> in context")
}