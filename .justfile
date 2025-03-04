server := "salmoning.ldgames.kuviman.com"
server_user := "ees"

test:
    echo This is a test

run-server:
    cargo run -- --server 127.0.0.1:1155

test-web:
    CONNECT=ws://localhost:1155 cargo geng run --release --platform web --index-file unused.html

update-server:
    docker run --rm -it -e CARGO_TARGET_DIR=/target -v `pwd`/docker-target:/target -v `pwd`:/src -w /src ghcr.io/geng-engine/cargo-geng cargo geng build --release
    rsync -e 'ssh -p 22' -avz docker-target/geng/ {{server_user}}@{{server}}:salmoning/
    ssh -p 22 {{server_user}}@{{server}} systemctl --user restart salmoning

publish-web:
    cargo geng build --release --platform web --index-file unused.html
    bash ui/sync
    CONNECT=wss://{{server}} cargo geng build --release --platform web --index-file unused.html
    butler -- push target/geng kuviman/salmoning:html5

migrate:
    scp -r {{server_user}}@{{server}}:salmoning/save bb:salmoning

scores:
    ssh -p 22 {{server_user}}@{{server}} 'cd salmoning/save; jq -r "[.money, .name] | @tsv" * | sort -n'

deploy:
    just update-server
    just publish-web
