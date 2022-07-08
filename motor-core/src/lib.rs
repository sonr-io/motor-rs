use prs::account::{CreateAccountRequest, CreateAccountResponse};

pub fn create_account(_req: CreateAccountRequest) -> CreateAccountResponse {
  CreateAccountResponse::default()
}