use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

use super::entry::Section;
use crate::anchor_trait::AnchorTrait;

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct SectionAnchor {
    // NOTE: these fields are here to ensure the uniqueness of every particular anchor
    //  and wouldn't be used to display data about course to a user
    pub title: String,
    pub teacher_address: Address,
    pub timestamp: u64,
}

impl AnchorTrait for SectionAnchor {
    fn entry_type() -> String {
        String::from("section_anchor")
    }
    fn link_to() -> String {
        Section::entry_type()
    }
    fn link_type() -> String {
        "section_anchor->section".to_owned()
    }
}

impl SectionAnchor {
    pub fn new(title: String, teacher_address: Address, timestamp: u64) -> Self {
        SectionAnchor {
            title: title,
            teacher_address: teacher_address,
            timestamp: timestamp,
        }
    }
}

pub fn section_anchor_def() -> ValidatingEntryType {
    entry!(
        name: SectionAnchor::entry_type(),
        description: "Anchor to the valid section",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<SectionAnchor>| {
            match validation_data{
                EntryValidationData::Create { .. } => {
                    Ok(())
                 },
                 EntryValidationData::Modify { .. } => {
                    Ok(())
                 },
                 EntryValidationData::Delete { .. } => {
                    Ok(())
                 }
            }
        },
        links:[]
    )
}