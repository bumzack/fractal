use reqwasm::http::Request;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlParagraphElement, ImageData, MouseEvent,
};
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

use common::fractal_templates::basic;
use common::image_tile::TileData;
use common::models::{
    FractalRequest, FractalResponse, WebSocketCommand, WebSocketRequest, WebSocketResponse,
};
use common::rational::complex_rational_numbers::ComplexRationalNumber;
use common::rational::rational_numbers::RationalNumber;
use common::rational::request::FractalRequestRational;

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

pub fn draw_to_canvas(
    fractal_response: &FractalResponse,
    context: &CanvasRenderingContext2d,
    canvas: &HtmlCanvasElement,
) {
    let width = fractal_response.fractal.width;
    let height = fractal_response.fractal.height;
    console_log!("width {}   height {}", width, height);

    set_canvas_width_height(width, height, canvas);

    console_log!("duration {}", fractal_response.duration_calculation);

    let image_data = context
        .get_image_data(0.0, 0.0, width.into(), height.into())
        .unwrap();

    console_log!(
        "writing pixels into image_data  width {}, height {}",
        width,
        height
    );

    let mut data = image_data.data();
    for x in 0..width {
        for y in 0..height {
            let idx_pixel = y * width + x;
            let idx = y * width * 4 + x * 4;
            data[idx as usize] = fractal_response.fractal.pixels[idx_pixel as usize].r;
            data[idx as usize + 1] = fractal_response.fractal.pixels[idx_pixel as usize].g;
            data[idx as usize + 2] = fractal_response.fractal.pixels[idx_pixel as usize].b;
            data[idx as usize + 3] = 255;
        }
    }
    let data =
        ImageData::new_with_u8_clamped_array_and_sh(wasm_bindgen::Clamped(&data), width, height)
            .unwrap();
    let res = context.put_image_data(&data, 0.0, 0.0);
    match res {
        Ok(_) => console_log!("writing data successful"),
        Err(e) => console_log!("error writing image data {:?}", e),
    }

    console_log!("updated data");
}

pub fn draw_to_canvas_tiles(
    tile: &TileData,
    context: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
) {
    console_log!(
        "draw tile to canvas {:?}.    width {}, height {}",
        tile,
        width,
        height
    );

    let image_data = context
        .get_image_data(0.0, 0.0, width.into(), height.into())
        .unwrap();

    console_log!(
        "writing pixels into image_data from tile width {}, height {}",
        width,
        height
    );
    let mut data = image_data.data();
    tile.points.iter().for_each(|p| {
        let idx = p.y * width * 4 + p.x * 4;
        data[idx as usize] = p.c.r;
        data[idx as usize + 1] = p.c.g;
        data[idx as usize + 2] = p.c.b;
        data[idx as usize + 3] = 255;
    });

    let data =
        ImageData::new_with_u8_clamped_array_and_sh(wasm_bindgen::Clamped(&data), width, height)
            .unwrap();
    let res = context.put_image_data(&data, 0.0, 0.0);
    match res {
        Ok(_r) => console_log!("writing data form tile successfull "),
        Err(e) => console_log!("error writing tile image data {:?}", e),
    }

    console_log!("updated canvas from tile");
}

pub fn clear_canvas(canvas: &HtmlCanvasElement) {
    canvas.set_width(0);
    canvas.set_height(0);
}

pub fn set_canvas_width_height(width: u32, height: u32, canvas: &HtmlCanvasElement) {
    canvas.set_width(width);
    canvas.set_height(height);
}

pub fn get_canvas_context() -> (CanvasRenderingContext2d, HtmlCanvasElement) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("fractal_canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let canvas = context.canvas().unwrap();
    (context, canvas)
}

pub fn set_info_text(fractal_response: FractalResponse, id: &str) {
    console_log!("trying to find element id {id}");
    let document = web_sys::window().unwrap().document().unwrap();
    let p = document.get_element_by_id(id).unwrap();
    let p: HtmlParagraphElement = p
        .dyn_into::<HtmlParagraphElement>()
        .map_err(|_| ())
        .unwrap();

    let pixels_per_msec = (fractal_response.fractal.height * fractal_response.fractal.width) as f64
        / fractal_response.duration_ms as f64;
    let txt = format!(
        "Duration: {} ms, Speed: {:.4} Pixels / ms",
        fractal_response.duration_ms, pixels_per_msec
    );
    p.set_inner_text(&txt);
}

const SERVER: &str = "http://localhost:3000";
const API_URL_SINGLE_THREADED: &str = "/api/singlethreaded";
const API_URL_MULTI_THREADED: &str = "/api/multithreaded";
const API_URL_MULTI_THREADED_RATIONAL: &str = "/api/multithreaded/rational";

const API_URL_RAYON: &str = "/api/rayon";

const JAVA_SERVER: &str = "http://localhost:4000";

async fn post_single_threaded() -> Result<(), reqwasm::Error> {
    console_log!("post_single_threaded!");

    let (context, canvas) = get_canvas_context();
    clear_canvas(&canvas);

    let fractal_request = dummy_request();
    let fractal_request = serde_json::json!(fractal_request).to_string();
    let url = format!("{}{}", SERVER, API_URL_SINGLE_THREADED);

    let re = Request::post(&url)
        .body(fractal_request)
        .header("content-type", "application/json")
        .send()
        .await?
        .text()
        .await;

    let response = re.expect("should be a valid Response/Body !!!");
    let fractal_response: serde_json::error::Result<FractalResponse> =
        serde_json::from_str(&response);

    let fractal_response = fractal_response.unwrap();
    draw_to_canvas(&fractal_response, &context, &canvas);
    console_log!("updated data");

    set_info_text(fractal_response, "rust-single-threaded");
    Ok(())
}

async fn post_multi_threaded() -> Result<(), reqwasm::Error> {
    console_log!("post_multi_threaded!");
    let (context, canvas) = get_canvas_context();
    clear_canvas(&canvas);

    let fractal_request = dummy_request();
    let fractal_request = serde_json::json!(fractal_request).to_string();
    console_log!("post_multi_threaded!    {}", fractal_request);

    let url = format!("{}{}", SERVER, API_URL_MULTI_THREADED);

    let re = Request::post(&url)
        .body(fractal_request)
        .header("content-type", "application/json")
        .send()
        .await?
        .text()
        .await;

    let response = re.expect("should be a valid Response/Body !!!");
    let fractal_response: serde_json::error::Result<FractalResponse> =
        serde_json::from_str(&response);

    let fractal_response = fractal_response.unwrap();
    draw_to_canvas(&fractal_response, &context, &canvas);
    console_log!("updated data");

    set_info_text(fractal_response, "rust-multi-threaded");
    Ok(())
}

async fn post_multi_threaded_rational() -> Result<(), reqwasm::Error> {
    console_log!("post_multi_threaded!");
    let (context, canvas) = get_canvas_context();
    clear_canvas(&canvas);

    let fractal_request = dummy_request_rational();
    let fractal_request = serde_json::json!(fractal_request).to_string();
    console_log!("post_multi_threaded!    {}", fractal_request);

    let url = format!("{}{}", SERVER, API_URL_MULTI_THREADED_RATIONAL);

    let re = Request::post(&url)
        .body(fractal_request)
        .header("content-type", "application/json")
        .send()
        .await?
        .text()
        .await;

    let response = re.expect("should be a valid Response/Body !!!");
    let fractal_response: serde_json::error::Result<FractalResponse> =
        serde_json::from_str(&response);

    let fractal_response = fractal_response.unwrap();
    draw_to_canvas(&fractal_response, &context, &canvas);
    console_log!("updated data");

    set_info_text(fractal_response, "rust-multi-threaded");
    Ok(())
}

async fn post_rayon() -> Result<(), reqwasm::Error> {
    console_log!("post_rayon!");

    let (context, canvas) = get_canvas_context();
    clear_canvas(&canvas);

    let fractal_request = dummy_request();
    let fractal_request = serde_json::json!(fractal_request).to_string();
    let url = format!("{}{}", SERVER, API_URL_RAYON);

    let re = Request::post(&url)
        .body(fractal_request)
        .header("content-type", "application/json")
        .send()
        .await?
        .text()
        .await;

    let response = re.expect("should be a valid Response/Body !!!");
    let fractal_response: serde_json::error::Result<FractalResponse> =
        serde_json::from_str(&response);

    let fractal_response = fractal_response.unwrap();
    draw_to_canvas(&fractal_response, &context, &canvas);
    console_log!("updated data");
    set_info_text(fractal_response, "rust-rayon");
    Ok(())
}

async fn post_single_java() -> Result<(), reqwasm::Error> {
    console_log!("post_single_java!");

    let (context, canvas) = get_canvas_context();
    clear_canvas(&canvas);

    let fractal_request = dummy_request();
    let fractal_request = serde_json::json!(fractal_request).to_string();
    let url = format!("{}{}", JAVA_SERVER, API_URL_SINGLE_THREADED);

    let re = Request::post(&url)
        .body(fractal_request)
        .header("content-type", "application/json")
        .send()
        .await?
        .text()
        .await;

    let response = re.expect("should be a valid Response/Body !!!");
    let fractal_response: serde_json::error::Result<FractalResponse> =
        serde_json::from_str(&response);

    let fractal_response = fractal_response.unwrap();
    draw_to_canvas(&fractal_response, &context, &canvas);
    console_log!("image data written to canvas");
    set_info_text(fractal_response, "java-single-threaded");
    Ok(())
}

async fn post_multi_java() -> Result<(), reqwasm::Error> {
    console_log!("post_multi_java!");

    let (context, canvas) = get_canvas_context();
    clear_canvas(&canvas);

    let fractal_request = dummy_request();
    let fractal_request = serde_json::json!(fractal_request).to_string();
    let url = format!("{}{}", JAVA_SERVER, API_URL_MULTI_THREADED);

    let re = Request::post(&url)
        .body(fractal_request)
        .header("content-type", "application/json")
        .send()
        .await?
        .text()
        .await;

    let response = re.expect("should be a valid Response/Body !!!");
    let fractal_response: serde_json::error::Result<FractalResponse> =
        serde_json::from_str(&response);

    let fractal_response = fractal_response.unwrap();
    draw_to_canvas(&fractal_response, &context, &canvas);
    console_log!("image data written to canvas");
    set_info_text(fractal_response, "java-multi-threaded");
    Ok(())
}

async fn post_multi_java_virtual() -> Result<(), reqwasm::Error> {
    console_log!("post_multi_java_virtual!");

    let (context, canvas) = get_canvas_context();
    clear_canvas(&canvas);

    let fractal_request = dummy_request();
    let fractal_request = serde_json::json!(fractal_request).to_string();
    let url = format!("{}{}/{}", JAVA_SERVER, API_URL_MULTI_THREADED, "virtual");

    let re = Request::post(&url)
        .body(fractal_request)
        .header("content-type", "application/json")
        .send()
        .await?
        .text()
        .await;

    let response = re.expect("should be a valid Response/Body !!!");
    let fractal_response: serde_json::error::Result<FractalResponse> =
        serde_json::from_str(&response);

    let fractal_response = fractal_response.unwrap();
    draw_to_canvas(&fractal_response, &context, &canvas);
    console_log!("image data written to canvas");
    set_info_text(fractal_response, "java-multi-threaded-virtual");
    Ok(())
}

async fn post_crossbeam_tiled() {
    console_log!("post_crossbeam_tiled!");

    let (context, canvas) = get_canvas_context();
    clear_canvas(&canvas);

    let ws = WebSocket::new("ws://localhost:3000/api/crossbeamtiles");

    let socket = match ws {
        Ok(websocket) => websocket,
        Err(e) => panic!("cant open websocket    err {:?}", e),
    };

    // let socket_clone = socket.clone();
    let fractal_request = dummy_request();
    let width = fractal_request.width;
    let height = fractal_request.height;
    set_canvas_width_height(width, height, &canvas);

    let mut cnt_tiles = 0;

    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        e.prevent_default();

        if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            // console_log!("message event, received Text: {:?}", txt);
            let t = txt.as_string().unwrap();
            let web_socket_response: serde_json::error::Result<WebSocketResponse> =
                serde_json::from_str(&t);

            if web_socket_response.is_ok() {
                console_log!("got a valid WebSocketResponse  {}", &t);
                let web_socket_response = web_socket_response.unwrap();

                if web_socket_response.tile.is_some() {
                    cnt_tiles += 1;

                    let tile = web_socket_response.tile.unwrap();
                    console_log!(
                        "got a tile with id  {:?}.  cnt_tiles: {}",
                        tile.idx,
                        cnt_tiles
                    );
                    draw_to_canvas_tiles(&tile, &context, width, height);
                }
            } else {
                console_log!(
                    "got a message not  a valid FractalResponse, so this is the text {}",
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

        let req = WebSocketRequest {
            command: WebSocketCommand::RENDERFRACTAL(fractal_request.clone()),
        };

        let req = serde_json::json!(req).to_string();
        console_log!("sending string to server {}", &req);
        match cloned_ws.send_with_str(&req) {
            Ok(_) => console_log!(
                "sending request to server to start sending tiles: message successfully sent"
            ),
            Err(err) => console_log!("error sending message: {:?}", err),
        }
    });
    socket.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();
}

fn dummy_request() -> FractalRequest {
    let (request, _, _) = basic(true);
    request
}

fn dummy_request_rational() -> FractalRequestRational {
    let center = ComplexRationalNumber {
        a: RationalNumber { num: -8, denom: 10 },
        b: RationalNumber { num: 0, denom: 1 },
    };

    let zoom = RationalNumber { num: 1, denom: 1 };

    let max_iterations: u32 = 1_000;

    let width: u32 = 1200;
    let height: u32 = 800;

    let complex_width = RationalNumber { num: 31, denom: 10 };

    let colors = 256;

    FractalRequestRational {
        center,
        width,
        height,
        complex_width,
        max_iterations,
        colors,
        x_tiles: 0,
        y_tiles: 0,
        zoom,
        name: "basis_rational".to_string(),
    }
}

#[component]
async fn MainContent<G: Html>(cx: Scope<'_>) -> View<G> {
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
                            "Funny image"
                        }
                        div(class="btn-toolbar mb-2 mb-md-0"){
                            div(class="btn-group me-2"){
                            }
                        }
                    }
                    div {
                        div(class ="canvas-container"  ) {
                            canvas(id="fractal_canvas", class="fractal-canvas" )
                        }
                    }
                }
            }
        }
    }
}

#[component]
async fn LeftNavItems<G: Html>(cx: Scope<'_>) -> View<G> {
    let start_singlethreaded = move |e: MouseEvent| {
        e.prevent_default();
        console_log!("start_singlethreaded  clicked.   event {:?}", e.target());
        spawn_local_scoped(cx, {
            async move {
                let res = post_single_threaded().await;
                match res {
                    Ok(_) => {
                        console_log!("all good");
                    }
                    Err(e) => {
                        console_log!("error calling server /api/singlethreaded target.  {:?}", e)
                    }
                }
            }
        });
    };

    let start_multithreaded = move |e: MouseEvent| {
        e.prevent_default();
        console_log!("start_multithreaded  clicked.   event {:?}", e.target());
        spawn_local_scoped(cx, {
            async move {
                let res = post_multi_threaded().await;
                match res {
                    Ok(_) => {
                        console_log!("all good");
                    }
                    Err(e) => {
                        console_log!("error calling server /api/multithreaded target.  {:?}", e)
                    }
                }
            }
        });
    };

    let start_rayon = move |e: MouseEvent| {
        e.prevent_default();
        console_log!("start_rayon  clicked.   event {:?}", e.target());
        spawn_local_scoped(cx, {
            async move {
                let res = post_rayon().await;
                match res {
                    Ok(_) => {
                        console_log!("all good");
                    }
                    Err(e) => {
                        console_log!("error calling server /api/rayon target.  {:?}", e)
                    }
                }
            }
        });
    };

    let start_rust_rational = move |e: MouseEvent| {
        e.prevent_default();
        console_log!("start_rust_rational  clicked.   event {:?}", e.target());
        spawn_local_scoped(cx, {
            async move {
                let res = post_multi_threaded_rational().await;
                match res {
                    Ok(_) => {
                        console_log!("all good");
                    }
                    Err(e) => {
                        console_log!(
                            "error calling server /api/multithreaded/rational target.  {:?}",
                            e
                        )
                    }
                }
            }
        });
    };

    let start_crossbeam_tiled = move |e: MouseEvent| {
        console_log!("start_crossbeam_tiled  clicked.  event {:?}", e.target());
        spawn_local_scoped(cx, {
            async move {
                post_crossbeam_tiled().await;
            }
        });
    };

    let start_java_single = move |e: MouseEvent| {
        console_log!("start_java_single  clicked.  event {:?}", e.target());
        spawn_local_scoped(cx, {
            async move {
                let res = post_single_java().await;
                match res {
                    Ok(_) => {
                        console_log!("all good");
                    }
                    Err(e) => {
                        console_log!("error calling server /api/rayon target.  {:?}", e)
                    }
                }
            }
        });
    };

    let start_java_multi = move |e: MouseEvent| {
        console_log!("start_java_multi  clicked.  event {:?}", e.target());
        spawn_local_scoped(cx, {
            async move {
                let res = post_multi_java().await;
                match res {
                    Ok(_) => {
                        console_log!("all good");
                    }
                    Err(e) => {
                        console_log!("error calling server /api/rayon target.  {:?}", e)
                    }
                }
            }
        });
    };

    let start_java_multi_virtual = move |e: MouseEvent| {
        console_log!("start_java_multi_virtual  clicked.  event {:?}", e.target());
        spawn_local_scoped(cx, {
            async move {
                let res = post_multi_java_virtual().await;
                match res {
                    Ok(_) => {
                        console_log!("all good");
                    }
                    Err(e) => {
                        console_log!("error calling server /api/rayon target.  {:?}", e)
                    }
                }
            }
        });
    };

    view! { cx,

        div(class = "row", style ="margin-bottom: 10px;") {
            div (class="col-12") {
                button(class="btn btn-primary", type="button", id="singlethreaded" ,on:click=start_singlethreaded) {
                    "Single threaded"
                }
                br {
                }
                p(id = "rust-single-threaded") {
                    "Duration:"
                }
            }
        }

         div(class = "row", style ="margin-bottom: 10px;") {
            div (class="col-12") {
                button(class="btn btn-primary", type="button", id="multithreaded" ,on:click=start_multithreaded) {
                        "Multi threaded"
                }
                br {
                }
                p(id = "rust-multi-threaded") {
                    "Duration:"
                }
            }
        }

         div(class = "row", style ="margin-bottom: 10px;") {
            div (class="col-12") {
                 button(class="btn btn-primary", type="button", id="crossbeam", on:click=start_crossbeam_tiled) {
                        "Crossbeam tiled"
                 }
                 br {
                 }
                 p(id = "crossbeam-tiled") {
                    "Duration:"
                 }
            }
        }

        div(class = "row", style ="margin-bottom: 10px;") {
            div (class="col-12") {
                button(class="btn btn-primary", type="button", id="rayon" ,on:click=start_rayon){
                    "Rayon"
                }
                br {
                }
                p(id = "rust-rayon") {
                    "Duration:"
                }
            }
        }

        div(class = "row", style ="margin-bottom: 10px;") {
            div (class="col-12") {
                button(class="btn btn-primary", type="button", id="rust-rational-btn" ,on:click=start_rust_rational){
                    "Rust rational numbers"
                }
                br {
                }
                p(id = "rust-rational") {
                    "Duration:"
                }
            }
        }


          div(class = "row", style ="margin-bottom: 10px;") {
            div (class="col-12") {
                button(class="btn btn-primary", type="button", id="java_singlethreaded" ,on:click=start_java_single){
                            "Java single threaded"
                }
                br {
                }
                p(id = "java-single-threaded") {
                    "Duration:"
                }
            }
        }

        div(class = "row", style ="margin-bottom: 10px;") {
            div (class="col-12") {
                 button(class="btn btn-primary", type="button", id="java_multithreaded" ,on:click=start_java_multi){
                            "Java multi threaded"
                }
                br {
                }
                p(id = "java-multi-threaded") {
                    "Duration:"
                }
            }
        }

        div(class = "row", style ="margin-bottom: 10px;") {
            div (class="col-12") {
                 button(class="btn btn-primary", type="button", id="java_multithreaded_virtual" ,on:click=start_java_multi_virtual){
                            "Java multi threaded virtual"
                }
                br {
                }
                p(id = "java-multi-threaded-virtual") {
                    "Duration:"
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
                    "FractalThingi"
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

pub struct AppState {
    _name: RcSignal<String>,
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| view! { cx, App {} });
}
