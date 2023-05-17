import { pokemon } from "./pokemon";

export class fusedpokemon extends pokemon {
    body: string;
    head: string;
    bodyID: number;
    headID: number;
    headGameID: number;
    bodyGameID: number;
    imgName: string | null;

    //Manually compiled dictionary of every pokemon that has a special exception in which type it gives
    //These IDs are the NATDEX ID, NOT INFINITE FUSION DEX
    static readonly typeExceptions: { [id: number]: string } = {
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


    constructor(head: pokemon, body: pokemon) {
        super(0, "Body: " + body.name + " | Head: " + head.name, fusedpokemon.calcType(body, head), fusedpokemon.calcStat(head.HP, body.HP, false), fusedpokemon.calcStat(head.attack, body.attack, true),
            fusedpokemon.calcStat(head.defense, body.defense, true), fusedpokemon.calcStat(head.spAttack, body.spAttack, false), fusedpokemon.calcStat(head.spDefense, body.spDefense, false),
            fusedpokemon.calcStat(head.speed, body.speed, true), head.abilities.concat(body.abilities));

        this.body = body.name
        this.head = head.name
        this.headID = head.id
        this.bodyID = body.id
        this.headGameID = head.newdexID
        this.bodyGameID = body.newdexID
        this.imgName = "https://raw.githubusercontent.com/SneedSeedFeed/FusionFinderAssets/main/CustomBattlers/" + head.newdexID + "." + body.newdexID + ".png"
    }

    //Works out the type.
    private static calcType(body: pokemon, head: pokemon): string[] {
        //First we set it to the head's primary and the body's last type (secondary if there is one)
        let bodyProvided = body.type[body.type.length - 1]
        let headProvided = head.type[0]

        //If the head and body have the same type, and the body has a second type available we use that as the secondary type instead
        if (headProvided == bodyProvided && body.type.length == 2) {
            bodyProvided = body.type[0]
        }

        //Apply the special exceptions for each pokemon
        if (body.id in fusedpokemon.typeExceptions) {
            bodyProvided = fusedpokemon.typeExceptions[body.id]
        }
        if (head.id in fusedpokemon.typeExceptions) {
            headProvided = fusedpokemon.typeExceptions[head.id]
        }
        if(headProvided == bodyProvided){return [headProvided]}

        return [headProvided, bodyProvided]
    }

    //Calculates a stat from two pokemon, switch whether it's body or head sided with the boolean
    private static calcStat(headstat: number, bodystat: number, bodySided: boolean): number {
        if (bodySided) {
            return Math.floor(2 * (bodystat / 3) + (headstat / 3))
        }
        else {
            return Math.floor((bodystat / 3) + 2 * (headstat / 3))
        }
    }

}