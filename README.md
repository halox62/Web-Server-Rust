# Web-Server-Rust


<table align="center">
  <tr>
    <td><img src="./images/logoWebServer.png" alt="logo" width="120"/></td>
    <td>
      <b>WEB SERVER extensible with secure, signed WebAssembly plugins.</b><br>
      WaspEdge is a web server written in Rust that allows you to add custom logic via WebAssembly (WASM) plugins, safely, isolated, and dynamically.<br>
      You can write extensions in Rust, C, Go, or AssemblyScript and upload them without restarting the server.
    </td>
  </tr>
</table>

![schema](./images/schema.png)

# create plugin
cargo new hello_plugin --lib
cd hello_plugin

# compila
rustup target add wasm32-unknown-unknown

cargo build --release --target wasm32-unknown-unknown

cp target/wasm32-unknown-unknown/release/hello_plugin.wasm ../rust_web/plugins/

# wasmsign2
cargo install wasmsign2-cli

# gen chiave
wasmsign2 keygen --public-key public.key --secret-key secret.key

# firmare il plugin:
wasmsign2 sign --input-file plugin.wasm --output-file plugin-signed.wasm --secret-key secret.key


