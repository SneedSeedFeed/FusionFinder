use fusion_datatypes::pokemon::Ability;
use fusion_datatypes::{FusedPokemon, Pokemon, Type};
use leptos::svg::view;
use leptos::*;

use std::collections::HashMap;
use std::rc::Rc;

const IMG_BASE_URL: &str =
    "CustomBattlers/";

#[component]
pub fn InfoColumn(
    selected_fusion: ReadSignal<Option<FusedPokemon>>,
    selected_pokemon: ReadSignal<Option<Pokemon>>,
    sprite_map: Rc<HashMap<(u16, u16), u8>>,
) -> impl IntoView {
    view! {
        <div>
            <div>
                {move || {
                    if let Some(fusion) = selected_fusion.get() {
                        view! {
                            <div class="flex">
                                <SpriteDisplay fusion=&fusion sprite_map=&sprite_map/>
                            </div>
                            <div>

                                {if let Some(pokemon) = selected_pokemon.get_untracked() {
                                    view! { <StatDisplay fusion=&fusion base=&pokemon/> }
                                } else {
                                    view! {}.into_view()
                                }}

                            </div>
                        }
                            .into_view()
                    } else {
                        view! {}.into_view()
                    }
                }}
            </div>
        </div>
    }
}

#[component]
fn SpriteDisplay<'a>(
    fusion: &'a FusedPokemon,
    sprite_map: &'a HashMap<(u16, u16), u8>,
) -> impl IntoView {
    let head_id = fusion.head_id;
    let body_id = fusion.body_id;
    let sprite_count = sprite_map.get(&(head_id, body_id));

    if let Some(sprite_count) = sprite_count {
        if *sprite_count > 1 {
            view! { <ClickableImageDisplay num_sprites=*sprite_count head_id=head_id body_id=body_id/> }
        } else {
            view! { <StaticImageDisplay head_id=head_id body_id=body_id/> }
        }
    } else {
        view! { "No Custom Sprites for this Pokémon ;w;" }.into_view()
    }
}

#[component]
fn ClickableImageDisplay(num_sprites: u8, head_id: u16, body_id: u16) -> impl IntoView {
    let letters = (0..num_sprites).map(map_to_alphabet).collect::<Vec<&str>>();
    let len = letters.len();
    let (current_letter, set_current_letter) = create_signal::<usize>(0);

    let img_src = {
        move || {
            let cur_letter = letters[current_letter.get()];
            format!("{IMG_BASE_URL}{head_id}.{body_id}{cur_letter}.png")
        }
    };

    let on_click = move |_| {
        set_current_letter.set((current_letter.get() + 1) % len);
    };

    view! {
        <p>{move || format!("Sprite {}/{len}", current_letter.get() + 1)}</p>
        <img class="cursor-pointer" src=img_src on:click=on_click/>
    }
}

#[component]
fn StaticImageDisplay(head_id: u16, body_id: u16) -> impl IntoView {
    let img_src = { move || format!("{IMG_BASE_URL}{head_id}.{body_id}.png") };

    view! { <img src=img_src/> }
}

#[component]
fn StatDisplay<'a>(fusion: &'a FusedPokemon, base: &'a Pokemon) -> impl IntoView {
    view! {
        <p>{&fusion.name}</p>
        <TypeDisplayCell primary_type=fusion.primary_type secondary_type=fusion.secondary_type/>
        <div class="">
            <StatDisplayCell new_stat=fusion.hp base_stat=base.hp stat_label="HP"/>
            <StatDisplayCell new_stat=fusion.attack base_stat=base.attack stat_label="Attack"/>
            <StatDisplayCell new_stat=fusion.defense base_stat=base.defense stat_label="Defense"/>
            <StatDisplayCell
                new_stat=fusion.special_attack
                base_stat=base.special_attack
                stat_label="Special Attack"
            />
            <StatDisplayCell
                new_stat=fusion.special_defense
                base_stat=base.special_defense
                stat_label="Special Defense"
            />
            <StatDisplayCell new_stat=fusion.speed base_stat=base.speed stat_label="Speed"/>
            <StatDisplayCell new_stat=fusion.bst base_stat=base.bst stat_label="BST"/>
        </div>
        <hr/>
        <AbilityDisplayCell abilities=&fusion.abilities/>
    }
}

#[component]
fn AbilityDisplayCell<'a>(abilities: &'a [Ability]) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-y-1">
            {abilities
                .iter()
                .map(|ability| {
                    view! { <p>{&ability.0} {if ability.1 { " (Hidden)" } else { "" }}</p> }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn TypeDisplayCell(primary_type: Type, secondary_type: Option<Type>) -> impl IntoView {
    view! {
        <div class="flex">
            <img src=format!("types/{}.png", primary_type)/>
            {if let Some(secondary_type) = secondary_type {
                view! { <img src=format!("types/{}.png", secondary_type)/> }.into_view()
            } else {
                view! {}.into_view()
            }}
        </div>
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
        d if d > 0 => view! { <p class="text-green-500">"+" {d}</p> },
        d if d < 0 => view! { <p class="text-red-500">{d}</p> },
        _ => view! { <p>"+0"</p> },
    };

    view! {
        <div class="flex w-full gap-1.5">
        <p>{stat_label}</p>
        <p>{new_stat}</p>
        {diff_label}
        </div>
    }
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
        _ => panic!("Number too large, tackle this problem when there's 26 variants of one fusion"),
    }
}


