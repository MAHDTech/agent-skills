# Manual setup

This page covers driving
[`tauri-driver`](https://crates.io/crates/tauri-driver) directly,
without the
[`@wdio/tauri-service`](https://webdriver.io/docs/desktop-testing/tauri).
Reach for it if you are not using Node.js, prefer
[Selenium](/develop/tests/webdriver/example/selenium/), or are
integrating WebDriver into a custom test harness. For most projects the
service is the easier path — it automates everything below and
additionally supports macOS. See the [WebDriver
overview](/develop/tests/webdriver/) to get started with it.

When driving `tauri-driver` directly, only Windows and Linux are
supported on desktop, as macOS has no WKWebView driver tool available.
iOS and Android work through Appium 2, but the process is not currently
streamlined.

## System Dependencies

Install the latest
[`tauri-driver`](https://crates.io/crates/tauri-driver) or update an
existing installation by running:

```
cargo install tauri-driver --locked
```

Terminal window

Because we currently utilize the platform’s native
[WebDriver](https://www.w3.org/TR/webdriver/) server, there are some
requirements for running
[`tauri-driver`](https://crates.io/crates/tauri-driver) on supported
platforms.

### Linux

We use `WebKitWebDriver` on Linux platforms. Check if this binary exists
already by running the `which WebKitWebDriver` command as some
distributions bundle it with the regular WebKit package. Other platforms
may have a separate package for them, such as `webkit2gtk-driver` on
Debian-based distributions.

### Windows

Make sure to grab the version of [Microsoft Edge
Driver](https://developer.microsoft.com/en-us/microsoft-edge/tools/webdriver/)
that matches your Windows Edge version that the application is being
built and tested on. This should almost always be the latest stable
version on up-to-date Windows installs. If the two versions do not
match, you may experience your WebDriver testing suite hanging while
trying to connect.

You can use the
[msedgedriver-tool](https://github.com/chippers/msedgedriver-tool) to
download the appropriate Microsoft Edge Driver:

```
cargo install --git https://github.com/chippers/msedgedriver-tool& "$HOME/.cargo/bin/msedgedriver-tool.exe"
```

Terminal window

The download contains a binary called `msedgedriver.exe`.
[`tauri-driver`](https://crates.io/crates/tauri-driver) looks for that
binary in the `$PATH` so make sure it’s either available on the path or
use the `--native-driver` option on
[`tauri-driver`](https://crates.io/crates/tauri-driver). You may want to
download this automatically as part of the CI setup process to ensure
the Edge, and Edge Driver versions stay in sync on Windows CI machines.
A guide on how to do this may be added at a later date.

## Example Applications

Below are step-by-step guides to show how to create a minimal example
application that is tested with WebDriver.

If you prefer to see the result of the guide and look over a finished
minimal codebase that utilizes it, you can look at
<https://github.com/tauri-apps/webdriver-example>.

[Selenium](/develop/tests/webdriver/example/selenium/)

[WebdriverIO](/develop/tests/webdriver/example/webdriverio/)

## Continuous Integration (CI)

The above examples also comes with a CI script to test with GitHub
Actions, but you may still be interested in the below WebDriver CI guide
as it explains the concept a bit more.

[Continuous Integration (CI)](/develop/tests/webdriver/ci/)

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
