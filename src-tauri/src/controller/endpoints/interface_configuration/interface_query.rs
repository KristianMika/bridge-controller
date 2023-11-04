use serde::Deserialize;

/// Holds query parameters for the interface endpoint.
#[derive(Deserialize)]
pub struct InterfaceQuery {
    tool: Option<String>,
}

impl InterfaceQuery {
    pub(crate) fn into_tool(self) -> Option<String> {
        self.tool
    }
}
