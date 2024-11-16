use wasm_bindgen::JsCast;
use web_sys::Element;

#[macro_export]
macro_rules! rsx {
    // Empty element
    ($tag:ident { }) => {{
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element(stringify!($tag))
            .unwrap()
    }};

    // Mixed attributes and content in any order
    ($tag:ident { $($item:tt)+}) => {{
        let elem = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element(stringify!($tag))
            .unwrap();
        
        rsx_internal!(elem, $($item)+);
        elem
    }};
}

#[macro_export]
macro_rules! rsx_internal {
    // Base case - nothing left to process
    ($elem:ident,) => {};

    // Single string literal (no comma)
    ($elem:ident, $text:literal) => {
        $elem.set_text_content(Some($text));
    };

    // Single attribute (no comma)
    ($elem:ident, $attr:ident = $value:expr) => {
        $elem.set_attribute(stringify!($attr), &$value.to_string()).unwrap();
    };

    // Single event handler (no comma)
($elem:ident, $event:ident => $handler:expr) => {
    let closure = Closure::wrap(Box::new($handler) as Box<dyn FnMut(web_sys::Event)>);
    $elem.add_event_listener_with_callback(
        stringify!($event),
        closure.as_ref().unchecked_ref()
    ).unwrap();
    closure.forget();
};

// Event handler followed by more items
($elem:ident, $event:ident => $handler:expr, $($rest:tt)*) => {
    let closure = Closure::wrap(Box::new($handler) as Box<dyn FnMut(web_sys::Event)>);
    $elem.add_event_listener_with_callback(
        stringify!($event),
        closure.as_ref().unchecked_ref()
    ).unwrap();
    closure.forget();
    rsx_internal!($elem, $($rest)*);
};

    // Handle expression with @ (no comma)
    ($elem:ident, @$text:expr) => {
        let text_content = $text;
        $elem.set_text_content(Some(&text_content.to_string()));
    };

    // Handle expression with @ followed by more items
    ($elem:ident, @$text:expr, $($rest:tt)*) => {
        let text_content = $text;
        $elem.set_text_content(Some(&text_content.to_string()));
        rsx_internal!($elem, $($rest)*);
    };

    // Multiple items with comma
    ($elem:ident, $text:literal, $($rest:tt)*) => {
        $elem.set_text_content(Some($text));
        rsx_internal!($elem, $($rest)*);
    };

    ($elem:ident, $attr:ident = $value:expr, $($rest:tt)*) => {
        $elem.set_attribute(stringify!($attr), &$value.to_string()).unwrap();
        rsx_internal!($elem, $($rest)*);
    };

//     // Single keyboard event handler (no comma)
// ($elem:ident, $event:ident => keyboard $handler:expr) => {
//     let closure = Closure::wrap(Box::new(|e: web_sys::Event| {
//         if let Ok(e) = e.dyn_into::<web_sys::KeyboardEvent>() {
//             ($handler)(e)
//         }
//     }) as Box<dyn FnMut(web_sys::Event)>);
//     $elem.add_event_listener_with_callback(
//         stringify!($event),
//         closure.as_ref().unchecked_ref()
//     ).unwrap();
//     closure.forget();
// };

// // Keyboard event handler followed by more items
// ($elem:ident, $event:ident => keyboard $handler:expr, $($rest:tt)*) => {
//     let closure = Closure::wrap(Box::new(|e: web_sys::Event| {
//         if let Ok(e) = e.dyn_into::<web_sys::KeyboardEvent>() {
//             ($handler)(e)
//         }
//     }) as Box<dyn FnMut(web_sys::Event)>);
//     $elem.add_event_listener_with_callback(
//         stringify!($event),
//         closure.as_ref().unchecked_ref()
//     ).unwrap();
//     closure.forget();
//     rsx_internal!($elem, $($rest)*);
// };

    // Handle nested element with content
    ($elem:ident, $child:ident { $($child_content:tt)+ }) => {{
        let child_elem = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element(stringify!($child))
            .unwrap();
        rsx_internal!(child_elem, $($child_content)+);
        $elem.append_child(&child_elem.unchecked_into::<web_sys::Node>()).unwrap();
    }};

    // Handle nested element with content followed by more items
    ($elem:ident, $child:ident { $($child_content:tt)+ }, $($rest:tt)*) => {{
        let child_elem = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element(stringify!($child))
            .unwrap();
        rsx_internal!(child_elem, $($child_content)+);
        $elem.append_child(&child_elem.unchecked_into::<web_sys::Node>()).unwrap();
        rsx_internal!($elem, $($rest)*);
    }};

    // Handle iterator mapping (with comma)
    ($elem:ident, $iter:expr, => |$index:ident, $item:ident| $body:expr) => {
        let elements = $iter.map(|($index, $item)| $body).collect::<Vec<_>>();
        for element in elements {
            $elem.append_child(&element.unchecked_into::<web_sys::Node>()).unwrap();
        }
    };

    // Handle iterator mapping followed by more items (with comma)
    ($elem:ident, $iter:expr, => |$index:ident, $item:ident| $body:expr, $($rest:tt)*) => {
        let elements = $iter.map(|($index, $item)| $body).collect::<Vec<_>>();
        for element in elements {
            $elem.append_child(&element.unchecked_into::<web_sys::Node>()).unwrap();
        }
        rsx_internal!($elem, $($rest)*);
    };


    // Handle child element
    ($elem:ident, $child:expr) => {
        $elem.append_child(&$child.unchecked_into::<web_sys::Node>()).unwrap();
    };

    // Handle child element followed by more items
    ($elem:ident, $child:expr, $($rest:tt)*) => {
        $elem.append_child(&$child.unchecked_into::<web_sys::Node>()).unwrap();
        rsx_internal!($elem, $($rest)*);
    };

    // // Handle if-else condition (no comma)
    // ($elem:ident, if ($cond:expr) { $then:expr } else { $else:expr }) => {
    //     let text = if $cond { $then } else { $else };
    //     $elem.set_text_content(Some(&text.to_string()));
    // };

    // // Handle if-else condition followed by more items
    // ($elem:ident, if ($cond:expr) { $then:expr } else { $else:expr }, $($rest:tt)*) => {
    //     let text = if $cond { $then } else { $else };
    //     $elem.set_text_content(Some(&text.to_string()));
    //     rsx_internal!($elem, $($rest)*);
    // };
}