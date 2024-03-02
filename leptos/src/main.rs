use leptos::*;
use serde::Serialize;
use tauri_sys::tauri;

use models::Resource;

#[derive(Serialize, Clone)]
struct SearchBGGArgs {
    query: ReadSignal<String>,
}

#[component]
fn alexandria() -> impl IntoView {
    let (query, set_query) = create_signal("".to_string());
	let fetch_bgg_resources = create_action(|input: &SearchBGGArgs| {
        let args = input.clone();
        async move { tauri::invoke::<_, Vec<Resource>>("search_bgg", &args).await.map_err(|err| err.to_string()) }
    });
    let search_args = SearchBGGArgs { query };
    let resource_row = move |resource: Resource| view! { <p>{resource.title} " (" {resource.year_published} ")"</p> };

    view! {
        <div>
            <h2>Boardgamegeek</h2>
            <div>
                <input type="text"
                       value=query
                       on:input=move |event| set_query.set(event_target_value(&event))
                />
            <button on:click=move |_| fetch_bgg_resources.dispatch(search_args.clone())>Search</button>
            </div>
            {move || match fetch_bgg_resources.value().get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(Err(error)) => view! { <p>"Error: " {error}</p> }.into_view(),
                Some(Ok(resources)) => resources.into_iter().map(resource_row).collect_view().into_view(),
            }}
        </div>
    }
}



fn main() {
    mount_to_body(|| view! { <Alexandria /> })
}
