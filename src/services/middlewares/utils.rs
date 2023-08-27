use hyper::{Body, Request};

pub struct MiddlewareUtil {}

impl MiddlewareUtil {
    fn without_auth_services() -> Vec<String> {
        vec![
            String::from("/user.User/LoginUser"),
            String::from("/user.User/RegisterUser"),
        ]
    }

    fn should_auth(req: &Request<Body>) -> bool {
        let path = req.uri().path().to_string();
        let without_auth_services = MiddlewareUtil::without_auth_services();
        !without_auth_services.contains(&path)
    }

    pub fn extract_token(req: &Request<Body>) -> Option<String> {
        Self::should_auth(req);
        let bearer_token = req.headers().get("authorization")?.to_str().ok()?;
        let token = bearer_token.replace("Bearer ", "");
        Some(token)
    }
}
