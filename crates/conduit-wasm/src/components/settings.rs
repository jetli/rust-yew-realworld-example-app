use stdweb::web::event::IEvent;
use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, Html, Properties,
    ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::agent::{is_authenticated, set_token, Auth};
use crate::components::list_errors::ListErrors;
use crate::error::Error;
use crate::routes::AppRoute;
use crate::types::{UserInfoWrapper, UserUpdateInfo, UserUpdateInfoWrapper};

/// Update settings of the author or logout
pub struct Settings {
    auth: Auth,
    error: Option<Error>,
    request: UserUpdateInfo,
    password: String,
    response: Callback<Result<UserInfoWrapper, Error>>,
    loaded: Callback<Result<UserInfoWrapper, Error>>,
    task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub callback: Callback<()>,
}

pub enum Msg {
    Request,
    Response(Result<UserInfoWrapper, Error>),
    Loaded(Result<UserInfoWrapper, Error>),
    Ignore,
    Logout,
    UpdateEmail(String),
    UpdateUsername(String),
    UpdatePassword(String),
    UpdateImage(String),
    UpdateBio(String),
}

impl Component for Settings {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        Settings {
            auth: Auth::new(),
            error: None,
            request: UserUpdateInfo::default(),
            password: String::default(),
            response: link.send_back(Msg::Response),
            loaded: link.send_back(Msg::Loaded),
            task: None,
            router_agent: RouteAgent::bridge(link.send_back(|_| Msg::Ignore)),
            props,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        if is_authenticated() {
            self.task = Some(self.auth.current(self.loaded.clone()));
        }
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Request => {
                let mut request = UserUpdateInfoWrapper {
                    user: self.request.clone(),
                };
                if !self.password.is_empty() {
                    request.user.password = Some(self.password.clone());
                }
                self.task = Some(self.auth.save(request, self.response.clone()));
            }
            Msg::Response(Ok(_)) => {
                self.error = None;
                self.task = None;
                self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
            }
            Msg::Response(Err(err)) => {
                self.error = Some(err);
                self.task = None;
            }
            Msg::Loaded(Ok(user_info)) => {
                self.error = None;
                self.task = None;
                self.request = UserUpdateInfo {
                    email: user_info.user.email,
                    username: user_info.user.username,
                    password: None,
                    image: user_info.user.image.unwrap_or(String::default()),
                    bio: user_info.user.bio.unwrap_or(String::default()),
                };
            }
            Msg::Loaded(Err(err)) => {
                self.error = Some(err);
                self.task = None;
            }
            Msg::Ignore => {}
            Msg::Logout => {
                // Clear global token after logged out
                set_token(None);
                // Notify app to clear current user info
                self.props.callback.emit(());
                // Redirect to home page
                self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
            }
            Msg::UpdateBio(bio) => {
                self.request.bio = bio;
            }
            Msg::UpdateEmail(email) => {
                self.request.email = email;
            }
            Msg::UpdateImage(image) => {
                self.request.image = image;
            }
            Msg::UpdatePassword(password) => {
                self.password = password;
            }
            Msg::UpdateUsername(username) => {
                self.request.username = username;
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="settings-page">
                <div class="container page">
                    <div class="row">
                        <div class="col-md-6 offset-md-3 col-xs-12">
                            <h1 class="text-xs-center">{ "Your Settings" }</h1>
                            <ListErrors error=&self.error/>
                            <form onsubmit=|ev| { ev.prevent_default(); Msg::Request }>
                                <fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control"
                                            type="text"
                                            placeholder="URL of profile picture"
                                            value={&self.request.image}
                                            oninput=|ev| Msg::UpdateImage(ev.value) />
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="text"
                                            placeholder="Username"
                                            value={&self.request.username}
                                            oninput=|ev| Msg::UpdateUsername(ev.value) />
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <textarea
                                            class="form-control form-control-lg"
                                            rows="8"
                                            placeholder="Short bio about you"
                                            value={&self.request.bio }
                                            oninput=|ev| Msg::UpdateBio(ev.value) >
                                        </textarea>
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="email"
                                            placeholder="Email"
                                            value={&self.request.email}
                                            oninput=|ev| Msg::UpdateEmail(ev.value) />
                                    </fieldset>
                                    <fieldset class="form-group">
                                        <input
                                            class="form-control form-control-lg"
                                            type="password"
                                            placeholder="New Password"
                                            value={&self.password}
                                            oninput=|ev| Msg::UpdatePassword(ev.value) />
                                    </fieldset>
                                    <button
                                        class="btn btn-lg btn-primary pull-xs-right"
                                        type="submit"
                                        disabled=false>
                                        { "Update Settings" }
                                    </button>
                                </fieldset>
                            </form>
                            <hr />
                            <button
                                class="btn btn-outline-danger"
                                onclick=|_| Msg::Logout >
                                { "Or click here to logout."}
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
