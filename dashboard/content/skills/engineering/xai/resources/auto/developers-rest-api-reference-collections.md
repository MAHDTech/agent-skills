+++
title = "developers-rest-api-reference-collections"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

{% raw %}
#### Collections API

# Collections REST API Overview

The Collections API allows you to manage your Collections `documents` and `collections` programmatically.

The base url for `collection` management is shared with [Management API](https://docs.x.ai/developers/rest-api-reference/management) at `https://management-api.x.ai/v1/`. You have to authenticate using **xAI Management API Key** with the header `Authorization: Bearer <your xAI Management API key>`.

> [!NOTE]
>
> For more details on provisioning xAI Management API key and using Management API, you can visit
>
> [Using Management API](https://docs.x.ai/developers/management-api-guide)
>
> .

The base url for searching within `collections` is shared with [REST API](https://docs.x.ai/developers/rest-api-reference) at `https://api.x.ai`. You have to authenticate with the header `Authorization: Bearer <your xAI API key>`.

* [Collection Management](https://docs.x.ai/developers/rest-api-reference/collections/collection)
* [Search in Collections](https://docs.x.ai/developers/rest-api-reference/collections/search)
{% endraw %}
