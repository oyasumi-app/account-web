use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CenteredBoxProps {
    pub children: Children,
}

/// Creates a centered box layout.
/// This is useful for pages like login and register.
#[function_component(CenteredBox)]
pub fn centered_box(props: &CenteredBoxProps) -> Html {
    html! {
        <div class="container container-centered">
            <div class="modal-dialog" style="width: inherit;" role="document">
                <div class="modal-content rounded-4 shadow">
                    <div class="modal-body"
                    style="padding: 1rem;"> // BUGFIX for Bootstrap 5.2.0 not providing --bs-modal-padding
                        { for props.children.iter() }
                    </div>
                </div>
            </div>
        </div>
    }
}
