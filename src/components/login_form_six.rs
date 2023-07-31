use serde::{ Deserialize, Serialize };
use wasm_bindgen_futures::spawn_local;
use web_sys::{ console, HtmlInputElement, Window };
use yew::prelude::*;

use crate::api::auth::login_user;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct LoginUserSchema {
    email: String,
    password: String,
}

#[function_component(LoginFormSix)]
pub fn login_form_six() -> Html {
    let error_handle = use_state(String::default);
    let error = (*error_handle).clone();

    let input_email_ref = use_node_ref();
    let input_email_handle = use_state(String::default);
    let input_email = (*input_email_handle).clone();

    let input_password_ref = use_node_ref();
    let input_password_handle = use_state(String::default);
    let input_password = (*input_password_handle).clone();

    let on_email_change = {
        let input_email_ref = input_email_ref.clone();

        Callback::from(move |_| {
            let input = input_email_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_email_handle.set(value);
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
        });
    });

    html! {
        <div class="container-half">
          <div
            class="bg-img order-1 order-md-2"
            style="background-image: url('images/bg_1.jpg')"
          ></div>
          <div class="contents order-2 order-md-1">
            <div class="container">
              <div class="row align-items-center justify-content-center">
                <div class="col-md-7">
                  <h2 class="heading-secondary mb-4">
                    {"Login to"} <strong>{"Login"}</strong>
                  </h2>
                  if !error.is_empty() {
                    <div class="error">{error}</div>
                  }
                  <p class="paragraph mb-4">
                    {"Welcome back!"}
                  </p>
                  <form method="post" onsubmit={onsubmit}>
                    <div class="form-group first">
                      <label for="username">{"Username"}</label>
                      <div class="input-group">
                        <span class="input-group-text"
                          ><i class="bi bi-person"></i
                        ></span>
                        <input
                          type="text"
                          class="form-control"
                          placeholder="Email"
                          id="username"
                          ref={input_email_ref}
                          oninput={on_email_change}
                          aria-required="true"
                        />
                      </div>
                    </div>
                    <div class="form-group last mb-3">
                      <label for="password">{"Password"}</label>
                      <div class="input-group">
                        <span class="input-group-text"
                          ><i class="bi bi-lock"></i
                        ></span>
                        <input
                          type="password"
                          class="form-control"
                          id="password"
                          placeholder="Password"
                          aria-required="true"
                          ref={input_password_ref}
                          oninput={on_password_change}
                        />
                      </div>
                    </div>
                    <div class="d-flex mb-5 align-items-center">
                      <label class="control control--checkbox mb-0"
                        ><span class="caption">{"Remember me"}</span>
                        <input type="checkbox" />
                        <div class="control__indicator"></div>
                      </label>
                      <span class="ml-auto"
                        ><a href="#" class="link-primary forgot-pass"
                          >{"Forgot Password"}</a
                        ></span
                      >
                    </div>
                    <input
                      type="submit"
                      value="Log In"
                      class="btn btn-block btn-primary"
                    />
                  </form>
                  <div class="social-buttons">
                    <a href="#" class="bi bi-linkedin"></a>
                    <a href="#" class="bi bi-facebook"></a>
                    <a href="#" class="bi bi-twitter"></a>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
    }
}