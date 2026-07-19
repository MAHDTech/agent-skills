+++
title = "learn-mobile-multiwindow"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# Multi-Window on Mobile

Tauri supports multiple windows on Android and iOS, allowing your app to
display content side-by-side on tablets or in separate scenes on iPad.

On **Android**, multi-window is implemented using [Activity
Embedding](https://developer.android.com/guide/topics/large-screens/activity-embedding),
which lets the system display two activities side by side on large
screens.

On **iOS**, multi-window uses the
[UIScene](https://developer.apple.com/documentation/uikit/uiscene) API,
which allows iPad users to open multiple instances of your app in
separate windows.

On **phones**, the system usually does not lay out two windows side by
side. On **Android**, creating another window still launches a separate
activity, but on handset-sized displays it is typically **pushed onto
the activity back stack**—so **Back** returns to the previous activity
instead of closing a split. On **iOS** (especially iPhone), opening or
creating another window often **replaces the current UI** with the new
scene’s content rather than keeping both visible at once; true
concurrent windows remain an **iPad** (and Stage Manager) experience.

## Shared Setup

Both platforms require a capability permission to create new windows
from the frontend.

### Capabilities

Add the `core:webview:allow-create-webview-window` permission to your
capability file so your frontend can create new windows.

If you are creating multiple windows, use a wildcard or list every
window label in the `windows` array:

```
{  "$schema": "../gen/schemas/desktop-schema.json",  "identifier": "default",  "description": "Capability for the main window",  "windows": ["main"],  "permissions": ["core:default", "core:webview:allow-create-webview-window"]}
```

src-tauri/capabilities/default.json

## Android

Android multi-window uses [Activity
Embedding](https://developer.android.com/guide/topics/large-screens/activity-embedding)
to split activities side by side on large screens (tablets, foldables).
You need to create an Android `Activity` for each window type, configure
split rules, and register an initializer.

1.  ### Add Dependencies

    Add the required AndroidX libraries to your `build.gradle.kts`:

    ```
    dependencies {    // ... existing dependencies    implementation("androidx.window:window:1.5.0")    implementation("androidx.startup:startup-runtime:1.2.0")}
    ```

    src-tauri/gen/android/app/build.gradle.kts

2.  ### Create a New Activity

    Create a Kotlin class for each additional window type. Each activity
    must extend `TauriActivity`:

    ```
    package com.example.app
    import android.os.Bundleimport android.os.PersistableBundle
    class DetailActivity: TauriActivity() {  override fun onCreate(savedInstanceState: Bundle?) {    super.onCreate(savedInstanceState)  }}
    ```

    src-tauri/gen/android/app/src/main/java/com/example/app/DetailActivity.kt

3.  ### Update AndroidManifest.xml

    Register the new activity and enable activity embedding by adding
    the `tools` namespace and the embedding property:

    ```
    <?xml version="1.0" encoding="utf-8"?><manifest xmlns:android="http://schemas.android.com/apk/res/android"  xmlns:tools="http://schemas.android.com/tools">
      <application ...>
        <property      android:name="android.window.PROPERTY_ACTIVITY_EMBEDDING_SPLITS_ENABLED"      android:value="true" />
        <!-- Existing MainActivity -->    <activity      android:name=".MainActivity"      android:exported="true"      ...>      ...    </activity>
        <!-- New activity for the detail window -->    <activity android:name=".DetailActivity" android:exported="true" />
        <!-- Register the split initializer -->    <provider android:name="androidx.startup.InitializationProvider"      android:authorities="${applicationId}.androidx-startup"      android:exported="false"      tools:node="merge">      <meta-data android:name="${applicationId}.SplitInitializer"        android:value="androidx.startup" />    </provider>
      </application></manifest>
    ```

    src-tauri/gen/android/app/src/main/AndroidManifest.xml

4.  ### Create the Split Initializer

    The initializer loads the split pair rules at app startup:

    ```
    package com.example.app
    import android.content.Contextimport androidx.startup.Initializerimport androidx.window.core.ExperimentalWindowApiimport androidx.window.embedding.RuleController
    @OptIn(ExperimentalWindowApi::class)class SplitInitializer : Initializer<RuleController> {  override fun create(context: Context): RuleController {    return RuleController.getInstance(context).apply {      setRules(RuleController.parseRules(context, R.xml.main_split_config))    }  }
      override fun dependencies(): List<Class<out Initializer<*>>> {    return emptyList()  }}
    ```

    src-tauri/gen/android/app/src/main/java/com/example/app/SplitInitializer.kt

5.  ### Define Split Rules

    Create an XML resource that tells the system how to pair activities
    and split the screen:

    ```
    <resources  xmlns:window="http://schemas.android.com/apk/res-auto">
      <SplitPairRule    window:splitRatio="0.33"    window:splitLayoutDirection="locale"    window:splitMinWidthDp="840"    window:splitMaxAspectRatioInPortrait="alwaysAllow"    window:finishPrimaryWithSecondary="never"    window:finishSecondaryWithPrimary="never"    window:clearTop="false">    <SplitPairFilter      window:primaryActivityName=".MainActivity"      window:secondaryActivityName=".DetailActivity"/>  </SplitPairRule>
    </resources>
    ```

    src-tauri/gen/android/app/src/main/res/xml/main_split_config.xml

    Key attributes:

    - `splitRatio` — how the screen is divided (0.33 gives the primary
      activity one-third)
    - `splitMinWidthDp` — minimum screen width to activate the split
      (840dp targets tablets)
    - `splitMaxAspectRatioInPortrait` — set to `alwaysAllow` to enable
      split in portrait mode
    - `primaryActivityName` / `secondaryActivityName` — which activity
      pair triggers the split

## iOS

On iOS, multi-window uses the UIScene API. iPad users can open new
windows by long-pressing the app icon and selecting “New window”, or
your app can create them programmatically.

1.  ### Enable Scene Support

    Create an `Info.ios.plist` file in your `src-tauri` directory to
    declare scene support:

    ```
    <?xml version="1.0" encoding="UTF-8"?><!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"  "http://www.apple.com/DTDs/PropertyList-1.0.dtd"><plist version="1.0"><dict>  <key>UIApplicationSceneManifest</key>  <dict>    <key>UIApplicationSupportsMultipleScenes</key>    <true/>    <key>UISceneConfigurations</key>    <dict/>  </dict></dict></plist>
    ```

    src-tauri/Info.ios.plist

2.  ### Handle Scene Requests

    When a user requests a new window on iPad (for example by
    long-pressing the app icon), Tauri emits a
    `RunEvent::SceneRequested` event. Handle it to create a new window:

    ```
    #[cfg_attr(mobile, tauri::mobile_entry_point)]pub fn run() {    #[cfg(target_os = "ios")]    let mut counter = 0;
        tauri::Builder::default()        .setup(|app| {            tauri::WebviewWindowBuilder::new(                app, "main", tauri::WebviewUrl::default()            ).build()?;            Ok(())        })        .build(tauri::generate_context!())        .expect("error while running tauri application")        .run(move |app, event| {            #[cfg(target_os = "ios")]            if let tauri::RunEvent::SceneRequested { .. } = event {                counter += 1;                tauri::WebviewWindowBuilder::new(                    app,                    format!("main-{counter}"),                    tauri::WebviewUrl::default(),                )                .build()                .unwrap();            }            #[cfg(not(target_os = "ios"))]            let _ = (app, event);        });}
    ```

    src-tauri/src/lib.rs

## Creating Windows

You can create additional windows from both Rust and the frontend
JavaScript API. The `WebviewWindowBuilder` (Rust) and `WebviewWindow`
(JavaScript) accept platform-specific options:

Android options:

- `activityName` — the name of the Android Activity class to create for
  this window.
- `createdByActivityName` — the name of the Activity that is creating
  this window. This determines which activity stack the new activity
  belongs to, which is important for the split rules to work correctly.
  When not set, it is automatically inherited from the manager (e.g.
  when building from a `Window` or `Webview` handle).

iOS options:

- `requestedBySceneIdentifier` — sets the identifier of the UIScene that
  is requesting the creation of this new scene, establishing a
  relationship between the two scenes. By default the system uses the
  foreground scene. When not set, it is automatically inherited from the
  manager.

- [JavaScript](#tab-panel-2729)
- [Rust](#tab-panel-2730)

```
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
function openDetail(id) {  const webview = new WebviewWindow(`detail-${id}`, {    url: `detail/${id}`,    activityName: 'DetailActivity',  });  webview.once('tauri://created', () => {    console.log('window created');  });  webview.once('tauri://error', (e) => {    console.error(e);  });}
```

```
use tauri::Manager;
let main_window = app.get_webview_window("main").unwrap();// use the main_window instance so the relationships are determined automaticallylet builder = tauri::WebviewWindowBuilder::new(main_window, "detail", tauri::WebviewUrl::App("detail/1".into()));
#[cfg(target_os = "android")]let builder = builder.activity_name("DetailActivity");
let window = builder.build()?;
```

## Window Instance APIs

Once a window has been created, you can retrieve its platform-specific
identifier:

- [JavaScript](#tab-panel-2731)
- [Rust](#tab-panel-2732)

```
const activityName = await window.activityName();const sceneId = await window.sceneIdentifier();
```

```
#[cfg(target_os = "android")]let activity = window.activity_name()?;
#[cfg(target_os = "ios")]let scene_id = window.scene_identifier()?;
```

These getters are useful for referencing a window’s identity when
creating related windows. For example, you can read a window’s
`activityName` to pass as `createdByActivityName` on a new window, or
read `sceneIdentifier` to pass as `requestedBySceneIdentifier`.

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

