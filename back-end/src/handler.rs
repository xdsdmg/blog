/// handler mod implements the HTTP handler for each api.
use std::fs;

use crate::http::context::Context;
use crate::http::error::HttpError;

/// file returns the content of the file specified by the path.
pub fn file(ctx: &mut Context) {
    let path: String = match &ctx.router_path {
        None => {
            let _ = ctx.text(500, &HttpError::ServiceInternalError.to_string());
            return;
        }
        Some(path) => path.to_string(),
    };

    let file_path = &ctx.request.uri.path[path.len()..];

    let blog_path: String;
    if let Ok(p) = fs::canonicalize(format!("./blog")) {
        if let Some(p) = p.to_str() {
            blog_path = p.to_string();
        } else {
            panic!("invalid blog path");
        }
    } else {
        panic!("invalid blog path");
    }

    let abs_path: String;
    if let Ok(p) = fs::canonicalize(format!("./blog{}", file_path)) {
        if let Some(p) = p.to_str() {
            abs_path = p.to_string();
        } else {
            let _ = ctx.text(403, &HttpError::FileNotFound.to_string());
            return;
        }
    } else {
        let _ = ctx.text(403, &HttpError::FileNotFound.to_string());
        return;
    }

    if !abs_path.starts_with(&blog_path) {
        let _ = ctx.text(403, &HttpError::FileNotFound.to_string());
        return;
    }

    if let Ok(content) = fs::read_to_string(abs_path) {
        let _ = ctx.text(200, &content);
    } else {
        let _ = ctx.text(403, &HttpError::FileNotFound.to_string());
        return;
    }
}

/// ping pong
pub fn ping(ctx: &mut Context) {
    if let Err(e) = ctx.text(200, "Pong!") {
        println!("handler failed, error: {}", HttpError::Parse(e));
    };
}
