use yew::prelude::*;

use crate::components::DashboardNavbar;

#[derive(Properties, PartialEq)]
pub struct DashboardLayoutProps {
    pub children: Children,
}

/// Creates a layout for generic dashboard pages.
/// Features a [`DashboardNavbar`] at the top.
#[function_component(DashboardLayout)]
pub fn dashboard_layout(props: &DashboardLayoutProps) -> Html {
    html! {
        <>
            <DashboardNavbar />
            <div class="container">
                { for props.children.iter() }
            </div>
        </>
    }
}
