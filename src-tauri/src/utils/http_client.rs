use reqwest::Client;
use serde_json::Value;
use crate::models::token_price::TokenPrice;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const PYTH_URL_PREFIX:&str = "https://hermes.pyth.network/v2/";

pub async fn get_pyth_price(symbols: &[String]) -> Result<Vec<TokenPrice>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut price_feed_ids: Vec<String> = Vec::new();
    let mut id_to_symbol: HashMap<String, String> = HashMap::new();

    // ------------------------
    // ① 查询 symbol 对应的 feedID
    // ------------------------
    for sym in symbols {
        let query_symbol = format!("Crypto.{}/USD", sym.to_uppercase());
        println!("query_symbol = {}", query_symbol);
        // 添加错误处理和日志
        let resp_text = match client
            .get(format!("{}price_feeds", PYTH_URL_PREFIX))
            .query(&[("asset_type", "crypto")])
            .send()
            .await
        {
            Ok(response) => {
                match response.text().await {
                    Ok(text) => text,
                    Err(e) => {
                        eprintln!("[ERROR] 解析响应文本失败: {}", e);
                        continue;  // 跳过这个 symbol，继续下一个
                    }
                }
            }
            Err(e) => {
                eprintln!("[ERROR] 请求 feedID 失败 ({}): {}", sym, e);
                continue;
            }
        };

        println!("[DEBUG] 查询 {} 的 resp_text 长度: {}", sym, resp_text.len());
        // println!("resp_text = {}", resp_text);
        
        // 安全解析 JSON
        let json_array: Vec<Value> = match serde_json::from_str(&resp_text) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("[ERROR] JSON 解析失败: {}", e);
                eprintln!("[DEBUG] 原始响应: {}", &resp_text[..200.min(resp_text.len())]);
                continue;
            }
        };

        println!("json_array .length = {}", json_array.len());
        //遍历数组
        for item in json_array {
            println!("item ={}", item);
            println!("attributes = {:?}, id= {:?}", item.get("attributes"), item.get("id"));
            // 使用模式匹配安全解包
            match (item.get("attributes"), item.get("id")) {
                (Some(attr), Some(id_val)) => {
                    if let Some(symbol_val) = attr.get("symbol").and_then(|v| v.as_str()) {
                        println!("symbol_val = {}, query_symbol = {}", symbol_val, query_symbol);
                        if symbol_val == query_symbol {
                            if let Some(id_str) = id_val.as_str() {
                                price_feed_ids.push(id_str.to_string());
                                id_to_symbol.insert(id_str.to_string(), sym.clone());
                                println!("[INFO] 找到 {} 的 feedID: {}", sym, id_str);
                            }
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    // 如果没有要查的 feedID → 直接返回空列表
    if price_feed_ids.is_empty() {
        eprintln!("[WARN] 没有找到任何 feedID");
        return Ok(vec![]);
    }

    println!("[INFO] 准备查询价格，feedIDs: {:?}", price_feed_ids);

    // ------------------------
    // ② 拼接 HTTP query 参数
    // ------------------------
    let mut params: Vec<(String, String)> = price_feed_ids
        .iter()
        .map(|id| ("ids[]".to_string(), id.clone()))
        .collect();
    params.push(("parsed".to_string(), "true".to_string()));

    // ------------------------
    // ③ 请求最新价格
    // ------------------------
    let resp_text = match client
        .get( format!("{}updates/price/latest", PYTH_URL_PREFIX))
        .query(&params)
        .send()
        .await
    {
        Ok(response) => {
            // 先检查状态码
            if !response.status().is_success() {
                eprintln!("[ERROR] HTTP 状态码: {}", response.status());
            }
            match response.text().await {
                Ok(text) => {
                    println!("[DEBUG] 价格响应长度: {}", text.len());
                    // 只打印前500字符避免刷屏
                    // if text.len() > 500 {
                    //     println!("[DEBUG] 价格响应(前500字符): {}", &text[..500]);
                    // } else {
                        println!("[DEBUG] 价格响应: {}", text);
                    // }
                    text
                }
                Err(e) => {
                    eprintln!("[ERROR] 读取价格响应失败: {}", e);
                    return Err(Box::new(e));
                }
            }
        }
        Err(e) => {
            eprintln!("[ERROR] 请求价格失败: {}", e);
            return Err(Box::new(e));
        }
    };


    // 安全解析 JSON
    let json: Value = match serde_json::from_str(&resp_text) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[ERROR] 价格 JSON 解析失败: {}", e);
            eprintln!("[DEBUG] 错误位置附近的文本: {}", &resp_text);
            return Err(Box::new(e));
        }
    };

    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() as i64,
        Err(e) => {
            eprintln!("[ERROR] 获取当前时间失败: {}", e);
            return Err(Box::new(e));
        }
    };

    let mut token_price_list: Vec<TokenPrice> = Vec::new();

    // 安全访问 parsed 数组
    let Some(feed_arr) = json.get("parsed").and_then(|v| v.as_array()) else {
        eprintln!("[WARN] 响应中没有 parsed 数组, json= {}", json);
        return Ok(vec![]);
    };

    println!("feed_arr = {:?}", feed_arr);
    for feed in feed_arr {
        // 使用模式匹配一次性安全解包所有字段
     match (
    feed.get("id").and_then(|v| v.as_str()),
    feed.get("price"),
) {
    (Some(id), Some(price_info)) => {
        match (
            price_info.get("price").and_then(|v| v.as_str()), 
            price_info.get("expo").and_then(|v| v.as_i64()),
            id_to_symbol.get(id),
        ) {
            (Some(price_str), Some(expo), Some(symbol)) => {
                // 将字符串价格解析为数字
                match price_str.parse::<i64>() {
                    Ok(price) => {
                        token_price_list.push(TokenPrice {
                            symbol: symbol.clone(),
                            usd: price,
                            expo,
                            updated_at: now,
                        });
                        
                        // 计算实际价格用于显示
                        let actual_price = price as f64 * 10_f64.powi(expo as i32);
                        println!("[INFO] 获取到价格: {} = {} (原始: {}e{})", 
                            symbol, actual_price, price, expo);
                    }
                    Err(e) => {
                        eprintln!("[WARN] 价格解析失败: id={}, price_str={}, error={}", 
                            id, price_str, e);
                        continue;
                    }
                }
            }
            _ => {
                eprintln!("[WARN] 价格数据不完整: id={}", id);
                continue;
            }
        }
    }
    _ => continue,
}
    }

    println!("[INFO] 成功获取 {} 个价格", token_price_list.len());
    Ok(token_price_list)
}