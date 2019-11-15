use stdweb::web::event::IEvent;
use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

use crate::agent::Auth;
use crate::error::Error;
use crate::types::{LoginInfo, LoginInfoWrapper, UserInfoWrapper};

pub struct Login {
    auth: Auth,
    login_info: LoginInfo,
    user_info: Option<UserInfoWrapper>,
    login_callback: Callback<Result<UserInfoWrapper, Error>>,
    login_task: Option<FetchTask>,
    error: Option<Error>,
}

pub enum Msg {
    Login,
    LoginReady(Result<UserInfoWrapper, Error>),
    UpdateEmail(String),
    UpdatePassword(String),
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        Login {
            auth: Auth::new(),
            login_info: LoginInfo::default(),
            user_info: None,
            login_callback: link.send_back(Msg::LoginReady),
            login_task: None,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Login => {
                let login_info = LoginInfoWrapper {
                    user: LoginInfo {
                        email: self.login_info.email.clone(),
                        password: self.login_info.password.clone(),
                    },
                };
                let task = self.auth.login(login_info, self.login_callback.clone());
                self.login_task = Some(task);
            }
            Msg::LoginReady(Ok(user_info)) => {
                self.user_info = Some(user_info);
                self.error = None;
            }
            Msg::LoginReady(Err(err)) => {
                self.user_info = None;
                self.error = Some(err);
            }
            Msg::UpdateEmail(email) => {
                self.login_info.email = email;
            }
            Msg::UpdatePassword(password) => {
                self.login_info.password = password;
            }
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
                                <RouterLink text="Need an account?" link="/#register"/>
                            </p>
                            { self.list_errors(&self.error) }
                            <form onsubmit=|ev| { ev.prevent_default(); Msg::Login }>
                                <fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="email"
                                            placeholder="Email"
                                            value=&self.login_info.email
                                            oninput=|ev| Msg::UpdateEmail(ev.value)
                                            />
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="password"
                                            placeholder="Password"
                                            value=&self.login_info.password
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
