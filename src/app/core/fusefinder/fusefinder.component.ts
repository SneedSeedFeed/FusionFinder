import { Component, OnInit } from '@angular/core';
//@ts-ignore
import rawDex from '../../../assets/pokedex.json'

import { pokemon } from '../_interfaces/pokemon';
import { fusedpokemon } from '../_interfaces/fusedpokemon';

@Component({
  selector: 'app-fusefinder',
  templateUrl: './fusefinder.component.html',
  styleUrls: ['./fusefinder.component.scss']
})
export class FusefinderComponent {

  //Manually compiled dictionary of every pokemon that has a special exception in which type it gives
  readonly typeExceptions: { [id: number]: string } = {
    1: "Grass",
    2: "Grass",
    3: "Grass",
    6: "Fire",
    74: "Rock",
    75: "Rock",
    76: "Rock",
    92: "Ghost",
    93: "Ghost",
    94: "Ghost",
    95: "Rock",
    123: "Bug",
    130: "Water",
    144: "Ice",
    145: "Electric",
    146: "Fire",
    149: "Dragon",
    208: "Steel",
    16: "Flying",
    17: "Flying",
    18: "Flying",
    21: "Flying",
    22: "Flying",
    83: "Flying",
    84: "Flying",
    85: "Flying",
    163: "Flying",
    164: "Flying",
    276: "Flying",
    277: "Flying",
    333: "Flying",
    396: "Flying",
    397: "Flying",
    398: "Flying",
    441: "Flying",
    519: "Flying",
    520: "Flying",
    521: "Flying",
    627: "Flying",
    628: "Flying",
    661: "Flying",
    731: "Flying",
    732: "Flying",
    733: "Flying",
    931: "Flying",
  }

  //Our list of pokemon in the game
  readonly pokedex: pokemon[] = []
  selectedPokemon!: pokemon

  //Our list of fusions for a given selection of pokemon
  fusions: fusedpokemon[] = []
  selectedFusion!: fusedpokemon

  //Array of our different sorts and their associated name
  readonly sortOptions: { name: string, sort: { (a: fusedpokemon, b: fusedpokemon): number } }[] = [{ name: "None", sort: () => { return 0 } },
  { name: "HP", sort: (a, b) => { return b.HP - a.HP } },
  { name: "Atk", sort: (a, b) => { return b.attack - a.attack } },
  { name: "Def", sort: (a, b) => { return b.defense - a.defense } },
  { name: "Sp. Atk", sort: (a, b) => { return b.spAttack - a.spAttack } },
  { name: "Sp. Def", sort: (a, b) => { return b.spDefense - a.spDefense } },
  { name: "Speed", sort: (a, b) => { return b.speed - a.speed } },
  { name: "BST", sort: (a, b) => { return this.getBST(b) - this.getBST(a) } }]
  selectedSort = this.sortOptions[0]

  //Array of our different type filters and their associated name
  readonly typeFilters: { name: string, filter: { (val: fusedpokemon): boolean } }[] = [{ name: "None", filter: () => { return true } },
  { name: "Normal", filter: (a) => { return a.type.includes("Normal") } },
  { name: "Fighting", filter: (a) => { return a.type.includes("Fighting") } },
  { name: "Flying", filter: (a) => { return a.type.includes("Flying") } },
  { name: "Poison", filter: (a) => { return a.type.includes("Poison") } },
  { name: "Ground", filter: (a) => { return a.type.includes("Ground") } },
  { name: "Rock", filter: (a) => { return a.type.includes("Rock") } },
  { name: "Bug", filter: (a) => { return a.type.includes("Bug") } },
  { name: "Ghost", filter: (a) => { return a.type.includes("Ghost") } },
  { name: "Steel", filter: (a) => { return a.type.includes("Steel") } },
  { name: "Fire", filter: (a) => { return a.type.includes("Fire") } },
  { name: "Water", filter: (a) => { return a.type.includes("Water") } },
  { name: "Grass", filter: (a) => { return a.type.includes("Grass") } },
  { name: "Electric", filter: (a) => { return a.type.includes("Electric") } },
  { name: "Psychic", filter: (a) => { return a.type.includes("Psychic") } },
  { name: "Ice", filter: (a) => { return a.type.includes("Ice") } },
  { name: "Dragon", filter: (a) => { return a.type.includes("Dragon") } },
  { name: "Dark", filter: (a) => { return a.type.includes("Dark") } },
  { name: "Fairy", filter: (a) => { return a.type.includes("Fairy") } }]
  selectedFilter = this.typeFilters[0]

  //Load data from pokedex.json (has been manually editted to make typings up to date, stats may be slightly off)
  constructor() {
    console.time("Init")
    let dexData: any[] = rawDex;
    dexData.forEach(value => {
      if (value.id && value.name && value.type && value.base && this.isInGame(value.id)) {
        this.pokedex.push({
          id: value.id,
          name: value.name.english,
          type: value.type,
          HP: value.base.HP,
          attack: value.base.Attack,
          defense: value.base.Defense,
          spAttack: value.base["Sp. Attack"],
          spDefense: value.base["Sp. Defense"],
          speed: value.base.Speed,
        })
      }
    })
    console.timeEnd("Init")
  }

  //When a user clicks a pokemon from the virtual scroller we select it and bring it up on the right hand side
  selectFusion(selected: fusedpokemon) {
    this.selectedFusion = selected
  }

  private isInGame(id: number): Boolean {
    let otherValids: number[] = [298, 360, 424, 429, 430, 438, 439, 440, 446, 458, 461, 462, 463, 464, 465, 466, 467, 468, 469, 470, 471, 472, 473, 474, 475, 476, 477, 280, 281, 282, 292, 352, 374, 375, 376, 399, 442, 448, 443, 444, 445, 303, 345, 346, 347, 348, 408, 409, 410, 411,
      289, 359, 355, 356, 321, 493, 387, 388, 389, 390, 391, 392, 393, 394, 395, 299, 679, 680, 681, 624, 625, 405, 306, 330, 350, 373, 601, 571, 700, 382, 383, 384, 483, 484, 487, 486, 491, 649, 643, 644, 646, 407, 426, 428, 286, 291, 354, 479, 579, 547, 553, 563, 596, 598, 607, 608, 609, 612,
      623, 771, 707, 663, 778, 637, 633, 634, 635, 380, 381, 385, 386, 290, 400, 447, 287, 288, 320, 403, 404, 304, 305, 328, 329, 349, 371, 372, 599, 600, 570, 406, 315, 425, 427, 285, 353, 577, 578, 546, 551, 552, 562, 595, 597, 610, 611, 622, 661, 662, 636, 618]

    if (id < 261) { return true }

    if (otherValids.includes(id)) {

      return true
    }

    return false;
  }

  //Updates and filters/sorts the fusion list 
  update() {
    if (this.selectedPokemon) {
      this.fusions = this.getAllFusions(this.selectedPokemon).filter(this.selectedFilter.filter).sort(this.selectedSort.sort)
    }
  }

  //Gets a list of all the fusions for a given pokemon, includes their self fusion twice because I can't be bothered to write one if statement
  private getAllFusions(toFuse: pokemon): fusedpokemon[] {
    let results: fusedpokemon[] = []

    this.pokedex.forEach(val => {
      results.push(this.getFusion(toFuse, val))
    })
    this.pokedex.forEach(val => {
      results.push(this.getFusion(val, toFuse))
    })
    return results
  }

  //Fuses two pokemon and calculates the results, no we don't calculate the name because I'm lazy
  private getFusion(body: pokemon, head: pokemon): fusedpokemon {
    return {
      id: 0,
      name: "Body: " + body.name + " | Head: " + head.name,
      type: this.calcType(body, head),
      HP: this.calcStat(head.HP, body.HP, false),
      spDefense: this.calcStat(head.spDefense, body.spDefense, false),
      spAttack: this.calcStat(head.spAttack, body.spAttack, false),
      attack: this.calcStat(head.attack, body.attack, true),
      defense: this.calcStat(head.defense, body.defense, true),
      speed: this.calcStat(head.speed, body.speed, true),
      head: head.name,
      body: body.name,
    }
  }

  //Calculates a stat from two pokemon, switch whether it's body or head sided with the boolean
  private calcStat(headstat: number, bodystat: number, bodySided: boolean): number {
    if (bodySided) {
      return Math.floor(2 * (bodystat / 3) + (headstat / 3))
    }
    else {
      return Math.floor((bodystat / 3) + 2 * (headstat / 3))
    }
  }

  //Works out the type. 
  private calcType(body: pokemon, head: pokemon): string[] {
    //First we set it to the head's primary and the body's last type (secondary if there is one)
    let bodyProvided = body.type[body.type.length - 1]
    let headProvided = head.type[0]

    //If the head and body have the same type, and the body has a second type available we use that as the secondary type instead
    if (headProvided == bodyProvided && body.type.length == 2) {
      bodyProvided = body.type[0]
    }

    //Apply the special exceptions for each pokemon
    if (body.id in this.typeExceptions) {
      bodyProvided = this.typeExceptions[body.id]
    }
    if (head.id in this.typeExceptions) {
      headProvided = this.typeExceptions[head.id]
    }
    return [headProvided, bodyProvided]
  }

  //Gets BST for a given pokemon (or fusion since it inherits from Pokemon)
  getBST(target: pokemon): number {
    return target.HP + target.attack + target.defense + target.spAttack + target.spDefense + target.speed
  }
}
