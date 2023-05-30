use anyhow::anyhow;

use crate::proto::{auth_service_client::AuthServiceClient, AuthRequest, User};

pub async fn is_role(token: impl Into<String>, role: impl Into<String>) -> anyhow::Result<User> {
    let uri = std::env::var("AUTH_SERVER_URL")?;
    let mut client = AuthServiceClient::connect(format!("http://{}", uri)).await?;
    let auth = client
        .is_user_authenticated_with_role(AuthRequest {
            auth_token: token.into(),
            role: role.into(),
        })
        .await?
        .into_inner();
    if !auth.is_authenticated {
        return Err(anyhow!("Not allowed"));
    }
    match auth.user {
        Some(user) => Ok(user),
        None => Err(anyhow!("User was somehow not there")),
    }
}
