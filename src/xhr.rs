use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::XmlHttpRequest;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use js_sys::{JSON, Object, Array};

// Cache for loaded resources
static RESOURCE_CACHE: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

#[derive(Debug)]
pub enum ResourceType {
    CSS,
    JSON,
    HTML,
    Text,
}

impl ResourceType {
    fn content_type(&self) -> &str {
        match self {
            ResourceType::CSS => "text/css",
            ResourceType::JSON => "application/json",
            ResourceType::HTML => "text/html",
            ResourceType::Text => "text/plain",
        }
    }
}

pub struct XhrRequest {
    url: String,
    resource_type: ResourceType,
    use_cache: bool,
}

impl XhrRequest {
    pub fn new(url: &str, resource_type: ResourceType) -> Self {
        Self {
            url: url.to_string(),
            resource_type,
            use_cache: true,
        }
    }

    pub fn disable_cache(mut self) -> Self {
        self.use_cache = false;
        self
    }

    pub fn send(&self) -> Result<String, JsValue> {
        // Check cache first if enabled
        if self.use_cache {
            if let Ok(cache) = RESOURCE_CACHE.lock() {
                if let Some(content) = cache.get(&self.url) {
                    return Ok(content.clone());
                }
            }
        }

        let xhr = XmlHttpRequest::new()?;
        xhr.open("GET", &self.url)?;
        xhr.set_request_header("Content-Type", self.resource_type.content_type())?;
        xhr.send()?;

        // Synchronous request for simplicity
        if xhr.status()? == 200 {
            if let Ok(Some(response_text)) = xhr.response_text() {
                // Cache the response if caching is enabled
                if self.use_cache {
                    if let Ok(mut cache) = RESOURCE_CACHE.lock() {
                        cache.insert(self.url.clone(), response_text.clone());
                    }
                }
                Ok(response_text)
            } else {
                Err(JsValue::from_str("Failed to get response text"))
            }
        } else {
            Err(JsValue::from_str(&format!(
                "Failed to load resource: {} (Status: {})",
                self.url,
                xhr.status()?
            )))
        }
    }
}

// Helper functions for common resource types
pub fn load_css(url: &str) -> Result<String, JsValue> {
    XhrRequest::new(url, ResourceType::CSS).send()
}

pub fn load_json(path: &str) -> Result<JsValue, JsValue> {
    let text = XhrRequest::new(path, ResourceType::JSON).send()?;
    JSON::parse(&text)
}

pub fn load_json_array(path: &str) -> Result<Array, JsValue> {
    let value = load_json(path)?;
    value.dyn_into::<Array>()
}

pub fn load_json_object(path: &str) -> Result<Object, JsValue> {
    let value = load_json(path)?;
    value.dyn_into::<Object>()
}

pub fn load_html(url: &str) -> Result<String, JsValue> {
    XhrRequest::new(url, ResourceType::HTML).send()
}

pub fn load_text(url: &str) -> Result<String, JsValue> {
    XhrRequest::new(url, ResourceType::Text).send()
}

// Helper to clear cache
pub fn clear_cache() {
    if let Ok(mut cache) = RESOURCE_CACHE.lock() {
        cache.clear();
    }
}

// Helper functions to extract values from JS objects
pub fn get_string_from_obj(obj: &Object, key: &str) -> Option<String> {
    js_sys::Reflect::get(obj, &JsValue::from_str(key))
        .ok()
        .and_then(|val| val.as_string())
}

pub fn get_array_from_obj(obj: &Object, key: &str) -> Option<Array> {
    js_sys::Reflect::get(obj, &JsValue::from_str(key))
        .ok()
        .and_then(|val| val.dyn_into::<Array>().ok())
}

pub fn get_string_array_from_obj(obj: &Object, key: &str) -> Vec<String> {
    get_array_from_obj(obj, key)
        .map(|arr| {
            arr.to_vec()
                .into_iter()
                .filter_map(|v| v.as_string())
                .collect()
        })
        .unwrap_or_default()
}