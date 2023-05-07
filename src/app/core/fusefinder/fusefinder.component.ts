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
export class FusefinderComponent implements OnInit {

  pokedex: pokemon[] = []

  fusions: fusedpokemon[] = []

  selectedPokemon!:pokemon

  ngOnInit(): void {
    console.time("Init")
    let dexData: any[] = rawDex;
    dexData.forEach(value => {
      if (value.id && value.name && value.type && value.base && this.isInGame(value.id)) {
        this.pokedex.push({ id: value.id, name: value.name.english, type: value.type, HP: value.base.HP, attack: value.base.Attack, defense: value.base.Defense, spAttack: value.base["Sp. Attack"], spDefense: value.base["Sp. Defense"], speed: value.base.Speed })
      }
    })
    console.log(this.pokedex)
    console.timeEnd("Init")
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

  updateFusions(){
    this.fusions = this.getAllFusions(this.selectedPokemon)
  }

  private getAllFusions(toFuse: pokemon): fusedpokemon[] {
    let results: fusedpokemon[] = []

    this.pokedex.forEach(val => {
      results.push(this.getFusion(toFuse, val))
    })
    this.pokedex.forEach(val => {
      results.push(this.getFusion(val, toFuse))
    })
    results.sort((a,b) => this.getBST(b) - this.getBST(a))
    return results
  }

  private getFusion(body: pokemon, head: pokemon): fusedpokemon {
    return {
      id: 0,
      name: "Body: " + body.name + " Head: " + head.name,
      type: ["Normal"],
      HP: this.calcStat(head.HP, body.HP, false),
      spDefense: this.calcStat(head.spDefense, body.spDefense, false),
      spAttack: this.calcStat(head.spAttack, body.spAttack, false),
      attack: this.calcStat(head.attack, body.attack, true),
      defense: this.calcStat(head.defense, body.defense, true),
      speed: this.calcStat(head.speed, body.speed, true),
      head: head.name,
      body: body.name
    }
  }

  private calcStat(headstat: number, bodystat: number, bodySided: boolean): number {
    if (bodySided) {
      return 2 * (bodystat / 3) + (headstat / 3)
    }
    else {
      return (bodystat / 3) + 2 * (headstat / 3)
    }
  }

  getBST(target: pokemon): number {
    return target.HP + target.attack + target.defense + target.spAttack + target.spDefense + target.speed
  }
}
