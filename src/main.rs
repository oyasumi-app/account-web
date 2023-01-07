use yew::prelude::*;
use yew_router::prelude::*;

mod routes;

#[macro_use]
mod api;

pub use routes::Route;

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={routes::switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting app");
    yew::Renderer::<Main>::new().render();
}
