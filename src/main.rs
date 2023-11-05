use fusion_datatypes::{FusedPokemon, Pokemon, Type};

use gloo::console::log;
use leptos::*;
use std::{cmp::Ordering, collections::HashMap, rc::Rc};
const NUMBER_OF_POKEMON: usize = 426;
const NUMBER_OF_FUSIONS: usize = (NUMBER_OF_POKEMON * 2) - 1;

#[derive(Debug, Clone, Copy)]
enum Sort {
    None,
    Bst,
    Hp,
    Attack,
    Defense,
    SpecialAttack,
    SpecialDefense,
    Speed,
}

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

    // Selection Signals
    let (selected_fusion, set_selected_fusion) = create_signal::<Option<Rc<FusedPokemon>>>(None);
    let (selected_pokemon, set_selected_pokemon) = create_signal::<Option<Rc<Pokemon>>>(None);

    // Sort Signals
    let (selected_sort, set_selected_sort) = create_signal::<Sort>(Sort::None);

    // Filter Signals
    let (filter_type, set_filter_type) = create_signal::<Option<Type>>(None);
    let (filter_min_hp, set_filter_min_hp) = create_signal::<u8>(0);

    view! {
        <section class="flex justify-center min-h-screen max-h-screen bg-slate-600">
            <div class="w-1/4 p-8 m-8 rounded-md bg-stone-500 drop-shadow-xl"><FilterColumn set_sort=set_selected_sort set_filter_type=set_filter_type/></div>
            <div class="w-1/4 p-8 m-8 rounded-md bg-stone-500 drop-shadow-xl"><SearchColumn selected_sort=selected_sort filter_type=filter_type dex=pokemon pokemon_select_set=set_selected_pokemon fusion_select_set=set_selected_fusion/></div>
            <div class="w-1/4 p-8 m-8 rounded-md bg-stone-500 drop-shadow-xl"><InfoColumn selected_fusion=selected_fusion selected_pokemon=selected_pokemon sprite_map=sprite_map/></div>
        </section>
    }
}

#[component]
fn FilterColumn(
    set_filter_type: WriteSignal<Option<Type>>,
    set_sort: WriteSignal<Sort>,
) -> impl IntoView {
    let types: Rc<Vec<Option<Type>>> = vec![
        None,
        Some(Type::Bug),
        Some(Type::Dark),
        Some(Type::Dragon),
        Some(Type::Electric),
        Some(Type::Fairy),
        Some(Type::Fighting),
        Some(Type::Fire),
        Some(Type::Flying),
        Some(Type::Ghost),
        Some(Type::Grass),
        Some(Type::Ground),
        Some(Type::Ice),
        Some(Type::Normal),
        Some(Type::Poison),
        Some(Type::Psychic),
        Some(Type::Rock),
        Some(Type::Steel),
        Some(Type::Water),
    ]
    .into();

    let sorts: Rc<Vec<Sort>> = vec![
        Sort::None,
        Sort::Bst,
        Sort::Hp,
        Sort::Attack,
        Sort::Defense,
        Sort::SpecialAttack,
        Sort::SpecialDefense,
        Sort::Speed,
    ]
    .into();

    view! {
        <div>
            <select on:change={
                let types = types.clone();
                move |event| {
                    let index = event_target_value(&event).parse::<usize>().unwrap();
                    set_filter_type.set(types[index]);
                }
            }>
                {
                    move || types.iter().enumerate().map(|(index,option)| view! { <option value={index}>{match option{
                    Some(t) => format!("{}", t),
                    None => "None".to_string()
                }}</option> }).collect_view() }
            </select>
           <select on:change={
                let sorts = sorts.clone();
                move |event| {
                     let index = event_target_value(&event).parse::<usize>().unwrap();
                     set_sort.set(sorts[index]);
                }
              }>
                {
                     move || sorts.iter().enumerate().map(|(index,sort)| view! { <option value={index}>{match sort{
                     Sort::None => "None".to_string(),
                     Sort::Bst => "BST".to_string(),
                     Sort::Hp => "HP".to_string(),
                     Sort::Attack => "Attack".to_string(),
                     Sort::Defense => "Defense".to_string(),
                     Sort::SpecialAttack => "Special Attack".to_string(),
                     Sort::SpecialDefense => "Special Defense".to_string(),
                     Sort::Speed => "Speed".to_string(),
                }}</option> }).collect_view() }
           </select>
        </div>
    }
}

#[component]
fn InfoColumn(
    selected_fusion: ReadSignal<Option<Rc<FusedPokemon>>>,
    selected_pokemon: ReadSignal<Option<Rc<Pokemon>>>,
    sprite_map: Rc<HashMap<(u16, u16), u8>>,
) -> impl IntoView {
    view! {
        <div>
            <div>{move || {
                if let Some(fusion) = selected_fusion.get() {
                view! {
                    <div class="flex">
                    {
                        let sprite_map = sprite_map.clone();
                        let head_id = fusion.head_id;
                        let body_id = fusion.body_id;
                        let sprite_count = sprite_map.get(&(head_id, body_id));
                        if let Some(sprite_count) = sprite_count{
                            let letters = (0..*sprite_count).map(map_to_alphabet).collect::<Vec<&str>>();
                            let (current_letter, set_current_letter) = create_signal::<usize>(0);
                            //letters.iter().map(|letter| view!{<img src={format!("https://raw.githubusercontent.com/SneedSeedFeed/FusionFinderAssets/rust/{}.{}{}.png", head_id, body_id, letter)}/>}).collect_view()
                            let len = letters.len();
                            view!{
                                <div class="flex flex-col">
                                    <div class="flex justify-center">
                                        <p>{move || format!("Sprite {}/{}", current_letter.get()+1, len)}</p>
                                        <img class={if len > 1 {"cursor-pointer"} else {""}} on:click={
                                            move |_| {
                                            set_current_letter.set((current_letter.get() + 1)% len)}}
                                            src={move || format!("https://raw.githubusercontent.com/SneedSeedFeed/FusionFinderAssets/rust/{}.{}{}.png", head_id, body_id, letters[current_letter.get()])}/>
                                    </div>
                                </div>
                            }.into_view()
                        }
                        else{
                            view!{"No Custom Sprites for this Pokémon ;w;"}.into_view()
                        }
                    }
                    </div>
                    <div>
                        {move ||
                            if let Some(pokemon) = selected_pokemon.get() {
                                view!{<StatDisplay fusion=fusion.clone() base=pokemon.clone()/>}
                            } else {
                                view! {}.into_view()
                            }
                        }
                    </div>
                }.into_view()
            } else {
                view! {}.into_view()
            }}}</div>
        </div>
    }
}

#[component]
fn StatDisplay(fusion: Rc<FusedPokemon>, base: Rc<Pokemon>) -> impl IntoView {
    view! {
        <p>{&fusion.name}</p>
        <div class="grid-cols-3 grid w-3/4">
        <StatDisplayCell new_stat=fusion.hp base_stat=base.hp stat_label="HP"/>
        <StatDisplayCell new_stat=fusion.attack base_stat=base.attack stat_label="Attack"/>
        <StatDisplayCell new_stat=fusion.defense base_stat=base.defense stat_label="Defense"/>
        <StatDisplayCell new_stat=fusion.special_attack base_stat=base.special_attack stat_label="Special Attack"/>
        <StatDisplayCell new_stat=fusion.special_defense base_stat=base.special_defense stat_label="Special Defense"/>
        <StatDisplayCell new_stat=fusion.speed base_stat=base.speed stat_label="Speed"/>
        <StatDisplayCell new_stat=fusion.bst base_stat=base.bst stat_label="BST"/>
        </div>
        <TypeDisplayCell primary_type=fusion.primary_type secondary_type=fusion.secondary_type/>
        <AbilityDisplayCell abilities=&fusion.abilities/>
    }
}

#[component]
fn AbilityDisplayCell<'a>(abilities: &'a [(String, bool)]) -> impl IntoView {
    view! {
        <div class="flex flex-col">
            {abilities.iter().map(|(ability, is_hidden)| view!{<p>{ability}{if *is_hidden {" (Hidden)"} else {""}}</p>}).collect_view()}
        </div>
    }
}

#[component]
fn TypeDisplayCell(primary_type: Type, secondary_type: Option<Type>) -> impl IntoView {
    view! {
        <p>"Types:"</p><img src={format!("types/{}.png",primary_type)}/>{if let Some(secondary_type) = secondary_type{view! {<img src={format!("types/{}.png",secondary_type)}/>}.into_view()} else {view! {}.into_view()}}
    }
}

#[component]
fn StatDisplayCell<T>(new_stat: T, base_stat: T, stat_label: &'static str) -> impl IntoView
where
    T: num_traits::AsPrimitive<i16> + IntoView,
{
    // Cast to i16 to avoid overflow
    let difference = new_stat.as_() - base_stat.as_();
    let diff_label = match difference {
        d if d > 0 => view! {<p class="text-green-500">"+"{d}</p>},
        d if d < 0 => view! {<p class="text-red-500">{d}</p>},
        _ => view! {<p>"+0"</p>},
    };

    view! {
        <p>{stat_label}</p><p>{new_stat}</p>{diff_label}
    }
}

#[component]
fn SearchColumn(
    dex: Rc<Vec<Pokemon>>,
    fusion_select_set: WriteSignal<Option<Rc<FusedPokemon>>>,
    pokemon_select_set: WriteSignal<Option<Rc<Pokemon>>>,
    filter_type: ReadSignal<Option<Type>>,
    selected_sort: ReadSignal<Sort>,
) -> impl IntoView {
    let (fusions, fusions_set) = create_signal::<Option<Vec<Rc<FusedPokemon>>>>(None);
    // I want to read the values of dex from multiple places at once, hence Rc

    view! {
        <select on:change={
            let dex = dex.clone();
            move |event| {
            let index = event_target_value(&event).parse::<usize>().unwrap();
            pokemon_select_set.set(Some(dex[index].clone().into()));
            fusions_set.set(Some(generate_fusions(index, &dex)));
        }}>
            <option disabled selected value> "Select a Pokémon" </option>
            {dex.iter().enumerate().map(|(index,pokemon)| view! { <option value={index}>{&pokemon.name}</option> }).collect_view() }
        </select>
        {move || if let Some(fusions) = fusions.get(){
            view!{
            <div class="overflow-y-scroll max-h-full h-auto mt-4">
            { let mut fusions_to_display = fusions.into_iter().filter(|fusion|{
            let filter = {move || filter_type.get()};
            match filter() {
                Some(t) => fusion.primary_type == t || fusion.secondary_type == Some(t),
                None => true
            }}
            ).collect::<Vec<_>>();

            let sort = match {move || selected_sort.get()}() {
                Sort::None => {|_: &Rc<FusedPokemon>, _: &Rc<FusedPokemon>| std::cmp::Ordering::Equal},
                Sort::Bst => {|a: &Rc<FusedPokemon>, b: &Rc<FusedPokemon>| b.bst.cmp(&a.bst)},
                Sort::Hp => {|a: &Rc<FusedPokemon>, b: &Rc<FusedPokemon>| b.hp.cmp(&a.hp)},
                Sort::Attack => {|a: &Rc<FusedPokemon>, b: &Rc<FusedPokemon>| b.attack.cmp(&a.attack)},
                Sort::Defense => {|a: &Rc<FusedPokemon>, b: &Rc<FusedPokemon>| b.defense.cmp(&a.defense)},
                Sort::SpecialAttack => {|a: &Rc<FusedPokemon>, b: &Rc<FusedPokemon>| b.special_attack.cmp(&a.special_attack)},
                Sort::SpecialDefense => {|a: &Rc<FusedPokemon>, b: &Rc<FusedPokemon>| b.special_defense.cmp(&a.special_defense)},
                Sort::Speed => {|a: &Rc<FusedPokemon>, b: &Rc<FusedPokemon>| b.speed.cmp(&a.speed)},
            };

            fusions_to_display.sort_by(sort);

            // We have to clone twice here, once for the closure and once for the view
            // I hate web development
            fusions_to_display.iter().map(|fusion| view!{<p class="cursor-pointer" on:click={
                let fusion = fusion.clone();
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
fn generate_fusions(target: usize, dex: &[Pokemon]) -> Vec<Rc<FusedPokemon>> {
    let mut fusions: Vec<Rc<FusedPokemon>> = Vec::with_capacity(NUMBER_OF_FUSIONS);
    for (index, pokemon) in dex.iter().enumerate() {
        if index == target {
            continue;
        }
        fusions.push(Rc::new(FusedPokemon::fuse(&dex[target], pokemon)));
        fusions.push(Rc::new(FusedPokemon::fuse(pokemon, &dex[target])));
    }
    fusions.push(Rc::new(FusedPokemon::fuse(&dex[target], &dex[target])));
    fusions
}

fn map_to_alphabet(num: u8) -> &'static str {
    match num {
        0 => "",
        1 => "a",
        2 => "b",
        3 => "c",
        4 => "d",
        5 => "e",
        6 => "f",
        7 => "g",
        8 => "h",
        9 => "i",
        10 => "j",
        11 => "k",
        12 => "l",
        13 => "m",
        14 => "n",
        15 => "o",
        16 => "p",
        17 => "q",
        18 => "r",
        19 => "s",
        20 => "t",
        21 => "u",
        22 => "v",
        23 => "w",
        24 => "x",
        25 => "y",
        26 => "z",
        _ => panic!("Number too large"),
    }
}
