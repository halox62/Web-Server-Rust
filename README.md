# Web-Server-Rust


<table align="center">
  <tr>
    <td><img src="./images/logoWebServer.png" alt="logo" width="250"/></td>
    <td>
      <b>WEB SERVER extensible with secure, signed WebAssembly plugins.</b><br>
      WaspEdge is a web server written in Rust that allows you to add custom logic via WebAssembly (WASM) plugins, safely, isolated, and dynamically.<br>
      You can write extensions in Rust, C, Go, or AssemblyScript and upload them without restarting the server.
    </td>
  </tr>
</table>

# Architecture

![schema](./images/schema.png)


# Configurazione
config.yaml:

server:
  enable_http: false/true
  enable_ws: false/true
  enable_quic: false/true
  http_port: port http
  ws_port: port webSocket
  quic_port: port quic
  cert_path: "certification"
  key_path: "key"


routes:
  - path: "/"
    upstream: "url"
    cache: false/true
    plugins:
      - name: "name plugins"
        options: {options}



# Start server
cargo run

# create plugin
cargo new hello_plugin --lib
cd hello_plugin

# compila
rustup target add wasm32-unknown-unknown

cargo build --release --target wasm32-unknown-unknown

cp target/wasm32-unknown-unknown/release/hello_plugin.wasm ../rust_web/plugins/

# Installa lo strumento di firma
cargo install wasmsign2-cli

# Genera una nuova coppia di chiavi (pubblica e privata)
wasmsign2 keygen --public-key public.key --secret-key secret.key

# Firma il file .wasm generato con la chiave privata
wasmsign2 sign \
  --input-file hello_plugin.wasm \
  --output-file hello_plugin-signed.wasm \
  --secret-key secret.key


