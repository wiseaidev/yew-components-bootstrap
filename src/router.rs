use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::error::Error;
use crate::pages::login_page_one::LoginPageOne;
use crate::pages::login_page_two::LoginPageTwo;
use crate::pages::login_page_three::LoginPageThree;
use crate::pages::login_page_four::LoginPageFour;
use crate::pages::login_page_five::LoginPageFive;
use crate::pages::login_page_six::LoginPageSix;
use crate::pages::login_page_seven::LoginPageSeven;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/error")]
    Error,
    #[at("/bootstrap-css/1")]
    LoginPageOne,
    #[at("/bootstrap-css/2")]
    LoginPageTwo,
    #[at("/bootstrap-css/3")]
    LoginPageThree,
    #[at("/bootstrap-css/4")]
    LoginPageFour,
    #[at("/bootstrap-css/5")]
    LoginPageFive,
    #[at("/bootstrap-css/6")]
    LoginPageSix,
    #[at("/bootstrap-css/7")]
    LoginPageSeven,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::LoginPageOne => html! { <LoginPageOne /> },
        Route::LoginPageTwo => html! { <LoginPageTwo /> },
        Route::LoginPageThree => html! { <LoginPageThree /> },
        Route::LoginPageFour => html! { <LoginPageFour /> },
        Route::LoginPageFive => html! { <LoginPageFive /> },
        Route::LoginPageSix => html! { <LoginPageSix /> },
        Route::LoginPageSeven => html! { <LoginPageSeven /> },
        Route::Error => html! { <Error /> },
    }
}