use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::ImageData;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[component]
async fn LeftNavItems<G: Html>(cx: Scope<'_>) -> View<G> {
    // wtf   ¯\_(ツ)_/¯

    let app_state = use_context::<Signal<AppState>>(cx);
    // let mut server = app_state.get().server.clone();

    // let s = server
    //     .drain(..)
    //     .map(|s| s.get().as_ref().clone())
    //     .collect::<Vec<ServerSource>>();

    // let iter = create_signal(cx, s);

    view! { cx,
        div {
             a(class="list-group-item list-group-item-action", href=format!("#list-item-{}",1)) {
                             "Single Threaded "
                    }
        }
        div {
             a(class="list-group-item list-group-item-action", href=format!("#list-item-{}",2)) {
                             "Multi Threaded via Rayon "
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
                    "FractalThingi"
                }
            }
        }
    }
}
#[derive(Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Color {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

#[derive(Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct FractalResponse {
    pub duration: String,
    pub fractal: FractalImage,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct FractalImage {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) pixels: Vec<Color>,
}

#[derive(Deserialize, Serialize)]
pub struct TileData {
    idx: usize,
    points: Vec<TileDataPoint>,
}

#[derive(Deserialize, Serialize)]
pub struct TileDataPoint {
    x: usize,
    y: usize,
    c: Color,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct ComplexNumber {
    pub(crate) a: f32,
    pub(crate) b: f32,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct FractalRequest {
    pub z1: ComplexNumber,
    pub z2: ComplexNumber,
    pub width: u32,
    pub max_iterations: u32,
    pub colors: u32,
    pub x_tiles: u32,
    pub y_tiles: u32,
}

#[component]
async fn MainContent<G: Html>(cx: Scope<'_>) -> View<G> {
    //    let app_state = use_context::<Signal<AppState>>(cx);

    let start_websocket = move |_| {
        console_log!("handle_save_stats clicked");
        let ws = WebSocket::new("ws://localhost:3000/api/crossbeamtiles");

        let socket = match ws {
            Ok(websocket) => websocket,
            Err(e) => panic!("cant open websocket"),
        };

        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            e.prevent_default();
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                console_log!("message event, received Text: {:?}", txt);
                let t = txt.as_string().unwrap();
                let fractal_response: serde_json::error::Result<FractalResponse> =
                    serde_json::from_str(&t);
                if fractal_response.is_ok() {
                    console_log!("got a valid FractalResponse  {}", &t);

                    let res = fractal_response.unwrap();
                    let width = res.fractal.width;
                    let height = res.fractal.height;

                    let document = web_sys::window().unwrap().document().unwrap();
                    let canvas = document.get_element_by_id("fractal_canvas").unwrap();
                    let canvas: web_sys::HtmlCanvasElement = canvas
                        .dyn_into::<web_sys::HtmlCanvasElement>()
                        .map_err(|_| ())
                        .unwrap();

                    let context = canvas
                        .get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<web_sys::CanvasRenderingContext2d>()
                        .unwrap();

                    console_log!("duration {}", res.duration);
                    let canvas = context.canvas().unwrap();
                    canvas.set_width(width);
                    canvas.set_height(height);

                    // let data = ImageData::new_with_u8_clamped_array_and_sh(
                    //     Clamped(&data),
                    //     res.fractal.width,
                    //     res.fractal.height,
                    // )
                    // .expect("this should work");
                    // ctx.put_image_data(&data, 0.0, 0.0);

                    let image_data = context
                        .get_image_data(0.0, 0.0, width.into(), height.into())
                        .unwrap();

                    console_log!(
                        "writing pixels into iamge_data  width {}, height {}",
                        width,
                        height
                    );
                    let mut data = image_data.data();
                    for x in 0..width {
                        for y in 0..height {
                            let idx_pixel = y * width + x;
                            let idx = y * width * 4 + x * 4;
                            data[idx as usize] = res.fractal.pixels[idx_pixel as usize].r;
                            data[idx as usize + 1] = res.fractal.pixels[idx_pixel as usize].g;
                            data[idx as usize + 2] = res.fractal.pixels[idx_pixel as usize].b;
                            data[idx as usize + 3] = 255;
                        }
                    }
                    let data = ImageData::new_with_u8_clamped_array_and_sh(
                        wasm_bindgen::Clamped(&data),
                        width,
                        height,
                    )
                    .unwrap();
                    let res = context.put_image_data(&data, 0.0, 0.0);
                    match res {
                        Ok(r) => console_log!("writing data successfull "),
                        Err(e) => console_log!("error writing image data {:?}", e),
                    }

                    console_log!("updated data");
                    //  let _ = context.put_image_data(&image_data, 0.0, 0.0);
                } else {
                    console_log!(
                        "gat a message not  a valid FractalResponse, so this is the text {}",
                        &t
                    );
                }
            } else {
                console_log!("message event, received Unknown: {:?}", e.data());
            }
        });

        // set message event handler on WebSocket
        socket.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        // forget the callback to keep it alive
        onmessage_callback.forget();

        let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
            e.prevent_default();
            console_log!("error event: {:?}", e.message());
        });

        socket.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        let cloned_ws = socket.clone();
        let onopen_callback = Closure::<dyn FnMut()>::new(move || {
            console_log!("socket opened");
            // match cloned_ws.send_with_str("ping") {
            //     Ok(_) => console_log!("message successfully sent"),
            //     Err(err) => console_log!("error sending message: {:?}", err),
            // }
            // // send off binary message
            // match cloned_ws.send_with_u8_array(&[0, 1, 2, 3]) {
            //     Ok(_) => console_log!("binary message successfully sent"),
            //     Err(err) => console_log!("error sending message: {:?}", err),
            // }
            let z1 = ComplexNumber { a: -2.0, b: 1.5 };
            let z2 = ComplexNumber { a: 1.8, b: -1.5 };
            // send off text message
            let req = FractalRequest {
                z1,
                z2,
                width: 200,
                max_iterations: 50,
                colors: 256,
                x_tiles: 20,
                y_tiles: 20,
            };
            let req = serde_json::json!(req).to_string();
            console_log!("sending string to server {}", &req);
            match cloned_ws.send_with_str(&req) {
                Ok(_) => console_log!("sending request to server: message successfully sent"),
                Err(err) => console_log!("error sending message: {:?}", err),
            }
        });
        socket.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
    };

    // , on:click=handle_save_stats

    view! { cx,
        div(class = "container-fluid") {
            div(class = "row") {
                div(class = "col-2") {
                    div(class = "list-group",  id="list-example") {
                        LeftNavItems
                    }
                }
                div(class="col"){
                    div(class="d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom") {
                        h1(class="h1"){
                            "Source Servers bla"
                        }
                        div(class="btn-toolbar mb-2 mb-md-0"){
                            div(class="btn-group me-2"){
                                button(class="btn btn-sm btn-outline-secondary", type="button"){
                                    "Render Image Single Threaded"
                                }

                                 button(class="btn btn-sm btn-outline-secondary", type="button"){
                                    "Reset Stats"
                                }
                            }
                        }
                    }
                    div(class ="scrollspy-example", data-bs-smooth-scroll="true", data-bs-spy="scroll",data-bs-target="#list-example", tabindex="0", on:click=start_websocket) {
                        button(class="btn btn-primary", type="button"){
                                    "Start WebSocket"
                        }

                        div(class ="canvas-container"  ) {
                            canvas(id="fractal_canvas",class="fractal-canvas", type="button"){
                                    "Start WebSocket"
                        }
                    }
                    }
                }
            }
        }
    }
}

pub struct AppState {
    name: RcSignal<String>,
}

#[component]
async fn App<G: Html>(cx: Scope<'_>) -> View<G> {
    let app_state = AppState {
        name: create_rc_signal("yoyo".to_string()),
    };

    let app_state = create_signal(cx, app_state);

    // create_effect(cx, || {
    //     println!("State changed. New state value = {:?}", app_state.get());
    // });

    provide_context_ref(cx, app_state);

    // let server_stats_vec = create_memo(cx, || {
    //
    //     println!("hi from create_memo server_stats_vec");
    // });

    view! { cx,
        main {
            Header
            MainContent
        }

    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| view! { cx, App {} });
}
