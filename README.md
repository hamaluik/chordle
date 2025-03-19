# chordle

A simple chores manager web application to keep track of your tasks, when you
last did them, and what chores you should be focussing on today.

## Building

To build the project, you will need to have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)

Once you have Rust installed, you can build the project by running:

```sh
cargo build --release
```

## Running

chordle is configured through several command line arguments. You can see the
full list of options by running:

```sh
$ chordle --help
```

Giving:

```plaintext
chordle 1.0.0-alpha.1
by Kenton Hamaluik <kenton@hamaluik.ca>
A simple button-based chore tracker

Usage: chordle [OPTIONS]

Options:
  -c, --colour <COLOUR>
          Control whether color is used in the output

          [env: COLOUR=]
          [default: auto]
          [possible values: auto, always, never]

  -v, --verbose...
          Enable debugging output

          Use multiple times to increase verbosity (e.g., -v, -vv, -vvv):

          [env: VERBOSE=]

  -b, --bind <BIND>
          The address to bind to in the form of <host>:<port>

          To listen on all interfaces, use `0.0.0.0:<port>`

          [env: BIND=]
          [default: 127.0.0.1:8080]

  -s, --sqlite-db <SQLITE_DB>
          The path to the SQLite database file

          This file will be created if it does not exist

          [env: SQLITE_DB=]
          [default: chordle.db]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Systemd Service

If you want to run chordle as a service on a Linux system, you can use the
following systemd service file:

```systemd
[Unit]
Description=Lich Service
After=network.target

[Service]
Environment="BIND=0.0.0.0:8080"
Environment="SQLITE_DB=/path/to/chordle.db"
ExecStart=/path/to/chordle
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

Replace `/path/to/chordle` with the path to the chordle binary, and
`/path/to/chordle.db` with the path to the SQLite database file you want to use.

Save this file as `/etc/systemd/system/chordle.service`, then run:

```sh
sudo systemctl daemon-reload
sudo systemctl enable --now chordle
```

This will start the chordle service and enable it to start on boot. You can
check the status of the service by running:

```sh
sudo systemctl status chordle
```

You can also view the logs of the service by running:

```sh
sudo journalctl -u chordle
```

### Nginx Reverse Proxy Configuration

If you want to run chordle behind an Nginx reverse proxy, you can use the
following configuration:

```nginx
server {
    listen 80;
    server_name chordle.mydomain.com;

    # permanent redirect to https
    return 301 https://$host$request_uri;
}

server {
    listen 443;
    server_name chord.mydomain.com;

    error_log /var/log/nginx/chordle.mydomain.com-error.log;
    access_log /var/log/nginx/chordle.mydomain.com-access.log;

    client_max_body_size 1024M;

    ssl_certificate /etc/nginx/ssl/chordle.mydomain.com.crt;
    ssl_certificate_key /etc/nginx/ssl/chordle.mydomain.com.key;

    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_prefer_server_ciphers on;
    ssl_ciphers HIGH:!aNULL:!MD5;

    location / {
        proxy_pass http://localhost:8080;
        proxy_ssl_session_reuse off;
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
        proxy_redirect off;
    }
}
```

### Docker

A docker image for chordle is available on ghcr.io. You can run it with the
following command:

```sh
docker run -d --name chordle -p 8080:8080 -v /path/to/data:/data ghcr.io/hamaluik/chordle:latest --bind 0.0.0.0:8080 --sqlite-db /data/chordle.db
```

Or the following docker-compose service:

```yaml
---
services:
  chordle:
    container_name: chordle
    image: ghcr.io/hamaluik/chordle:latest
    restart: unless-stopped
    environment:
      - PUID=1000
      - PGID=1000
      - TZ=America/Edmonton
      - BIND=0.0.0.0:7777
      - SQLITE_DB=/data/chordle.db
    command: -v
    ports:
      - '7777:7777'
    volumes:
      - ./data:/data
```


## License

This project is licensed under the Apache-2.0 license, see the [LICENSE](LICENSE)
file for more information.

