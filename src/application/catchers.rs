use rocket_contrib::json;
use rocket::{ Request, catch };

#[catch(500)]
pub fn internal_error() -> json::Json<json::JsonValue> {
    json::Json(json!({
        "message": "Whoops! Looks like we messed up.",
    }))
}

#[catch(404)]
pub fn not_found(req: &Request) -> json::Json<json::JsonValue> {
    json::Json(json!({
        "message": format!("I couldn't find '{}'...", req.uri()),
    }))
}