# Iron CMS
[![Build Status](https://travis-ci.org/mrLSD/iron-cms.svg?branch=master)](https://travis-ci.org/mrLSD/iron-cms) [![License](http://img.shields.io/badge/license-mit-blue.svg?style=flat-square)](https://raw.githubusercontent.com/mrLSD/iron-cms/master/LICENSE)

Inspired by Rust, CMS based on Iron framework

### Our aims and basic features

* We love **Rust** and we want create amazing tool and infrastructure for scaffolding and smart development. Fast development, efficient code, safe environment. Big and bold goal, but it is worth it.
* We investigate several Rust framework, our base for that - performance and smart development with chosen tools and framework infrastructure.
* We **not guaranteeing** use Iron as basic framework before ending investigation.
* We developed middlewares for Form Request parsing and complex form data valdation
* We use **only stable** version of Rust and their libraries
* We tests cover our code as far as it possible
* We trying develop full-featured tool, including: templates, for fetching and validation, auth, cookies, session, database, http-security tools,cloud integration, admin/backend tools, caching, migrations, CI-orientation, flexible integration new features.
* We want to attract interest in Rust and to the fact that web development with Rust is funny and effective.

#### Web site
https://iron-cms.github.io

#### Current status:
Active development

#### How to build and run:
```
$ git clone http://github.com/mrlsd/iron-cms
$ make release
$ target/release/iron-cms
```

#### Requirements:
* Rust 1.13+
  
#### Some useful command:
* **install Rust:** `make install`
* **run:** `make`
* **build:** `make build`
* **release build:** `make release`
* **test:** `make test`


####License: MIT
