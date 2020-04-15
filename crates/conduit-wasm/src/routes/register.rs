use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, Event, Html, InputData,
    Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::components::list_errors::ListErrors;
use crate::error::Error;
use crate::routes::AppRoute;
use crate::services::{set_token, Auth};
use crate::types::{RegisterInfo, RegisterInfoWrapper, UserInfo, UserInfoWrapper};

/// Register page
pub struct Register {
    auth: Auth,
    error: Option<Error>,
    props: Props,
    request: RegisterInfo,
    response: Callback<Result<UserInfoWrapper, Error>>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    task: Option<FetchTask>,
    link: ComponentLink<Self>,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    /// Callback when user is registered in successfully
    pub callback: Callback<UserInfo>,
}

pub enum Msg {
    Request,
    Response(Result<UserInfoWrapper, Error>),
    Ignore,
    UpdateEmail(String),
    UpdatePassword(String),
    UpdateUsername(String),
}

impl Component for Register {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Register {
            auth: Auth::new(),
            error: None,
            request: RegisterInfo::default(),
            response: link.callback(Msg::Response),
            task: None,
            props,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Request => {
                let request = RegisterInfoWrapper {
                    user: self.request.clone(),
                };
                self.task = Some(self.auth.register(request, self.response.clone()));
            }
            Msg::Response(Ok(user_info)) => {
                set_token(Some(user_info.user.token.clone()));
                self.props.callback.emit(user_info.user);
                self.error = None;
                self.task = None;
                self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
            }
            Msg::Response(Err(err)) => {
                self.error = Some(err);
                self.task = None;
            }
            Msg::UpdateEmail(email) => {
                self.request.email = email;
            }
            Msg::UpdatePassword(password) => {
                self.request.password = password;
            }
            Msg::UpdateUsername(username) => {
                self.request.username = username;
            }
            Msg::Ignore => {}
        }
        true
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(|ev: Event| {
            ev.prevent_default();
            Msg::Request
        });
        let oninput_username = self
            .link
            .callback(|ev: InputData| Msg::UpdateUsername(ev.value));
        let oninput_email = self
            .link
            .callback(|ev: InputData| Msg::UpdateEmail(ev.value));
        let oninput_password = self
            .link
            .callback(|ev: InputData| Msg::UpdatePassword(ev.value));

        html! {
            <div class="auth-page">
                <div class="container page">
                    <div class="row">
                        <div class="col-md-6 offset-md-3 col-xs-12">
                            <h1 class="text-xs-center">{ "Sign Up" }</h1>
                            <p class="text-xs-center">
                                <RouterAnchor<AppRoute> route=AppRoute::Login>
                                    { "Have an account?" }
                                </RouterAnchor<AppRoute>>
                            </p>
                            <ListErrors error=&self.error />
                            <form onsubmit=onsubmit>
                                <fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="text"
                                            placeholder="Username"
                                            value=&self.request.username
                                            oninput=oninput_username
                                            />
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="email"
                                            placeholder="Email"
                                            value=&self.request.email
                                            oninput=oninput_email
                                            />
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="password"
                                            placeholder="Password"
                                            value=&self.request.password
                                            oninput=oninput_password
                                            />
                                    </fieldset>
                                    <button
                                        class="btn btn-lg btn-primary pull-xs-right"
                                        type="submit"
                                        disabled=false>
                                        { "Sign up" }
                                    </button>
                                </fieldset>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
