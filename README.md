# api.dataroot.ca
api.dataroot.ca: A Rust API using PostgreSQL, Redis, RabbitMQ, and Hexagonal Architecture

## Table of Contents

<!-- TOC -->

- [api.dataroot.ca](#api.dataroot.ca)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
  - [Usage](#usage)
  - [Requirements](#requirements)
  - [Installation](#installation)
  - [Author Information](#author-information)
  <!-- TOC -->

## Overview
The `api.dataroot.ca` project provides a backend API built with Rust that serves data and utilizes Redis for caching. It is designed to efficiently deliver data to the [dataroot.ca](https://dataroot.ca).


## Usage
The `api.dataroot.ca` API provides endpoints for retrieving data from various sources and serving it to clients. It utilizes Redis for caching to improve performance.

For detailed documentation on how to interact with the API, including endpoint details, request/response formats, and authentication, please refer to the [Swagger documentation](https://api.dataroot.ca/docs). The Swagger documentation provides comprehensive API reference and can assist you in understanding how to use the API effectively.


## Requirements
To run the `api.dataroot.ca` backend, you will need the following:

- Rust programming language (version 1.66.1)
- Dependencies specified in the `Cargo.toml` file

## Installation
Use docker-compose to start requirements resources

```bash
docker-compose up -d
```

Create a .env file with this default envs in env.example

```bash
cargo run
```

## Author Information

This module is maintained by the contributors listed on [GitHub](https://github.com/datarootca/api.dataroot.ca/graphs/contributors).

