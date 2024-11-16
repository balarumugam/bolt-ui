use wasm_bindgen::prelude::*;
use web_sys::XmlHttpRequest;
use js_sys::{JSON, Object, Reflect, Array};
use std::cell::RefCell;

thread_local! {
    static CONTENT_CACHE: RefCell<Option<Content>> = RefCell::new(None);
}


#[derive(Clone, Debug)]
pub struct Article {
    pub title: String,
    pub content: String,
    pub date: String,
    pub author: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Content {
    pub articles: Vec<Article>,
}

// Safe helper function to get string from object
fn get_string_from_obj(obj: &Object, key: &str) -> Option<String> {
    Reflect::get(obj, &JsValue::from_str(key))
        .ok()
        .and_then(|val| val.as_string())
}

// Safe helper function to get string array from object
fn get_string_array_from_obj(obj: &Object, key: &str) -> Vec<String> {
    Reflect::get(obj, &JsValue::from_str(key))
        .ok()
        .and_then(|val| Some(js_sys::Array::from(&val)))
        .map(|arr| {
            arr.to_vec()
                .into_iter()
                .filter_map(|v| v.as_string())
                .collect()
        })
        .unwrap_or_default()
}

fn parse_json(json_str: &str) -> Option<Content> {
    let parsed = JSON::parse(json_str).ok()?;
    let obj = parsed.dyn_into::<Object>().ok()?;
    
    let articles = js_sys::Reflect::get(&obj, &"articles".into()).ok()?;
    let articles_array = js_sys::Array::from(&articles);
    
    let articles: Vec<Article> = (0..articles_array.length())
        .filter_map(|i| {
            let article = articles_array.get(i);
            let obj = article.dyn_into::<Object>().ok()?;
            
            Some(Article {
                title: get_string_from_obj(&obj, "title").unwrap_or_default(),
                content: get_string_from_obj(&obj, "content").unwrap_or_default(),
                date: get_string_from_obj(&obj, "date").unwrap_or_default(),
                author: get_string_from_obj(&obj, "author").unwrap_or_default(),
                tags: get_string_array_from_obj(&obj, "tags"),
            })
        })
        .collect();

    Some(Content { articles })
}

fn create_fallback_article() -> Article {
    Article {
        title: "Loading...".into(),
        content: "Content is loading...".into(),
        date: String::new(),
        author: String::new(),
        tags: Vec::new(),
    }
}

pub fn load_content() -> Content {

    // Check if content is already cached
    if let Some(cached_content) = CONTENT_CACHE.with(|cache| cache.borrow().clone()) {
        return cached_content;
    }

    // Create fallback content in case of loading failure
    let fallback = Content {
        articles: vec![create_fallback_article()]
    };

    let xhr = XmlHttpRequest::new().unwrap();
    xhr.open_with_async("GET", "content.json", false).unwrap();
    xhr.send().unwrap();


    // Check if request was successful (status 200)
    let content = if xhr.status().unwrap() == 200 {
        if let Ok(response_text) = xhr.response_text() {
            if let Some(text) = response_text {
                parse_json(&text).unwrap_or(fallback.clone())
            } else {
                fallback.clone()
            }
        } else {
            fallback.clone()
        }
    } else {
        fallback.clone()
    };

    // Cache the content
    CONTENT_CACHE.with(|cache| {
        *cache.borrow_mut() = Some(content.clone());
    });


    content
}