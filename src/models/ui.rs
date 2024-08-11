
use askama::Template;

use crate::models::users::{User};
use crate::models::loans::{Loan};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate{
    pub(crate) error: Option<String>,
    pub(crate) message: Option<String>
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate{
    pub(crate) error: Option<String>
}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub user: User,
    pub loans: Vec<Loan>
}

#[derive(Template)]
#[template(path = "add-loan.html")]
pub struct AddLoanTemplate{
    pub user: User,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate;