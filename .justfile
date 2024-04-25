test:
    echo This is a test

run-server:
    cargo run -- --server 127.0.0.1:1155

test-web:
    CONNECT=ws://localhost:1155 cargo geng run --release --platform web --index-file unused.html

update-server:
    docker run --rm -it -e CARGO_TARGET_DIR=/target -v `pwd`/docker-target:/target -v `pwd`:/src -w /src ghcr.io/geng-engine/cargo-geng cargo geng build --release
    rsync -avz docker-target/geng/ kuviman@bb:salmoning/
    ssh kuviman@bb systemctl --user restart salmoning
    ssh kuviman@bb 'rm -rf salmoning/save'

publish-web:
    bash ui/sync
    CONNECT=wss://salmoning.badcop.games cargo geng build --release --platform web --index-file unused.html
    butler -- push target/geng badcop/salmoning:html5

deploy:
    just update-server
    just publish-web
