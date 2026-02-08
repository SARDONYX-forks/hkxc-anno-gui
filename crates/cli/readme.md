# CLI

Forked the serde-hkx CLI and integrated hkxc-anno-cli to create an extended version of hkxc.

## Examples

```shell
# - hkx -> xml
./hkxc convert --input ./defaultmale.hkx --format xml

# - xml -> hkx(32bit)
./hkxc convert -i ./defaultmale.xml -o ./defaultmale.hkx --format win32 --log-level "debug" --log-file "./convert_to_x64_bytes.log"
# - xml -> hkx(64bit)
./hkxc convert -i ./defaultmale.xml -v amd64 --stdout --log-level "trace"

# - hkx(32bit) -> hkx(64bit)
./hkxc convert -i ./defaultmale_x86.hkx -o ./defaultmale_x64.hkx -v amd64 --log-level "debug" --log-file "./convert_x86_to_x64_bytes.log"
# - hkx(64bit) -> hkx(32bit)
./hkxc convert -i ./defaultmale_x64.hkx -o ./defaultmale_x86.hkx -v win32 --log-level "trace" --log-file "./convert_x64_to_x86_bytes.log"
```

## Licenses

- [serde-hkx CLI](https://github.com/SARDONYX-sard/serde-hkx/tree/0.7.3/crates/cli)

  ```txt
  SPDX-FileCopyrightText: (C) 2025 SARDONYX
  SPDX-License-Identifier: Apache-2.0 OR MIT
  ```

- [hkxc-anno-cli](https://github.com/beefclot/hkxc-anno-cli)

  ```txt
  SPDX-FileCopyrightText: (C) 2025 beefclot
  SPDX-License-Identifier: GPL-3.0
  ```
