use leptos::web_sys;

pub fn join_by_br(items: Vec<&str>) -> String {
    items.join("<br />")
}

pub fn get_current_path() -> String {
    web_sys::window()
        .expect("should have a window")
        .location()
        .pathname()
        .expect("should have a pathname")
}
