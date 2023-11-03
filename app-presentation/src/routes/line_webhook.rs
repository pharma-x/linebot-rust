use crate::context::errors::SignatureVerificationError;
use crate::model::line_webhook::{LineWebhookEvent, LineWebhookRequest, LineWebhookRequests};
use crate::module::{Modules, ModulesExt};
use axum::{
    body::Bytes,
    extract::Extension,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;
use std::sync::Arc;
use tracing::error;

/*
 * Jsonを受け取るときは、引数の順番に気をつける必要がある
 * https://github.com/tokio-rs/axum/discussions/1755
 * https://docs.rs/axum/latest/axum/extract/index.html#the-order-of-extractors
*/
#[tracing::instrument(skip(modules))]
pub async fn line_webhook_handler(
    Extension(modules): Extension<Arc<Modules>>,
    headers: HeaderMap,
    body_bytes: Bytes,
) -> Result<impl IntoResponse, StatusCode> {
    let channel_secret = env::var("LINE_CHANNEL_SECRET")
        .unwrap_or_else(|_| panic!("LINE_CHANNEL_SECRET must be set!"));
    // x-line-signature ヘッダーを文字列として取得します。
    let x_line_signature = headers
        .get("x-line-signature")
        .ok_or(StatusCode::BAD_REQUEST)?
        .as_bytes();
    // リクエストボディをバイト列として取得します。
    let http_request_body = body_bytes.as_ref();
    // 署名を検証します。
    if let Err(err) =
        verify_line_webhook_signature(&channel_secret, http_request_body, x_line_signature)
    {
        error!("Error: {}", err);
        return Err(StatusCode::UNAUTHORIZED);
    }
    // バイト列からpayloadをパースする
    let payload: LineWebhookRequests = serde_json::from_slice(&body_bytes).map_err(|err| {
        error!("Failed to parse JSON: {}", err);
        StatusCode::BAD_REQUEST
    })?;
    let requests: Vec<LineWebhookRequest> = payload.into();

    // すぐにstatus code 200で返すために、非同期で処理を行う
    tokio::spawn(process_line_events(requests, modules));

    Ok(StatusCode::OK)
}

async fn process_line_events(
    requests: Vec<LineWebhookRequest>,
    modules: Arc<Modules>,
) -> anyhow::Result<()> {
    for request in requests {
        let event = &request.event;
        match event {
            LineWebhookEvent::Follow(_) => modules
                .linebot_webhook_usecase()
                .create_follow_event(request.into())
                .await
                .map_err(|err| anyhow::anyhow!("Unexpected error: {:?}", err))?,
            LineWebhookEvent::Unfollow(e) => {
                println!("Unfollow event: {:?}", e);
            }
            LineWebhookEvent::Message(e) => {
                println!("Message event: {:?}", e);
            }
            LineWebhookEvent::Postback(e) => {
                println!("Postback event: {:?}", e);
            }
            LineWebhookEvent::VideoPlayComplete(e) => {
                println!("Other event: {:?}", e);
            }
        }
    }
    Ok(())
}

/// Verify LINE webhook signature
///
/// # Arguments
/// `channel_secret` - Channel secret string
/// `http_request_body` - HTTP request body string
/// `x_line_signature` - The 'x-line-signature' header from the HTTP request
///
fn verify_line_webhook_signature(
    channel_secret: &String,
    http_request_body: &[u8],
    x_line_signature: &[u8],
) -> anyhow::Result<()> {
    // Create HMAC-SHA256 instance with the channel secret as the key
    let mut mac = Hmac::<Sha256>::new_from_slice(channel_secret.as_bytes())
        .map_err(|_| SignatureVerificationError::CannotCreateMac)?;
    // Input the HTTP request body to the HMAC
    mac.update(http_request_body);
    // Obtain the result of the HMAC as a byte array
    let result = mac.finalize().into_bytes();
    // Encode the byte array into a base64 string
    let signature = general_purpose::STANDARD.encode(result);

    if signature.as_bytes() != x_line_signature {
        error!("Signature verification failed: Computed signature does not match the provided signature");
        return Err(SignatureVerificationError::InvalidSignature.into());
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{model::line_webhook::LineWebhookFollowEvent, module::test::TestModules};

    use super::*;
    use adapter::model::{
        event::EventTable,
        talk_room::{TalkRoomCardTable, TalkRoomTable, TalkRoomWrapper},
    };
    use application::model::event::CreateUserEvent;
    use domain::{
        model::{
            event::NewEvent,
            line_user::LineUserProfile,
            primary_user_id::PrimaryUserId,
            talk_room::NewTalkRoom,
            user::{User, UserProfile},
            user_auth::AuthUserId,
        },
        repository::{
            talk_room::MockTalkRoomRepository, user::MockUserRepository,
            user_auth::MockUserAuthRepository,
        },
    };
    use dotenv::dotenv;
    use fake::{Fake, Faker};
    use mockall::predicate;

    #[test]
    fn test_verify_line_webhook_signature() {
        dotenv().ok();
        let channel_secret = env::var("LINE_CHANNEL_SECRET")
            .unwrap_or_else(|_| panic!("LINE_CHANNEL_SECRET must be set!"));
        let http_request_body = b"test_request_body";
        let invalid_signature = b"invalid_signature";

        // Compute the expected signature using the test channel secret and request body
        let mut mac = Hmac::<Sha256>::new_from_slice(channel_secret.as_bytes()).unwrap();
        mac.update(http_request_body);
        let expected_signature = mac.finalize().into_bytes();
        let expected_signature_str = general_purpose::STANDARD.encode(expected_signature);

        /*
         * 計算されたシグネチャが期待されるシグネチャと一致することを検証します
         */
        let result = verify_line_webhook_signature(
            &channel_secret,
            http_request_body,
            expected_signature_str.as_bytes(),
        );
        assert!(result.is_ok());

        /*
         * 無効なシグネチャが検証に失敗することを検証します
         */
        let result =
            verify_line_webhook_signature(&channel_secret, http_request_body, invalid_signature);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_process_line_follow_event() {
        dotenv().ok();
        let user_auth_repository = MockUserAuthRepository::new();
        let mut user_repository = MockUserRepository::new();
        let mut talk_room_repository = MockTalkRoomRepository::new();

        let destintion = "line_id".to_string();
        let line_webhook_event = LineWebhookEvent::Follow(Faker.fake::<LineWebhookFollowEvent>());
        let request = LineWebhookRequest::new(destintion, line_webhook_event);

        let create_user_event = CreateUserEvent::from(request.clone());
        let create_line_user_auth = create_user_event.create_line_user_auth;
        /*
         * ユーザーが存在するパターン
         */
        let auth_user_id = AuthUserId::from(create_line_user_auth);
        let primary_user_id = PrimaryUserId::new("primay_user_id".to_string());
        let user = User::new(
            primary_user_id.clone(),
            UserProfile::Line(LineUserProfile::new(
                auth_user_id.clone(),
                "display_name".to_string(),
                "picture_url".to_string(),
            )),
        );
        // let cloned_user = user.clone();
        let new_event = NewEvent::from(create_user_event.create_event);
        let new_talk_room = NewTalkRoom::from((user.clone(), new_event.clone()));
        user_repository
            .expect_get_user()
            .with(predicate::eq(auth_user_id))
            .once()
            .returning(move |_| Ok(user.clone()));
        /*
         * talk_roomが存在するパターン
         */
        let talk_room = TalkRoomWrapper::from((
            TalkRoomTable::from(new_talk_room.clone()),
            TalkRoomCardTable::from(new_talk_room.clone()),
            EventTable::from(new_event.clone()),
        ))
        .0;
        // let new_talk_room = NewTalkRoom::from((talk_room.clone(), new_event)).clone();
        talk_room_repository
            .expect_get_talk_room()
            .with(predicate::eq(primary_user_id))
            .once()
            .returning(move |_| Ok(talk_room.clone()));

        /*
         * talk_roomをupdateし、talk_roomのサブコレクションにeventを追加する
         */
        talk_room_repository
            .expect_create_event()
            // .with(predicate::eq(new_talk_room))
            .withf(|_| true)
            .once()
            .returning(move |_| Ok(()));

        /*
         * 最後にtest用のモジュールで処理が通れば成功
         */
        let modules = Arc::new(
            TestModules::new(user_auth_repository, user_repository, talk_room_repository).await,
        );
        let response = modules
            .linebot_webhook_usecase()
            .create_follow_event(request.into())
            .await
            .map_err(|err| anyhow::anyhow!("Unexpected error: {:?}", err));

        assert!(response.is_ok());
    }
}
