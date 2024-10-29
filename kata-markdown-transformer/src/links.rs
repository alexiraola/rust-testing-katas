use std::collections::HashSet;

use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub struct Link {
    pub link: String,
    pub text: String,
    pub url: String,
}

pub struct Links {
    links: Vec<Link>,
    unique: Vec<String>,
}

impl Links {
    pub fn create(markdown: String) -> Links {
        let links = Links::parse_links(markdown);
        let unique = Links::unique_links(links.as_ref());

        Links { links, unique }
    }

    pub fn all_links(&self) -> Vec<Link> {
        self.links.clone()
    }

    pub fn index_of(&self, url: &String) -> Option<usize> {
        self.unique.iter().position(|link| link == url)
    }

    pub fn map_unique(&self, transform: fn(&String, usize) -> String) -> Vec<String> {
        self.unique
            .iter()
            .enumerate()
            .map(|(index, link)| transform(link, index + 1))
            .collect()
    }

    fn parse_links(markdown: String) -> Vec<Link> {
        let mut links: Vec<Link> = vec![];
        let pattern = r"\[([^\]]+)\]\(([^\)]+)\)";
        let re = Regex::new(pattern).unwrap();
        for cap in re.captures_iter(&markdown) {
            links.push(Link {
                link: cap[0].to_string(),
                text: cap[1].to_string(),
                url: cap[2].to_string(),
            })
        }
        links
    }

    fn unique_links(links: &Vec<Link>) -> Vec<String> {
        let mut unique_links: Vec<String> = links
            .iter()
            .fold(HashSet::new(), |mut unique_links, link| {
                unique_links.insert(link.url.clone());
                unique_links
            })
            .into_iter()
            .collect();
        unique_links.sort();
        unique_links
    }
}
