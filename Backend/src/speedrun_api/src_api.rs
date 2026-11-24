#![allow(dead_code)]

use std::collections::HashMap;

use crate::speedrun_api::src_cache::LEADERBOARD_CACHE;
use crate::speedrun_api::types::category::{Category, CategoryType};
use crate::speedrun_api::types::game::Game;
use crate::speedrun_api::types::leaderboard::Leaderboard;
use crate::speedrun_api::types::variable::{Variable, VariablesResponse};
use crate::speedrun_api::types::{self, category};
use crate::speedrun_api::{http_utils, src_cache};

const API_BASE_URL: &str = "https://www.speedrun.com/api/v1/";

pub async fn get_game(game_id: &str) -> Option<Game> {
    let cached_game = src_cache::GAME_CACHE.get(game_id);
    if cached_game.is_some() {
        return cached_game;
    }

    let str = format!("{}games/{}", API_BASE_URL, game_id);
    let result = http_utils::get_http_result(&str).await;

    let body = match result {
        Ok(body) => body,
        Err(err) => {
            println!("Failed to parse JSON: {}", err);
            return None;
        }
    };

    let result: Result<types::game::GameResponse, serde_json::Error> = serde_json::from_str(&body);
    let map = match result {
        Ok(parsed) => parsed,
        Err(err) => {
            println!("Failed to parse JSON: {}", err);
            println!("{}", body);
            return None;
        }
    };

    src_cache::GAME_CACHE.insert(&map.data);
    Some(map.data)
}

pub async fn get_category(category_id: &str) -> Option<Category> {
    let cached_category = src_cache::CATEGORY_CACHE.get(category_id);
    if cached_category.is_some() {
        return cached_category;
    }

    let str = format!("{}categories/{}", API_BASE_URL, category_id);
    let result = http_utils::get_http_result(&str).await;

    let body = match result {
        Ok(body) => body,
        Err(err) => {
            println!("Failed to parse JSON: {}", err);
            return None;
        }
    };

    let result: Result<types::category::CategoryResponse, serde_json::Error> =
        serde_json::from_str(&body);
    let map = match result {
        Ok(parsed) => parsed,
        Err(err) => {
            println!("Failed to parse JSON: {}", err);
            println!("{}", body);
            return None;
        }
    };

    src_cache::CATEGORY_CACHE.insert(&map.data);
    Some(map.data)
}

pub async fn get_all_categories_for_game(game: &str) -> Vec<category::Category> {
    let mut ret_val: Vec<category::Category> = Vec::new();

    let request_str = format!("{}games/{}/categories", API_BASE_URL, game);
    if src_cache::ALL_CATS_CACHE.contains_key(&request_str) {
        return src_cache::ALL_CATS_CACHE
            .get(&request_str)
            .unwrap()
            .to_vec();
    }

    let result = http_utils::get_http_result(&request_str).await;

    let body = match result {
        Ok(body) => body,
        Err(err) => {
            println!("HTTP request returned an error: {}", err);
            return Vec::new();
        }
    };

    let result: Result<types::category::CategoriesResponse, serde_json::Error> =
        serde_json::from_str(&body);
    match result {
        Ok(parsed) => {
            for var in parsed.data {
                ret_val.push(var);
            }
        }
        Err(err) => {
            println!("Failed to parse category response JSON: {}", err);
        }
    }

    src_cache::ALL_CATS_CACHE.insert(request_str, ret_val.clone());
    ret_val

    //let map = match result {
    //	Ok(parsed) => parsed,
    //	Err(err) => {
    //		println!("Failed to parse JSON: {}", err);
    //	}
    //};

    //println!("{:?}", map);
}

pub async fn get_leaderboard(
    game: &str,
    category: &str,
    variables: &HashMap<String, String>,
) -> Option<types::leaderboard::Leaderboard> {
    get_leaderboard_from_base_url(API_BASE_URL, game, category, variables).await
}

async fn get_leaderboard_from_base_url(
    base_url: &str,
    game: &str,
    category: &str,
    variables: &HashMap<String, String>,
) -> Option<types::leaderboard::Leaderboard> {
    let url = format!("{base_url}leaderboards/{game}/category/{category}");

    let args: HashMap<String, String> = variables
        .iter()
        .map(|(k, v)| (format!("var-{k}"), v.clone()))
        .collect();

    let cache_key = types::leaderboard::get_leaderboard_cache_key(game, category, variables);
    if let Some(cached_leaderboard) = LEADERBOARD_CACHE.get(&cache_key) {
        return Some(cached_leaderboard);
    }

    let result = http_utils::get_http_result_with_args(&url, args).await;

    let body = match result {
        Ok(body) => body,
        Err(err) => {
            println!("HTTP request returned an error: {}", err);
            return None;
        }
    };

    let result: Result<types::leaderboard::LeaderboardResponse, serde_json::Error> =
        serde_json::from_str(&body);
    let map = match result {
        Ok(parsed) => parsed,
        Err(err) => {
            println!("Failed to parse JSON: {}", err);
            println!("{}", body);
            return None;
        }
    };

    LEADERBOARD_CACHE.insert_with_key(cache_key, &map.data);
    Some(map.data)
}

pub fn get_subcategories_for_category(
    category_id: &str,
    variables: &Vec<Variable>,
) -> Vec<HashMap<String, String>> {
    let mut result: Vec<HashMap<String, String>> = Vec::new();

    for var in variables {
        if !var.is_subcategory {
            continue;
        }

        if !var.category.is_some() || !var.category.as_ref().unwrap().contains(category_id) {
            continue;
        }

        if var.id == "kn0k0d78" {
            continue;
        }

        for val in &var.values.values {
            let mut map = HashMap::new();
            map.insert(var.id.clone(), val.0.clone());
            result.push(map);
        }
    }

    result
}

pub async fn get_all_fullgame_leaderboards(game_id: &str) -> Vec<Leaderboard> {
    let mut result: Vec<Leaderboard> = Vec::new();

    let vars = get_variables_for_game(game_id).await;
    let categories = get_all_categories_for_game(game_id).await;
    for cat in categories {
        if let CategoryType::PerLevel = cat.category_type {
            continue; // Ignore level categories
        }

        let subcats = get_subcategories_for_category(&cat.id, &vars);

        if subcats.is_empty() {
            let vars = HashMap::new();
            if let Some(lb) = get_leaderboard(game_id, &cat.id, &vars).await {
                result.push(lb);
            }
        } else {
            for subcat_combo in &subcats {
                if let Some(lb) = get_leaderboard(game_id, &cat.id, subcat_combo).await {
                    result.push(lb);
                }
            }
        }
    }

    result
}

pub async fn get_user(user_id: &str) -> Option<types::user::User> {
    let cached_user = src_cache::USER_CACHE.get(user_id);
    if cached_user.is_some() {
        return cached_user;
    }

    let str = format!("{}users/{}", API_BASE_URL, user_id);
    let result = http_utils::get_http_result(&str).await;

    let body = match result {
        Ok(body) => body,
        Err(err) => {
            println!("Failed to parse JSON: {}", err);
            return None;
        }
    };

    let result: Result<types::user::UserResponse, serde_json::Error> = serde_json::from_str(&body);
    let map = match result {
        Ok(parsed) => parsed,
        Err(err) => {
            println!("Failed to parse JSON: {}", err);
            return None;
        }
    };

    src_cache::USER_CACHE.insert(&map.data);
    Some(map.data)
}

pub async fn get_variable(variable_id: &str) -> Option<Variable> {
    let cached_var = src_cache::VARIABLE_CACHE.get(variable_id);
    if cached_var.is_some() {
        return cached_var;
    }

    let str = format!("{}variables/{}", API_BASE_URL, variable_id);
    let result = http_utils::get_http_result(&str).await;

    let body = match result {
        Ok(body) => body,
        Err(err) => {
            println!("Failed to parse JSON: {}", err);
            return None;
        }
    };

    let result: Result<types::variable::VariableResponse, serde_json::Error> =
        serde_json::from_str(&body);
    let map = match result {
        Ok(parsed) => parsed,
        Err(err) => {
            println!("Failed to parse JSON: {}", err);
            println!("{}", body);
            return None;
        }
    };

    src_cache::VARIABLE_CACHE.insert(&map.data);
    Some(map.data)
}

pub async fn get_variables_for_game(game_id: &str) -> Vec<Variable> {
    let str = format!("{}games/{}/variables", API_BASE_URL, game_id);
    if src_cache::ALL_VARS_CACHE.contains_key(&str) {
        return src_cache::ALL_VARS_CACHE.get(&str).unwrap().to_vec();
    }

    let result = http_utils::get_http_result(&str).await;

    let body = match result {
        Ok(body) => body,
        Err(err) => {
            println!("Failed to parse JSON for variable HTTP result: {}", err);
            return Vec::new();
        }
    };

    let result: Result<VariablesResponse, serde_json::Error> = serde_json::from_str(&body);
    let map = match result {
        Ok(parsed) => parsed,
        Err(err) => {
            println!("Failed to parse JSON for game variables: {}", err);
            println!("{}", body);
            return Vec::new();
        }
    };

    src_cache::ALL_VARS_CACHE.insert(str, map.data.clone());
    map.data
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn test_get_leaderboard_caches_by_request_key_even_when_response_values_differ() {
        let payload = serde_json::json!({
            "data": {
                "weblink": "https://example.com/lb",
                "game": "game_be06",
                "category": "cat_be06",
                "level": null,
                "platform": null,
                "region": null,
                "emulators": null,
                "video-only": false,
                "timing": "realtime",
                "values": {
                    "extra_default_var": "default_val"
                },
                "runs": [],
                "links": []
            }
        })
        .to_string();

        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind mock server");
        let addr = listener.local_addr().expect("local addr");
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{payload}",
            payload.len()
        );

        let server_task = tokio::spawn(async move {
            // Serve exactly one HTTP request; a cache miss on the second call would fail.
            if let Ok((mut stream, _)) = listener.accept().await {
                let mut buf = [0u8; 4096];
                let _ = stream.read(&mut buf).await;
                let _ = stream.write_all(response.as_bytes()).await;
            }
        });

        let base_url = format!("http://{addr}/");
        let req_vars_1 = HashMap::from([
            ("var_b".to_string(), "val_b".to_string()),
            ("var_a".to_string(), "val_a".to_string()),
        ]);
        let req_vars_2 = HashMap::from([
            ("var_a".to_string(), "val_a".to_string()),
            ("var_b".to_string(), "val_b".to_string()),
        ]);

        let first =
            get_leaderboard_from_base_url(&base_url, "game_be06", "cat_be06", &req_vars_1).await;
        assert!(first.is_some());
        server_task.await.expect("mock server task finished");

        // Second request uses reverse insertion order and must hit cache despite differing response values
        let second =
            get_leaderboard_from_base_url(&base_url, "game_be06", "cat_be06", &req_vars_2).await;
        assert!(second.is_some());
        assert_eq!(second.unwrap().weblink, "https://example.com/lb");
    }
}
