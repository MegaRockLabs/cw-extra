RUST_BACKTRACE=1 docker run --rm -v "$(pwd)":/code  \
    --mount type=volume,source="$(basename "$(pwd)")_cach",target=/target  \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry  \
    cosmwasm/workspace-optimizer:0.14.0