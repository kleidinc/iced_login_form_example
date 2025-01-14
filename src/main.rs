#![allow(dead_code, unused_variables, unused_imports)]

use iced::widget::{
    button, center, column, container, horizontal_space, row, stack, text, text_input,
};
use iced::{Application, Element, Fill, Subscription, Task, Theme};
use sqlx::postgres::PgPool;

fn main() -> iced::Result {
    iced::application(
        "Iced Login Form Experiment",
        IcedLogin::update,
        IcedLogin::view,
    )
    // .theme(|_| Theme::Dark)
    .theme(IcedLogin::theme)
    .run_with(IcedLogin::new)
}

#[derive(Debug, Clone, Default)]
struct User {
    user_id: Option<uuid::Uuid>,
    first_name: String,
    last_name: String,
    telephone: String,
    password: String,
}

impl User {
    fn new() -> Self {
        Self {
            // The initial state where the user is not logged in
            user_id: None,
            first_name: String::new(),
            last_name: String::new(),
            telephone: String::new(),
            password: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct IcedLogin {
    // When user is None show this form
    is_login_form_shown: bool,
    // If user presses register show this form
    is_register_form_shown: bool,
    user: User,
    user_authenticated: bool,
    // Obviously when sharing this connection we need to use a smartpointer
    pgpool: Option<PgPool>,
}

#[derive(Debug, Clone)]
enum Error {
    WrongLoginPassword,
    RegistrationError,
}

#[derive(Debug, Clone)]
enum Message {
    ShowLogin,
    Login,
    LoginResult(Result<User, Error>),
    ShowRegister,
    Register,
    RegistrationResult(Result<User, Error>),
    Logout,
    HandleInputTelephone(String),
    HandleInputPassword(String),
    HandleInputFirstName(String),
    HandleInputLastName(String),
    SwitchFromLoginToRegister,
    SwitchFromRegisterToLogin,
}

impl IcedLogin {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                is_login_form_shown: false,
                is_register_form_shown: false,
                user: User::new(),
                user_authenticated: false,
                pgpool: None,
            },
            // Task::perform ( start the database connection)
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ShowLogin => {
                self.is_login_form_shown = true;
                Task::none()
            }
            Message::Login => {
                self.is_login_form_shown = false;
                Task::none()
            }
            Message::LoginResult(result) => {
                //
                Task::none()
            }
            Message::Logout => {
                self.user_authenticated = false;
                Task::none()
            }
            Message::ShowRegister => {
                self.is_register_form_shown = true;
                Task::none()
            }
            Message::Register => {
                self.is_register_form_shown = false;
                Task::none()
            }
            Message::RegistrationResult(result) => {
                //
                Task::none()
            }
            Message::HandleInputTelephone(telephone) => {
                self.user.telephone = telephone;
                Task::none()
            }
            Message::HandleInputPassword(password) => {
                self.user.password = password;
                // TODO: create like a real connection to a postgres db to check with a
                // Task::perform
                Task::none()
            }
            Message::HandleInputFirstName(first_name) => {
                self.user.first_name = first_name;
                Task::none()
            }
            Message::HandleInputLastName(last_name) => {
                self.user.last_name = last_name;
                Task::none()
            }
            Message::SwitchFromLoginToRegister => {
                self.is_login_form_shown = false;
                self.is_register_form_shown = true;
                Task::none()
            }
            Message::SwitchFromRegisterToLogin => {
                self.is_register_form_shown = false;
                self.is_login_form_shown = true;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let login_form = container(
            column![
                row![
                    text("Login Form").size(20),
                    horizontal_space(),
                    button("Switch to Registration").on_press(Message::SwitchFromLoginToRegister)
                ],
                row![text("Telephone")],
                row![text_input("", &self.user.telephone).on_input(Message::HandleInputTelephone)],
                row![text("Password")],
                row![text_input("", &self.user.password).on_input(Message::HandleInputPassword)],
                row![button(text("Login")).on_press(Message::Login)]
            ]
            .spacing(5),
        )
        .width(400);
        let registration_form = container(
            column![
                row![
                    text("Registration Form").size(20),
                    horizontal_space(),
                    button("Switch to Login").on_press(Message::SwitchFromRegisterToLogin)
                ],
                row![text("First Name")],
                row![text_input("", &self.user.first_name).on_input(Message::HandleInputFirstName)],
                row![text("Last Name")],
                row![text_input("", &self.user.last_name).on_input(Message::HandleInputLastName)],
                row![text("Telephone")],
                row![text_input("", &self.user.telephone).on_input(Message::HandleInputTelephone)],
                row![text("Password")],
                row![text_input("", &self.user.password).on_input(Message::HandleInputPassword)],
                row![button("Register").on_press(Message::Register)]
            ]
            .spacing(10),
        )
        .width(400);
        let dashboard = container(
            column![
                text("Dashboard").size(25),
                row![
                    text("User Status:"),
                    text(if self.user_authenticated {
                        "Authenticated"
                    } else {
                        "Not Authenticated"
                    })
                ],
                row![
                    button("Login").on_press(Message::ShowLogin),
                    button("Register").on_press(Message::ShowRegister)
                ]
                .spacing(10)
            ]
            .spacing(20),
        )
        .padding(10)
        .width(800)
        .height(600)
        .center_x(Fill);
        //
        // column![dashboard, registration_form, login_form].into()
        if self.is_login_form_shown {
            show_form(dashboard, login_form)
        } else if self.is_register_form_shown {
            show_form(dashboard, registration_form)
        } else {
            dashboard.into()
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dracula
    }
}

fn show_form<'a, Message>(
    dashboard: impl Into<Element<'a, Message>>,
    form: impl Into<Element<'a, Message>>,
) -> Element<'a, Message>
where
    Message: 'a + Clone,
{
    stack![
        dashboard.into(),
        container(form)
            .style(container::rounded_box)
            // you could use the center widget instead
            .center_x(Fill)
            .center_y(Fill)
    ]
    .into()
}

// TODO: Connect to the Database
async fn connect_to_db() -> Result<PgPool, Error> {
    todo!()
}

async fn save_user(user: User) -> Result<uuid::Uuid, Error> {
    // save the user data to the database
    todo!()
}

async fn authenticate_user(telephone: String, password: String) -> Result<User, Error> {
    // check if the password corresponds to the telephone
    // return the User data
    todo!()
}

async fn get_all_users() -> Result<Vec<User>, Error> {
    todo!()
}
