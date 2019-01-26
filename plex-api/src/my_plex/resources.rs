use crate::media_container::{Device, MediaContainer};
use crate::my_plex::{MyPlexAccount, MyPlexApiErrorResponse, MyPlexError, Result};
use reqwest::StatusCode;
use serde_xml_rs;

impl MyPlexAccount {
    /// Returns the list of resources, registered with current MyPlex account.
    ///
    /// Resource is any available device, despite of its `provides` setting.
    pub fn get_resources(&self) -> Result<Vec<Device>> {
        let mut response = self.get("https://plex.tv/api/resources")?;
        if response.status() == StatusCode::OK {
            let mc: MediaContainer = serde_xml_rs::from_str(response.text()?.as_str())?;
            let resources = mc.get_devices();
            Ok(resources.unwrap_or_default())
        } else {
            let err: MyPlexApiErrorResponse = serde_xml_rs::from_str(response.text()?.as_str())?;
            Err(MyPlexError::from(err))
        }
    }
}
