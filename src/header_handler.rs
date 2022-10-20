use warp::Filter;

const XAUTH: &str = "X-Auth-Token";

pub fn auth() -> impl Filter<Extract = (ContextUser,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::header::<String>(XAUTH))
        .and_then(|xauth: String| async move {
            if !xauth.starts_with("ok:") {
                return Err(warp::reject::custom(AuthError));
            }
            // Ok::<(), warp::Rejection>(())
            if let Some(id) = xauth
                .split(":")
                .skip(1)
                .next()
                .and_then(|v| v.parse::<i64>().ok())
            {
                Ok::<ContextUser, warp::Rejection>(ContextUser { id })
            } else {
                return Err(warp::reject::custom(AuthErrorNumberNeed));
            }
        })
}

pub struct ContextUser {
    pub id: i64,
}

#[derive(Debug)]
pub struct AuthError;
impl warp::reject::Reject for AuthError {}

#[derive(Debug)]
pub struct AuthErrorNumberNeed;
impl warp::reject::Reject for AuthErrorNumberNeed {}
