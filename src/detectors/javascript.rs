use super::Detector;

pub struct Javascript {}

impl Javascript {
    pub fn new() -> Javascript {
        Javascript {}
    }
}

impl Detector for Javascript {
    fn markers(&self) -> Vec<String> {
        vec!["package.json", "node_modules"]
            .iter()
            .map(|m| m.to_string())
            .collect()
    }

    fn deletables(&self) -> Vec<String> {
        vec!["node_modules", "dist", "public", ".cache"]
            .iter()
            .map(|m| m.to_string())
            .collect()
    }
}
