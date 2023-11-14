use fusion_datatypes::{FusedPokemon, Pokemon, Type};
use fusion_finder_macros::stat_change;
use gloo::console::log;
use indexmap::IndexSet;
use leptos::{svg::view, *};
use serde::de::value;
use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};

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

impl Sort {
    pub fn sort_fusions(&self, a: &FusedPokemon, b: &FusedPokemon) -> std::cmp::Ordering {
        match self {
            Sort::None => std::cmp::Ordering::Equal,
            Sort::Bst => b.bst.cmp(&a.bst),
            Sort::Hp => b.hp.cmp(&a.hp),
            Sort::Attack => b.attack.cmp(&a.attack),
            Sort::Defense => b.defense.cmp(&a.defense),
            Sort::SpecialAttack => b.special_attack.cmp(&a.special_attack),
            Sort::SpecialDefense => b.special_defense.cmp(&a.special_defense),
            Sort::Speed => b.speed.cmp(&a.speed),
        }
    }
}

impl Display for Sort {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Sort::None => write!(f, "None"),
            Sort::Bst => write!(f, "BST"),
            Sort::Hp => write!(f, "HP"),
            Sort::Attack => write!(f, "Attack"),
            Sort::Defense => write!(f, "Defense"),
            Sort::SpecialAttack => write!(f, "Special Attack"),
            Sort::SpecialDefense => write!(f, "Special Defense"),
            Sort::Speed => write!(f, "Speed"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Filter {
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
impl Filter {
    pub fn filter_stats(&self, val: &FusedPokemon) -> bool {
        if let Some(filter_type) = self.filter_type {
            if val.primary_type != filter_type && val.secondary_type != Some(filter_type) {
                return false;
            }
        }
        if val.hp < self.min_hp {
            return false;
        }
        if val.attack < self.min_atk {
            return false;
        }
        if val.defense < self.min_def {
            return false;
        }
        if val.special_attack < self.min_spatk {
            return false;
        }
        if val.special_defense < self.min_spdef {
            return false;
        }
        if val.speed < self.min_spd {
            return false;
        }
        if val.bst < self.min_bst {
            return false;
        }
        true
    }
}

#[component]
pub fn FilterColumn(
    set_filters: WriteSignal<Filter>,
    set_sort: WriteSignal<Sort>,
    ability_map: Rc<IndexSet<&'static str>>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <SortSelect set_sort=set_sort/>
            <TypeSelect set_filters=set_filters/>
            <AbilitySelect set_filters=set_filters ability_map=ability_map.clone()/>
            <StatSlider for_stat=Stat::Hp set_filters=set_filters/>
            <StatSlider for_stat=Stat::Atk set_filters=set_filters/>
            <StatSlider for_stat=Stat::Def set_filters=set_filters/>
            <StatSlider for_stat=Stat::SpAtk set_filters=set_filters/>
            <StatSlider for_stat=Stat::SpDef set_filters=set_filters/>
            <StatSlider for_stat=Stat::Spd set_filters=set_filters/>
            <StatSlider for_stat=Stat::Bst set_filters=set_filters/>
            <CustomSpriteToggle set_filters=set_filters/>
        </div>
    }
}

#[component]
fn CustomSpriteToggle(set_filters: WriteSignal<Filter>) -> impl IntoView {
    let custom_sprite_on_change = {
        move |event| {
            set_filters.update(|filters| filters.custom_sprite = event_target_checked(&event))
        }
    };

    view! {
        <div>
            <label for="has_sprite">"Has Custom Sprite: "</label>
            <input id="has_sprite" on:change=custom_sprite_on_change type="checkbox"/>
        </div>
    }
}

#[component]
fn AbilitySelect(
    set_filters: WriteSignal<Filter>,
    ability_map: Rc<IndexSet<&'static str>>,
) -> impl IntoView {
    let ability_on_change = {
        move |event| {
            let index = event_target_value(&event).parse::<usize>().unwrap();
            set_filters.update(|filters| {
                filters.has_ability = if index == 0 { None } else { Some(index - 1) }
            });
        }
    };

    let ability_view = ability_map
        .iter()
        .enumerate()
        .map(|(index, option)| view! { <option value=index + 1>{*option}</option> })
        .collect_view();

    view! {
        <div>
            <label for="ability">"Has Ability: "</label>
            <select id="ability" on:change=ability_on_change>
                <option value="0">"None"</option>
                {ability_view}
            </select>
        </div>
    }
}

#[derive(Clone, Copy)]
enum Stat {
    Hp,
    Atk,
    Def,
    SpAtk,
    SpDef,
    Spd,
    Bst,
}

#[component]
fn StatSlider(for_stat: Stat, set_filters: WriteSignal<Filter>) -> impl IntoView {
    let (value, set_value) = create_signal::<u16>(0);

    let slider_on_input = {
        move |event| {
            let value = event_target_value(&event).parse::<u16>().unwrap();
            set_value.set(value);
        }
    };

    let number_on_input = {
        move |event| {
            let value = event_target_value(&event).parse::<u16>().unwrap();
            set_value.set(value);
        }
    };

    create_effect(move |_| {
        let value = value.get();
        match for_stat {
            Stat::Hp => set_filters.update(|filters| filters.min_hp = value as u8),
            Stat::Atk => set_filters.update(|filters| filters.min_atk = value as u8),
            Stat::Def => set_filters.update(|filters| filters.min_def = value as u8),
            Stat::SpAtk => set_filters.update(|filters| filters.min_spatk = value as u8),
            Stat::SpDef => set_filters.update(|filters| filters.min_spdef = value as u8),
            Stat::Spd => set_filters.update(|filters| filters.min_spd = value as u8),
            Stat::Bst => set_filters.update(|filters| filters.min_bst = value),
        }
    });

    const STATMAX: &str = "255";
    let (id, label, max) = match for_stat {
        Stat::Hp => ("min_hp", "Min HP", STATMAX),
        Stat::Atk => ("min_atk", "Min Attack", STATMAX),
        Stat::Def => ("min_def", "Min Defense", STATMAX),
        Stat::SpAtk => ("min_spatk", "Min Special Attack", STATMAX),
        Stat::SpDef => ("min_spdef", "Min Special Defense", STATMAX),
        Stat::Spd => ("min_spd", "Min Speed", STATMAX),
        Stat::Bst => ("min_bst", "Min BST", "720"),
    };

    view! {
        <div>
            <label for=id>{label}</label>
            <div id=id>
                <input
                    type="number"
                    min="0"
                    max=max
                    on:input=number_on_input
                    prop:value=move || value.get()
                />
                <input
                    type="range"
                    min="0"
                    max=max
                    prop:value=move || value.get()
                    on:input=slider_on_input
                />
            </div>
        </div>
    }
}

#[component]
fn SortSelect(set_sort: WriteSignal<Sort>) -> impl IntoView {
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

    let sorts_on_change = {
        let sorts = sorts.clone();
        move |event| {
            let index = event_target_value(&event).parse::<usize>().unwrap();
            set_sort.set(sorts[index]);
        }
    };

    let sorts_view = sorts
        .iter()
        .enumerate()
        .map(|(index, sort)| view! { <option value=index>{sort.to_string()}</option> })
        .collect_view();

    view! {
        <div>
            <label for="sort">"Sort By: "</label>
            <select id="sort" on:change=sorts_on_change>
                {sorts_view}
            </select>
        </div>
    }
}

#[component]
fn TypeSelect(set_filters: WriteSignal<Filter>) -> impl IntoView {
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

    let type_select_on_change = {
        let types = types.clone();
        move |event| {
            let index = event_target_value(&event).parse::<usize>().unwrap();
            set_filters.update(|filters| filters.filter_type = types[index]);
        }
    };

    let types_view = {
        types
            .iter()
            .enumerate()
            .map(|(index, option)| {
                view! {
                    <option value=index>
                        {match option {
                            Some(t) => format!("{}", t),
                            None => "None".to_string(),
                        }}

                    </option>
                }
            })
            .collect_view()
    };

    view! {
        <div>
            <label for="type">"Has Type: "</label>
            <select on:change=type_select_on_change>{types_view}</select>
        </div>
    }
}
