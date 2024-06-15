use wasm_bindgen::prelude::*;
use web_sys::{HtmlButtonElement, HtmlImageElement, HtmlInputElement, HtmlTableCellElement, HtmlTableElement, HtmlTableRowElement};



// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // Your code goes here!
    
    let window = web_sys::window().expect("no window");
    let document = window.document().expect("no document");
    
    let bs_add_button = document.get_element_by_id("add-button").unwrap().dyn_into::<HtmlButtonElement>().expect("no `add-button`");
    let a = Closure::<dyn Fn()>::new(move || {
        let name = document.get_element_by_id("user-name").unwrap().dyn_into::<HtmlInputElement>().expect("no `user-name`").value();
        let pfpurl = document.get_element_by_id("user-pfp").unwrap().dyn_into::<HtmlInputElement>().expect("no `user-pfp`").value();
        let email = document.get_element_by_id("user-email").unwrap().dyn_into::<HtmlInputElement>().expect("no `user-email`").value();
        let phone = document.get_element_by_id("user-phone").unwrap().dyn_into::<HtmlInputElement>().expect("no `user-phone`").value();
        let table = document.get_element_by_id("users-table").unwrap().dyn_into::<HtmlTableElement>().expect("no `users-table`");
        let old_index = table.rows().item(table.rows().length() - 1).unwrap().children().item(0).unwrap().inner_html();
        let old_index = old_index.parse::<u32>().expect("not number");
        let row = table.insert_row().unwrap().dyn_into::<HtmlTableRowElement>().expect("cannot create row");
        let cell1 = row.insert_cell().unwrap().dyn_into::<HtmlTableCellElement>().expect("cannot create cell");
        cell1.set_inner_html(&format!("{}", old_index + 1));
        let cell2 = row.insert_cell().unwrap().dyn_into::<HtmlTableCellElement>().expect("cannot create cell");
        cell2.set_inner_html(&format!("<img src=\"{}\" class=\"pfp\"", pfpurl));
        let img = document.create_element("img").unwrap().dyn_into::<HtmlImageElement>().expect("cannot create img");
        img.set_src(&pfpurl);
        img.set_class_name("pfp");
        cell2.append_child(&img.into());
        cell2.set_class_name("pfp");
        let cell3 = row.insert_cell().unwrap().dyn_into::<HtmlTableCellElement>().expect("cannot create cell");
        cell3.set_inner_html(&name);
        cell3.set_class_name("name");
        let cell4 = row.insert_cell().unwrap().dyn_into::<HtmlTableCellElement>().expect("cannot create cell");
        cell4.set_inner_html(&email);
        cell4.set_class_name("email");
        let cell5 = row.insert_cell().unwrap().dyn_into::<HtmlTableCellElement>().expect("cannot create cell");
        cell5.set_inner_html(&phone);
        cell5.set_class_name("phone");
        
    });
    
    bs_add_button.set_onclick(Some(a.as_ref().unchecked_ref()));
    a.forget();

    Ok(())
}
