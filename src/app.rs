use crate::cli::RefractorArgs;
use crate::mirrors::{MirrorMeta, SortCondition};
use clap::Parser;
use std::io::Write;
use tabwriter::TabWriter;

pub struct App<'a> {
    mirror_meta: &'a MirrorMeta,
}

impl<'a> App<'a> {
    pub async fn start(mirror_meta: &MirrorMeta) {
        let app = App { mirror_meta };
        let args = RefractorArgs::parse();

        if args.list_countries {
            app.list_countries();
        }
        else if args.fastest.is_some() {
            app.sort_mirrors().await;
        }
    }

    fn list_countries(&self) {
        let mut tw = TabWriter::new(std::io::stdout()).padding(1);
        tw.write_all(self.mirror_meta.country_wise_count().as_bytes())
            .unwrap();
        tw.flush().unwrap();
    }

    async fn sort_mirrors(&self) {
        self.mirror_meta.sort(SortCondition::Score).await;
    }
}
