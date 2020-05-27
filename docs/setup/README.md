# getting set up

The first thing to do is make your copy of [`secrets-SAMPLE.toml`](../../secrets-SAMPLE.toml):

```bash
cp secrets-SAMPLE.toml secrets.toml
```

In order to get your api key, you'll need to visit the [UCSB API Developer Portal](https://developer.ucsb.edu/). Creating an account should only consist of logging in with your UCSB netid.

After you're logged in, you'll need to create an app [here](https://developer.ucsb.edu/user/me/apps). The permission you're looking for is the public API for "Academic - Curriculums". After you create your app, you'll be able to retrieve its `Consumer Key` which serves as the API key. Plop that into your [`secrets.toml`](../../secrets.toml) under `api_key` and you'll be good to run with

```bash
rustup run nightly cargo run
```
