import { Component } from '@angular/core';
//@ts-ignore
import rawDex from '../../../assets/pokedex.json'
//@ts-ignore
import spriteList from '../../../assets/spritelist.json'

import { pokemon } from '../_classes/pokemon';
import { fusedpokemon } from '../_classes/fusedpokemon';
import { ability } from '../_interfaces/ability';

@Component({
  selector: 'app-fusefinder',
  templateUrl: './fusefinder.component.html',
  styleUrls: ['./fusefinder.component.scss']
})
export class FusefinderComponent {
  sliders: { display: string, stat: string, minValue: number, maxValue: number }[] = [{ display: "HP", stat: "HP", minValue: 0, maxValue: 255 },
  { display: "Atk", stat: "attack", minValue: 0, maxValue: 165 },
  { display: "Def", stat: "defense", minValue: 0, maxValue: 230 },
  { display: "Sp. Atk", stat: "spAttack", minValue: 0, maxValue: 154 },
  { display: "Sp. Def", stat: "spDefense", minValue: 0, maxValue: 230 },
  { display: "Speed", stat: "speed", minValue: 0, maxValue: 160 },
  { display: "BST", stat: "BST", minValue: 0, maxValue: 720 }]
  sliderFilter: { (val: pokemon): boolean } = (a) => {
    let meetsMin: boolean = true
    this.sliders.forEach(slider => {
      if (slider.stat) {
        if (a[slider.stat] < slider.minValue) { meetsMin = false }
      } else {
        if (a.BST < slider.minValue) { meetsMin = false }
      }
    })
    return meetsMin
  }

  readonly legendaryIDs: number[] = [144, 145, 146, 150, 151, 249, 250, 251, 243, 244, 245, 380, 381, 382, 383, 384, 385, 386, 483, 484, 486, 487, 491, 493, 643, 644, 646, 649]
  legendaryFilters: { name: string, filter: { (val: fusedpokemon): boolean } }[] = [{ name: "Include legendary/mythic pokemon", filter: () => { return true } }, {
    name: "No legendary/mythic", filter: (a) => {
      if ((this.legendaryIDs.includes(a.headID) && a.headID != this.selectedPokemon.id || this.legendaryIDs.includes(a.bodyID) && a.bodyID != this.selectedPokemon.id)) { return false }
      return true
    }
  }]
  selectedLegendaryFilter = this.legendaryFilters[0].filter

  readonly spriteDict: {[headID: string]: string[]} = spriteList
  spriteFilters: { name: string, filter: { (val: fusedpokemon): boolean } }[] = [{ name: "Any sprites", filter: () => { return true } }, {
    name: "Custom Sprites Only", filter: (a) => {
      if (this.spriteDict[a.headGameID.toString()].includes(a.bodyGameID.toString())) { return true }
      return false
    }
  }]
  selectedSpriteFilter = this.spriteFilters[0].filter

  //Our list of pokemon in the game
  readonly pokedex: pokemon[] = []
  selectedPokemon!: pokemon

  //Our list of fusions for a given selection of pokemon
  allFusions: fusedpokemon[] = []
  filteredfusions: fusedpokemon[] = []
  selectedFusion!: fusedpokemon

  //Array of our different sorts and their associated name
  readonly sortOptions: { name: string, sort: { (a: fusedpokemon, b: fusedpokemon): number } }[] = [{ name: "None", sort: () => { return 0 } },
  { name: "HP", sort: (a, b) => { return b.HP - a.HP } },
  { name: "Atk", sort: (a, b) => { return b.attack - a.attack } },
  { name: "Def", sort: (a, b) => { return b.defense - a.defense } },
  { name: "Sp. Atk", sort: (a, b) => { return b.spAttack - a.spAttack } },
  { name: "Sp. Def", sort: (a, b) => { return b.spDefense - a.spDefense } },
  { name: "Speed", sort: (a, b) => { return b.speed - a.speed } },
  { name: "BST", sort: (a, b) => { return b.BST - a.BST } }]
  selectedSort = this.sortOptions[0]

  //Array of our different type filters and their associated name
  readonly typeFilters: { name: string, filter: { (val: fusedpokemon): boolean } }[] = [{ name: "None", filter: () => { return true } },
  { name: "Normal", filter: (a) => { return a.type.includes("normal") } },
  { name: "Fighting", filter: (a) => { return a.type.includes("fighting") } },
  { name: "Flying", filter: (a) => { return a.type.includes("flying") } },
  { name: "Poison", filter: (a) => { return a.type.includes("poison") } },
  { name: "Ground", filter: (a) => { return a.type.includes("ground") } },
  { name: "Rock", filter: (a) => { return a.type.includes("rock") } },
  { name: "Bug", filter: (a) => { return a.type.includes("bug") } },
  { name: "Ghost", filter: (a) => { return a.type.includes("ghost") } },
  { name: "Steel", filter: (a) => { return a.type.includes("steel") } },
  { name: "Fire", filter: (a) => { return a.type.includes("fire") } },
  { name: "Water", filter: (a) => { return a.type.includes("water") } },
  { name: "Grass", filter: (a) => { return a.type.includes("grass") } },
  { name: "Electric", filter: (a) => { return a.type.includes("electric") } },
  { name: "Psychic", filter: (a) => { return a.type.includes("psychic") } },
  { name: "Ice", filter: (a) => { return a.type.includes("ice") } },
  { name: "Dragon", filter: (a) => { return a.type.includes("dragon") } },
  { name: "Dark", filter: (a) => { return a.type.includes("dark") } },
  { name: "Fairy", filter: (a) => { return a.type.includes("fairy") } }]
  selectedFilter = this.typeFilters[0]

  abilitySearch: string = "";
  abilitySearchFunc: { (val: pokemon): boolean } = (val) => {
    if (this.abilitySearch.trim() == "") { return true }
    let hasAbility: boolean = false;
    val.abilities.forEach(ability => {
      if (ability.name.toUpperCase().includes(this.abilitySearch.toUpperCase())) {
        hasAbility = true
      }
    })
    return hasAbility
  };

  //Load data from pokedex.json (has been manually editted to make typings up to date, stats may be slightly off)
  constructor() {
    console.time("Init")
    let dexData: any[] = rawDex;
    dexData.forEach(value => {
      if (value.id && value.name && value.type && value.base && this.isInGame(value.id)) {
        this.pokedex.push(new pokemon(value.id, value.name.english, value.type, value.base.HP, value.base.Attack, value.base.Defense, value.base["Sp. Attack"], value.base["Sp. Defense"], value.base.Speed,
          this.convertRawAbilities(value.profile.ability)))
      }
    })
    console.timeEnd("Init")
  }

  //When a user clicks a pokemon from the virtual scroller we select it and bring it up on the right hand side
  selectFusion(selected: fusedpokemon) {
    this.selectedFusion = selected
  }

  //Checks a given pokemon from our raw data is actually in the game
  private isInGame(id: number): Boolean {
    if (id < 252 || pokemon.IDList[id]) {
      return true
    }

    return false;
  }

  //Updates fusion list with new pokemon 
  update() {
    if (this.selectedPokemon) {
      this.allFusions = this.getAllFusions(this.selectedPokemon)
      this.updateFilters()
    }
  }

  //Updates the filters
  updateFilters() {
    if (this.selectedPokemon) {
      console.time("Filtered in")
      this.filteredfusions = this.allFusions.filter(this.selectedFilter.filter).filter(this.abilitySearchFunc).filter(this.selectedLegendaryFilter).filter(this.sliderFilter).filter(this.selectedSpriteFilter).sort(this.selectedSort.sort)
      console.timeEnd("Filtered in")
    }
  }

  //Gets a list of all the fusions for a given pokemon, includes their self fusion twice because I can't be bothered to write one if statement
  private getAllFusions(toFuse: pokemon): fusedpokemon[] {
    console.time("Fusions Generated")
    let results: fusedpokemon[] = []

    this.pokedex.forEach(val => {
      results.push(this.getFusion(toFuse, val))
    })
    this.pokedex.forEach(val => {
      results.push(this.getFusion(val, toFuse))
    })
    console.timeEnd("Fusions Generated")
    return results
  }

  //Fuses two pokemon and calculates the results, no we don't calculate the name because I'm lazy
  private getFusion(body: pokemon, head: pokemon): fusedpokemon {
    return new fusedpokemon(head, body)
  }

  //Pokedex stores abilities in a way I dislike so I convert it to our ability interface for our pokemon
  private convertRawAbilities(val: string[][]): ability[] {
    let abilityList: ability[] = []
    val.forEach(abilityPair => {
      abilityList.push({ name: abilityPair[0], isHidden: Boolean(JSON.parse(abilityPair[1])) })
    })
    return abilityList
  }
}
