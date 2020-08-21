use super::{ //Ummmm above should all probably get deleted but since content is linked to section anchor I've made changes. Also I feel like super is the wrong "expression" for retrieving section anchor as its in another directory
    anchor::SectionAnchor,
    entry::{Content},
}; 
use crate::anchor_trait::AnchorTrait;
use crate::helper;
use hdk::holochain_core_types::chain_header::ChainHeader;
use hdk::{LinkValidationData, ValidationData};
use holochain_entry_utils::HolochainEntry;

pub fn create(entry: Content, validation_data: ValidationData) -> Result<(), String> {
    helper::validate_only_teacher_can_do( //only teachers to create content? 
        &entry.teacher_address,
        validation_data.sources(),
        "create their courses",
    )?;
    helper::validate_entity_title(&entry.title, &Content::entry_type())
}

pub fn modify(
    new_entry: Content,
    old_entry: Content,
    _old_entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    helper::validate_only_teacher_can_do( //teachers only assumption, else delete rule
        &old_entry.teacher_address,
        validation_data.sources(),
        "modify their content",
    )?;
    helper::validate_entity_title(&new_entry.title, &Course::entry_type())?;
    validate_no_teacher_change(old_entry, new_entry)
}

// this fn is only needed in the current module so it's private
fn validate_no_teacher_change(old_entry: Content, new_entry: Contetnt) -> Result<(), String> {
    if new_entry.teacher_address != old_entry.teacher_address {
        return Err(String::from("Cannot change the teacher of the content"));
    }
    Ok(())
}

pub fn delete(
    entry: Content,
    _entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &entry.teacher_address,
        validation_data.sources(),
        "delete their content",
    )
}

// =========================== ContentAnchor validation
/* Pointers changed to direct to the section anchor, though I reckon its an unnecessary section hence commenting it out

pub fn anchor_create(entry: SectionAnchor, validation_data: ValidationData) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &entry.teacher_address,
        validation_data.sources(),
        "create their content",
    )?;
    helper::validate_entity_title(&entry.title, &SectionAnchor::entry_type())
}

// NOTE: we don't accept any parameters here because we don't need them to always return an error
// because this anchor can never be modified
pub fn anchor_modify() -> Result<(), String> {
    Err(String::from(
        "Can't modify the CourseAnchor entry: it can only be created or deleted",
    ))
}

pub fn anchor_delete(
    entry: SectionAnchor,
    _entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &entry.teacher_address,
        validation_data.sources(),
        "delete their content",
    )
}
*/ 