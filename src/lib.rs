use stride_plugin::{
    emit,
    event::{HostEvent, NetworkRequestType, PluginEvent},
    plugin,
    task::{Annotation, Task},
    Plugin, Storage,
};

use scraper::{Html, Selector};

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
enum RecordType {
    Job,
    Project,
    News,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Record {
    ty: RecordType,
    link: String,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
struct FinkiPlugin {
    records: Vec<Record>,
}

const SELECTORS: &[(&str, RecordType, &str)] = &[
    (
        "Check Finki Proxy",
        RecordType::Job,
        ".view-jobs-list a:not(:has(img))",
    ),
    (
        "Check Finki Project",
        RecordType::Project,
        ".view-projects-list a:not(:has(img))",
    ),
    (
        "Check Finki News",
        RecordType::News,
        ".view-announcements-list a:not(:has(img))",
    ),
];

impl Plugin for FinkiPlugin {
    fn init() -> Self {
        Storage::get("self")
            .expect("plugin should have storage access")
            .unwrap_or_default()
    }

    // Simple plugin that clones every task that is created with a suffix.
    fn event(&mut self, event: HostEvent) -> bool {
        match event {
            HostEvent::Timer { .. } => emit(&PluginEvent::NetworkRequest {
                ty: NetworkRequestType::Get,
                host: "https://www.finki.ukim.mk/".to_string(),
            }),
            HostEvent::NetworkResponse { content, .. } => {
                let content = String::from_utf8_lossy(&content);
                let html = Html::parse_document(&content);

                for (prefix, ty, selector) in SELECTORS {
                    let selector = Selector::parse(selector).unwrap();
                    for node in html.select(&selector) {
                        let Some(href) = node.attr("href") else {
                            continue;
                        };
                        if self.records.iter().any(|record| record.link == href) {
                            continue;
                        }
                        let text = node.text().collect::<String>();
                        self.records.push(Record {
                            ty: *ty,
                            link: href.to_string(),
                        });
                        Storage::set("self", self).expect("should have storage write permissions");

                        let mut link = String::new();
                        if href.starts_with("/") {
                            link.push_str("https://www.finki.ukim.mk");
                            link.push_str(href);
                        } else {
                            link.push_str(href);
                        }
                        let mut task = Task::new(format!("{prefix} {text}"));
                        task.annotations
                            .push(Annotation::now(format!("https://www.finki.ukim.mk/{href}")));
                        emit(&PluginEvent::TaskCreate { task });
                    }
                }

                return true;
            }
            _ => return false,
        }

        true
    }
}

// Register plugin.
plugin!(FinkiPlugin);
