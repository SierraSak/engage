use unity::prelude::*;
use unity::system::List;
use crate::gamedata::{
    item::ItemData,
    animal::AnimalData,
};

#[unity::class("App", "HubAccessData")]
pub struct HubAccessData {
    pub aid: Option<&'static Il2CppString>,
    pub dispos_data: &'static HubDisposData,
    //
}

#[unity::class("App", "HubDisposData")]
pub struct HubDisposData {}

#[unity::class("App", "HubAccessManager")]
pub struct HubAccessManager {
    pub scene_name: &'static Il2CppString,
    pub access_list: &'static List<HubAccessData>,
    pub dispos_list: &'static List<HubDisposData>,
    pub dispos_item_list: &'static List<HubDisposData>,
    talk_limit: * const u8,
    pub animal_list: &'static List<AnimalData>, 
}

impl HubDisposData {
    pub fn get_locator(&self) -> &'static Il2CppString { unsafe { dispos_hub_get_locator(self, None) } }
}

impl HubAccessManager {
    pub fn get_not_taken_bond_frags(&self) -> i32 { unsafe { GetNotTakenPieceOfBond(self, None) }}
}

impl HubAccessData {
    // marks the access point as interacted
    pub fn done(&self) -> bool { unsafe { access_data_done(self, None)}}
    
    // Checks if access point is interacted
    pub fn get_is_done(&self) -> bool { unsafe { access_data_is_done(self, None)}}

    pub fn get_item_count(&self) -> i32 { unsafe { access_data_item_count(self, None) }}
    pub fn get_talk_item(&self) -> Option<&'static Il2CppString> { unsafe { hub_access_get_talk_item(self, None) }}

    pub fn is_animal(&self) -> bool { unsafe { access_data_is_animal(self, None)}}
    pub fn try_get_pid(&self) -> Option<&'static Il2CppString> { unsafe { access_data_try_get_pid(self, None) } }
}

#[unity::from_offset("App", "HubAccessData", "get_TalkItem")]
pub fn hub_access_get_talk_item(this: &HubAccessData, method_info: OptionalMethod) -> Option<&'static Il2CppString>;

#[skyline::from_offset(0x21733a0)]
pub fn GetNotTakenPieceOfBond(this: &HubAccessManager, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "HubDisposData", "get_Locator")]
pub fn dispos_hub_get_locator(this: &HubDisposData, method_info: OptionalMethod) -> &'static Il2CppString;

#[unity::from_offset("App", "HubAccessData", "get_IsDone")]
pub fn access_data_is_done(this: &HubAccessData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "HubAccessData", "DoneAccess")]
pub fn access_data_done(this: &HubAccessData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "HubAccessData", "get_ItemCount")]
pub fn access_data_item_count(this: &HubAccessData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "HubAccessData", "get_IsAnimal")]
pub fn access_data_is_animal(this: &HubAccessData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "HubAccessData", "TryGetPID")]
pub fn access_data_try_get_pid(this: &HubAccessData, method_info: OptionalMethod) -> Option<&'static Il2CppString>;
