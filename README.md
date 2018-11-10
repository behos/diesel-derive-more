# Deprecation notice

**Other packages are providing the same functionality now**

It's been a while since I started this package and it appears that other packages have come up which offer the same functionality and more.

# diesel-derive-more

[![Build Status](https://travis-ci.org/behos/diesel-derive-more.svg?branch=master)](https://travis-ci.org/behos/diesel-derive-more)

This package provides some helpers for working with diesel models.

# Provides

* DefaultInsertable: Creates a default struct to use as an insertable model
* DBEnum: Allows serializing and deserializing enums for DB storage
* Deserialization for default insertable through the 'serialization' feature

This is work in progress (created to support one of my projects) so it will continue to evolve.

# Testing

Create a postgres test db and add a .env file to the root of the project pointing to that
db

```
DATABASE_URL=postgres://dev:password@127.0.0.1/dev
```

Initialize the db by running:

```
pushd tests && diesel migration run && popd
```

You can run this when you make changes to the test schemas as well
