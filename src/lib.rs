
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn debug_print_string(s: &str) {
    log(s);
}

pub fn debug_print_jsvalue(v: &JsValue) {
    if v.is_string() {
        let v = v.as_string().unwrap();
        debug_print_string(&v);
        return;
    }
    
    if v.is_null() {
        debug_print_string("null");
        return;
    }

    let n = v.as_f64();
    if n.is_some() {
        let s = format!("{}", n.unwrap());
        debug_print_string(&s);
        return;
    }
}

#[macro_export]
macro_rules! CloudflareKV {
    ($ActualKV:tt, $KvUtil:tt) => {
        #[wasm_bindgen]
        extern "C" {
            type $ActualKV;
        
            #[wasm_bindgen(static_method_of = $ActualKV)]
            fn get(key: &JsValue) -> Promise;
        
            #[wasm_bindgen(static_method_of = $ActualKV)]
            fn list() -> Promise;
        }

        struct $KvUtil;

        impl $KvUtil {
            pub async fn get(key: &str) -> Option<String> {
                let key = JsValue::from(key);

                let promise = $ActualKV::get(&key);

                let result = JsFuture::from(promise).await;

                match result {
                    Ok(value) => value.as_string(),
                    _ => None,
                }
            }

            pub async fn list() -> Option<KvList> {
                let promise = $ActualKV::list();

                let result = JsFuture::from(promise).await;

                match result {
                    Ok(list) => {
                        let obj = Object::from(list);
            
                        let x: Result<KvList, _> = obj.into_serde();
                        return x.ok();
                    },
                    _ => {},
                };
            
                return None;
            }
        }
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KvKey {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KvList {
    pub keys: Vec<KvKey>,
    pub list_complete: bool,
    pub cursor: Option<String>,
}
