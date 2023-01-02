

#[catch(404)]
pub fn not_found_catcher() -> &'static str {
    return "Unfortunately requested query was not found - error 404";
}