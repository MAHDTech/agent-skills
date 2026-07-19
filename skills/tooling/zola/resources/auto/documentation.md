Title: Overview \| Zola URL Source:
https://www.getzola.org/documentation/ Published Time: Sun, 19 Jul 2026
07:29:22 GMT Markdown Content: \##
\[🔗\](https://www.getzola.org/documentation/#zola-at-a-glance)Zola at a
Glance Zola is a static site generator (SSG), similar to
\[Hugo\](https://gohugo.io/), \[Pelican\](https://blog.getpelican.com/),
and \[Jekyll\](https://jekyllrb.com/) (for a comprehensive list of SSGs,
please see \[Jamstack\](https://jamstack.org/generators)). It is written
in \[Rust\](https://www.rust-lang.org/) and uses the
\[Tera\](https://keats.github.io/tera/) template engine, which is
similar to \[Jinja2\](https://jinja.palletsprojects.com/en/2.10.x/),
\[Django
templates\](https://docs.djangoproject.com/en/2.2/topics/templates/),
\[Liquid\](https://shopify.github.io/liquid/), and
\[Twig\](https://twig.symfony.com/). Content is written in
\[CommonMark\](https://commonmark.org/), a strongly defined, highly
compatible specification of
\[Markdown\](https://www.markdownguide.org/). Zola uses
\[pulldown-cmark\](https://github.com/raphlinus/pulldown-cmark#pulldown-cmark)
to parse markdown files. The goal of this library is 100% compliance
with the CommonMark spec. It adds a few additional features such as
parsing footnotes, Github flavored tables, Github flavored task lists
and strikethrough. SSGs use dynamic templates to transform content into
static HTML pages. Static sites are thus very fast and require no
databases, making them easy to host. A comparison between static and
dynamic sites, such as WordPress, Drupal, and Django, can be found
\[here\](https://dev.to/ashenmaster/static-vs-dynamic-sites-61f). To get
a taste of Zola, please see the quick overview below. \##
\[🔗\](https://www.getzola.org/documentation/#first-steps-with-zola)First
Steps with Zola Unlike some SSGs, Zola makes no assumptions regarding
the structure of your site. In this overview, we'll be making a simple
blog site. \###
\[🔗\](https://www.getzola.org/documentation/#initialize-site)Initialize
Site \> This overview is based on Zola 0.19.1. Please see the detailed
\[installation instructions for your
platform\](https://www.getzola.org/documentation/getting-started/installation/).
With Zola installed, let's initialize our site: \`\$ zola init myblog\`
You will be asked a few questions. \`\`\` \> What is the URL of your
site? (https://example.com): \> Do you want to enable Sass compilation?
\[Y/n\]: \> Do you want to enable syntax highlighting? \[y/N\]: \> Do
you want to build a search index of the content? \[y/N\]: \`\`\` For our
blog, let's accept the default values (i.e., press Enter for each
question). We now have a \`myblog\` directory with the following
structure: \`\`\` ├── zola.toml ├── content ├── sass ├── static ├──
templates └── themes \`\`\` For reference, by the \*\*end\*\* of this
overview, our \`myblog\` directory will have the following structure:
\`\`\` ├── zola.toml ├── content/ │ └── blog/ │ ├── \_index.md │ ├──
first.md │ └── second.md ├── sass/ ├── static/ ├── templates/ │ ├──
base.html │ ├── blog-page.html │ ├── blog.html │ └── index.html └──
themes/ \`\`\` Change directory into the newly-created \`myblog\`
directory. \###
\[🔗\](https://www.getzola.org/documentation/#templates)Templates We'll
first create some templates to describe the structure of our site. \####
\[🔗\](https://www.getzola.org/documentation/#home-page-template)Home
Page Template Let's make a template for a home page. Create
\`templates/base.html\` with the following content. This step will make
more sense as we move through this overview. \`\`\`

{% block content %} {% endblock content %}

\`\`\` Now, let's create \`templates/index.html\` with the following
content. \`\`\` {% extends "base.html" %} {% block content %}

# This is my blog made with Zola.

{% endblock content %} \`\`\` This tells Zola that \`index.html\`
extends our \`base.html\` file and replaces the block called "content"
with the text between the \`{% block content %}\` and \`{% endblock
content %}\` tags. \####
\[🔗\](https://www.getzola.org/documentation/#blog-template)Blog
Template To create a template for a page that lists all blog posts,
create \`templates/blog.html\` with the following content. \`\`\` {%
extends "base.html" %} {% block content %}

# {{ section.title }}

- {% for page in section.pages %}
- [{{ page.title }}](https://r.jina.ai/https://www.getzola.org/documentation/%7B%7B%20page.permalink%20%7C%20safe%20%7D%7D)
  {% endfor %}

{% endblock content %} \`\`\` As done by \`index.html\`, \`blog.html\`
extends \`base.html\`, but in this template we want to list the blog
posts. Here we also see expressions such as \`{{ section.\[...\] }}\`
and \`{{ page.\[...\] }}\` which will be replaced with values from our
\[content\](https://www.getzola.org/documentation/getting-started/overview/#content)
when zola combines content with this template to render a page. In the
list below the header, we loop through all the pages in our section
(\`blog\` directory; more on this when we create content) and output
each page title and URL using \`{{ page.title }}\` and \`{{
page.permalink \| safe }}\`, respectively. We use the \`\| safe\` filter
because the permalink doesn't need to be HTML escaped (escaping would
cause \`/\` to render as \`/\`). \####
\[🔗\](https://www.getzola.org/documentation/#blog-post-template)Blog
Post Template We have templates describing our home page and a page that
lists all blog posts. Let's now create a template for an individual blog
post. Create \`templates/blog-page.html\` with the following content.
\`\`\` {% extends "base.html" %} {% block content %}

# {{ page.title }}

**{{ page.date }}**

{{ page.content \| safe }} {% endblock content %} \`\`\` \> Note the
\`\| safe\` filter for \`{{ page.content }}\`. \###
\[🔗\](https://www.getzola.org/documentation/#zola-live-reloading)Zola
Live Reloading Now that we've outlined our site's structure, let's start
the Zola development server in the \`myblog\` directory. \`\`\` \$ zola
serve Building site... Checking all internal links with anchors. \>
Successfully checked 0 internal link(s) with anchors. -\> Creating 0
pages (0 orphan) and 0 sections Done in 13ms. Web server is available at
http://127.0.0.1:1111 Listening for changes in
.../myblog/{zola.toml,content,sass,static,templates} Press Ctrl+C to
stop \`\`\` If you point your web browser to
\[http://127.0.0.1:1111\](http://127.0.0.1:1111/), you will see a
message saying, "This is my blog made with Zola." If you go to
\[http://127.0.0.1:1111/blog/\](http://127.0.0.1:1111/blog/), you will
currently get a 404 which we will fix next. \###
\[🔗\](https://www.getzola.org/documentation/#content)Content We'll now
create some content that Zola will use to generate site pages based on
our templates. \####
\[🔗\](https://www.getzola.org/documentation/#sections)Sections We'll
start by creating \`content/blog/\_index.md\`. This file tells Zola that
\`blog\` is a
\[section\](https://www.getzola.org/documentation/content/section/),
which is how content is categorized in Zola. In the \`\_index.md\` file,
we'll set the following variables in
\[TOML\](https://github.com/toml-lang/toml) format: \`\`\` +++ title =
"List of blog posts" sort_by = "date" template = "blog.html"
page_template = "blog-page.html" +++ \`\`\` \> Note that although no
variables are mandatory, the opening and closing \`+++\` are required.
\* \_sort\\by = "date"\_ tells Zola to use the date to order our section
pages (more on pages below). \* \_template = "blog.html"\_ tells Zola to
use \`templates/blog.html\` as the template for listing the Markdown
files in this section. \* \_page\\template = "blog-page.html"\_ tells
Zola to use \`templates/blog-page.html\` as the template for individual
Markdown files. For a full list of section variables, please see the
\[section\](https://www.getzola.org/documentation/content/section/)
documentation. The value of our \`title\` variable here is available to
templates such as \`blog.html\` as \`{{ section.title }}\`. If you now
go to \[http://127.0.0.1:1111/blog/\](http://127.0.0.1:1111/blog/), you
will see an empty list of posts. \####
\[🔗\](https://www.getzola.org/documentation/#markdown)Markdown We'll
now create some blog posts. Create \`content/blog/first.md\` with the
following content. \`\`\` +++ title = "My first post" date = 2019-11-27
+++ This is my first blog post. \`\`\` The \_title\_ and \_date\_ will
be available to us in the \`blog-page.html\` template as \`{{ page.title
}}\` and \`{{ page.date }}\`, respectively. All text below the closing
\`+++\` will be available to templates as \`{{ page.content }}\`. If you
now go back to our blog list page at
\[http://127.0.0.1:1111/blog/\](http://127.0.0.1:1111/blog/), you should
see our lonely post. Let's add another. Create
\`content/blog/second.md\` with the contents: \`\`\` +++ title = "My
second post" date = 2019-11-28 +++ This is my second blog post. \`\`\`
Back at \[http://127.0.0.1:1111/blog/\](http://127.0.0.1:1111/blog/),
our second post shows up on top of the list because it's newer than the
first post and we had set \_sort\\by = "date"\_ in our \`\_index.md\`
file. As a final step, let's modify \`templates/index.html\` (our home
page) to link to our list of blog posts: \`\`\` {% extends "base.html"
%} {% block content %}

# This is my blog made with Zola.

[Posts](https://r.jina.ai/https://www.getzola.org/documentation/%7B%7B%20get_url(path='@/blog/_index.md')%20%7D%7D).

{% endblock content %} \`\`\` This has been a quick overview of Zola.
You can now dive into the rest of the documentation.
