use regex::Regex;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement, Window};
use yew::prelude::*;

use crate::api::auth::login_user;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct LoginUserSchema {
    email: String,
    password: String,
}

#[function_component(LoginFormSeven)]
pub fn login_form_seven() -> Html {
    let error_handle = use_state(String::default);
    let error = (*error_handle).clone();

    let email_valid_handle = use_state(|| true);
    let email_valid = (*email_valid_handle).clone();

    let password_valid_handle = use_state(|| true);
    let password_valid = (*password_valid_handle).clone();

    let input_email_ref = use_node_ref();
    let input_email_handle = use_state(String::default);
    let input_email = (*input_email_handle).clone();

    let input_password_ref = use_node_ref();
    let input_password_handle = use_state(String::default);
    let input_password = (*input_password_handle).clone();

    let validate_email = |email: &str| {
        let pattern = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,3}$").unwrap();
        pattern.is_match(email)
    };

    let validate_password = |password: &str| !password.is_empty();

    let on_email_change = {
        let input_email_ref = input_email_ref.clone();

        Callback::from(move |_| {
            let input = input_email_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_email_handle.set(value);
                email_valid_handle.set(validate_email(&input.value()));
            }
        })
    };

    let on_password_change = {
        let input_password_ref = input_password_ref.clone();

        Callback::from(move |_| {
            let input = input_password_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_password_handle.set(value);
                password_valid_handle.set(validate_password(&input.value()));
            }
        })
    };

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        let email_ref = input_password.clone();
        let password_ref = input_password.clone();
        let error_handle = error_handle.clone();
        console::log_1(&format!("Email: {}, Password: {}", input_email, input_password).into());

        spawn_local(async move {
            let email_val = email_ref.clone();
            let password_val = password_ref.clone();
            let error_handle = error_handle.clone();
            if email_valid && password_valid {
                let response = login_user(email_val, password_val).await;
                match response {
                    Ok(_) => {
                        console::log_1(&"success".into());
                        let window: Window = web_sys::window().expect("window not available");
                        let location = window.location();
                        let _ = location.set_href("/error");
                    }
                    Err(err) => {
                        error_handle.set(err);
                    }
                }
            } else {
                error_handle.set("Please provide a valid email and password!".into());
            }
        });
    });

    html! {

        <div class="limiter">
          <div
            class="container-login-seven"
          >
            <div class="wrap-login p-l-110 p-r-110 p-t-62 p-b-33">
              <form class="login-form validate-form flex-sb flex-w" onsubmit={onsubmit}>
                if !error.is_empty() {
                  <div class="error">{error}</div>
                }
                <span class="login-form-title p-b-53">{"Login"}</span>
                <div class="p-t-31 p-b-9">
                  <label for="username" class="txt1">{"Username"}</label>
                </div>
                <div
                  class="d-flex wrap-input validate-input"
                  data-validate="Username is required"
                >
                  <i class="bi bi-person"></i>
                  <input
                    id="username"
                    class="input"
                    type="text"
                    name="username"
                    required={true}
                    placeholder="Email"
                    aria-required="true"
                    ref={input_email_ref}
                    oninput={on_email_change}
                  />
                  <span class="focus-input" aria-hidden="true"></span>
                </div>
                if !email_valid {
                    <div class="error-txt">{"Enter a valid email address"}</div>
                }
                <div class="p-t-13 p-b-9">
                  <label for="password" class="txt1">{"Password"}</label>
                </div>
                <div
                  class="d-flex wrap-input validate-input"
                  data-validate="Password is required"
                >
                  <i class="bi bi-lock"></i>
                  <input
                    id="password"
                    class="input"
                    type="password"
                    name="password"
                    required={true}
                    aria-required="true"
                    placeholder="Password"
                    ref={input_password_ref}
                    oninput={on_password_change}
                  />
                  <span class="focus-input" aria-hidden="true"></span>
                </div>
                if !password_valid {
                   <div class="error-txt">{"Password can't be blank"}</div>
                }
                <div class="container-login-form-btn m-t-17">
                  <button class="login-form-btn" type="submit">
                    <i class="bi bi-box-arrow-in-right me-2"></i>
                    {"Sign In"}
                  </button>
                </div>
                <div class="w-full text-center p-t-55">
                  <span class="txt2">{"Not a member?"}</span>
                  <a href="#" class="txt2 bo1">{"Sign up now"}</a>
                </div>
                <div class="w-full text-center p-t-20">
                  <span class="txt2">{"Or Sign In With"}</span>
                  <div>
                    <button
                      class="btn-social btn-face m-b-20"
                      aria-label="Sign in with Facebook"
                    >
                      <i class="bi bi-facebook"></i>
                    </button>
                    <button
                      class="btn-social btn-google m-b-20"
                      aria-label="Sign in with Google"
                    >
                      <i class="bi bi-google"></i>
                    </button>
                    <button
                      class="btn-social btn-twitter m-b-20"
                      aria-label="Sign in with Twitter"
                    >
                      <i class="bi bi-twitter"></i>
                    </button>
                    <button
                      class="btn-social btn-linkedin m-b-20"
                      aria-label="Sign in with LinkedIn"
                    >
                      <i class="bi bi-linkedin"></i>
                    </button>
                  </div>
                </div>
              </form>
            </div>
          </div>
        </div>
    }
}
