use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::error::Error;
use crate::pages::login_page_one::LoginPageOne;
use crate::pages::login_page_two::LoginPageTwo;
use crate::pages::login_page_three::LoginPageThree;
use crate::pages::login_page_four::LoginPageFour;

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
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::LoginPageOne => html! { <LoginPageOne /> },
        Route::LoginPageTwo => html! { <LoginPageTwo /> },
        Route::LoginPageThree => html! { <LoginPageThree /> },
        Route::LoginPageFour => html! { <LoginPageFour /> },
        Route::Error => html! { <Error /> },
    }
}