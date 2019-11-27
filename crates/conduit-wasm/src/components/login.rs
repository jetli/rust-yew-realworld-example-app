use stdweb::web::event::IEvent;
use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, Html, Properties,
    ShouldRender,
};
use yew_router::{
    agent::{RouteAgent, RouteRequest::ChangeRoute},
    prelude::*,
};

use crate::agent::{set_token, Auth};
use crate::error::Error;
use crate::routes::AppRoute;
use crate::types::{LoginInfo, LoginInfoWrapper, UserInfo, UserInfoWrapper};

pub struct Login {
    auth: Auth,
    error: Option<Error>,
    login_request: LoginInfo,
    login_response: Callback<Result<UserInfoWrapper, Error>>,
    login_task: Option<FetchTask>,
    props: Props,
    router_agent: Box<dyn Bridge<RouteAgent<()>>>,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub callback: Callback<UserInfo>,
}

pub enum Msg {
    LoginRequest,
    LoginResponse(Result<UserInfoWrapper, Error>),
    NoOp,
    UpdateEmail(String),
    UpdatePassword(String),
}

impl Component for Login {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.send_back(|_| Msg::NoOp));
        Login {
            auth: Auth::new(),
            error: None,
            login_request: LoginInfo::default(),
            login_response: link.send_back(Msg::LoginResponse),
            login_task: None,
            props,
            router_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoginRequest => {
                let login_request = LoginInfoWrapper {
                    user: LoginInfo {
                        email: self.login_request.email.clone(),
                        password: self.login_request.password.clone(),
                    },
                };
                let task = self.auth.login(login_request, self.login_response.clone());
                self.login_task = Some(task);
            }
            Msg::LoginResponse(Ok(user_info)) => {
                set_token(Some(user_info.user.token.clone()));
                self.props.callback.emit(user_info.user);
                self.error = None;
                self.login_task = None;
                self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
            }
            Msg::LoginResponse(Err(err)) => {
                self.error = Some(err);
                self.login_task = None;
            }
            Msg::UpdateEmail(email) => {
                self.login_request.email = email;
            }
            Msg::UpdatePassword(password) => {
                self.login_request.password = password;
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
                            <h1 class="text-xs-center">{ "Sign In" }</h1>
                            <p class="text-xs-center">
                                <RouterLink text="Need an account?" link="/#/register"/>
                            </p>
                            { self.list_errors(&self.error) }
                            <form onsubmit=|ev| { ev.prevent_default(); Msg::LoginRequest }>
                                <fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="email"
                                            placeholder="Email"
                                            value=&self.login_request.email
                                            oninput=|ev| Msg::UpdateEmail(ev.value)
                                            />
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="password"
                                            placeholder="Password"
                                            value=&self.login_request.password
                                            oninput=|ev| Msg::UpdatePassword(ev.value)
                                            />
                                    </fieldset>
                                    <button
                                        class="btn btn-lg btn-primary pull-xs-right"
                                        type="submit"
                                        disabled=false>
                                        { "Sign in" }
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

impl Login {
    fn list_errors(&self, error: &Option<Error>) -> Html<Self> {
        if let Some(error) = error {
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
            html! {
                <>
                </>
            }
        }
    }
}
