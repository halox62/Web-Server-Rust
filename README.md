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


# Project Structure
```
rust_web/
├── Cargo.toml        
├── keys/                   
│   ├── trusted_keys.json
|   ├── public.key
├── plugins/                   
│   ├── plugin.wasm 
├── config.yaml                
└── README.md         

```


# Configurazione
config.yaml:
```yaml
server:
  enable_http: false
  enable_ws: false
  enable_quic: false
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

```


# Start server
```rust
cargo run
```


# Install plugin
1. download plugin with public key
2. move the plugin in to plugins folder
3. move the public key in to keys folder
4. insert the path key in to file json trusted_key.json


# Create plugin
```rust
cargo new plugin --lib
cd plugin
```

# Compile plugin
```rust
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/hello_plugin.wasm ../rust_web/plugins/
```

# Install signature tool
```rust
cargo install wasmsign2-cli
```

# Generate key
```rust
wasmsign2 keygen --public-key public.key --secret-key secret.key
```

# Signature plugin with private key
```rust
wasmsign2 sign \
  --input-file plugin.wasm \
  --output-file plugin-signed.wasm \
  --secret-key secret.key
```


