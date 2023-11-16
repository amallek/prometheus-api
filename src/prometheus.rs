use crate::constants::{APPLICATION_JSON, PROMETHEUS_CONFIG_FILE};
use actix_web::{HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<T>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub success: Option<bool>,
}

impl<T> Response<T> {
    pub fn new() -> Self {
        Self {
            results: None,
            result: None,
            error: None,
            success: Some(false),
        }
    }
}

type RestResult = Response<Config>;
type RestResultScrape = Response<ScrapeConfig>;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    global: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alerting: Option<Alerts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_files: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scrape_configs: Option<Vec<ScrapeConfig>>,
}

// Implement Clone
impl Clone for Config {
    fn clone(&self) -> Self {
        Self {
            global: self.global.clone(),
            alerting: self.alerting.clone(),
            rule_files: self.rule_files.clone(),
            scrape_configs: self.scrape_configs.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ScrapeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    job_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scrape_interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scrape_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    honor_timestamps: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metrics_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization: Option<Authorization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    follow_redirects: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_http2: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_config: Option<TlsConfig>,
    static_configs: Vec<StaticConfig>,
}

// Implement Clone
impl Clone for ScrapeConfig {
    fn clone(&self) -> Self {
        Self {
            job_name: self.job_name.clone(),
            scrape_interval: self.scrape_interval.clone(),
            scrape_timeout: self.scrape_timeout.clone(),
            scheme: self.scheme.clone(),
            honor_timestamps: self.honor_timestamps.clone(),
            metrics_path: self.metrics_path.clone(),
            authorization: self.authorization.clone(),
            follow_redirects: self.follow_redirects.clone(),
            enable_http2: self.enable_http2.clone(),
            static_configs: self.static_configs.clone(),
            tls_config: self.tls_config.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct StaticConfig {
    targets: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<BTreeMap<String, String>>,
}

// Implement Clone
impl Clone for StaticConfig {
    fn clone(&self) -> Self {
        Self {
            targets: self.targets.clone(),
            labels: self.labels.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TlsConfig {
    insecure_skip_verify: bool,
}

// Implement Clone
impl Clone for TlsConfig {
    fn clone(&self) -> Self {
        Self {
            insecure_skip_verify: self.insecure_skip_verify.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Authorization {
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials: Option<String>,
}

// Implement Clone
impl Clone for Authorization {
    fn clone(&self) -> Self {
        Self {
            r#type: self.r#type.clone(),
            credentials: self.credentials.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Alerts {
    alertmanagers: Vec<ScrapeConfig>,
}

// Implement Clone
impl Clone for Alerts {
    fn clone(&self) -> Self {
        Self {
            alertmanagers: self.alertmanagers.clone(),
        }
    }
}

#[derive(Serialize)]
struct Info {
    version: String,
}

#[get("/info")]
pub async fn info(_req: HttpRequest) -> HttpResponse {
    let info = Info {
        version: option_env!("CARGO_PKG_VERSION")
            .unwrap_or("NOT_FOUND")
            .to_string(),
    };
    HttpResponse::Ok().content_type(APPLICATION_JSON).json(info)
}

#[get("/config")]
pub async fn config_get(_req: HttpRequest) -> HttpResponse {
    let mut restresult = RestResult::new();

    let config = _config_get();
    if let Err(_) = config {
        restresult.error = Some("failed to read config file".to_string());
        return HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(restresult);
    }
    let config = config.unwrap();

    restresult.result = Some(config);
    restresult.success = Some(true);

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(restresult)
}

#[put("/endpoint")]
pub async fn endpoint_add(_req: HttpRequest, body: String) -> HttpResponse {
    let mut restresult = RestResult::new();

    let config = _config_get();
    if let Err(_) = config {
        restresult.error = Some("failed to read config file".to_string());
        return HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(restresult);
    }
    let mut config = config.unwrap();

    let order = serde_json::from_str::<ScrapeConfig>(body.as_str());
    if let Err(_) = order {
        restresult.error = Some("failed to parse JSON".to_string());
        return HttpResponse::ExpectationFailed()
            .content_type(APPLICATION_JSON)
            .json(restresult);
    }
    let order = order.unwrap();

    // add order to config
    if let None = config.scrape_configs {
        config.scrape_configs = Some(Vec::new());
    }
    let mut scrape_configs = config.scrape_configs.unwrap();

    // check of label_name already exists
    for i in 0..scrape_configs.len() {
        if scrape_configs[i].job_name == order.job_name {
            restresult.error = Some("job already exists".to_string());
            return HttpResponse::ExpectationFailed()
                .content_type(APPLICATION_JSON)
                .json(restresult);
        }
    }

    scrape_configs.push(order);
    config.scrape_configs = Some(scrape_configs);

    // Keep original config to restore in case of error
    let original_config = _config_get_raw().unwrap();

    if let Err(_) = _config_write(serde_yaml::to_string(&config).unwrap().as_str()) {
        restresult.error = Some("failed to write config file".to_string());
        return HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(restresult);
    }

    if let Err(_) = _prometheus_reload() {
        restresult.error = Some("failed to reload prometheus (recovered last version)".to_string());
        _config_write(original_config.as_str()).unwrap();
        return HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(restresult);
    }

    restresult.success = Some(true);

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(restresult)
}

#[get("/endpoint/{endpoint}")]
pub async fn endpoint_get(_req: HttpRequest) -> HttpResponse {
    let mut restresult = RestResultScrape::new();
    let job_name = Some(
        _req.match_info()
            .get("endpoint")
            .unwrap_or("unknown")
            .to_string(),
    );

    let config = _config_get();
    if let Err(_) = config {
        restresult.error = Some("failed to read config file".to_string());
        return HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(restresult);
    }

    let mut config = config.unwrap();

    // update config
    if let None = config.scrape_configs {
        config.scrape_configs = Some(Vec::new());
    }

    let scrape_configs = config.scrape_configs.unwrap();
    // check of label_name already exists
    for i in 0..scrape_configs.len() {
        if scrape_configs[i].job_name == job_name {
            restresult.result = Some(scrape_configs[i].clone());
            break;
        }
    }

    if restresult.result.is_none() {
        restresult.error = Some("job not found".to_string());
        return HttpResponse::ExpectationFailed()
            .content_type(APPLICATION_JSON)
            .json(restresult);
    }

    restresult.success = Some(true);

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(restresult)
}

#[post("/endpoint")]
pub async fn endpoint_update(_req: HttpRequest, body: String) -> HttpResponse {
    let mut restresult = RestResult::new();

    let config = _config_get();
    if let Err(_) = config {
        restresult.error = Some("failed to read config file".to_string());
        return HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(restresult);
    }

    let mut config = config.unwrap();

    let order = serde_json::from_str::<ScrapeConfig>(body.as_str());
    if let Err(_) = order {
        restresult.error = Some("failed to parse JSON".to_string());
        return HttpResponse::ExpectationFailed()
            .content_type(APPLICATION_JSON)
            .json(restresult);
    }

    let order = order.unwrap();

    // update config
    if let None = config.scrape_configs {
        config.scrape_configs = Some(Vec::new());
    }
    let mut scrape_configs = config.scrape_configs.unwrap();

    // check of label_name already exists
    let mut found = false;
    for i in 0..scrape_configs.len() {
        if scrape_configs[i].job_name == order.job_name {
            scrape_configs[i] = order;
            found = true;
            break;
        }
    }

    if !found {
        restresult.error = Some("job not found".to_string());
        return HttpResponse::ExpectationFailed()
            .content_type(APPLICATION_JSON)
            .json(restresult);
    }

    config.scrape_configs = Some(scrape_configs);

    // Keep original config to restore in case of error
    let original_config = _config_get_raw().unwrap();

    if let Err(_) = _config_write(serde_yaml::to_string(&config).unwrap().as_str()) {
        restresult.error = Some("failed to write config file".to_string());
        return HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(restresult);
    }

    if let Err(_) = _prometheus_reload() {
        restresult.error = Some("failed to reload prometheus (recovered last version)".to_string());
        _config_write(original_config.as_str()).unwrap();
        return HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(restresult);
    }

    restresult.success = Some(true);
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(restresult)
}

#[delete("/endpoint/{endpoint}")]
pub async fn endpoint_delete(_req: HttpRequest) -> HttpResponse {
    let mut restresult = RestResult::new();
    let job_name = Some(
        _req.match_info()
            .get("endpoint")
            .unwrap_or("unknown")
            .to_string(),
    );

    let config = _config_get();
    if let Err(_) = config {
        restresult.error = Some("failed to read config file".to_string());
        return HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(restresult);
    }

    let mut config = config.unwrap();

    // update config
    if let None = config.scrape_configs {
        config.scrape_configs = Some(Vec::new());
    }
    let mut scrape_configs = config.scrape_configs.unwrap();

    // check if label_name already exists
    let mut found = false;
    for i in 0..scrape_configs.len() {
        if scrape_configs[i].job_name == job_name {
            scrape_configs.remove(i);
            found = true;
            break;
        }
    }

    if !found {
        restresult.error = Some("job not found".to_string());
        return HttpResponse::ExpectationFailed()
            .content_type(APPLICATION_JSON)
            .json(restresult);
    }

    config.scrape_configs = Some(scrape_configs);

    // keep original config to restore in case of error
    let original_config = _config_get_raw().unwrap();

    if let Err(_) = _config_write(serde_yaml::to_string(&config).unwrap().as_str()) {
        restresult.error = Some("failed to write config file".to_string());
        return HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(restresult);
    }

    if let Err(_) = _prometheus_reload() {
        restresult.error = Some("failed to reload prometheus (recovered last version)".to_string());
        _config_write(original_config.as_str()).unwrap();
        return HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(restresult);
    }

    restresult.success = Some(true);
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(restresult)
}

fn _config_write(config: &str) -> Result<(), std::io::Error> {
    std::fs::write(PROMETHEUS_CONFIG_FILE, config)
}

fn _config_get_raw() -> Result<String, ()> {
    let yaml = std::fs::read_to_string(PROMETHEUS_CONFIG_FILE);
    if yaml.is_err() {
        return Err(());
    }
    Ok(yaml.unwrap())
}

fn _config_get() -> Result<Config, ()> {
    let yaml = _config_get_raw();
    if yaml.is_err() {
        return Err(());
    }
    let parsed = serde_yaml::from_str(yaml.unwrap().as_str());
    if let Err(_) = parsed {
        return Err(());
    }

    Ok(parsed.unwrap())
}

fn _prometheus_pid() -> Result<String, std::io::Error> {
    let pid = std::process::Command::new("pgrep")
        .arg("-x")
        .arg("prometheus")
        .output()
        .unwrap();
    if !pid.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get prometheus pid",
        ));
    }
    Ok(String::from_utf8_lossy(&pid.stdout).to_string())
}

fn _prometheus_reload() -> Result<(), std::io::Error> {
    let pid = _prometheus_pid();
    if let Err(e) = pid {
        return Err(e);
    }
    let status = std::process::Command::new("kill")
        .arg("-SIGHUP")
        .arg(pid.unwrap().trim())
        .status()
        .unwrap();
    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to reload prometheus",
        ));
    }
    Ok(())
}
