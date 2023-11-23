# toolshed

This repo contains a collection of crates that are shared between The Graph's network
services.

### Crates

* **toolshed:** A collection of rust modules that are shared between The Graph's network services.

    ```toml
    toolshed = { git = "https://github.com/edgeandnode/toolshed", tag = "v0.2.3" }
    ```
* **graphql:** A collection of GraphQL related Rust modules that are share between The Graph's network services.

    ```toml
    graphql = { git = "https://github.com/edgeandnode/toolshed", tag = "graphql-v0.2.0" }
    ```
* **graphql-http:** A _reqwest_ based GraphQL-over-HTTP client.

    ```toml
    graphql-http = { git = "https://github.com/eandn/toolshed", tag = "graphql-http-v0.1.1" }
    ```
