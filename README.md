# FerroProve verified corpus

[![cargo test](https://github.com/brian-mwirigi/ferroprove-verified-corpus/actions/workflows/ci.yml/badge.svg)](https://github.com/brian-mwirigi/ferroprove-verified-corpus/actions/workflows/ci.yml)

Thirteen safe Rust libraries lifted from the public [DARPA TRACTOR Battery 01](https://github.com/DARPA-TRACTOR-Program/PUBLIC-Test-Corpus) `*_lib` cases. Each crate is `#![forbid(unsafe_code)]`. `cargo test` checks the published vectors.

The FerroProve compiler is not in this repository.

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
