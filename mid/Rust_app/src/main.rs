use serde::{Deserialize, Serialize};
use std::env;
use warp::Filter;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct HomeResponse {
    message: String,
    version: String,
    hostname: String,
    environment: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct HealthResponse {
    status: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct InfoResponse {
    rust_version: String,
    platform: String,
    working_directory: String,
    user: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct CalcResponse {
    result: f64,
    operation: String,
    x: f64,
    y: f64,
}

// Функция для вычислений
fn calculate(x: f64, y: f64, op: &str) -> Result<f64, String> {
    match op {
        "add" => Ok(x + y),
        "subtract" => Ok(x - y),
        "multiply" => Ok(x * y),
        "divide" => {
            if y == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(x / y)
            }
        }
        _ => Err(format!("Unknown operation: {}", op)),
    }
}

// Модульные тесты для функции calculate
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(calculate(2.0, 3.0, "add").unwrap(), 5.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(calculate(5.0, 3.0, "subtract").unwrap(), 2.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(calculate(2.0, 3.0, "multiply").unwrap(), 6.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(calculate(6.0, 2.0, "divide").unwrap(), 3.0);
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(calculate(6.0, 0.0, "divide").is_err());
    }

    #[test]
    fn test_unknown_operation() {
        assert!(calculate(6.0, 2.0, "power").is_err());
    }
}

#[tokio::main]
async fn main() {
    // Получаем переменные окружения
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let environment = env::var("ENV").unwrap_or_else(|_| "development".to_string());

    // Получаем имя хоста
    let hostname = hostname::get()
        .map(|h| h.into_string().unwrap_or_else(|_| "unknown".to_string()))
        .unwrap_or_else(|_| "unknown".to_string());

    // Эндпоинт: главная страница
    let home = warp::path::end()
        .map(move || {
            let response = HomeResponse {
                message: "Добро пожаловать в Rust-приложение в Docker!".to_string(),
                version: "1.0.0".to_string(),
                hostname: hostname.clone(),
                environment: environment.clone(),
            };
            warp::reply::json(&response)
        });

    // Эндпоинт: проверка здоровья
    let health = warp::path("health")
        .map(|| {
            let response = HealthResponse {
                status: "healthy".to_string(),
            };
            warp::reply::json(&response)
        });

    // Эндпоинт: информация о системе
    let info = warp::path("info")
        .map(|| {
            let response = InfoResponse {
                rust_version: "1.86".to_string(),
                platform: std::env::consts::OS.to_string(),
                working_directory: env::current_dir()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|_| "unknown".to_string()),
                user: env::var("USER").unwrap_or_else(|_| "unknown".to_string()),
            };
            warp::reply::json(&response)
        });

    // Эндпоинт: калькулятор
    let calc = warp::path("calc")
        .and(warp::path::param::<f64>())
        .and(warp::path::param::<String>())
        .and(warp::path::param::<f64>())
        .map(|x: f64, op: String, y: f64| {
            match calculate(x, y, &op) {
                Ok(result) => {
                    let response = CalcResponse {
                        result,
                        operation: op,
                        x,
                        y,
                    };
                    warp::reply::json(&response)
                }
                Err(err) => {
                    let error_response = serde_json::json!({
                        "error": err,
                        "x": x,
                        "y": y,
                        "operation": op
                    });
                    warp::reply::json(&error_response)
                }
            }
        });

    // Объединяем маршруты
    let routes = home.or(health).or(info).or(calc);

    println!("Запуск Rust-приложения на {}:{}", host, port);
    let ip: std::net::IpAddr = host.parse().expect("Invalid IP address");
    let addr = std::net::SocketAddr::new(ip, port);
    warp::serve(routes).run(addr).await;
}