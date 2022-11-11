use crate::mirrors::MirrorMeta;
use crate::cli::RefractorArgs;
use std::io::Write;
use clap::Parser;
use tabwriter::TabWriter;

pub struct App<'a> {
    mirror_meta: &'a MirrorMeta
}

impl<'a> App<'a> {
    pub fn start(mirror_meta: &MirrorMeta) {
        let app = App { mirror_meta };
        let args = RefractorArgs::parse();
        
        if args.list_countries {
            app.list_countries();
        }
    }
    
    fn list_countries(&self) {
        let mut tw = TabWriter::new(std::io::stdout()).padding(1);
        tw.write_all(self.mirror_meta.country_wise_count().as_bytes()).unwrap();
        tw.flush().unwrap();
    }
}
