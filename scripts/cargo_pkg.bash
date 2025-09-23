cargo add \
axum \
axum-extra -F typed-headers \
serde -F derive \
tokio -F macros,rt-multi-thread \
tower-http -F cors \
validator -F derive
