use crate::filter::{Filter, Sort};

use fusion_datatypes::{FusedPokemon, Pokemon};
use indexmap::IndexSet;
use leptos::*;
use std::collections::HashMap;
use std::rc::Rc;

// Generates number of pokemon and fusions so we can size vectors with capacity
build_const!("constants");

//TODO! split this up into more components
#[component]
pub fn SearchColumn(
    dex: Rc<Vec<Pokemon>>,
    sprite_map: Rc<HashMap<(u16, u16), u8>>,
    fusion_select_set: WriteSignal<Option<FusedPokemon>>,
    pokemon_select_set: WriteSignal<Option<Pokemon>>,
    ability_map: Rc<IndexSet<&'static str>>,
    filters: ReadSignal<Filter>,
    selected_sort: ReadSignal<Sort>,
) -> impl IntoView {
    let (fusions, fusions_set) = create_signal::<Option<Vec<FusedPokemon>>>(None);

    view! {
        <select on:change={
            let dex = dex.clone();
            move |event| {
            let index = event_target_value(&event).parse::<usize>().unwrap();
            pokemon_select_set.set(Some(dex[index].clone()));
            fusions_set.set(Some(generate_fusions(index, &dex)));
        }}>
            <option disabled selected value> "Select a Pokémon" </option>
            {dex.iter().enumerate().map(|(index,pokemon)| view! { <option value={index}>{&pokemon.name}</option> }).collect_view() }
        </select>
        {move || if let Some(fusions) = fusions.get(){
            view!{
            <div class="overflow-y-scroll max-h-full h-auto mt-4">
            {
            let filter_state = {move || filters.get()}();
            let mut fusions_to_display = fusions.into_iter()
            // Type filter
            .filter(|fusion| filter_state.filter_stats(fusion))
            .filter(|fusion| if filter_state.custom_sprite {sprite_map.get(&(fusion.head_id, fusion.body_id)).is_some()} else {true})
            .filter(|fusion| if let Some(ability_index) = filter_state.has_ability {fusion.abilities.iter().any(|(ability,_)| ability_map.get_index_of(ability.as_str()).unwrap() == ability_index)} else {true})
            .collect::<Vec<_>>();

            let sort = match {move || selected_sort.get()}() {
                Sort::None => {|_: &FusedPokemon, _: &FusedPokemon| std::cmp::Ordering::Equal},
                Sort::Bst => {|a: &FusedPokemon, b: &FusedPokemon| b.bst.cmp(&a.bst)},
                Sort::Hp => {|a: &FusedPokemon, b: &FusedPokemon| b.hp.cmp(&a.hp)},
                Sort::Attack => {|a: &FusedPokemon, b: &FusedPokemon| b.attack.cmp(&a.attack)},
                Sort::Defense => {|a: &FusedPokemon, b: &FusedPokemon| b.defense.cmp(&a.defense)},
                Sort::SpecialAttack => {|a: &FusedPokemon, b: &FusedPokemon| b.special_attack.cmp(&a.special_attack)},
                Sort::SpecialDefense => {|a: &FusedPokemon, b: &FusedPokemon| b.special_defense.cmp(&a.special_defense)},
                Sort::Speed => {|a: &FusedPokemon, b: &FusedPokemon| b.speed.cmp(&a.speed)},
            };

            fusions_to_display.sort_by(sort);

            // We have to clone twice here, once for the closure and once for the view
            // I hate web development
            fusions_to_display.iter().map(|fusion| view!{<p class="cursor-pointer" on:click={
                let fusion=fusion.clone();
                move |_| fusion_select_set.set(Some(fusion.clone()))}>{&fusion.name}</p>}).collect_view()
        }
            </div>
            }.into_view()
        } else {
            view! {}.into_view()
        }}
    }
}

// Imperative style is easier than functional style for this due to the loop returning two values at once
// Doing functional style might be faster but this is fine
fn generate_fusions(target: usize, dex: &[Pokemon]) -> Vec<FusedPokemon> {
    let mut fusions: Vec<FusedPokemon> = Vec::with_capacity(NUMBER_OF_FUSIONS);
    for (index, pokemon) in dex.iter().enumerate() {
        if index == target {
            continue;
        }
        fusions.push(FusedPokemon::fuse(&dex[target], pokemon));
        fusions.push(FusedPokemon::fuse(pokemon, &dex[target]));
    }
    fusions.push(FusedPokemon::fuse(&dex[target], &dex[target]));
    fusions
}
