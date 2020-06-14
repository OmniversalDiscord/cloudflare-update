# cloudflare-update

A command line utility for adding or removing subdomains on CloudFlare.

## Usage

Ensure a valid CloudFlare API token is set using environment variable `CLOUDFLARE_TOKEN`.

To add a subdomain:
```
cloudflare-update add <subdomain>
```

To remove a subdomain:
```
cloudflare-update remove <subdomain>
```

## Building

```
git clone https://github.com/OmniversalDiscord/cloudflare-update.git
cd cloudflare-update
cargo build --release
```