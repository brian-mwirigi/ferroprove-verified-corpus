# FerroProve verified corpus

[![cargo test](https://github.com/brian-mwirigi/ferroprove-verified-corpus/actions/workflows/ci.yml/badge.svg)](https://github.com/brian-mwirigi/ferroprove-verified-corpus/actions/workflows/ci.yml)

Safe Rust libraries lifted from the public [DARPA TRACTOR Battery 01](https://github.com/DARPA-TRACTOR-Program/PUBLIC-Test-Corpus) `*_lib` cases, plus the classic modules below (musl `atoi`, zlib `adler32`, OpenBSD `timingsafe_bcmp`, Redis `crc16`, GCC `crc32`, Git `hex`, packet headers, and the rest). Each crate is `#![forbid(unsafe_code)]`. `cargo test` checks the published vectors.

The FerroProve compiler is not in this repository. These crates are the generated proofs only: `Cargo.toml`, `src/lib.rs` (`#![forbid(unsafe_code)]`), `src/ffi.rs`, `include/ferroprove.h`, and `CISA_COMPLIANCE_CERTIFICATE.md`.

| Crate | Vectors | Lift | Certificate |
| --- | ---: | ---: | --- |
| [`encode_quant_lib`](crates/encode_quant_lib) | 8 | 1586.0 ms | [CISA_COMPLIANCE_CERTIFICATE.md](crates/encode_quant_lib/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`flac_validate_lib`](crates/flac_validate_lib) | 6 | 1221.0 ms | [CISA_COMPLIANCE_CERTIFICATE.md](crates/flac_validate_lib/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`hdr_compare_lib`](crates/hdr_compare_lib) | 8 | 1579.0 ms | [CISA_COMPLIANCE_CERTIFICATE.md](crates/hdr_compare_lib/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`hsl_to_rgb_lib`](crates/hsl_to_rgb_lib) | 8 | 1550.0 ms | [CISA_COMPLIANCE_CERTIFICATE.md](crates/hsl_to_rgb_lib/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`hsv_to_rgb_lib`](crates/hsv_to_rgb_lib) | 8 | 1532.0 ms | [CISA_COMPLIANCE_CERTIFICATE.md](crates/hsv_to_rgb_lib/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`max_size_frame_lib`](crates/max_size_frame_lib) | 8 | 1488.0 ms | [CISA_COMPLIANCE_CERTIFICATE.md](crates/max_size_frame_lib/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`md5_digest_lib`](crates/md5_digest_lib) | 5 | 1042.0 ms | [CISA_COMPLIANCE_CERTIFICATE.md](crates/md5_digest_lib/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`merge_sort_lib`](crates/merge_sort_lib) | 5 | 1110.0 ms | [CISA_COMPLIANCE_CERTIFICATE.md](crates/merge_sort_lib/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`rev16_lib`](crates/rev16_lib) | 5 | 1810.0 ms | [CISA_COMPLIANCE_CERTIFICATE.md](crates/rev16_lib/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`stereo_samples_lib`](crates/stereo_samples_lib) | 2 | 942.0 ms | [CISA_COMPLIANCE_CERTIFICATE.md](crates/stereo_samples_lib/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`crc16_lib`](crates/crc16_lib) | 3 | 1243.0 ms | [CISA_COMPLIANCE_CERTIFICATE.md](crates/crc16_lib/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`normalize_lib`](crates/normalize_lib) | 5 | 1108.0 ms | [CISA_COMPLIANCE_CERTIFICATE.md](crates/normalize_lib/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`rgb_to_hsv_lib`](crates/rgb_to_hsv_lib) | 5 | 1047.0 ms | [CISA_COMPLIANCE_CERTIFICATE.md](crates/rgb_to_hsv_lib/CISA_COMPLIANCE_CERTIFICATE.md) |

```bash
cargo test --workspace
```

Upstream oracles: `DARPA-TRACTOR-Program/PUBLIC-Test-Corpus`, Battery 01.

## Classic lifts

Same layout. `cargo test` checks the vectors produced with each lift.

| Crate | Source | Tests | Certificate |
| --- | --- | ---: | --- |
| [`cjson`](crates/cjson) | cJSON-style `hex_digit` → `hex_byte` → `scan_string` | 6 | [CISA_COMPLIANCE_CERTIFICATE.md](crates/cjson/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`tlv`](crates/tlv) | TLV length decode | 4 | [CISA_COMPLIANCE_CERTIFICATE.md](crates/tlv/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`bits`](crates/bits) | Bitstream unpack | 3 | [CISA_COMPLIANCE_CERTIFICATE.md](crates/bits/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`expr`](crates/expr) | Expression evaluator globals | 6 | [CISA_COMPLIANCE_CERTIFICATE.md](crates/expr/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`pkthdr`](crates/pkthdr) | Packet header and histogram | 4 | [CISA_COMPLIANCE_CERTIFICATE.md](crates/pkthdr/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`cve4303`](crates/cve4303) | cJSON v1.0.0 `parse_hex4` | 4 | [CISA_COMPLIANCE_CERTIFICATE.md](crates/cve4303/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`gcc_crc32`](crates/gcc_crc32) | GCC libiberty `crc32.c` | 4 | [CISA_COMPLIANCE_CERTIFICATE.md](crates/gcc_crc32/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`redis_crc16`](crates/redis_crc16) | Redis 7.2 `src/crc16.c` | 5 | [CISA_COMPLIANCE_CERTIFICATE.md](crates/redis_crc16/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`atoi`](crates/atoi) | musl `src/stdlib/atoi.c` | 4 | [CISA_COMPLIANCE_CERTIFICATE.md](crates/atoi/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`timingsafe_bcmp`](crates/timingsafe_bcmp) | OpenBSD `timingsafe_bcmp.c` | 6 | [CISA_COMPLIANCE_CERTIFICATE.md](crates/timingsafe_bcmp/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`adler32`](crates/adler32) | zlib 1.3.1 `adler32.c` | 5 | [CISA_COMPLIANCE_CERTIFICATE.md](crates/adler32/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`hex`](crates/hex) | Git 2.30 `hex.c` `hex_to_bytes` | 6 | [CISA_COMPLIANCE_CERTIFICATE.md](crates/hex/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`sodium_memcmp`](crates/sodium_memcmp) | libsodium 1.0.19 `sodium_memcmp` | 6 | [CISA_COMPLIANCE_CERTIFICATE.md](crates/sodium_memcmp/CISA_COMPLIANCE_CERTIFICATE.md) |
| [`crc8`](crates/crc8) | Linux 6.8 `lib/crc8.c` | 5 | [CISA_COMPLIANCE_CERTIFICATE.md](crates/crc8/CISA_COMPLIANCE_CERTIFICATE.md) |
