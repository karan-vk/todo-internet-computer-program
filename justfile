start: candid-export
    dfx start --clean --background
    dfx deploy

stop:
    dfx stop

build: candid-export
    dfx build

upgrade: build
    dfx canister install --all --mode upgrade

candid-export:
    cargo build --release --target wasm32-unknown-unknown --package todo_backend
    candid-extractor target/wasm32-unknown-unknown/release/todo_backend.wasm > src/todo_backend/todo_backend.did

_call := "dfx canister call todo_backend"

call METHOD *ARGS:
    {{ _call }} {{ METHOD }} {{ ARGS }}

add text:
    {{ _call }} add '{{ text }}'

get id:
    {{ _call }} get '{{ id }}'

list:
    {{ _call }} list

update id text:
    {{ _call }} update '({{ id }}, "{{ text }}")'

delete id:
    {{ _call }} delete '{{ id }}'

mark-complete id:
    {{ _call }} mark_complete '{{ id }}'

mark-incomplete id:
    {{ _call }} mark_incomplete '{{ id }}'
