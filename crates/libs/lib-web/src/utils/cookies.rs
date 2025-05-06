use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use time::Duration;

pub fn set_refresh_cookie(jar: CookieJar, token: &str) -> CookieJar {
    let cookie = Cookie::build(("refreshToken", token.to_string()))
        .path("/")
        .max_age(Duration::days(30))
        .http_only(true)
        .secure(false)
        .build();
    jar.add(cookie)
}

pub fn remove_cookie_from_jar(jar: CookieJar, name: &str) -> CookieJar {
    let name = name.to_string();
    jar.remove(Cookie::build((name, "")).path("/").build())
}
