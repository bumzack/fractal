use reqwasm::http::Request;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;

use common::imageserver::models::imageservermodels::Images;

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

const SERVER: &str = "http://api.bumzack.at";
const API_URL_IMAGES: &str = "/api/images";

async fn read_images() -> Result<Images, reqwasm::Error> {
    console_log!("read_images!");

    let url = format!("{}{}", SERVER, API_URL_IMAGES);

    let re = Request::get(&url).send().await?.text().await;

    let response = re.expect("should be a valid Response/Body !!!");
    let images: serde_json::error::Result<Images> = serde_json::from_str(&response);

    let images = images.unwrap();
    console_log!("got images data");

    Ok(images)
}

#[component]
async fn MainContent<G: Html>(cx: Scope<'_>) -> View<G> {
    let images = read_images().await.expect("should load images");
    let img = create_signal(cx, images.images);

    view! { cx,
        div(class="album py-5 bg-body-tertiary") {
             div(class="container") {
                div (class = "row row-cols-1 row-cols-sm-2 row-cols-md-3 g-3") {
                        Keyed (
                            iterable=  img,
                            view=|cx, x| view! { cx,
                              div (class = "col") {
                                div(class="card shadow-sm") {
                                    img (class="card-img-top", src=(x.url)) {

                                    }
                                     div (class="card-body") {
                                        p(class="card-text") {
                                            (x.prompt)
                                        }
                                    }
                                }
                                }
                            },
                            key=|x| x.id,
                        )
                    }
            }
        }
    }
}

#[component]
async fn LeftNavItems<G: Html>(cx: Scope<'_>) -> View<G> {
    // let start_singlethreaded = move |e: MouseEvent| {
    //     e.prevent_default();
    //     console_log!("start_singlethreaded  clicked.   event {:?}", e.target());
    //     spawn_local_scoped(cx, {
    //         async move {
    //             let res = post_single_threaded().await;
    //             match res {
    //                 Ok(_) => {
    //                     console_log!("all good");
    //                 }
    //                 Err(e) => {
    //                     console_log!("error calling server /api/singlethreaded target.  {:?}", e)
    //                 }
    //             }
    //         }
    //     });
    // };

    // button(class="btn btn-primary", type="button", id="singlethreaded" ,on:click=start_singlethreaded) {
    //     "Single threaded"
    // }

    view! { cx,
        div(class = "row", style ="margin-bottom: 10px;") {
            div (class="col-12") {

                br {
                }
            }
        }
    }
}

#[component]
async fn Header<G: Html>(cx: Scope<'_>) -> View<G> {
    view! { cx,
        header(class = "py-3 mb-3 border-bottom") {
            div(class = "container-fluid d-grid gap-3 align-items-center", style ="rid-template-columns: 1fr 2fr;") {
                span(class="navbar-brand mb-0 h1") {
                    "Dolphin Thingi"
                }
            }
        }
    }
}

#[component]
async fn App<G: Html>(cx: Scope<'_>) -> View<G> {
    let app_state = AppState {
        _name: create_rc_signal("yoyo".to_string()),
    };

    let app_state = create_signal(cx, app_state);
    provide_context_ref(cx, app_state);
    view! { cx,
        main {
            Header
            MainContent
        }

    }
}

pub struct AppState {
    _name: RcSignal<String>,
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| view! { cx, App {} });
}
