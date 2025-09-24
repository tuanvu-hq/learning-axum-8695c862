cargo add \
axum \
axum-extra -F typed-headers \
dotenvy \
dotenvy_macro \
sea-orm -F sqlx-postgres,runtime-tokio-rustls \
serde -F derive \
tokio -F macros,rt-multi-thread \
tower-http -F cors \
validator -F derive
