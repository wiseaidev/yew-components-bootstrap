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

#[function_component(LoginFormEight)]
pub fn login_form_eight() -> Html {
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
        <section class="ftco-section">
          <div class="container">
            <div class="row justify-content-center">
              <div class="col-md-6 text-center mb-5">
                <h2 class="heading-section">{"Login Page"}</h2>
              </div>
            </div>
            <div class="row justify-content-center">
              <div class="col-md-6 col-lg-4">
                <div class="login-wrap py-5">
                  <div
                    class="img d-flex align-items-center justify-content-center profile-picture"
                    alt="Login Image"
                  ></div>
                  <h3 class="text-center mb-0">{"Welcome Back!"}</h3>
                  <p class="text-center">
                    {"Fill in your credentials below to log in!"}
                  </p>
                  <form action="#" class="login-form" autocomplete="off" onsubmit={onsubmit}>
                    if !error.is_empty() {
                      <div class="error">{error}</div>
                    }
                    <div class="form-group">
                      <div
                        class="icon d-flex align-items-center justify-content-center"
                      >
                        <span class="bi bi-person-fill" aria-hidden="true"></span>
                      </div>
                      <input
                        type="text"
                        class="form-control"
                        placeholder="Email"
                        ref={input_email_ref}
                        oninput={on_email_change}
                        aria-required="true"
                        aria-label="Username"
                      />
                    </div>
                    <div class="form-group">
                      <div
                        class="icon d-flex align-items-center justify-content-center"
                      >
                        <span class="bi bi-lock-fill" aria-hidden="true"></span>
                      </div>
                      <input
                        type="password"
                        class="form-control"
                        placeholder="Password"
                        aria-required="true"
                        ref={input_password_ref}
                        oninput={on_password_change}
                        aria-label="Password"
                      />
                    </div>
                    <div class="form-group d-md-flex">
                      <div class="w-100 text-md-right">
                        <a href="#" aria-label="Forgot Password">{"Forgot Password"}</a>
                      </div>
                    </div>
                    <div class="form-group">
                      <button
                        type="submit"
                        class="btn form-control btn-primary rounded submit px-3"
                        aria-label="Get Started"
                      >
                        {"Get Started"}
                      </button>
                    </div>
                  </form>
                  <div class="w-100 text-center mt-4 text">
                    <p class="mb-0">{"Don't have an account?"}</p>
                    <a href="#" aria-label="Sign Up">{"Sign Up"}</a>
                  </div>
                  <div
                    class="social-buttons"
                    role="navigation"
                    aria-label="Social Buttons"
                  >
                    <a
                      href="#"
                      class="social-button facebook-icon"
                      aria-label="Facebook"
                    >
                      <i class="bi bi-facebook" aria-hidden="true"></i>
                    </a>
                    <a
                      href="#"
                      class="social-button linkedin-icon"
                      aria-label="LinkedIn"
                    >
                      <i class="bi bi-linkedin" aria-hidden="true"></i>
                    </a>
                    <a
                      href="#"
                      class="social-button twitter-icon"
                      aria-label="Twitter"
                    >
                      <i class="bi bi-twitter" aria-hidden="true"></i>
                    </a>
                    <a
                      href="#"
                      class="social-button instagram-icon"
                      aria-label="Instagram"
                    >
                      <i class="bi bi-instagram" aria-hidden="true"></i>
                    </a>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>
    }
}