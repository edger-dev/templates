_list:
    cd {{ justfile_directory() }} ; just -l
_run_package bin package-name output:
    cd {{ invocation_directory() }} ; \
    cargo run --manifest-path {{ justfile_directory() }}/Cargo.toml \
        --bin {{ bin }} -- \
        --package-name {{ package-name }} --output {{ output }}
_run_config bin config output:
    cd {{invocation_directory()}} ; \
    cargo run --manifest-path {{justfile_directory()}}/Cargo.toml \
        --bin {{ bin }} -- \
        --config {{ config }} --output {{ output }}
rust_template package-name output: (_run_package "rust_template" package-name output)
rust_lib package-name output: (_run_package "rust_lib" package-name output)
k8s_clash config output: (_run_config "k8s_clash" config output)
k8s_minio config output: (_run_config "k8s_minio" config output)
k8s_plantuml config output: (_run_config "k8s_plantuml" config output)
k8s_languagetool config output: (_run_config "k8s_languagetool" config output)
k8s_libretranslate config output: (_run_config "k8s_libretranslate" config output)

