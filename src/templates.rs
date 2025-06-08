use crate::{Blogpost, Page};
use maud::{DOCTYPE, Markup, PreEscaped, html};
use pichu::Markdown;

#[derive(Debug)]
pub struct Context {
    pub title: String,
    pub css_hash: String,
}

impl Context {
    fn wrap(&self, title: impl Into<Option<String>>, content: Markup) -> Markup {
        html! {
            (DOCTYPE)
            head {
                title { (title.into().map(|title| format!("{} – {}", self.title, title)).unwrap_or(self.title.clone())) }
                meta charset="utf-8";
                link rel="stylesheet" href=(format!("/main.css?hash={}", self.css_hash));
            }
            main {
                nav {
                    li {
                        a href="/" { "Home" }
                    }
                    li {
                        a href="/blog" { "Blog" }
                    }
                    li {
                        a href="/about" { "About" }
                    }
                }
                (content)
            }
        }
    }

    pub fn render_index(&self) -> Markup {
        self.wrap(
            None,
            html! {
                section {
                    h1 { (self.title) }
                    p {
                        "Welcome to the pichu starter. Click around!"
                    }
                }
            },
        )
    }

    pub fn render_blogpost(&self, post: &Markdown<Blogpost>) -> Markup {
        self.wrap(
            post.frontmatter.title.clone(),
            html! {
                article {
                    h1 { (post.frontmatter.title) }
                    span { (post.frontmatter.published.format("%Y-%m-%d") )}
                    (PreEscaped(&post.html))
                }
            },
        )
    }

    pub fn render_blog(&self, posts: &Vec<Markdown<Blogpost>>) -> Markup {
        self.wrap(
            "Blog".to_string(),
            html! {
                section {
                    h1 { "Blog" }
                    ul {
                        @for post in posts {
                            li {
                                a href=(format!("/blog/{}", post.basename)) { (post.frontmatter.title) }
                            }
                        }
                    }
                }
            },
        )
    }

    pub fn render_page(&self, page: &Markdown<Page>) -> Markup {
        self.wrap(
            page.frontmatter.title.clone(),
            html! {
                section {
                    h1 { (page.frontmatter.title) }
                    (PreEscaped(&page.html))
                }
            },
        )
    }
}
