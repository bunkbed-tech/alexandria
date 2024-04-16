use leptonic::prelude::*;
use leptos::*;
use leptos_meta::{Meta, Title};
use leptos_router::*;
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

#[derive(Serialize)]
struct SaveResourceTaggingArgs<'a> {
    status: &'a ReadSignal<bool>,
    resource: &'a Resource,
}

#[component]
fn resource_attribute_button(resource: Resource) -> impl IntoView {
    let (owned, set_owned) = create_signal(false);
    let (want_to_own, set_want_to_own) = create_signal(false);
    let (want_to_try, set_want_to_try) = create_signal(false);
    let save_resource_tagging = create_action(move |_: &()| async move {
        set_owned.update(|status| *status = !*status);
        let args = to_value(&SaveResourceTaggingArgs {
            status: &owned,
            resource: &resource,
        })
        .unwrap();
        from_value::<Resource>(invoke("save_resource_tagging", args).await)
            .map_err(|err| err.to_string())
    });

    view! {
        <Col md=1 sm=1 xs=1 style="justify-content: center;">
            <Stack spacing=Size::Em(0.6) style="justify-content: flex-end;">
                <img src={&resource.thumbnail} />
                <p style="max-width: 200px; white-space: nowrap; overflow: hidden; text-align: center; text-overflow: ellipsis;">{resource.title} " (" {resource.year_published} ")"</p>
                <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.3)>
                    <ButtonGroup>
                        <Button on_click=move |_| save_resource_tagging.dispatch(())>{move || match owned.get() {
                            true => "Owned",
                            false => "Add to Collection",
                        }}</Button>
                        <Button on_click=move |_| set_want_to_own.update(|status| {*status = !*status})>{move || match want_to_own.get() {
                            true => "Wishlist",
                            false => "Want to Own?",
                        }}</Button>
                        <Button on_click=move |_| set_want_to_try.update(|status| {*status = !*status})>{move || match want_to_try.get() {
                            true => "Want to Try",
                            false => "Want to Try?",
                        }}</Button>
                    </ButtonGroup>
                </Stack>
            </Stack>
        </Col>
    }
}

#[component]
fn results(resources: Vec<Resource>) -> impl IntoView {
    let resource_chunks: Vec<Vec<Resource>> =
        resources.chunks(3).map(|chunk| chunk.to_vec()).collect();
    let resource_tile = move |resource: Resource| {
        view! {
            <ResourceAttributeButton resource=resource />
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
fn media_list_search() -> impl IntoView {
    let (query, set_query) = create_signal(String::new());
    let fetch_bgg_resources = create_action(move |_: &()| async move {
        let args = to_value(&SearchBGGArgs { query: &query }).unwrap();
        from_value::<Vec<Resource>>(invoke("search_bgg", args).await).map_err(|err| err.to_string())
    });

    view! {
        <H2>BoardGameGeek</H2>
        <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0)>
            <TextInput get=query set=set_query placeholder="Enter a query ..."/>
            <Button color=ButtonColor::Primary on_click=move |_| fetch_bgg_resources.dispatch(())>Search</Button>
        </Stack>
        {move || match fetch_bgg_resources.pending().get() {
            true => view! { <Skeleton animated=false>"Loading..."</Skeleton> }.into_view(),
            false => match fetch_bgg_resources.value().get() {
                None => view! {}.into_view(),
                Some(Err(error)) => view! { <p>"Error: " {error}</p> }.into_view(),
                Some(Ok(resources)) => view! { <Results resources=resources /> }.into_view(),
            },
        }}
    }
}

#[component]
fn page() -> impl IntoView {
    view! {
        <Box style="padding: 0.5em; display: flex; flex-direction: column; align-items: center; overflow-y: scroll; width: 100%; height: 100%;">
            <Stack spacing=Size::Em(2.0)>
                <Outlet />
            </Stack>
        </Box>
    }
}

#[component]
fn sidebar_button(children: Children) -> impl IntoView {
    view! {
        <Button style="padding: 12px; border: 0; border-radius: 0; width: 100%; justify-content: left; color: var(--collapsible-header-color); background-color: var(--collapsible-header-background-color);" on_click=move |_| {}>{children()}</Button>
    }
}

#[component]
fn search_page() -> impl IntoView {
    view! {
        <MediaListSearch />
    }
}

#[component]
fn owned_page() -> impl IntoView {
    view! {
        <Tabs mount=Mount::Once>
            <Tab name="tab-1" label="Board Games".into_view()><MediaListSearch /></Tab>
            <Tab name="tab-2" label="Tab 2".into_view()>"Content of tab 2"</Tab>
            <Tab name="tab-3" label="Tab 3".into_view()>"Content of tab 3"</Tab>
        </Tabs>
    }
}

#[component]
fn alexandria() -> impl IntoView {
    tracing::info!("Welcome to Alexandria!");

    view! {
        <Meta name="charset" content="UTF-8"/>
        <Meta name="description" content="Track everything"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        <Title text="Alexandria"/>


        <Router>
        <Root default_theme=LeptonicTheme::default()>
            <Box style="display: flex; flex-direction: row; justify-content: flex-start; align-items: flex-start; width: 100%; height: 100vh; overflow: hidden;">
                <Drawer side=DrawerSide::Left shown=true style="overflow-y: scroll; height: 100vh;">
                    <Stack spacing=Size::Em(0.0)>
                        <SidebarButton><a href="/">Search</a></SidebarButton>
                        <Collapsible>
                            <CollapsibleHeader slot>"Lists"</CollapsibleHeader>
                            <CollapsibleBody slot><SidebarButton><a href="/owned">"Owned"</a></SidebarButton></CollapsibleBody>
                        </Collapsible>
                    </Stack>
                </Drawer>
                <Routes>
                    <Route path="" view=move || view! { <Page/> }>
                        <Route path="/" view=move || view! { <SearchPage /> } />
                        <Route path="/owned" view=move || view! { <OwnedPage /> } />
                    </Route>
                </Routes>
            </Box>
        </Root>
        </Router>
    }
}

fn main() {
    mount_to_body(|| view! { <Alexandria /> })
}
