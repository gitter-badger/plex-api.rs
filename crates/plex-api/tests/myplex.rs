mod fixtures;

use fixtures::offline::myplex::*;
use fixtures::offline::Mocked;
use httpmock::Method::GET;
use httpmock::Method::PUT;
use plex_api::{
    url::{MYPLEX_CLAIM_TOKEN_PATH, MYPLEX_PRIVACY_PATH},
    MyPlex,
};

#[plex_api_test_helper::async_offline_test]
async fn privacy(#[future] myplex: Mocked<MyPlex>) {
    let myplex = myplex.await;
    let (myplex, mock_server) = myplex.decompose();

    let m = mock_server.mock(|when, then| {
        when.method(GET).path(MYPLEX_PRIVACY_PATH);
        then.status(200)
            .header("content-type", "text/json")
            .body_from_file("tests/files/myplex/api/v2/user/privacy.json");
    });

    let privacy_result = myplex.privacy().await;
    m.assert();

    let mut privacy = privacy_result.expect("failed to get privacy");

    assert!(
        privacy.opt_out_playback,
        "Unexpected opt_out_playback value"
    );
    assert!(
        privacy.opt_out_library_stats,
        "Unexpected opt_out_library_stats value"
    );

    let m = mock_server.mock(|when, then| {
        when.method(PUT)
            .path(MYPLEX_PRIVACY_PATH)
            .x_www_form_urlencoded_tuple("optOutPlayback", "0")
            .x_www_form_urlencoded_tuple("optOutLibraryStats", "0");
        then.status(204)
            .header("content-type", "text/json")
            .body_from_file("tests/files/myplex/api/v2/user/privacy.json");
    });

    let privacy_update_result = privacy.update(false, false).await;
    m.assert();

    assert!(
        privacy_update_result.is_ok(),
        "failed to update privacy settings"
    );

    assert!(
        !privacy.opt_out_playback,
        "Unexpected opt_out_playback value"
    );
    assert!(
        !privacy.opt_out_library_stats,
        "Unexpected opt_out_library_stats value"
    );
}

#[plex_api_test_helper::async_offline_test]
async fn claim_token(#[future] myplex: Mocked<MyPlex>) {
    let myplex = myplex.await;
    let (myplex, mock_server) = myplex.decompose();

    let mut mock = mock_server.mock(|when, then| {
        when.method(GET).path(MYPLEX_CLAIM_TOKEN_PATH);
        then.status(200)
            .header("content-type", "text/json")
            .body_from_file("tests/files/myplex/api/claim/token.json");
    });

    let token = myplex
        .claim_token()
        .await
        .expect("failed to get claim_token");
    mock.assert();
    mock.delete();

    assert_eq!(
        token.to_string(),
        "claim-TOKEN",
        "Received unexpected token"
    );
    assert!(!token.is_expired(), "Token is expired");

    let mut mock = mock_server.mock(|when, then| {
        when.method(GET).path(MYPLEX_CLAIM_TOKEN_PATH);
        then.status(503)
            .header("content-type", "text/json")
            .body(r#"{"error": "Something went wrong"}"#);
    });

    let token_result = myplex.claim_token().await;
    mock.assert();
    mock.delete();

    assert!(
        token_result.is_err(),
        "Expected error when getting claim_token"
    );

    assert!(
        matches!(
            token_result.err().unwrap(),
            plex_api::Error::FailedToGetClaimToken(_)
        ),
        "Unexpected error returned"
    );
}
