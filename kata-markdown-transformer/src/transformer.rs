use regex::Regex;

pub struct Transformer {
    markdown: String,
    links: Vec<Link>,
}
impl Transformer {
    fn new(markdown: &str) -> Self {
        Self {
            markdown: markdown.to_string(),
            links: vec![],
        }
    }

    fn transform(&self) -> String {
        self.markdown.clone()
    }

    fn links(&self) -> Vec<Link> {
        let mut links: Vec<Link> = vec![];
        let pattern = r"\[([^\]]+)\]\(([^\)]+)\)";
        let re = Regex::new(pattern).unwrap();
        for cap in re.captures_iter(&self.markdown) {
            links.push(Link {
                text: cap[1].to_string(),
                url: cap[2].to_string(),
            })
        }
        links
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Link {
    text: String,
    url: String,
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
    fn finds_links() {
        let transformer = Transformer::new("[link](https://link.com)");

        assert_eq!(
            transformer.links(),
            vec![Link {
                text: "link".to_string(),
                url: "https://link.com".to_string()
            }]
        );
    }
}
