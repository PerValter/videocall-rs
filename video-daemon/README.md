# Video Daemon Rust Client

## Setup

### Create your own certificate authority (ca.crt)

Make sure openssl is installed on your system

Make a directory for the certs
```sh
mkdir ~/certs
cd ~/certs
```

Generate a private key to generate certificates from

```sh
openssl genrsa -out ca.key 2048
```

Generate your root certificate

```sh
openssl req -x509 -new -nodes -key ca.key -sha256 -days 1825 -out ca.pem
```

Convert it to der format

```sh
openssl x509 -in ca.pem -out ca.der -outform DER
```