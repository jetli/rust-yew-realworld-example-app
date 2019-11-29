use stdweb::web::event::IEvent;
use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, Html, Properties,
    ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::agent::{set_token, Auth};
use crate::components::list_errors::ListErrors;
use crate::error::Error;
use crate::routes::AppRoute;
use crate::types::{RegisterInfo, RegisterInfoWrapper, UserInfo, UserInfoWrapper};

pub struct Register {
    auth: Auth,
    error: Option<Error>,
    props: Props,
    request: RegisterInfo,
    response: Callback<Result<UserInfoWrapper, Error>>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    task: Option<FetchTask>,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub callback: Callback<UserInfo>,
}

pub enum Msg {
    Request,
    Response(Result<UserInfoWrapper, Error>),
    NoOp,
    UpdateEmail(String),
    UpdatePassword(String),
    UpdateUsername(String),
}

impl Component for Register {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.send_back(|_| Msg::NoOp));
        Register {
            auth: Auth::new(),
            error: None,
            request: RegisterInfo::default(),
            response: link.send_back(Msg::Response),
            task: None,
            props,
            router_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Request => {
                let request = RegisterInfoWrapper {
                    user: RegisterInfo {
                        email: self.request.email.clone(),
                        password: self.request.password.clone(),
                        username: self.request.username.clone(),
                    },
                };
                let task = self.auth.register(request, self.response.clone());
                self.task = Some(task);
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
            Msg::NoOp => {}
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="auth-page">
                <div class="container page">
                    <div class="row">
                        <div class="col-md-6 offset-md-3 col-xs-12">
                            <h1 class="text-xs-center">{ "Sign Up" }</h1>
                            <p class="text-xs-center">
                                <RouterLink text="Have an account?" link="#/login"/>
                            </p>
                            <ListErrors error=self.error.clone() />
                            <form onsubmit=|ev| { ev.prevent_default(); Msg::Request }>
                                <fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="text"
                                            placeholder="Username"
                                            value=&self.request.username
                                            oninput=|ev| Msg::UpdateUsername(ev.value)
                                            />
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="email"
                                            placeholder="Email"
                                            value=&self.request.email
                                            oninput=|ev| Msg::UpdateEmail(ev.value)
                                            />
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="password"
                                            placeholder="Password"
                                            value=&self.request.password
                                            oninput=|ev| Msg::UpdatePassword(ev.value)
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
