use fusion_datatypes::{FusedPokemon, Pokemon};
use gloo::console::log;
use leptos::*;
use std::{collections::HashMap, rc::Rc};

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let pokemon: Vec<Pokemon> =
        bincode::deserialize(include_bytes!("../infinite_dex.bin")).unwrap();
    let (selected_fusion, set_selected_fusion) = create_signal::<Option<Rc<FusedPokemon>>>(None);

    let sprite_map: HashMap<(u16, u16), u8> =
        bincode::deserialize(include_bytes!("../sprite_count.bin")).unwrap();
    view! {
        <section class="flex justify-center min-h-screen bg-slate-600">
            <div class="w-1/4 p-8 m-8 rounded-md bg-stone-500 drop-shadow-xl">"Filter column"</div>
            <div class="w-1/4 p-8 m-8 rounded-md bg-stone-500 drop-shadow-xl"><SearchColumn dex=pokemon fusion_select_set=set_selected_fusion/></div>
            <div class="w-1/4 p-8 m-8 rounded-md bg-stone-500 drop-shadow-xl"><InfoColumn selection=selected_fusion sprite_map=sprite_map/></div>
        </section>
    }
}

#[component]
fn InfoColumn(
    selection: ReadSignal<Option<Rc<FusedPokemon>>>,
    sprite_map: HashMap<(u16, u16), u8>,
) -> impl IntoView {
    let sprite_map = Rc::new(sprite_map);
    view! {
        <div>
            <div hidden={move || selection.get().is_none()}>{move || {
                if let Some(fusion) = selection.get() {
                view! {
                    <div>
                    {
                        let sprite_map = sprite_map.clone();
                        let head_id = fusion.head_id;
                        let body_id = fusion.body_id;
                        let sprite_count = sprite_map.get(&(head_id, body_id));
                        if let Some(sprite_count) = sprite_count{
                            let letters = (0..*sprite_count).map(map_to_alphabet).collect::<Vec<&str>>();
                            letters.iter().map(|letter| view!{<img src={format!("https://raw.githubusercontent.com/SneedSeedFeed/FusionFinderAssets/rust/{}.{}{}.png", head_id, body_id, letter)}/>}).collect_view()
                        }
                        else{
                            view!{"No sprites"}.into_view()
                        }
                    }
                    </div>
                    <div>
                        <h1>{&fusion.name}</h1>
                        <p>{format!("HP: {}", fusion.hp)}</p>
                        <p>{format!("Attack: {}", fusion.attack)}</p>
                        <p>{format!("Defense: {}", fusion.defense)}</p>
                        <p>{format!("Special Attack: {}", fusion.special_attack)}</p>
                        <p>{format!("Special Defense: {}", fusion.special_defense)}</p>
                        <p>{format!("Speed: {}", fusion.speed)}</p>
                        <p>{format!("Primary Type: {}", fusion.primary_type)}</p>
                    </div>
                }.into_view()
            } else {
                view! {}.into_view()
            }}}</div>
        </div>
    }
}

#[component]
fn SearchColumn(
    dex: Vec<Pokemon>,
    fusion_select_set: WriteSignal<Option<Rc<FusedPokemon>>>,
) -> impl IntoView {
    let (fusions, fusions_set) = create_signal::<Option<Vec<Rc<FusedPokemon>>>>(None);
    // I want to read the values of dex from multiple places at once, hence Rc
    let dex = Rc::new(dex);

    view! {
        <select on:change={
            let dex = dex.clone();
            move |event| {
            let index = event_target_value(&event).parse::<usize>().unwrap();
            fusions_set.set(Some(generate_fusions(index, &dex)));
        }}>
            <option disabled selected value> "Select a Pokémon" </option>
            {dex.iter().enumerate().map(|(index,pokemon)| view! { <option value={index}>{&pokemon.name}</option> }).collect_view() }
        </select>
        <div hidden={move ||fusions.get().is_none()}>{move || if let Some(fusions) = fusions.get(){
            fusions.iter().map(|fusion| view!{<p on:click={
                let cloned_fusion = fusion.clone();
                move |_| fusion_select_set.set(Some(cloned_fusion.clone()))}>{&fusion.name}</p>}).collect_view()
        } else {
            view! {}.into_view()
        } }</div>
    }
}

// Imperative style is easier than functional style for this due to the loop returning two values at once
// Doing functional style might be faster but this is fine
fn generate_fusions(target: usize, dex: &[Pokemon]) -> Vec<Rc<FusedPokemon>> {
    let mut fusions: Vec<Rc<FusedPokemon>> = Vec::with_capacity(923);
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
