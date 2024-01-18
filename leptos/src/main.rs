use leptos::*;
use serde::Serialize;
use tauri_sys::tauri;

#[derive(Serialize)]
struct EmptyArgs {}

#[derive(Serialize)]
struct SearchBGGArgs{
    query: String,
}

#[component]
fn alexandria() -> impl IntoView {
    let (query, set_query) = create_signal("".to_string());
    let resources = create_local_resource(|| (), |_| async move { tauri::invoke::<_, String>("list_resources", &EmptyArgs {}).await });
    let bgg_resources = create_local_resource(|| (), |_| async move { tauri::invoke::<_, String>("search_bgg", &EmptyArgs { query }).await });
    view! {
        <div>
            <h1>List of resources</h1>
            {move || match resources.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(result) => view! { <p>{result}</p> }.into_view()
            }}
            <h2>Boardgame Geek</h2>
            <div>
                <input type="text"
                       value=query
                       on:input=move |event| set_query.set(event.target.value)
                />
                <button>Search</button>
            </div>
            {move || match bgg_resources.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(result) => view! { <p>{result}</p> }.into_view()
            }}
        </div>
    }
}



fn main() {
    mount_to_body(|| view! { <Alexandria /> })
}
