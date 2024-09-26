
use askama::Template;

use crate::models::users::{User};
use crate::models::loans::{FullLoan, Loan};
use crate::models::loan_applications::{LoanApplicationWithBorrower, LoanApplicationWithLender};
use crate::models::payments::PaymentConfig;

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
    pub loans: Vec<FullLoan>
}

#[derive(Template)]
#[template(path = "add-loan.html")]
pub struct AddLoanTemplate{
    pub user: User,
}

#[derive(Template)]
#[template(path = "complete-loan-payment.html")]
pub struct CompleteLoanPaymentTemplate{
    pub user: User,
    pub loan: Loan,
    pub payment_config: PaymentConfig
}

#[derive(Template)]
#[template(path = "mpesa-checkout.html")]
pub struct MpesaCheckoutTemplate{
    pub customerPhone: String,
    pub user: User
}

#[derive(Template)]
#[template(path = "lended-loans.html")]
pub struct LendedLoansTemplate {
    pub user: User,
    pub lended_loans: Vec<LoanApplicationWithBorrower>
}

#[derive(Template)]
#[template(path = "applied-loans.html")]
pub struct AppliedLoansTemplate {
    pub user: User,
    pub applied_loans: Vec<LoanApplicationWithLender>
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate;