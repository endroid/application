# Certificates

Execute the following commands on the host (not within the container)

```bash
brew install mkcert
mkcert -install
mkcert -cert-file localhost.pem -key-file localhost-key.pem localhost
chmod 644 localhost.pem localhost-key.pem
```
