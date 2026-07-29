+++
title = "docs-tutorials-unreal-part-1"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "spacetimedb"
+++

{% raw %}
Version: 2.0.0

On this page

Need help with the tutorial? [Join our Discord
server](https://discord.gg/spacetimedb)!

> A completed version of the game we'll create in this tutorial is
> available at:
>
> [https://github.com/clockworklabs/SpacetimeDB/tree/v1.12.0/demo/Blackholio](https://github.com/clockworklabs/SpacetimeDB/tree/v1.12.0/demo/Blackholio)

## Setting up the Tutorial Unreal Project

In this section, we will guide you through the process of setting up a
Unreal Project that will serve as the starting point for our tutorial.
By the end of this section, you will have a basic Unreal project and be
ready to implement the server functionality.

### Step 1: Create a Blank Unreal Project

SpacetimeDB supports Unreal version `5.6`. See [the
overview](https://spacetimedb.com/docs/tutorials/unreal/) for more information on specific
supported versions.

Launch Unreal 5.6 and create a new project by selecting Games from the
Unreal Project Browser.

warning

Select the **Blank** template and in **Project Defaults** select
**C++**.

For **Project Name** use `blackholio`.

Click **Create** to generate the blank project.

![Create Blank
Project](https://spacetimedb.com/docs/assets/images/part-1-01-create-project-378b79c8a34e3304733a550fc939c6f3.png)

### Import the SpacetimeDB Unreal SDK

While the SpacetimeDB Unreal client SDK is in preview releases, it can
only be installed from GitHub:

> [https://github.com/clockworklabs/SpacetimeDB/tree/v1.12.0/clients/unreal/src](https://github.com/clockworklabs/SpacetimeDB/tree/v1.12.0/clients/unreal/src)

Once the SDK is stabilized, we'll find a more ergonomic way to
distribute it.

note

Before beginning make sure to close the Unreal project and IDE.

#### Installation steps

1.  Navigate to your Unreal project directory and create a `Plugins`
    folder if it doesn’t already exist:

    ``` codeBlockStandalone_LlrK
    cd blackholio
    mkdir Plugins
    ```

2.  Download or clone the SDK from GitHub and copy the SpacetimeDbSdk
    folder into your new Plugins directory.

    - This should create `/blackholio/Plugins/SpacetimeDbSdk`.

3.  In the root of the Unreal project, right click the
    blackholio.uproject and select **Generate Visual Studio project
    files**. On Windows 11 you may need to expand **Show more options**
    to select the generate option.

![Generate project
files](https://spacetimedb.com/docs/assets/images/part-1-02-01-generate-project-61146b2e33b2039df6d88670b6f594bc.png)
![Generate project
files](data:image/png;base64,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)

### Create the GameManager Actor

- C++
- Blueprint

1.  Open the `blackholio` project in your IDE (Visual Studio or
    JetBrains Rider) and run the project to launch the Unreal Editor.
    - This will enable **Live Coding**, making the workflow a bit
      smoother.
    - Unreal will prompt you to build the `SpacetimeDbSdk` plugin. Do
      so.
2.  Open **Tools -\> New C++ Class** in the top menu, select **Actor**
    as the parent and click **Next**
3.  Select **Public** Class Type
4.  Name the class `GameManager`.

The `GameManager` class will be where we will put the high level
initialization and coordination logic for our game.

> **Note:** In a production Unreal project, you would typically
> implement this logic in a Subsystem. For simplicity, this tutorial
> uses a singleton actor.

1.  Open the `blackholio` project to launch the Unreal Editor.
2.  **Create a GameManager Blueprint**
    - In the **Content Drawer**, click **Add**, then select **Blueprint
      -\> Blueprint Class**.
    - Click **Actor**.
    - Name the blueprint `BP_GameManager`.

### Set Up the Level

Set up the empty level, add the new `GameManager` to the level, and add
lighting.

- C++
- Blueprint

1.  **Create a new level**

    - Open **File -\> New Level** in the top menu, select **Empty
      Level**, and click **Create**.
    - Save the level and name it `Blackholio`.

2.  **Create a GameManager Blueprint**

    - In the **Content Drawer**, click **Add**, then select **Blueprint
      -\> Blueprint Class**.
    - Expand **All Classes**, search for **GameManager**, highlight it,
      and click **Select**.
    - Name the blueprint `BP_GameManager`.

    ![Pick Parent
    Class](https://spacetimedb.com/docs/assets/images/part-1-03-create-blueprint-98f1a712896285dde7aad0eed871b272.png)

3.  **Update Maps & Modes**

    - Open **Edit -\> Project Settings** in the top menu, then select
      **Project -\> Maps & Modes** on the left.
    - Set **Editor Startup Map** to `Blackholio`.
    - Set **Game Default Map** to `Blackholio`.

4.  **Add to the Level**

    - Drag the `BP_GameManager` blueprint from the **Content Drawer**
      into the scene view.

5.  **Add a Directional Light**

    - Click **Add** in the top toolbar, then select **Lights -\>
      Directional Light**.
    - Set **Rotation** to -105.0, -31.0, -14.0.

6.  **Add a Post Process Volume**

    - Click **Add** in the top toolbar, then select **Volumes -\> Post
      Process Volume**.
    - Enable and set **Exposure -\> Exposure Compensation** to 0.0.
    - Enable and set **Exposure -\> Min EV100** to 1.0.
    - Enable and set **Exposure -\> Max EV100** to 1.0.
    - Enable **Post Process Volume Settings -\> Infinite Extend
      (Unbounded)**.

1.  **Create a new level**

    - Open **File -\> New Level** in the top menu, select **Empty
      Level**, and click **Create**.
    - Save the level and name it `Blackholio`.

2.  **Update Maps & Modes**

    - Open **Edit -\> Project Settings** in the top menu, then select
      **Project -\> Maps & Modes** on the left.
    - Set **Editor Startup Map** to `Blackholio`.
    - Set **Game Default Map** to `Blackholio`.

3.  **Add to the Level**

    - Drag the `BP_GameManager` blueprint from the **Content Drawer**
      into the scene view.

4.  **Add a Directional Light**

    - Click **Add** in the top toolbar, then select **Lights -\>
      Directional Light**.
    - Set **Rotation** to -105.0, -31.0, -14.0.

5.  **Add a Post Process Volume**

    - Click **Add** in the top toolbar, then select **Volumes -\> Post
      Process Volume**.
    - Enable and set **Exposure -\> Exposure Compensation** to 0.0.
    - Enable and set **Exposure -\> Min EV100** to 1.0.
    - Enable and set **Exposure -\> Max EV100** to 1.0.
    - Enable **Post Process Volume Settings -\> Infinite Extend
      (Unbounded)**.

### Add a Simple GameMode

Create a simple GameMode to tweak the startup settings and connect it to
the World Settings.

- C++
- Blueprint

1.  **Create the C++ class**

    - Open **Tools -\> New C++ Class** in the top menu, select
      **GameModeBase** as the parent, and click **Next**.
    - Select **Public** as the class type.
    - Name the class `BlackholioGameMode`.

2.  **Create a GameMode Blueprint**

    - In the **Content Drawer**, click **Add**, then select **Blueprint
      -\> Blueprint Class**.
    - Expand **All Classes**, search for `BlackholioGameMode`, highlight
      it, and click **Select**.
    - Name the blueprint `BP_BlackholioGameMode`.

3.  **Update World Settings**

    - Open **Window -\> World Settings** in the top menu.
    - Change **GameMode Override** from **None** to
      `BP_BlackholioGameMode`.
    - Save the level.

1.  **Create a GameMode Blueprint**

    - In the **Content Drawer**, click **Add**, then select **Blueprint
      -\> Blueprint Class**.
    - Expand **All Classes**, and click `Game Mode Base`.
    - Name the blueprint `BP_GameMode`.

2.  **Update World Settings**

    - Open **Window -\> World Settings** in the top menu.
    - Change **GameMode Override** from **None** to `BP_GameMode`.
    - Save the level.

At this point, the foundation of the Unreal project is set up. Pressing
Play will show a blank screen, but the game should start without errors.
Next, we’ll create the SpacetimeDB server module so we have something to
connect to.

### Create the Server Module

We've now got the very basics set up. In [part
2](https://spacetimedb.com/docs/tutorials/unreal/part-2) you'll learn the basics of how to
create a SpacetimeDB server module and how to connect to it from your
client.

{% endraw %}
