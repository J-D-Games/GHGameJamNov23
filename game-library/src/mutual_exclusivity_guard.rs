//! Plugin macro for checking if mutually exclusive components exists on the same entity, and if so, panic.

use bevy::prelude::*;

pub fn mutually_exclusive_component_panic<A: Component, B: Component>(query: Query<(Entity, &A, &B, Option<&Name>)>) {
    if !query.is_empty() {
        let mut error = String::new();
        for (entity, _marker1, _marker2, name) in query.iter() {
            match name {
                Some(name) => {
                    error.push_str(format!("Entity {:?} with name {} has two mutually exclusive components, {} and {}.\n", entity, name, std::any::type_name::<A>(), std::any::type_name::<B>()).as_str());
                },
                None => {
                    error.push_str(format!("Entity {:?} has two mutually exclusive components, {} and {}.\n", entity, std::any::type_name::<A>(), std::any::type_name::<B>()).as_str());
                }
            }
        }
        panic!("{}", error);
    }
}

#[macro_export]
macro_rules! define_mutually_exclusive_components {
    ( $firt_type:ty, $second_type:ty ) => {
        (
            game_library::mutual_exclusivity_guard::mutually_exclusive_component_panic::<$firt_type, $second_type>, 
        )
    };
    ( $firt_type:ty, $second_type:ty, $($other_types:ty),+ ) => {
        (
            game_library::mutual_exclusivity_guard::mutually_exclusive_component_panic::<$firt_type, $second_type>, 
            $(
                game_library::mutual_exclusivity_guard::mutually_exclusive_component_panic::<$firt_type, $other_types>, 
            )+
            (define_mutually_exclusive_components!($second_type$(,$other_types)+))
        )
    };
}

pub use define_mutually_exclusive_components;