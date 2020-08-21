use super::{
    anchor::SectionAnchor,
    catalog_anchor::CourseCatalogAnchor, //I feel this is probably not needed
    entry::{Section, MAX_TITLE_LEN}, //MAX_Title should probably go as well, should entry though? 
};
use crate::anchor_trait::AnchorTrait;
use crate::helper;
use hdk::holochain_core_types::chain_header::ChainHeader;
use hdk::{LinkValidationData, ValidationData};
use holochain_entry_utils::HolochainEntry;

pub fn create(entry: Section, validation_data: ValidationData) -> Result<(), String> {
    helper::validate_only_teacher_can_do( //can only teachers create sections? If so good, else probably dont need this helper
        &entry.teacher_address,
        validation_data.sources(),
        "create course setions",
    )?;
    helper::validate_entity_title(&entry.title, &Section::entry_type()) //no need for title length here I guess
}

pub fn modify(
    new_entry: Section,
    old_entry: Section,
    _old_entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    helper::validate_only_teacher_can_do( //same as above, if needed cool - if not this should not be needed
        &old_entry.teacher_address,
        validation_data.sources(),
        "modify their sections",
    )?;
    helper::validate_entity_title(&new_entry.title, &Section::entry_type())?;
    validate_no_teacher_change(old_entry, new_entry) //if teachers matter
}

// this fn is only needed in the current module so it's private
fn validate_no_teacher_change(old_entry: Section, new_entry: Section) -> Result<(), String> {
    if new_entry.teacher_address != old_entry.teacher_address {
        return Err(String::from("Cannot change the teacher of the Section"));
    }
    Ok(())
}

pub fn delete(
    entry: Section,
    _entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    helper::validate_only_teacher_can_do(//unless students can create sections in which case this rule goes
        &entry.teacher_address,
        validation_data.sources(),
        "delete their Sections",
    )
}

// =========================== SectionAnchor validation
pub fn anchor_create(entry: SectionAnchor, validation_data: ValidationData) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &entry.teacher_address,
        validation_data.sources(),
        "create their sections",
    )?;
    helper::validate_entity_title(&entry.title, &SectionAnchor::entry_type()) //deleted max_title as theres no max entry in the anchor code.. propably no title anyway, its an anchor
}

// NOTE: we don't accept any parameters here because we don't need them to always return an error
// because this anchor can never be modified
pub fn anchor_modify() -> Result<(), String> {
    Err(String::from(
        "Can't modify the SectionAnchor entry: it can only be created or deleted",
    ))
}

pub fn anchor_delete(
    entry: SectionAnchor,
    _entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    helper::validate_only_teacher_can_do(//teachers only CRUD sections assumption still
        &entry.teacher_address,
        validation_data.sources(),
        "delete their section anchor",
    )
}

//  =========================== SectionAnchor links validation

pub fn anchor_to_section_link(validation_data: LinkValidationData) -> Result<(), String> {
    match validation_data {
        hdk::LinkValidationData::LinkAdd {
            link,
            validation_data,
        } => {
            // get author of this entry
            let author = validation_data.package.chain_header.provenances()[0].source();
            // get link base: entry from which the link goes
            let base: SectionAnchor = hdk::utils::get_as_type(link.link.base().clone())?;
            // get link target: entry to which the link goes
            let target: Section = hdk::utils::get_as_type(link.link.target().clone())?;
            if base.teacher_address != target.teacher_address {
                // notice that we're using return and ending this statement with ; symbol
                // You can do both: skip ; symbol in the last fn statement or explicitly add return to it and then leave ; as is
                return Err(String::from(
                    "Can't link SectionAnchor to Section because their teacher addresses are different",
                ));
            } else if author != base.teacher_address {
                return Err(String::from(
                    "Can't link SectionAnchor to Section because your address isn't specified as teacher address for this Section",
                ));
            }
            Ok(())
        }
        hdk::LinkValidationData::LinkRemove {
            link,
            validation_data,
        } => {
            // get author of this entry
            let author = validation_data.package.chain_header.provenances()[0].source();
            // get link base: entry from which the link goes
            let base: SectionAnchor = hdk::utils::get_as_type(link.link.base().clone())?;
            if author != base.teacher_address {
                return Err(String::from(
                    "Can't remove link from SectionAnchor to Section because your address isn't specified as teacher_address for this Section",
                ));
            }
            Ok(())
        }
    }
}
