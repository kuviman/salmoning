test:
    echo This is a test

update-server:
    docker run --rm -it -e CARGO_TARGET_DIR=/target -v `pwd`/docker-target:/target -v `pwd`:/src -w /src ghcr.io/geng-engine/cargo-geng cargo geng build --release
    rsync -avz docker-target/geng/ ees@ees.kuviman.com:salmoning/
    ssh ees@server.salmoning.kuviman.com systemctl --user restart salmoning

publish-web:
    CONNECT=wss://server.salmoning.kuviman.com cargo geng build --release --platform web
    nix run nixpkgs#butler -- push target/geng kuviman/salmoning:html5

deploy:
    just update-server
    just publish-web