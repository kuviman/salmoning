test:
    echo This is a test

run-server:
    cargo run -- --server 127.0.0.1:1155

test-web:
    CONNECT=ws://localhost:1155 cargo geng run --release --platform web --index-file unused.html

update-server:
    docker run --rm -it -e CARGO_TARGET_DIR=/target -v `pwd`/docker-target:/target -v `pwd`:/src -w /src ghcr.io/geng-engine/cargo-geng cargo geng build --release
    rsync -avz docker-target/geng/ ees@ees.kuviman.com:salmoning/
    ssh ees@server.salmoning.kuviman.com systemctl --user restart salmoning

publish-web:
    bash ui/sync
    CONNECT=wss://server.salmoning.kuviman.com cargo geng build --release --platform web --index-file unused.html
    nix run nixpkgs#butler -- push target/geng kuviman/salmoning:html5

deploy:
    just update-server
    just publish-web
