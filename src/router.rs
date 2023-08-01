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
use crate::pages::login_page_eight::LoginPageEight;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/error")]
    Error,
    #[at("/1")]
    LoginPageOne,
    #[at("/2")]
    LoginPageTwo,
    #[at("/3")]
    LoginPageThree,
    #[at("/4")]
    LoginPageFour,
    #[at("/5")]
    LoginPageFive,
    #[at("/6")]
    LoginPageSix,
    #[at("/7")]
    LoginPageSeven,
    #[at("/")]
    LoginPageEight,
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
        Route::LoginPageEight => html! { <LoginPageEight /> },
        Route::Error => html! { <Error /> },
    }
}