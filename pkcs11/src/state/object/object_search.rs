use super::template::Template;

pub(crate) struct ObjectSearch {
    template: Template,
}

impl ObjectSearch {
    pub(crate) fn new(template: Template) -> Self {
        Self { template }
    }

    pub(crate) fn get_template(&self) -> &Template {
        &self.template
    }
}
