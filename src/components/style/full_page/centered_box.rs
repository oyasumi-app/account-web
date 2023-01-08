use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CenteredBoxProps {
    #[prop_or_default]
    pub title: AttrValue,
    pub children: Children,
}

/// Creates a centered box layout.
/// This is useful for pages like login and register.
#[function_component(CenteredBox)]
pub fn centered_box(props: &CenteredBoxProps) -> Html {
    html! {
        <div class="container container-centered">
            <div class="modal-dialog modal-dialog-centered rounded-4 shadow" role="document">
                <div class="modal-content">
                    <div class="modal-header p-5 pb-4 border-bottom-0">
                        <h1 class="fw-bold mb-0 fs-2">  // BUGFIX for Bootstrap 5.2.0: should be "modal-title fs-5"
                            {props.title.clone()}
                        </h1>
                    </div>
                    <div class="modal-body p-5 pt-0"
                    style="padding: 1rem;"> // BUGFIX for Bootstrap 5.2.0 not providing --bs-modal-padding
                        { for props.children.iter() }
                    </div>
                </div>
            </div>
        </div>
    }
}
