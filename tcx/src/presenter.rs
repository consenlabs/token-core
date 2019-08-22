use serde_json::json;
use serde_json::{Map, Value};
use tcx_chain::{Account, HdKeystore};

use crate::Result;

pub trait Presenter {
    fn present(&self) -> Result<String>;
    //    fn present_with_account(&self) -> String;
}

fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

fn _presented_wallet(keystore: &HdKeystore) -> Value {
    let mut pw = Map::new();
    pw.insert("id".to_string(), json!(keystore.id.to_string()));
    pw.insert("name".to_string(), json!(keystore.meta.name));
    pw.insert(
        "passwordHint".to_string(),
        json!(keystore.meta.password_hint),
    );
    pw.insert("createdAt".to_string(), json!(keystore.meta.timestamp));
    pw.insert("source".to_string(), json!(keystore.meta.source));

    if !keystore.active_accounts.is_empty() {
        if keystore.active_accounts.len() > 1 {
            panic!("Only one accounts in token 2.5")
        }
        let acc = keystore.active_accounts.first().unwrap();
        pw.insert("address".to_string(), json!(acc.address.to_string()));
        pw.insert("chainType".to_string(), json!(acc.coin.to_string()));
        let mut obj = Value::Object(pw);
        if let Some(extra) = acc.extra.as_object() {
            merge(&mut obj, &Value::Object(extra.clone()))
        }
        obj
    } else {
        Value::Object(pw)
    }
}

impl Presenter for HdKeystore {
    fn present(&self) -> Result<String> {
        let mut pw = Map::new();
        pw.insert("id".to_string(), json!(&self.id.to_string()));
        pw.insert("name".to_string(), json!(&self.meta.name));
        pw.insert("passwordHint".to_string(), json!(&self.meta.password_hint));
        pw.insert("createdAt".to_string(), json!(&self.meta.timestamp));
        pw.insert("source".to_string(), json!(&self.meta.source));

        if !&self.active_accounts.is_empty() {
            if self.active_accounts.len() > 1usize {
                return Err(format_err!("Only one account in token 2.5"));
            }
            let acc = &self.active_accounts.first().unwrap();
            println!("has account");
            pw.insert("address".to_string(), json!(acc.address.to_string()));
            pw.insert("chainType".to_string(), json!(acc.coin.to_string()));
            let mut obj = Value::Object(pw);
            if let Some(extra) = acc.extra.as_object() {
                println!("try to merge");
                merge(&mut obj, &Value::Object(extra.clone()))
            }
            return serde_json::to_string(&obj)
                .map_err(|_| format_err!("present err when convert to json"));
        } else {
            return serde_json::to_string(&pw)
                .map_err(|_| format_err!("present err when convert to json"));
        }
    }
}
