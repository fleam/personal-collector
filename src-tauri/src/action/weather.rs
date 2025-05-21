// src-tauri/src/action/weather.rs

use crate::model::weather::WeatherResponse;
use reqwest;
use serde_json;

#[tauri::command]
pub async fn get_weather_for_city(city_code: String) -> Result<WeatherResponse, String> {
    let url = format!("http://t.weather.sojson.com/api/weather/city/{}", city_code);
    println!("{} {}", "url==>", url);
    let client = reqwest::Client::new();
    let res = client.get(&url).send().await.map_err(|e| e.to_string())?;

    println!("判断状态");
    if !res.status().is_success() {
        println!("{}", "请求失败");
        return Err("HTTP 请求失败".to_string());
    }

    println!("开始解析");
    let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    println!("{} {}", "解析成功", json);

    if json["status"] != 200 {
        println!("{}", "未知错误");
        return Err(json["message"].as_str().unwrap_or("未知错误").to_string());
    }

    // 手动解析 JSON 到结构体
    // let response: WeatherResponse = serde_json::from_value(json).map_err(|e| e.to_string())?;
    // 手动解析 JSON 到结构体
    let response: WeatherResponse = serde_json::from_value(json).map_err(|e| {
        println!("WeatherResponse 转换失败: {:?}", e);
        e.to_string()
    })?;
    println!("{}", "返回成功");

    Ok(response)
}
