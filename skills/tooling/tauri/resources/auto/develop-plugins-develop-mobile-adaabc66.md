# Mobile Plugin Development

Plugins can run native mobile code written in Kotlin (or Java) and
Swift. The default plugin template includes an Android library project
using Kotlin and a Swift package including an example mobile command
showing how to trigger its execution from Rust code.

## Initialize Plugin Project

Follow the steps in the [Plugin Development
guide](https://v2.tauri.app/develop/plugins/#initialize-plugin-project) to initialize a new
plugin project.

If you have an existing plugin and would like to add Android or iOS
capabilities to it, you can use `plugin android init` and
`plugin ios init` to bootstrap the mobile library projects and guide you
through the changes needed.

The default plugin template splits the plugin’s implementation into two
separate modules: `desktop.rs` and `mobile.rs`.

The desktop implementation uses Rust code to implement a functionality,
while the mobile implementation sends a message to the native mobile
code to execute a function and get a result back. If shared logic is
needed across both implementations, it can be defined in `lib.rs`:

```
use tauri::Runtime;
impl<R: Runtime> <plugin-name><R> {  pub fn do_something(&self) {    // do something that is a shared implementation between desktop and mobile  }}
```

src/lib.rs

This implementation simplifies the process of sharing an API that can be
used both by commands and Rust code.

### Develop an Android Plugin

A Tauri plugin for Android is defined as a Kotlin class that extends
`app.tauri.plugin.Plugin` and is annotated with
`app.tauri.annotation.TauriPlugin`. Each method annotated with
`app.tauri.annotation.Command` can be called by Rust or JavaScript.

Tauri uses Kotlin by default for the Android plugin implementation, but
you can switch to Java if you prefer. After generating a plugin, right
click the Kotlin plugin class in Android Studio and select the “Convert
Kotlin file to Java file” option from the menu. Android Studio will
guide you through the project migration to Java.

### Develop an iOS Plugin

A Tauri plugin for iOS is defined as a Swift class that extends the
`Plugin` class from the `Tauri` package. Each function with the `@objc`
attribute and the `(_ invoke: Invoke)` parameter (for example
`@objc private func download(_ invoke: Invoke) { }`) can be called by
Rust or JavaScript.

The plugin is defined as a [Swift
package](https://www.swift.org/package-manager/) so that you can use its
package manager to manage dependencies.

## Plugin Configuration

Refer to the [Plugin Configuration
section](https://v2.tauri.app/develop/plugins/#plugin-configuration) of the Plugin
Development guide for more details on developing plugin configurations.

The plugin instance on mobile has a getter for the plugin configuration:

- [Android](#tab-panel-4584)
- [iOS](#tab-panel-4585)

```
import android.app.Activityimport android.webkit.WebViewimport app.tauri.annotation.TauriPluginimport app.tauri.annotation.InvokeArg
@InvokeArgclass Config {    var timeout: Int? = 3000}
@TauriPluginclass ExamplePlugin(private val activity: Activity): Plugin(activity) {  private var timeout: Int? = 3000
  override fun load(webView: WebView) {    getConfig(Config::class.java).let {       this.timeout = it.timeout    }  }}
```

```
struct Config: Decodable {  let timeout: Int?}
class ExamplePlugin: Plugin {  var timeout: Int? = 3000
  @objc public override func load(webview: WKWebView) {    do {      let config = try parseConfig(Config.self)      self.timeout = config.timeout    } catch {}  }}
```

## Lifecycle Events

Plugins can hook into several lifecycle events:

- [load](#load): When the plugin is loaded into the web view
- [onNewIntent](#onnewintent): Android only, when the activity is
  re-launched

There are also the additional [lifecycle events for
plugins](https://v2.tauri.app/develop/plugins/#lifecycle-events) in the Plugin Development
guide.

### load

- **When**: When the plugin is loaded into the web view
- **Why**: Execute plugin initialization code

- [Android](#tab-panel-4586)
- [iOS](#tab-panel-4587)

```
import android.app.Activityimport android.webkit.WebViewimport app.tauri.annotation.TauriPlugin
@TauriPluginclass ExamplePlugin(private val activity: Activity): Plugin(activity) {  override fun load(webView: WebView) {    // perform plugin setup here  }}
```

```
class ExamplePlugin: Plugin {  @objc public override func load(webview: WKWebView) {    let timeout = self.config["timeout"] as? Int ?? 30  }}
```

### onNewIntent

**Note**: This is only available on Android.

- **When**: When the activity is re-launched. See
  [Activity#onNewIntent](https://developer.android.com/reference/android/app/Activity#onNewIntent(android.content.Intent))
  for more information.
- **Why**: Handle application re-launch such as when a notification is
  clicked or a deep link is accessed.

```
import android.app.Activityimport android.content.Intentimport app.tauri.annotation.TauriPlugin
@TauriPluginclass ExamplePlugin(private val activity: Activity): Plugin(activity) {  override fun onNewIntent(intent: Intent) {    // handle new intent event  }}
```

## Adding Mobile Commands

There is a plugin class inside the respective mobile projects where
commands can be defined that can be called by the Rust code:

- [Android](#tab-panel-4594)
- [iOS](#tab-panel-4595)

```
import android.app.Activityimport app.tauri.annotation.Commandimport app.tauri.annotation.TauriPlugin
@TauriPluginclass ExamplePlugin(private val activity: Activity): Plugin(activity) {  @Command  fun openCamera(invoke: Invoke) {    val ret = JSObject()    ret.put("path", "/path/to/photo.jpg")    invoke.resolve(ret)  }}
```

If you want to use a Kotlin `suspend` function, you need to use a custom
coroutine scope

```
import android.app.Activityimport app.tauri.annotation.Commandimport app.tauri.annotation.TauriPlugin
// Change to Dispatchers.IO if it is intended for fetching dataval scope = CoroutineScope(Dispatchers.Default + SupervisorJob())
@TauriPluginclass ExamplePlugin(private val activity: Activity): Plugin(activity) {  @Command  fun openCamera(invoke: Invoke) {    scope.launch {      openCameraInner(invoke)    }  }
  private suspend fun openCameraInner(invoke: Invoke) {    val ret = JSObject()    ret.put("path", "/path/to/photo.jpg")    invoke.resolve(ret)  }}
```

```
class ExamplePlugin: Plugin {  @objc public func openCamera(_ invoke: Invoke) throws {    invoke.resolve(["path": "/path/to/photo.jpg"])  }}
```

Use the
[`tauri::plugin::PluginHandle`](https://docs.rs/tauri/2.0.0/tauri/plugin/struct.PluginHandle.html)
to call a mobile command from Rust:

```
use std::path::PathBuf;use serde::{Deserialize, Serialize};use tauri::Runtime;
#[derive(Serialize)]#[serde(rename_all = "camelCase")]pub struct CameraRequest {  quality: usize,  allow_edit: bool,}
#[derive(Deserialize)]pub struct Photo {  path: PathBuf,}

impl<R: Runtime> <plugin-name;pascal-case><R> {  pub fn open_camera(&self, payload: CameraRequest) -> crate::Result<Photo> {    self      .0      .run_mobile_plugin("openCamera", payload)      .map_err(Into::into)  }}
```

## Command Arguments

Arguments are serialized to commands and can be parsed on the mobile
plugin with the `Invoke::parseArgs` function, taking a class describing
the argument object.

### Android

On Android, the arguments are defined as a class annotated with
`@app.tauri.annotation.InvokeArg`. Inner objects must also be annotated:

```
import android.app.Activityimport android.webkit.WebViewimport app.tauri.annotation.Commandimport app.tauri.annotation.InvokeArgimport app.tauri.annotation.TauriPlugin
@InvokeArginternal class OpenAppArgs {  lateinit var name: String  var timeout: Int? = null}
@InvokeArginternal class OpenArgs {  lateinit var requiredArg: String  var allowEdit: Boolean = false  var quality: Int = 100  var app: OpenAppArgs? = null}
@TauriPluginclass ExamplePlugin(private val activity: Activity): Plugin(activity) {  @Command  fun openCamera(invoke: Invoke) {    val args = invoke.parseArgs(OpenArgs::class.java)  }}
```

### iOS

On iOS, the arguments are defined as a class that inherits `Decodable`.
Inner objects must also inherit the Decodable protocol:

```
class OpenAppArgs: Decodable {  let name: String  var timeout: Int?}
class OpenArgs: Decodable {  let requiredArg: String  var allowEdit: Bool?  var quality: UInt8?  var app: OpenAppArgs?}
class ExamplePlugin: Plugin {  @objc public func openCamera(_ invoke: Invoke) throws {    let args = try invoke.parseArgs(OpenArgs.self)
    invoke.resolve(["path": "/path/to/photo.jpg"])  }}
```

## Calling Rust From Mobile Plugins

It is often preferable to write plugin code in Rust, for performance and
reusability. While Tauri doesn’t directly provide a mechanism to call
Rust from your plugin code, using JNI on Android and FFI on iOS allows
plugins to call shared code, even when the application WebView is
suspended.

### Android

In your plugin’s `Cargo.toml`, add the jni crate as a dependency:

```
[target.'cfg(target_os = "android")'.dependencies]jni = "0.21"
```

Load the application library statically and define native functions in
your Kotlin code. In this example, the Kotlin class is
`com.example.HelloWorld`, we need to reference the full package name
from the Rust side.

```
private const val TAG = "MyPlugin"
init {  try {    // Load the native library (libapp_lib.so)    // This is the shared library built by Cargo with crate-type = ["cdylib"]    System.loadLibrary("app_lib")    Log.d(TAG, "Successfully loaded libapp_lib.so")  } catch (e: UnsatisfiedLinkError) {    Log.e(TAG, "Failed to load libapp_lib.so", e)    throw e  }}
external fun helloWorld(name: String): String?
```

Then in your plugin’s Rust code, define the function JNI will look for.
The function format is `Java_package_class_method`, so for our class
above this becomes `Java_com_example_HelloWorld_helloWorld` to get
called by our `helloWorld` method:

```
#[cfg(target_os = "android")]#[no_mangle]pub extern "system" fn Java_com_example_HelloWorld_helloWorld(    mut env: JNIEnv,    _class: JClass,    name: JString,) -> jstring {    log::debug!("Calling JNI Hello World!");    let result = format!("Hello, {}!", name);
    match env.new_string(result) {        Ok(jstr) => jstr.into_raw(),        Err(e) => {            log::error!("Failed to create JString: {}", e);            std::ptr::null_mut()        }    }}
```

### iOS

iOS only uses standard C FFI, so doesn’t need any new dependencies. Add
the hook in your Swift code, as well as any necessary cleanup. These
functions can be named anything valid, but must be annotated with
`@_silgen_name(FFI_FUNC)`, where FFI_FUNC is a function name to be
called from Rust:

```
@_silgen_name("hello_world_ffi")private static func helloWorldFFI(_ name: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?
@_silgen_name("free_hello_result_ffi")private static func freeHelloResult(_ result: UnsafeMutablePointer<CChar>)
static func helloWorld(name: String) -> String? {  // Call Rust FFI  let resultPtr = name.withCString({ helloWorldFFI($0) })
  // Convert C string to Swift String  let result = String(cString: resultPtr)
  // Free the C string  freeHelloResult(resultPtr)
  return result}
```

Then, implement the Rust side. The `extern` functions here must match
the `@_silgen_name` annotations on the Swift side:

```
#[no_mangle]pub unsafe extern "C" fn hello_world_ffi(c_name: *const c_char) -> *mut c_char {    let name = match CStr::from_ptr(c_name).to_str() {        Ok(s) => s,        Err(e) => {            log::error!("[iOS FFI] Failed to convert C string: {}", e);            return std::ptr::null_mut();        }    };
    let result = format!("Hello, {}!", name);
    match CString::new(result) {        Ok(c_str) => c_str.into_raw(),        Err(e) => {            log::error!("[iOS FFI] Failed to create C string: {}", e);            std::ptr::null_mut()        }    }}
#[no_mangle]pub unsafe extern "C" fn free_hello_result_ffi(result: *mut c_char) {    if !result.is_null() {        drop(CString::from_raw(result));    }}
```

## Android 16KB Memory Pages

Google is moving to make 16KB memory pages a requirement in all new
Android app submissions. Building with an NDK version 28 or higher
should automatically generate bundles that meet this requirement, but in
the event an older NDK version must be used or generated files aren’t
16KB aligned, the following can be added to `.cargo/config.toml` to flag
this to `rustc`:

```
[target.aarch64-linux-android]rustflags = ["-C", "link-arg=-Wl,-z,max-page-size=16384"]
```

## Permissions

If a plugin requires permissions from the end user, Tauri simplifies the
process of checking and requesting permissions.

- [Android](#tab-panel-4588)
- [iOS](#tab-panel-4589)

First define the list of permissions needed and an alias to identify
each group in code. This is done inside the `TauriPlugin` annotation:

```
@TauriPlugin(  permissions = [    Permission(strings = [Manifest.permission.POST_NOTIFICATIONS], alias = "postNotification")  ])class ExamplePlugin(private val activity: Activity): Plugin(activity) { }
```

First override the `checkPermissions` and `requestPermissions`
functions:

```
class ExamplePlugin: Plugin {  @objc open func checkPermissions(_ invoke: Invoke) {    invoke.resolve(["postNotification": "prompt"])  }
  @objc public override func requestPermissions(_ invoke: Invoke) {    // request permissions here    // then resolve the request    invoke.resolve(["postNotification": "granted"])  }}
```

Tauri automatically implements two commands for the plugin:
`checkPermissions` and `requestPermissions`. Those commands can be
directly called from JavaScript or Rust:

- [JavaScript](#tab-panel-4590)
- [Rust](#tab-panel-4591)

```
import { invoke, PermissionState } from '@tauri-apps/api/core'
interface Permissions {  postNotification: PermissionState}
// check permission stateconst permission = await invoke<Permissions>('plugin:<plugin-name>|checkPermissions')
if (permission.postNotification === 'prompt-with-rationale') {  // show information to the user about why permission is needed}
// request permissionif (permission.postNotification.startsWith('prompt')) {  const state = await invoke<Permissions>('plugin:<plugin-name>|requestPermissions', { permissions: ['postNotification'] })}
```

```
use serde::{Serialize, Deserialize};use tauri::{plugin::PermissionState, Runtime};
#[derive(Deserialize)]#[serde(rename_all = "camelCase")]struct PermissionResponse {  pub post_notification: PermissionState,}
#[derive(Serialize)]#[serde(rename_all = "camelCase")]struct RequestPermission {  post_notification: bool,}
impl<R: Runtime> Notification<R> {  pub fn request_post_notification_permission(&self) -> crate::Result<PermissionState> {    self.0      .run_mobile_plugin::<PermissionResponse>("requestPermissions", RequestPermission { post_notification: true })      .map(|r| r.post_notification)      .map_err(Into::into)  }
  pub fn check_permissions(&self) -> crate::Result<PermissionResponse> {    self.0      .run_mobile_plugin::<PermissionResponse>("checkPermissions", ())      .map_err(Into::into)  }}
```

## Plugin Events

Plugins can emit events at any point of time using the `trigger`
function:

- [Android](#tab-panel-4592)
- [iOS](#tab-panel-4593)

```
@TauriPluginclass ExamplePlugin(private val activity: Activity): Plugin(activity) {    override fun load(webView: WebView) {      trigger("load", JSObject())    }
    override fun onNewIntent(intent: Intent) {      // handle new intent event      if (intent.action == Intent.ACTION_VIEW) {        val data = intent.data.toString()        val event = JSObject()        event.put("data", data)        trigger("newIntent", event)      }    }
    @Command    fun openCamera(invoke: Invoke) {      val payload = JSObject()      payload.put("open", true)      trigger("camera", payload)    }}
```

```
class ExamplePlugin: Plugin {  @objc public override func load(webview: WKWebView) {    trigger("load", data: [:])  }
  @objc public func openCamera(_ invoke: Invoke) {    trigger("camera", data: ["open": true])  }}
```

The helper functions can then be called from the NPM package by using
the
[`addPluginListener`](https://v2.tauri.app/reference/javascript/api/namespacecore/#addpluginlistener)
helper function:

```
import { addPluginListener, PluginListener } from '@tauri-apps/api/core';
export async function onRequest(  handler: (url: string) => void): Promise<PluginListener> {  return await addPluginListener(    '<plugin-name>',    'event-name',    handler  );}
```

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT
