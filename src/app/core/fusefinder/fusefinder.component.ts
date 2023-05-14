import { Component, OnInit } from '@angular/core';
//@ts-ignore
import rawDex from '../../../assets/pokedex.json'

import { pokemon } from '../_interfaces/pokemon';
import { fusedpokemon } from '../_interfaces/fusedpokemon';
import { ability } from '../_interfaces/ability';

@Component({
  selector: 'app-fusefinder',
  templateUrl: './fusefinder.component.html',
  styleUrls: ['./fusefinder.component.scss']
})
export class FusefinderComponent {
  //For non gen 1-2, this is a dictionary/map of natdex to infinite fusion dex numbers
  readonly IDList: { [natdex: number]: number } = {
    298: 252, 360: 253, 424: 254, 429: 255, 430: 256, 438: 257, 439: 258, 440: 259,
    446: 260, 458: 261, 461: 262, 462: 263, 463: 264, 464: 265, 465: 266, 466: 267, 467: 269, 468: 269,
    469: 270, 470: 271, 471: 272, 472: 273, 473: 274, 474: 275, 252: 276, 253: 277, 254: 278, 255: 279,
    256: 280, 257: 281, 258: 282, 259: 283, 260: 284, 280: 285, 281: 286, 282: 287, 475: 288, 292: 289,
    352: 290, 374: 291, 375: 292, 376: 293, 399: 294, 442: 295, 448: 296, 443: 297, 444: 298, 445: 299,
    303: 300, 345: 301, 346: 302, 347: 303, 348: 304, 408: 305, 409: 306, 410: 307, 411: 308, 289: 309,
    359: 310, 355: 311, 356: 312, 477: 313, 321: 314, 493: 315, 387: 316, 388: 317, 389: 318, 390: 319,
    391: 320, 392: 321, 393: 322, 394: 323, 395: 324, 299: 325, 476: 326, 679: 327, 680: 328, 681: 329,
    624: 330, 625: 331, 405: 332, 306: 333, 330: 334, 350: 335, 373: 336, 601: 337, 571: 338, 700: 339,
    382: 340, 383: 341, 384: 342, 483: 343, 484: 344, 487: 345, 486: 346, 491: 347, 649: 348, 643: 349,
    644: 350, 646: 351, 407: 352, 426: 353, 428: 354, 286: 355, 291: 356, 354: 357, 479: 358, 579: 359,
    547: 360, 553: 361, 563: 362, 596: 363, 598: 364, 607: 365, 608: 366, 609: 367, 612: 368, 623: 369,
    771: 370, 707: 371, 663: 372, 778: 373, 637: 374, 633: 375, 634: 376, 635: 377, 380: 378, 381: 379,
    385: 380, 386: 381, 290: 382, 400: 383, 447: 384, 287: 385, 288: 386, 320: 387, 403: 388, 404: 389,
    304: 390, 305: 391, 328: 392, 329: 393, 349: 394, 371: 395, 372: 396, 599: 397, 600: 398, 570: 399,
    406: 400, 315: 401, 425: 402, 427: 403, 285: 404, 353: 405, 577: 406, 578: 407, 546: 408, 551: 409,
    552: 410, 562: 411, 595: 412, 597: 413, 610: 414, 611: 415, 622: 416, 661: 417, 662: 418, 636: 419,
    618: 420
  }



  readonly legendaryIDs: number[] = [144, 145, 146, 150, 151, 249, 250, 251, 243, 244, 245, 380, 381, 382, 383, 384, 385, 386, 483, 484, 486, 487, 491, 493, 643, 644, 646, 649]
  legendaryFilters: { name: string, filter: { (val: fusedpokemon): boolean } }[] = [{ name: "Include legendary/mythic pokemon", filter: () => { return true } }, {
    name: "No legendary/mythic", filter: (a) => {
      if (this.legendaryIDs.includes(a.headID) || this.legendaryIDs.includes(a.bodyID)) { return false }
      return true
    }
  }]
  selectedLegendaryFilter = this.legendaryFilters[0].filter

  //Manually compiled dictionary of every pokemon that has a special exception in which type it gives
  //These IDs are the NATDEX ID, NOT INFINITE FUSION DEX
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

  abilitySearch: string = "";
  abilitySearchFunc: { (val: pokemon): boolean } = () => { return true };

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
          abilities: this.convertRawAbilities(value.profile.ability),
          newdexID: this.getNewDexID(value.id)
        })
      }
    })
    console.timeEnd("Init")
  }

  //Pokedex stores abilities in a way I dislike so I convert it to our ability interface for our pokemon
  private convertRawAbilities(val: string[][]): ability[] {
    let abilityList: ability[] = []
    val.forEach(abilityPair => {
      abilityList.push({ name: abilityPair[0], isHidden: Boolean(JSON.parse(abilityPair[1])) })
    })
    return abilityList
  }

  //When a user clicks a pokemon from the virtual scroller we select it and bring it up on the right hand side
  selectFusion(selected: fusedpokemon) {
    this.selectedFusion = selected
  }

  //Checks a given pokemon from our raw data is actually in the game
  private isInGame(id: number): Boolean {
    if (id < 252 || this.IDList[id]) {
      return true
    }

    return false;
  }

  keyUp() {
    //Function called in filter can't access variables in the component. So every time the user inputs a key we create a new function. Cludgy but fuck it it works
    this.abilitySearchFunc = (val) => {
      if (this.abilitySearch.trim() == "") { return true }
      let hasAbility: boolean = false;
      val.abilities.forEach(ability => {
        if (ability.name.toUpperCase().includes(this.abilitySearch.toUpperCase())) {
          hasAbility = true
        }
      })
      return hasAbility
    }

    this.update()
  }

  //Updates and filters/sorts the fusion list 
  update() {
    if (this.selectedPokemon) {
      this.fusions = this.getAllFusions(this.selectedPokemon).filter(this.selectedFilter.filter).filter(this.abilitySearchFunc).filter(this.selectedLegendaryFilter).sort(this.selectedSort.sort)
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
      headID: head.id,
      headGameID: head.newdexID,
      bodyGameID: body.newdexID,
      body: body.name,
      bodyID: body.id,
      abilities: head.abilities.concat(body.abilities),
      newdexID: 0,
      imgName: this.getImgLoc(head.newdexID, body.newdexID)
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

  private getImgLoc(headID: number, bodyID: number): string | null {
    let address = "https://raw.githubusercontent.com/SneedSeedFeed/FusionFinderAssets/main/CustomBattlers/" + headID + "." + bodyID + ".png"

    return address
  }

  //Gets BST for a given pokemon (or fusion since it inherits from Pokemon)
  getBST(target: pokemon): number {
    return target.HP + target.attack + target.defense + target.spAttack + target.spDefense + target.speed
  }

  private getNewDexID(natDexID: number): number {
    if (natDexID <= 251) {
      return natDexID
    }
    if (this.IDList[natDexID]) {
      return this.IDList[natDexID]
    }

    return natDexID
  }
}
