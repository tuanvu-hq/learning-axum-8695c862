cargo add \
axum -F macros \
axum-extra -F typed-headers \
chrono \
dotenvy \
dotenvy_macro \
jsonwebtoken \
dotenvy_macro \
sea-orm -F sqlx-postgres,runtime-tokio-rustls \
serde -F derive \
serde_with \
tokio -F macros,rt-multi-thread \
tower-http -F cors \
validator -F derive
