mod filter;
mod info;
mod search;

use filter::{Filter, FilterColumn, Sort};
use fusion_datatypes::{FusedPokemon, Pokemon, Type};
use search::SearchColumn;

use gloo::console::log;
use indexmap::IndexSet;
use info::InfoColumn;
use leptos::*;
use serde_json::de::Read;
use std::{cmp::Ordering, collections::HashMap, rc::Rc};

// Generates number of pokemon and fusions so we can size vectors with capacity
#[macro_use]
extern crate build_const;
build_const!("constants");

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let pokemon: Rc<Vec<Pokemon>> =
        bincode::deserialize::<Vec<Pokemon>>(include_bytes!("../infinite_dex.bin"))
            .unwrap()
            .into();
    let sprite_map: Rc<HashMap<(u16, u16), u8>> =
        bincode::deserialize::<HashMap<(u16, u16), u8>>(include_bytes!("../sprite_count.bin"))
            .unwrap()
            .into();
    let abilities: Rc<IndexSet<&str>> =
        bincode::deserialize::<IndexSet<&str>>(include_bytes!("../abilities.bin"))
            .unwrap()
            .into();

    // Selection Signals
    let (selected_fusion, set_selected_fusion) = create_signal::<Option<FusedPokemon>>(None);
    let (selected_pokemon, set_selected_pokemon) = create_signal::<Option<Pokemon>>(None);

    // Sort Signals
    let (selected_sort, set_selected_sort) = create_signal::<Sort>(Sort::None);

    // Filter Signals
    let (filters, set_filters) = create_signal::<Filter>(Filter::default());

    const COL_STYLE: &str = "w-1/4 p-8 m-8 rounded-md bg-stone-500 drop-shadow-xl";

    view! {
        <section class="flex justify-center min-h-screen max-h-screen bg-slate-600">
            <div class=COL_STYLE><FilterColumn set_filters=set_filters set_sort=set_selected_sort ability_map=abilities.clone()/></div>
            <div class=COL_STYLE><SearchColumn ability_map=abilities sprite_map=sprite_map.clone() filters=filters selected_sort=selected_sort dex=pokemon pokemon_select_set=set_selected_pokemon fusion_select_set=set_selected_fusion/></div>
            <div class=COL_STYLE><InfoColumn selected_fusion=selected_fusion selected_pokemon=selected_pokemon sprite_map=sprite_map/></div>
        </section>
    }
}
