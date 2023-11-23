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

#[function_component(LoginFormTwo)]
pub fn login_form_two() -> Html {
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
        <>
          <button
            class="dark-mode-toggle"
            aria-label="Toggle Dark Mode"
          >
            <i class="bi bi-sun-fill"></i>
          </button>
          <div class="login-container-two">
            <h2>{"Welcome Back!"}</h2>
            <div class="image-placeholder"></div>
            <form onsubmit={onsubmit}>
              <div class="mb-3">
                if !error.is_empty() {
                  <div class="error">{error}</div>
                }
                <label for="username" class="form-label">{"Username"}</label>
                <div class="input-group">
                  <div class="input-group-prepend">
                    <span class="input-group-text"><i class="bi bi-person"></i></span>
                  </div>
                  <input
                    type="text"
                    class="form-control"
                    id="username"
                    placeholder="Email"
                    required={true}
                    name="username"
                    ref={input_email_ref}
                    oninput={on_email_change}
                    aria-required="true"
                  />
                </div>
              </div>
              if !email_valid {
                  <div class="error-txt">{"Enter a valid email address"}</div>
              }
              <div class="mb-3">
                <label for="password" class="form-label">{"Password"}</label>
                <div class="input-group">
                  <div class="input-group-prepend">
                    <span class="input-group-text"><i class="bi bi-lock"></i></span>
                  </div>
                  <input
                    type="password"
                    class="form-control"
                    id="password"
                    name="password"
                    required={true}
                    placeholder="Password"
                    aria-required="true"
                    ref={input_password_ref}
                    oninput={on_password_change}
                  />
                </div>
              </div>
              if !password_valid {
                 <div class="error-txt">{"Password can't be blank"}</div>
              }
              <div class="remember-me">
                <input
                  type="checkbox"
                  id="rememberMe"
                  name="rememberMe"
                  aria-label="Remember Me"
                />
                <label for="rememberMe">{"Remember Me"}</label>
              </div>
              <button type="submit" class="btn btn-primary" aria-label="Login Button">
                <i class="bi bi-box-arrow-in-left"></i>{"Login"}
              </button>
              <div class="forgot-password">
                <a href="#" aria-label="Forgot Password Link">{"Forgot Password?"}</a>
              </div>
              <div class="form-divider" aria-hidden="true">{"Or login with"}</div>
              <div class="social-buttons">
                <button class="btn facebook-btn" aria-label="Login with Facebook">
                  <i class="bi bi-facebook"></i>
                </button>
                <button class="btn google-btn" aria-label="Login with Google">
                  <i class="bi bi-google"></i>
                </button>
                <button class="btn linkedin-btn" aria-label="Login with LinkedIn">
                  <i class="bi bi-linkedin"></i>
                </button>
                <button class="btn twitter-btn" aria-label="Login with Twitter">
                  <i class="bi bi-twitter"></i>
                </button>
              </div>
            </form>
          </div>
        </>
    }
}
