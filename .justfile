test:
    echo This is a test

run-server:
    cargo run -- --server 127.0.0.1:1155

test-web:
    CONNECT=ws://localhost:1155 cargo geng run --release --platform web --index-file unused.html

update-server:
    docker run --rm -it -e CARGO_TARGET_DIR=/target -v `pwd`/docker-target:/target -v `pwd`:/src -w /src ghcr.io/geng-engine/cargo-geng cargo geng build --release
    rsync -e 'ssh -p 22222' -avz docker-target/geng/ kuviman@salmoning.badcop.games:salmoning/
    ssh -p 22222 kuviman@salmoning.badcop.games systemctl --user restart salmoning

publish-web:
    cargo geng build --release --platform web --index-file unused.html
    bash ui/sync
    CONNECT=wss://salmoning.badcop.games cargo geng build --release --platform web --index-file unused.html
    butler -- push target/geng kuviman/salmoning:html5

migrate:
    scp -r ees@server.salmoning.kuviman.com:salmoning/save bb:salmoning

scores:
    ssh -p 22222 kuviman@salmoning.badcop.games 'cd salmoning/save; jq -r "[.money, .name] | @tsv" * | sort -n'

deploy:
    just update-server
    just publish-web
