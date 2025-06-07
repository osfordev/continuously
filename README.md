# Continuously Backup

## Quick Start

```shell
# Expected Cargo version 1.83
cargo --version

cargo build
cargo test

cargo build --target x86_64-unknown-linux-gnu

# Dry-run via Cargo
cargo run --package continuously_backup_bin -- --help

# Dry-run directly
./target/x86_64-unknown-linux-gnu/debug/continuously --help

# Real copy session sample: File System --coping--> Google Cloud Storage
./target/x86_64-unknown-linux-gnu/debug/continuously \
  copy-session create \
  --source-file=/dev/vg0/luks-drone-20250106 \
  --destination-google-cloud-bucket-name=logical-volume-backups \
  --destination-google-cloud-object-name=dg01/vg0/luks-drone/20250106 \
  --destination-google-cloud-mime-type=application/octet-stream \
  --destination-google-cloud-service-account-json-file=/etc/continuously/absolute-garden-272819-d3d737919552.local.json

# Real backup all LUKS volumes (snapshot) from some server. Format: /dev/vg0/luks-home-bootsnap
HOSTNAME=$(hostname --short)
NOW_YYYYMMDD=$(date --utc +"%Y%m%d")
for VG_NAME in $(vgs --noheadings -o vg_name | tr -d '  '); do
  for LV_NAME in $(lvs --noheadings -o lv_name "${VG_NAME}" | tr -d '  ' | grep bootsnap | sed 's/-bootsnap$//g'); do
    ./target/x86_64-unknown-linux-gnu/debug/continuously \
      copy-session create \
      "--source-file=/dev/${VG_NAME}/${LV_NAME}-bootsnap" \
      --destination-google-cloud-bucket-name=logical-volume-backups \
      "--destination-google-cloud-object-name=${HOSTNAME}/${VG_NAME}/${LV_NAME}/${NOW_YYYYMMDD}" \
      --destination-google-cloud-mime-type=application/octet-stream \
      --destination-google-cloud-service-account-json-file=/etc/continuously/absolute-garden-272819-d3d737919552.local.json
  done
done
for VG_NAME in $(vgs --noheadings -o vg_name | tr -d '  '); do
  for LV_NAME in $(lvs --noheadings -o lv_name "${VG_NAME}" | tr -d '  ' | grep bootsnap | sed 's/-bootsnap$//g'); do
    lvremove "/dev/${VG_NAME}/${LV_NAME}-bootsnap"
  done
done
```

## Components

```text
                          ┌──────────────────────┐                           
                          │  Continuously Backup │                           
                          │     Rust Library     │                           
                          └────▲──────▲─────▲────┘                           
                               │      │     │                                
              ┌────────────────┴────┐ │ ┌───┴─────────────────┐              
              │ Continuously Backup │ │ │ Continuously Backup │              
              │  Shared Library .so │ │ │      Rust Cli       │              
              └─────▲───────────────┘ │ └─────────────▲───────┘              
                    │                 │               │                      
 ┌──────────────────┴───────┐  ┌──────┴───────┐  ┌────┴-───────────────────┐ 
 │                          │  │              │  │                         │ 
 │  Other Language Clients  │  │ Rust Clients │  │ Shell/Scripting Clients │ 
 │                          │  │              │  │                         │ 
 └──────────────────────────┘  └──────────────┘  └─────────────────────────┘ 
                                                                             
The diagram was created in https://asciiflow.com/
```

## References

- [Google Cloud Storage OAuth 2.0 scopes](https://cloud.google.com/storage/docs/oauth-scopes)
- [Google Cloud Storage Resumable uploads](https://cloud.google.com/storage/docs/resumable-uploads)

## Developer Notes

### Workstation Requirements

- [ ] [rustup](https://rust-lang.github.io/rustup/index.html)

### IDE

#### VSCode

- [Rust in Visual Studio Code](https://code.visualstudio.com/docs/languages/rust)

### Use Cargo(Rust) via Docker

```shell
alias cargo='docker run --rm --env USER="${USER}" --user "$(id -u)":"$(id -g)" --volume /etc/passwd:/etc/passwd:ro --volume /etc/group:/etc/group:ro --volume "${PWD}":/work -w /work rust:1.82.0 cargo'
```
