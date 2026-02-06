// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::Path;
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::config::{ServerConfig, Version};
use crate::error::Error;
use crate::server::builder::ServerBuilder;

async fn graphiql(
    ide_title: axum::Extension<Option<String>>,
    path: Option<Path<String>>,
) -> impl axum::response::IntoResponse {
    let endpoint = if let Some(Path(path)) = path {
        format!("/graphql/{}", path)
    } else {
        "/graphql".to_string()
    };

    // Use a custom HTML with specific versions of React that are compatible
    let title = if let axum::Extension(Some(title)) = ide_title {
        title
    } else {
        "Mys GraphQL IDE".to_string()
    };

    // Custom GraphiQL HTML with React 17 (which doesn't use useInsertionEffect)
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="utf-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1" />
            <title>{}</title>
            <link
                rel="stylesheet"
                href="https://unpkg.com/graphiql@1.5.0/graphiql.min.css"
            />
        </head>
        <body style="margin: 0;">
            <div id="graphiql" style="height: 100vh;"></div>

            <script
                crossorigin
                src="https://unpkg.com/react@17.0.2/umd/react.production.min.js"
            ></script>
            <script
                crossorigin
                src="https://unpkg.com/react-dom@17.0.2/umd/react-dom.production.min.js"
            ></script>
            <script
                crossorigin
                src="https://unpkg.com/graphiql@1.5.0/graphiql.min.js"
            ></script>

            <script>
                const fetcher = GraphiQL.createFetcher({{ url: '{}' }});
                ReactDOM.render(
                    React.createElement(GraphiQL, {{ fetcher }}),
                    document.getElementById('graphiql')
                );
            </script>
        </body>
        </html>
        "#,
        title, endpoint
    );

    axum::response::Html(html)
}

pub async fn start_graphiql_server(
    server_config: &ServerConfig,
    version: &Version,
    cancellation_token: CancellationToken,
) -> Result<(), Error> {
    info!("Starting server with config: {:#?}", server_config);
    info!("Server version: {}", version);
    start_graphiql_server_impl(
        ServerBuilder::from_config(server_config, version, cancellation_token).await?,
        server_config.ide.ide_title.clone(),
    )
    .await
}

async fn start_graphiql_server_impl(
    server_builder: ServerBuilder,
    ide_title: String,
) -> Result<(), Error> {
    let address = server_builder.address();

    // Add GraphiQL IDE handler on GET request to `/`` endpoint
    let server = server_builder
        .route("/", axum::routing::get(graphiql))
        .route("/{version}", axum::routing::get(graphiql))
        .route("/graphql", axum::routing::get(graphiql))
        .route("/graphql/{version}", axum::routing::get(graphiql))
        .layer(axum::extract::Extension(Some(ide_title)))
        .build()?;

    info!("Launch GraphiQL IDE at: http://{}", address);

    server.run().await
}
