#![allow(dead_code, unused_variables, unused_imports)]

use iced::widget::{
    button, center, column, container, horizontal_space, row, stack, text, text_input,
};
use iced::{Application, Element, Fill, Subscription, Task, Theme};
use sqlx::postgres::PgPool;
use std::sync::Arc;

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
    pgpool: Option<Arc<PgPool>>,
    is_connected_to_db: bool,
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
    RegistrationResult(Result<uuid::Uuid, Error>),
    Logout,
    HandleInputTelephone(String),
    HandleInputPassword(String),
    HandleInputFirstName(String),
    HandleInputLastName(String),
    SwitchFromLoginToRegister,
    SwitchFromRegisterToLogin,
    ConnectoToDatabase,
    DatabaseConnectionResult(Result<PgPool, Error>),
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
                is_connected_to_db: false,
            },
            // Start the Database Connection
            Task::perform(connect_to_db(), Message::DatabaseConnectionResult),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // TODO: We don't really need this right? But what happens if during the working of the
            // application, the database connection falls? Then we can still call this function and
            // run
            Message::ConnectoToDatabase => {
                Task::perform(connect_to_db(), Message::DatabaseConnectionResult)
            }
            Message::DatabaseConnectionResult(result) => {
                //
                if let Ok(result) = result {
                    self.pgpool = Some(Arc::new(result));
                    println!("Connected to the Database! ");
                    self.is_connected_to_db = true;
                    Task::none()
                } else {
                    self.is_connected_to_db = false;
                    // BUG: This could create an infinite loop ... if no db
                    println!("This could create a loop though, reconnecting?");
                    Task::perform(connect_to_db(), Message::DatabaseConnectionResult)
                }
            }
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
                if let Some(arc_pgpool) = &self.pgpool {
                    // let pgpool = Arc::clone(&arc_pgpool);
                    // let pgpool_deref = *pgpool;
                    Task::perform(
                        register_user(
                            self.user.first_name.clone(),
                            self.user.last_name.clone(),
                            self.user.telephone.clone(),
                            self.user.password.clone(),
                            Arc::clone(arc_pgpool),
                        ),
                        Message::RegistrationResult,
                    )
                } else {
                    println!("We dont have a database connection so the shit hit the fan");
                    Task::none()
                }
            }
            Message::RegistrationResult(result) => {
                if let Ok(result) = result {
                    self.user.user_id = Some(result);
                    println!("The user_id created {:?}", result);
                } else {
                    panic!()
                }
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
                    text("Connection to database:"),
                    text(if self.is_connected_to_db {
                        "Connected"
                    } else {
                        "Not Connected"
                    }),
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

// TODO: Take the database_url from the environment variables
async fn connect_to_db() -> Result<PgPool, Error> {
    let pool = PgPool::connect("postgres://alex:1234@localhost/icedforms").await;
    if let Ok(pool) = pool {
        Ok(pool)
    } else {
        // TODO: handle the error better for connecting to the DB Better
        panic!()
    }
}

async fn register_user<'a>(
    first_name: String,
    last_name: String,
    telephone: String,
    password: String,
    pgpool: Arc<PgPool>,
) -> Result<uuid::Uuid, Error> {
    // save the user data to the database
    let rec = sqlx::query!(
        r#"
INSERT INTO "user"(first_name, last_name, telephone, password)
VALUES ($1, $2, $3, $4)
RETURNING user_id
    "#,
        first_name,
        last_name,
        telephone,
        password,
    )
    .fetch_one(&*pgpool)
    .await;
    if let Ok(result) = rec {
        Ok(result.user_id)
    } else {
        panic!()
    }
}

async fn authenticate_user(telephone: String, password: String) -> Result<User, Error> {
    // check if the password corresponds to the telephone
    // return the User data
    todo!()
}

async fn get_all_users() -> Result<Vec<User>, Error> {
    todo!()
}
