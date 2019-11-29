use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::error::Error;

pub struct ListErrors {
    pub props: Props,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub error: Option<Error>,
}

pub enum Msg {}

impl Component for ListErrors {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ListErrors { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html<Self> {
        if let Some(error) = &self.props.error {
            html! {
                <ul class="error-messages">
                    {
                        match error {
                            Error::UnprocessableEntity(error_info) => {
                                html! {
                                    <>
                                    {for error_info.errors.iter().map(|(key, value)| {
                                        html! {
                                            <li>
                                            { key }
                                            {for value.iter().map(|e| {
                                                html! {
                                                    <>{" "} {e}</>
                                                }
                                            })}
                                            </li>
                                        }
                                    })}
                                    </>
                                }
                            }
                            _ => {
                                html! {
                                    <li>{error}</li>
                                }
                            }

                        }
                    }
                </ul>
            }
        } else {
            html! {}
        }
    }
}
