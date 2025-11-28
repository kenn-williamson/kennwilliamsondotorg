# Changelog

## [1.5.0](https://github.com/kenn-williamson/kennwilliamsondotorg/compare/v1.4.0...v1.5.0) (2025-11-28)


### Features

* **tests:** add testcontainer reuse for faster test execution ([86ca822](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/86ca8224817f8252e04121d598e85e0a0faeac53))


### Bug Fixes

* **frontend:** resolve markdown-it-prism browser compatibility error ([7102334](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/7102334961f33e3c10f9e528798861b21dbc7eb2))

## [1.4.0](https://github.com/kenn-williamson/kennwilliamsondotorg/compare/v1.3.0...v1.4.0) (2025-11-27)


### Features

* **about:** make faith, theology, origins, and vision pages public ([b73e4a8](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/b73e4a85d8a2923b003e99de18859c508b8c4c52))

## [1.3.0](https://github.com/kenn-williamson/kennwilliamsondotorg/compare/v1.2.4...v1.3.0) (2025-11-20)


### Features

* **turnstile:** add Cloudflare Turnstile CAPTCHA bot protection ([7950c40](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/7950c40eadc38798c7e41c47c64bd17d49c1c9cf))

## [1.2.4](https://github.com/kenn-williamson/kennwilliamsondotorg/compare/v1.2.3...v1.2.4) (2025-11-19)


### Bug Fixes

* resolve session role duplication and improve markdown rendering ([64c2fe0](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/64c2fe00e5d26af635bd4afb3a51a62fd41e8ca6))

## [1.2.3](https://github.com/kenn-williamson/kennwilliamsondotorg/compare/v1.2.2...v1.2.3) (2025-11-19)


### Bug Fixes

* rename blog components directory and fix word wrapping ([60f7d05](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/60f7d0500b342b5305473a8b220566d1be2fa8ba))
* update Dockerfile.dev to improve dependency caching and clarify comments ([7747a6b](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/7747a6b4c42d7a6c34d5562130c80ad0cf22491c))

## [1.2.2](https://github.com/kenn-williamson/kennwilliamsondotorg/compare/v1.2.1...v1.2.2) (2025-11-19)


### Bug Fixes

* add blog S3 env var and update docs for microblog deployment ([e2fc50f](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/e2fc50f8e58b341ba366db17d97c14faece8bdf3))

## [1.2.1](https://github.com/kenn-williamson/kennwilliamsondotorg/compare/v1.2.0...v1.2.1) (2025-11-19)


### Bug Fixes

* add AWS S3 bucket environment variable for blog images ([3867147](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/3867147fb170cfa0f18a7b520bffc84fc16a9d0a))

## [1.2.0](https://github.com/kenn-williamson/kennwilliamsondotorg/compare/v1.1.0...v1.2.0) (2025-11-19)


### Features

* add local CI validation script to prevent remote failures ([42702e8](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/42702e8286664119ce2b7fcccad1596945cf9f23))
* implement microblog Phase 0-2 (infrastructure + repository layer) ([e064049](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/e064049a797479b12d70d5760646253d168df675))
* implement microblog Phase 3 (service layer with 47 passing tests) ([9f68160](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/9f68160ce734d517adb475c756da85c18613fde5))
* implement microblog Phase 4 (API routes with 11 passing HTTP tests) ([a3276eb](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/a3276eb00b0e865f65baca591dd0a04b94999d39))
* implement microblog Phase 5 (UI, refactoring, and polish) ([223f34b](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/223f34bccfa0dcb18ed993a468949fa83f721641))


### Bug Fixes

* remove em dashes and update Teddy's age in about pages ([24a7377](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/24a7377e39cb28b7438ade5ae2a8df1b5f103538))
* resolve CI errors (clippy, tests, cargo audit, npm audit) ([9d312c6](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/9d312c6ecc2a03034ed43f2bb7dc21ccbfe0d65f))

## [1.1.0](https://github.com/kenn-williamson/kennwilliamsondotorg/compare/v1.0.3...v1.1.0) (2025-11-13)


### Features

* update terminology from 'Christian Anarchism' to 'Christian Voluntarism' for clarity and consistency ([8a24544](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/8a2454483a6a65841168ca96d0ece166e9d7f4c6))

## [1.0.3](https://github.com/kenn-williamson/kennwilliamsondotorg/compare/v1.0.2...v1.0.3) (2025-11-05)


### Bug Fixes

* **ssl:** add ECDSA cipher support and www subdomain to certificates ([8918591](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/8918591c081647e03b647878fa865f7a675a312a))
* **ui:** remove negative margin hacks for consistent cross-browser rendering ([d8437d1](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/d8437d1a8b36ac278e9c201f1085ab5271977062))

## [1.0.2](https://github.com/kenn-williamson/kennwilliamsondotorg/compare/v1.0.1...v1.0.2) (2025-11-04)


### Bug Fixes

* **cd:** automatically trigger deployment when release-please creates release ([bcef902](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/bcef902bad61cfaa4569ce0dcd2dabd249b9919d))

## [1.0.1](https://github.com/kenn-williamson/kennwilliamsondotorg/compare/v1.0.0...v1.0.1) (2025-11-04)


### Bug Fixes

* **social-share:** add missing Open Graph image properties for better SEO ([5584364](https://github.com/kenn-williamson/kennwilliamsondotorg/commit/558436436e06edeb95d23d1d9feca2a7bc52ab3f))
