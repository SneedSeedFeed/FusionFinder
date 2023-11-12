use fusion_datatypes::{FusedPokemon, Pokemon, Type};
use indexmap::IndexSet;
use leptos::*;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub enum Sort {
    None,
    Bst,
    Hp,
    Attack,
    Defense,
    SpecialAttack,
    SpecialDefense,
    Speed,
}
#[derive(Debug, Clone, Copy, Default)]
pub struct Filters {
    pub filter_type: Option<Type>,
    pub custom_sprite: bool,
    pub has_ability: Option<usize>,
    pub min_hp: u8,
    pub min_atk: u8,
    pub min_def: u8,
    pub min_spatk: u8,
    pub min_spdef: u8,
    pub min_spd: u8,
    pub min_bst: u16,
}

#[component]
pub fn FilterColumn(
    set_filters: WriteSignal<Filters>,
    set_sort: WriteSignal<Sort>,
    ability_map: Rc<IndexSet<&'static str>>,
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
        <div class="flex flex-col">
            <select on:change={
                let types = types.clone();
                move |event| {
                    let index = event_target_value(&event).parse::<usize>().unwrap();
                    set_filters.update(|filters| filters.filter_type = types[index]);
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

           <select on:change={
                move |event| {
                    let index = event_target_value(&event).parse::<usize>().unwrap();
                    set_filters.update(|filters| filters.has_ability = if index == 0 {None} else {Some(index-1)});
                }
            }>
                <option value="0">"No Ability Filter"</option>
                {
                    move || ability_map.iter().enumerate().map(|(index,option)| view! { <option value={index+1}>{*option}</option> }).collect_view()
                }
            </select>

            <label for="min_hp">"Min HP"</label>
            <input id="min_hp" type="range" min="0" max="255" value="0" on:input={move |event|{
                let value = event_target_value(&event).parse::<u8>().unwrap();
                set_filters.update(|filters| filters.min_hp = value);}
           }/>
            <input type="range" min="0" max="255" value="0" on:input={move |event|{
                let value = event_target_value(&event).parse::<u8>().unwrap();
                set_filters.update(|filters| filters.min_atk = value);}
           }/>
            <input type="range" min="0" max="255" value="0" on:input={move |event|{
                let value = event_target_value(&event).parse::<u8>().unwrap();
                set_filters.update(|filters| filters.min_def = value);}
            }/>
            <input type="range" min="0" max="255" value="0" on:input={move |event|{
                    let value = event_target_value(&event).parse::<u8>().unwrap();
                    set_filters.update(|filters| filters.min_spatk = value);}
            }/>
            <input type="range" min="0" max="255" value="0" on:input={move |event|{
                    let value = event_target_value(&event).parse::<u8>().unwrap();
                    set_filters.update(|filters| filters.min_spdef = value);}
            }/>
            <input type="range" min="0" max="255" value="0" on:input={move |event|{
                    let value = event_target_value(&event).parse::<u8>().unwrap();
                    set_filters.update(|filters| filters.min_spd = value);}
            }/>
            <input type="range" min="0" max="255" value="0" on:input={move |event|{
                    let value = event_target_value(&event).parse::<u8>().unwrap();
                    set_filters.update(|filters| filters.min_bst = value as u16);}
            }/>
            <input on:change=move |event| set_filters.update(|filters| filters.custom_sprite= event_target_checked(&event)) type="checkbox"/>
        </div>
    }
}
