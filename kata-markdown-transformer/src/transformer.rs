use crate::links::Links;

pub struct Transformer {
    markdown: String,
    links: Links,
}

impl Transformer {
    pub fn new(markdown: &str) -> Self {
        Self {
            markdown: markdown.to_string(),
            links: Links::create(markdown.to_string()),
        }
    }

    pub fn transform(&self) -> String {
        let transformed =
            self.links
                .all_links()
                .iter()
                .fold(self.markdown.to_string(), |md, link| {
                    let index = self.links.index_of(&link.url);
                    let new_link = format!("{} [^anchor{}]", link.text, index.unwrap() + 1);

                    md.replace(link.link.as_str(), new_link.as_str())
                });
        let footnote = self
            .links
            .map_unique(|link, index| format!("[^anchor{}]: {}", index, link));
        let result: Vec<String> = std::iter::once(transformed).chain(footnote).collect();
        result.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_without_links_returns_same_text() {
        let transformer = Transformer::new("text without links");
        assert_eq!(transformer.transform(), String::from("text without links"));
    }

    #[test]
    fn transforms_one_link() {
        let transformer =
            Transformer::new("[this book](https://codigosostenible.com) and some other text.");
        assert_eq!(transformer.transform(), String::from("this book [^anchor1] and some other text.\n[^anchor1]: https://codigosostenible.com"));
    }

    #[test]
    fn transforms_many_links() {
        let transformer =
            Transformer::new("[this book](https://codigosostenible.com) and some [other book](https://example.com) are good choices.");
        assert_eq!(transformer.transform(), String::from("this book [^anchor1] and some other book [^anchor2] are good choices.\n[^anchor1]: https://codigosostenible.com\n[^anchor2]: https://example.com"));
    }

    #[test]
    fn transforms_many_repeated_links() {
        let transformer =
            Transformer::new("[this book](https://codigosostenible.com) and some [other book](https://example.com) are good choices. The best one is [the first](https://codigosostenible.com).");
        assert_eq!(transformer.transform(), String::from("this book [^anchor1] and some other book [^anchor2] are good choices. The best one is the first [^anchor1].\n[^anchor1]: https://codigosostenible.com\n[^anchor2]: https://example.com"));
    }
}
