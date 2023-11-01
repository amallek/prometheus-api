# Prometheus API


## Overview
Lightweight Prometheus API serves as a powerful tool for managing your Prometheus configuration remotely. This code was developed as part of one of our latest projects, and I've decided to publish it because I find it incredibly useful. Written in Rust, the project offers basic authentication and various API endpoints for versatile management of your Prometheus setup.

## Features

- **Flexible Endpoints**: Numerous API calls available for managing Prometheus settings.
- **Basic Authentication**: Secure your Prometheus configurations with a configurable `x-auth` token.
- **Rust-powered**: Built with the speed and safety of Rust.

## Prerequisites

- Rust (latest stable version)
- Prometheus installed and running

## Getting started

To get started, follow the instructions below.

##### Step 1 - Clone this repository

```shell
git clone https://github.com/amallek/prometheus-api.git
```
##### Step 2 - Navigate to the project directory

```shell
cd prometheus-api
```

##### Step 3 - Build the project
```shell
# Uncomment the next line if Rust is not installed
# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo build --release
```

##### Step 4 - Run the application
```shell
# Uncomment to generate self-signed TLS certs
# ./helpers/create-cert.sh
./target/release/prometheus-api
```

##### Step 5 - Install as a service (optional)

If desired, you can install prometheus-api as a service. An example script is available in helpers/create-service.sh.

```shell
# Uncomment the next line to execute the script
# ./helpers/create-service.sh
systemctl start prometheus-api
```


## 🛠 API Endpoints

The API provides a range of functionalities through the following endpoints:

### `prometheus::info()`

- **Endpoint**: `GET /info`
- **Description**: Retrieves basic information about the Prometheus API 
- **Authentication**: Required (`x-auth` token)
  
### `prometheus::config_get()`

- **Endpoint**: `GET /config`
- **Description**: Fetches the current `prometheus.yml` configuration in JSON format.
- **Authentication**: Required (`x-auth` token)

### `prometheus::endpoint_get()`

- **Endpoint**: `GET /endpoint/{endpoint}`
- **Description**: Obtains details about a specified Prometheus scraping endpoint.
- **Authentication**: Required (`x-auth` token)

### `prometheus::endpoint_add()`

- **Endpoint**: `PUT /endpoint`
- **Description**: Adds a new scraping endpoint to the Prometheus configuration.
- **Payload**: JSON object with endpoint details.
- **Authentication**: Required (`x-auth` token)

### `prometheus::endpoint_delete()`

- **Endpoint**: `DELETE /endpoint/{endpoint}`
- **Description**: Removes a specified scraping endpoint from the Prometheus configuration.
- **Authentication**: Required (`x-auth` token)

### `prometheus::endpoint_update()`

- **Endpoint**: `POST /endpoint`
- **Description**: Updates an existing scraping endpoint in the Prometheus configuration.
- **Payload**: JSON object with updated endpoint details.
- **Authentication**: Required (`x-auth` token)

The payload required consists of literally what you would see in the prometheus.yaml file, but JSON encoded. For instance:

##### Add

```shell
curl -ikX PUT \
    'https://localhost:9090/endpoint' \
    -H 'x-auth: my_password' \
    -d '{
 "job_name": "my-job-name",
 "scrape_interval": "60",
 "scrape_timeout": "5",
 "scheme": "https",
 "static_configs": [
    {
        "targets": ["my.machine.example.com:9091"],
        "labels": {"label1":"val","label2":"val"}
    }
 ]
}'
#{"success":true}
```

##### Get

```shell
curl -ik 'https://localhost:9090/endpoint/my-job-name' \
    -H 'x-auth: my_password'
```

##### Remove

```shell
curl -ikX DELETE \
    'https://localhost:9090/endpoint/my-job-name' \
    -H 'x-auth: my_password'
```

## Authentication

Include an `x-auth` token in every API call to ensure authentication. The token can be updated or replaced by custom logic in the `verify_token()` method (see `src/security.rs`).

## License

This project is licensed under the terms of the MIT License (see LICENSE).

