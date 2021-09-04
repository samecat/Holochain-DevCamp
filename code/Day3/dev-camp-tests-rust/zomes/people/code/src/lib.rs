#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
extern crate boolinator;
use boolinator::Boolinator;
use hdk::{
    holochain_core_types::{
        hash::HashString,
        dna::entry_types::Sharing,
        json::JsonString,
        entry::Entry,
        error::HolochainError,
        cas::content::Address,
    },
    error::ZomeApiResult,
};
use holochain_wasm_utils::api_serialization::get_links::GetLinksResult;


#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct Person {
    name: String,
}

fn handle_add_person(name: String) -> ZomeApiResult<Address> {
    let person_entry = Entry::App(
            "person".into(),  //AppEntryType; use the entry! name
            Person {name: name}.into() //AppEntryValue; instantiate a Person on the fly with the input name
            //it's mere coincidence that struct member name is the same as function input name
        );
    hdk::commit_entry(&person_entry)
}

fn handle_update_person(address: HashString, name: String) -> ZomeApiResult<Address> {
    let person_entry = Entry::App(
            "person".into(),  //AppEntryType; use the entry! name
            Person {name: name}.into() //AppEntryValue; instantiate a Person on the fly with the input name
            //it's mere coincidence that struct member name is the same as function input name
        );
    hdk::update_entry(person_entry,&address)
}

fn handle_remove_person(address: HashString) -> ZomeApiResult<()> {
    hdk::remove_entry(&address)
}

fn handle_get_person(address: HashString) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}

fn handle_link_people(base: HashString, target: HashString, tag: String) -> ZomeApiResult<()> {
    hdk::link_entries(&base, &target, tag);
    Ok(())
}

fn handle_get_relationships(address: HashString, tag: String) -> ZomeApiResult<GetLinksResult> {
    hdk::get_links(&address, tag)
}

define_zome! {
    entries: [
        entry!(
            name: "person", //this is the entry! name
            description: "a human with a name; but not this",
            sharing: Sharing::Public,
            native_type: Person,  //need to define it as struct above, and needs to be used in validation input type below
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |person1: Person, _ctx: hdk::ValidationData| {
                if person1.name.len() < 2
                {
                    //Err(HolochainError::ErrorGeneric(String::from("Length of name is too short")))
                    Err(String::from("Length of name is too short"))
                }
                else {
                    Ok(())
                }
                
            },
            links: [
                to!(
                    "person", //target of link
                    tag: "is friends with", //name of link; enforced by get_links, otherwise you can fetch empty results
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                )
            ]
        )
    ]

    genesis: || { Ok(()) }

    functions: [
        add_person: {
            inputs: |name: String|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_add_person
        }
        update_person: {
            inputs: |address: HashString, name: String|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_update_person
        }
        remove_person: {
            inputs: |address: HashString|,
            outputs: |result: ZomeApiResult<()>|,
            handler: handle_remove_person
        }
        get_person: {
            inputs: |address: HashString|,
            outputs: |result: ZomeApiResult<Option<Entry>>|,
            handler: handle_get_person
        }
        link_people: {
            inputs: |base: HashString, target: HashString, tag: String|,
            outputs: |result: ZomeApiResult<()>|,
            handler: handle_link_people
        }
        get_relationships: {
            inputs: |address: HashString, tag: String|,
            outputs: |result: ZomeApiResult<GetLinksResult>|,
            handler: handle_get_relationships           
        }
    ]

    traits: {
        hc_public [add_person, update_person, remove_person, get_person, link_people, get_relationships]
    }
}
