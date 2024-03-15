use leptonic::prelude::*;
use leptos::*;
use leptos_meta::{Meta, Title};
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

use models::Resource;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize)]
struct SearchBGGArgs<'a> {
    query: &'a ReadSignal<String>,
}

#[component]
fn results(resources: Vec<Resource>) -> impl IntoView {
    let resource_chunks: Vec<Vec<Resource>> =
        resources.chunks(3).map(|chunk| chunk.to_vec()).collect();
    let resource_tile = move |resource: Resource| {
        view! {
            <Col md=3 sm=4 xs=6>
                <Skeleton animated=false>{resource.title} " (" {resource.year_published} ")"</Skeleton>
            </Col>
        }
    };
    let resource_row = move |chunk: Vec<Resource>| {
        view! {
            <Row>{chunk.into_iter().map(resource_tile).collect_view().into_view()}</Row>
        }
    };
    view! {
        <Grid spacing=Size::Em(0.6)>
            {resource_chunks.into_iter().map(resource_row).collect_view().into_view()}
       </Grid>
    }
}

#[component]
fn alexandria() -> impl IntoView {
    tracing::info!("Welcome to Alexandria!");

    let (query, set_query) = create_signal(String::new());
    let fetch_bgg_resources = create_action(move |_: &()| async move {
        let args = to_value(&SearchBGGArgs { query: &query }).unwrap();
        from_value::<Vec<Resource>>(invoke("search_bgg", args).await).map_err(|err| err.to_string())
    });

    view! {
        <Meta name="charset" content="UTF-8"/>
        <Meta name="description" content="Track everything"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        <Title text="Alexandria"/>

        <Root default_theme=LeptonicTheme::default()>
            <Box style="display: flex; flex-direction: column; align-items: center; padding: 1em; min-height: 100%; min-width: 100%">
                <H2>BoardGameGeek</H2>
                <Stack spacing=Size::Em(2.0)>
                    <div style="width: 100%;">
                        <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0)>
                            <TextInput get=query set=set_query placeholder="Enter a query ..."/>
                            <Button on_click=move |_| fetch_bgg_resources.dispatch(())>Search</Button>
                        </Stack>
                        {move || match fetch_bgg_resources.pending().get() {
                            true => view! { <p>"Loading..."</p> }.into_view(),
                            false => match fetch_bgg_resources.value().get() {
                                None => view! {}.into_view(),
                                Some(Err(error)) => view! { <p>"Error: " {error}</p> }.into_view(),
                                Some(Ok(resources)) => view! { <Results resources=resources /> }.into_view(),
                            },
                        }}
                    </div>
                </Stack>
            </Box>
        </Root>
    }
}

fn main() {
    mount_to_body(|| view! { <Alexandria /> })
}
