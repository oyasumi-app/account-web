use yew::prelude::*;
use yew_router::prelude::*;

extern crate console_error_panic_hook;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;


mod components;
mod routes;

#[macro_use]
mod api;

pub use routes::Route;

#[function_component(Main)]
fn app() -> Html {
    assert!(false);
    html! {
        <BrowserRouter>
            <Switch<Route> render={routes::switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn panic_hook(info: &std::panic::PanicInfo) {
    // Defer to the default panic hook
    console_error_panic_hook::hook(info);

    // Produce a string in the same way as the default panic hook
    let panic_info = {

        #[wasm_bindgen]
        extern {
            #[wasm_bindgen(js_namespace = console)]
            fn error(msg: String);

            type Error;

            #[wasm_bindgen(constructor)]
            fn new() -> Error;

            #[wasm_bindgen(structural, method, getter)]
            fn stack(error: &Error) -> String;
        }

        let mut msg = info.to_string();

        // Add the error stack to our message.
        //
        // This ensures that even if the `console` implementation doesn't
        // include stacks for `console.error`, the stack is still available
        // for the user. Additionally, Firefox's console tries to clean up
        // stack traces, and ruins Rust symbols in the process
        // (https://bugzilla.mozilla.org/show_bug.cgi?id=1519569) but since
        // it only touches the logged message's associated stack, and not
        // the message's contents, by including the stack in the message
        // contents we make sure it is available to the user.
        msg.push_str("\n\nStack:\n\n");
        let e = Error::new();
        let stack = e.stack();
        msg.push_str(&stack);

        msg
    };

    let mut panic_info_str = String::new();
    html_escape::encode_text_to_string(&panic_info, &mut panic_info_str);


    // Render the big_error component
 
    let props = components::style::full_page::BigErrorProps {
        short_name: "Unrecoverable Error".into(),
        text: Some("There was an error that was not handled by the code, causing a \"panic\".".into()),
        what_to_do: Some("This is almost certainly a bug with the application. Consider sending the diagnostic information below to the developer.".into()),
        diagnostics: Some(panic_info_str.into()),
    };

    let big_error_html = &components::style::full_page::big_error_when_panic("", &props);

    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    body.set_inner_html(big_error_html);


}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting app");
    yew::set_custom_panic_hook(Box::new(panic_hook));
    yew::Renderer::<Main>::new().render();
}
